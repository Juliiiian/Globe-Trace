use std::net::IpAddr;
use std::thread;
use tauri::{AppHandle, Emitter};
use tracert::trace::Tracer;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn trace(app: AppHandle, ip: String, hops: u8) {
    eprintln!("Starting trace command with IP: {} and hops: {}", ip, hops);

    let dst_ip: IpAddr = match ip.parse() {
        Ok(ip) => ip,
        Err(_) => {
            eprintln!("Invalid IP address: {}", ip);
            app.emit("trace_fail", "Invalid IP address")
                .expect("Failed to emit trace_fail event");
            return;
        }
    };

    if dst_ip.is_unspecified() {
        eprintln!("Unspecified or invalid IP address: {}", dst_ip);
        app.emit("trace_fail", "Invalid or unspecified IP address")
            .expect("Failed to emit trace_fail event");
        return;
    }

    eprintln!("Creating tracer for IP: {}", dst_ip);
    let mut tracer = match Tracer::new(dst_ip) {
        Ok(tracer) => tracer,
        Err(_) => {
            eprintln!("Failed to create tracer for IP: {}", dst_ip);
            app.emit("trace_fail", "Failed to create tracer")
                .expect("Failed to emit trace_fail event");
            return;
        }
    };

    tracer.set_max_hop(hops);
    tracer.set_send_rate(std::time::Duration::from_secs(2)); // Increase timeout to 2 seconds
    tracer.set_receive_timeout(std::time::Duration::from_secs(15)); // Increase timeout to 2 seconds
    tracer.set_trace_timeout(std::time::Duration::from_secs(60)); // Increase timeout to 2 seconds

    let rx = tracer.get_progress_receiver();

    // Spawn a single thread to handle the traceroute and progress updates
    thread::spawn(move || {
        eprintln!("Traceroute thread started");

        // Spawn another thread to run the traceroute
        let trace_handle = thread::spawn(move || tracer.trace());

        // Emit progress for each hop
        while let Ok(msg) = rx.lock().unwrap().recv() {
            eprintln!(
                "Received hop: Seq: {}, IP: {}, Hop: {:?}, RTT: {:?}",
                msg.seq, msg.ip_addr, msg.hop, msg.rtt
            );

            if let Err(e) = app.emit(
                "hop",
                format!(
                    "Seq: {}, IP: {}, Hop: {:?}, RTT: {:?}",
                    msg.seq, msg.ip_addr, msg.hop, msg.rtt
                ),
            ) {
                eprintln!("Failed to emit hop event: {}", e);
            }
        }

        // Wait for the traceroute to complete
        match trace_handle.join() {
            Ok(Ok(result)) => {
                eprintln!("Traceroute completed successfully");

                // Emit the final result
                if let Err(e) = app.emit("trace_complete", format!("Trace completed: {:?}", result))
                {
                    eprintln!("Failed to emit trace_complete event: {}", e);
                }
            }
            Ok(Err(e)) => {
                eprintln!("Traceroute failed with error: {}", e);

                // Emit an error if the traceroute fails
                if let Err(err) = app.emit("trace_error", format!("Trace error: {}", e)) {
                    eprintln!("Failed to emit trace_error event: {}", err);
                }
            }
            Err(e) => {
                eprintln!("Traceroute thread panicked: {:?}", e);
            }
        }

        // Emit finish_trace event
        if let Err(e) = app.emit("finish_trace", "Trace process finished") {
            eprintln!("Failed to emit finish_trace event: {}", e);
        }

        eprintln!("Traceroute thread finished");
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![trace])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

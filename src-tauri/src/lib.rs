use serde_json::json;
use std::net::IpAddr;
use std::net::ToSocketAddrs;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use tauri::State;
use tauri::{AppHandle, Emitter};
use tracert::trace::Tracer;

struct AppState {
    cancel_flag: Arc<AtomicBool>,
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn trace(app: AppHandle, ip: String, hops: u8, state: State<AppState>) {
    eprintln!("Starting trace command with IP: {} and hops: {}", ip, hops);

    state.cancel_flag.store(false, Ordering::Relaxed);

    let dst_ip: IpAddr = match ip.parse() {
        Ok(ip) => ip,
        Err(_) => {
            // Attempt to resolve the input as a domain name
            match ip.to_socket_addrs() {
                Ok(mut addrs) => {
                    if let Some(addr) = addrs.next() {
                        app.emit("trace_dns", addr.ip())
                            .expect("Failed to emit trace_dns event");
                        addr.ip()
                    } else {
                        app.emit("trace_fail", "No address found for domain")
                            .expect("Failed to emit trace_fail event");
                        return;
                    }
                }
                Err(_) => {
                    app.emit("trace_fail", "Invalid IP address or domain")
                        .expect("Failed to emit trace_fail event");
                    return;
                }
            }
        }
    };

    if dst_ip.is_unspecified() {
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
    tracer.set_send_rate(std::time::Duration::from_secs(2));
    tracer.set_receive_timeout(std::time::Duration::from_secs(15));
    tracer.set_trace_timeout(std::time::Duration::from_secs(60));

    let rx = tracer.get_progress_receiver();
    let cancel_flag = Arc::clone(&state.cancel_flag);

    // Spawn a single thread to handle the traceroute and progress updates
    thread::spawn(move || {
        eprintln!("Traceroute thread started");

        let trace_handle = thread::spawn(move || tracer.trace());

        while let Ok(msg) = rx.lock().unwrap().recv() {
            if cancel_flag.load(Ordering::Relaxed) {
                eprintln!("Trace cancelled by user.");
                break;
            }

            eprintln!(
                "Received hop: Seq: {}, IP: {}, Hop: {:?}, RTT: {:?}",
                msg.seq, msg.ip_addr, msg.hop, msg.rtt
            );

            let hop_data = json!({
                "seq": msg.seq,
                "host_name": msg.seq,
                "ip_addr": msg.ip_addr.to_string(),
                "hop": msg.hop,
                "ttl": msg.ttl,
                "node_type": format!("{:?}", msg.node_type),
                "rtt": msg.rtt.as_millis(),
            });

            if let Err(e) = app.emit("hop", hop_data) {
                eprintln!("Failed to emit hop event: {}", e);
            }
        }

        if !cancel_flag.load(Ordering::Relaxed) {
            match trace_handle.join() {
                Ok(Ok(result)) => {
                    eprintln!("Traceroute completed successfully");

                    if let Err(e) = app.emit(
                        "trace_complete",
                        json!({
                            "status": format!("{:?}", result.status),
                            "probe_time": result.probe_time.as_millis(),
                        }),
                    ) {
                        eprintln!("Failed to emit trace_complete event: {}", e);
                    }
                }
                Ok(Err(e)) => {
                    eprintln!("Traceroute failed with error: {}", e);

                    if let Err(err) = app.emit("trace_error", format!("Trace error: {}", e)) {
                        eprintln!("Failed to emit trace_error event: {}", err);
                    }
                }
                Err(e) => {
                    eprintln!("Traceroute thread panicked: {:?}", e);
                }
            }
        } else {
            eprintln!("Traceroute cancelled by user.");
            if let Err(e) = app.emit("trace_cancelled", "Trace cancelled") {
                eprintln!("Failed to emit trace_cancelled event: {}", e);
            }
        }

        eprintln!("Traceroute thread finished");
    });
}

#[tauri::command]
fn cancel_trace(state: State<AppState>) {
    state.cancel_flag.store(true, Ordering::Relaxed);
    eprintln!("Trace cancellation requested.");
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState {
            cancel_flag: Arc::new(AtomicBool::new(false)),
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![trace, cancel_trace])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

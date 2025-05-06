export type Geo = {
	city: string;
	country: string;
	ip: string;
	latitude: number;
	longitude: number;
	state: string;
};

export type Hop = HopEvent & {
	geo: Geo | null;
};

export type Trace = {
	id: string;
	name: string;
	dest_ip: string;
	max_hops: number;
	send_rate: number;
	receive_timeout: number;
	trace_timeout: number;
	is_finished: boolean;
	hops: Hop[];
};

export type HopEvent = {
	seq: number; // Sequence number of the hop
	host_name: string; // Host name of the hop,
	ip_addr: string; // IP address of the hop
	hop: number | null; // Hop number (or null if unavailable)
	ttl: number; // Time-to-live value
	node_type: string; // Node type as a string
	rtt: string; // Round-trip time as a formatted string
};

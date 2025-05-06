import { getContext, setContext } from 'svelte';

const CTX = 'Settings_CTX';

export type Geo = {
	city: string;
	country: string;
	ip: string;
	latitude: number;
	longitude: number;
	state: string;
};

export type Hop = {
	id: string;
	name: string;
	ip: string;
	rtt: number;
	geo: Geo;
};

export type Trace = {
	id: string;
	name: string;
	dest_ip: string;
	max_hops: number;
	send_rate: number;
	receive_timeout: number;
	trace_timeout: number;
	hops: Hop[];
};

function Store(initData: Trace[]) {
	let data: Trace[] = $state(initData);
	let currentTraceId: string | null = $state(null);

	const startTrace = (trace: {
		name: string;
		dest_ip: string;
		max_hops: number;
		send_rate: number;
		receive_timeout: number;
		trace_timeout: number;
	}) => {
		const id = crypto.randomUUID();
		data.push({
			id: id,
			name: trace.name,
			dest_ip: trace.dest_ip,
			max_hops: trace.max_hops,
			send_rate: trace.send_rate,
			receive_timeout: trace.receive_timeout,
			trace_timeout: trace.trace_timeout,
			hops: []
		});
		currentTraceId = id;
	};

	const addHop = (traceId: string, hop: Hop) => {
		const trace = data.find((t) => t.id === traceId);
		if (trace) {
			trace.hops.push(hop);
		}
	};

	return {
		...data,
		startTrace,
		addHop,
		get currentTraceId() {
			return currentTraceId;
		}
	};
}

export function setStore(initData: Trace[]) {
	const state = Store(initData);
	setContext(CTX, state);
	return state;
}

export function getStore() {
	const state = getContext<ReturnType<typeof Store>>(CTX);
	return state;
}

export type StoreType = ReturnType<typeof Store>;

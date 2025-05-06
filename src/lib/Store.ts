import { getContext, setContext } from 'svelte';
import type { Hop, Trace } from './types';
import { derived, writable, type Writable } from 'svelte/store';

const CTX = 'Settings_CTX';

function Store(initData: Trace[]) {
	const {
		subscribe,
		update,
		set
	}: Writable<{
		traces: Trace[];
		error: string | null;
		currentTraceId: string | null;
	}> = writable({
		traces: initData,
		error: null,
		currentTraceId: null
	});

	const startTrace = (trace: {
		name: string;
		dest_ip: string;
		max_hops: number;
		send_rate: number;
		receive_timeout: number;
		trace_timeout: number;
	}) => {
		const id = crypto.randomUUID();

		update((data) => {
			data.traces = [
				...data.traces,
				{
					id: id,
					name: trace.name,
					dest_ip: trace.dest_ip,
					max_hops: trace.max_hops,
					send_rate: trace.send_rate,
					receive_timeout: trace.receive_timeout,
					trace_timeout: trace.trace_timeout,
					is_finished: false,
					hops: []
				}
			];
			data.currentTraceId = id;
			return data;
		});
	};

	const addHop = async (traceId: string, hop: Hop) => {
		try {
			const res = await fetch(`https://api.hackertarget.com/geoip/?q=${hop.ip_addr}&output=json`);

			if (res.ok) {
				const geo = await res.json();

				hop.geo = geo;
			} else {
				console.log('Error fetching geo data:', res.statusText);
			}
		} catch (error) {
			console.log(error);
		}

		update((data) => {
			const traceIndex = data.traces.findIndex((t) => t.id === traceId);
			if (traceIndex !== -1) {
				data.traces[traceIndex].hops = [...data.traces[traceIndex].hops, hop];
			}
			return data;
		});
	};

	const finishTrace = (traceId: string) => {
		update((data) => {
			const traceIndex = data.traces.findIndex((t) => t.id === traceId);
			if (traceIndex !== -1) {
				data.traces[traceIndex].is_finished = true;
			}
			return data;
		});
	};

	return {
		subscribe,
		update,
		set,

		startTrace,
		addHop,
		finishTrace
	};
}

export const getCurrentTrace = () => {
	return derived(getStore(), ($traces) => {
		return $traces.traces.filter((trace) => trace.id === $traces.currentTraceId)[0];
	});
};

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

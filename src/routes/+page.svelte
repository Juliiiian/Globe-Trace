<script lang="ts">
	import { setStore } from '$src/lib/Store.svelte';
	import type { Hop, HopEvent } from '$src/lib/types';
	import { invoke } from '@tauri-apps/api/core';
	import { listen } from '@tauri-apps/api/event';
	import { onDestroy } from 'svelte';

	const traces = setStore([]);

	let ip = $state('');
	let error = $state('');

	const listener = listen<HopEvent>('hop', (event) => {
		console.log(event);

		if (traces.currentTraceId === null) {
			return;
		}

		const hop = { ...event.payload, geo: null } as Hop;

		traces.addHop(traces.currentTraceId, hop);
	});

	async function greet(event: Event) {
		event.preventDefault();
		// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
		await invoke('trace', { ip, hops: 200 });
		traces.startTrace({
			dest_ip: ip,
			max_hops: 200,
			name: 'Tests',
			send_rate: 1,
			receive_timeout: 10,
			trace_timeout: 30
		});
	}

	onDestroy(() => {
		listener.then((unlisten) => unlisten());
	});
</script>

<main class="m-0 flex size-full flex-col justify-center p-0 text-center">
	<h1>Starte Trace</h1>

	<form class="row" onsubmit={greet}>
		<input id="greet-input" placeholder="Enter a IP..." bind:value={ip} />
		<button type="submit" class="p-2">Greet</button>
	</form>
	{#each traces.data as trace (trace.id)}
		<div>
			Hops:
			<div>
				{#each trace.hops as hop}
					<div class="flex flex-row justify-between">
						<div>{hop.seq}</div>
						<div>{hop.host_name}</div>
						<div>{hop.ip_addr}</div>
						<div>{hop.node_type}</div>
						<div>{hop.rtt}</div>
						<div>{hop.ttl}</div>
						<div>Geo:</div>
						<div>{hop.geo?.country}</div>
						<div>{hop.geo?.city}</div>
						<div>{hop.geo?.latitude}</div>
						<div>{hop.geo?.longitude}</div>
					</div>
				{/each}
			</div>
		</div>
	{/each}
</main>

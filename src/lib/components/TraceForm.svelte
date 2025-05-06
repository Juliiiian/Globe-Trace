<script lang="ts">
	import type { Hop, HopEvent, Trace } from '$src/lib/types';
	import {
		Card,
		CardContent,
		CardDescription,
		CardHeader,
		CardTitle
	} from '$src/lib/components/ui/card';
	import Input from './ui/input/input.svelte';
	import Button from './ui/button/button.svelte';
	import { onDestroy } from 'svelte';
	import { getStore } from '../Store';
	import { listen } from '@tauri-apps/api/event';
	import { invoke } from '@tauri-apps/api/core';

	const traces = getStore();

	let ip = $state('');

	const listenerHop = listen<HopEvent>('hop', (event) => {
		if ($traces.currentTraceId === null) {
			return;
		}

		const hop = { ...event.payload, geo: null } as Hop;

		traces.addHop($traces.currentTraceId, hop);
	});

	const listenerError = listen<string>('trace_fail', (event) => {
		$traces.currentTraceId = null;
		$traces.error = event.payload;
	});

	async function startTrace(event: Event) {
		event.preventDefault();
		traces.startTrace({
			dest_ip: ip,
			max_hops: 200,
			name: 'Tests',
			send_rate: 1,
			receive_timeout: 10,
			trace_timeout: 30
		});
		await invoke('trace', { ip, hops: 200 });
	}
	onDestroy(() => {
		listenerHop.then((unlisten) => unlisten());
		listenerError.then((unlisten) => unlisten());
	});
</script>

<Card class="">
	<CardHeader class="pb-3">
		<CardTitle>Trace Configuration</CardTitle>
		<CardDescription>Enter a domain or IP address to trace</CardDescription>
	</CardHeader>
	<CardContent>
		<Input type="text" placeholder="Enter a domain or IP address" bind:value={ip} class="w-full" />
		{#if $traces.error}
			<p class="mt-2 text-sm text-red-500">{$traces.error}</p>
		{/if}
		<Button type="submit" class="mt-4 w-full" onclick={startTrace}>Start Trace</Button>
	</CardContent>
</Card>

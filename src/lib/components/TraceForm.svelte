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
	import { getCurrentTrace, getStore } from '../Store';
	import { listen } from '@tauri-apps/api/event';
	import { invoke } from '@tauri-apps/api/core';

	const traces = getStore();

	const curTrace = getCurrentTrace();

	let domain_ip = $state('');

	const listenerHop = listen<HopEvent>('hop', (event) => {
		if ($traces.currentTraceId === null) {
			return;
		}

		const hop = { ...event.payload, geo: null } as Hop;

		traces.addHop($traces.currentTraceId, hop);
	});

	const listenerError = listen<string>('trace_fail', (event) => {
		$traces.error = event.payload;
		if ($traces.currentTraceId === null) {
			return;
		}
		traces.finishTrace($traces.currentTraceId, 'Error');
		$traces.currentTraceId = null;
	});

	const listenerCancel = listen<string>('trace_cancelled', (event) => {
		if ($traces.currentTraceId === null) {
			return;
		}
		traces.finishTrace($traces.currentTraceId, 'Canceled');
		$traces.currentTraceId = null;
	});

	const listenerComplete = listen<string>('trace_complete', (event) => {
		if ($traces.currentTraceId === null) {
			return;
		}
		traces.finishTrace($traces.currentTraceId, 'Finished');
	});

	const listenerDNS = listen<string>('trace_dns', (event) => {
		if ($traces.currentTraceId === null) {
			return;
		}
		traces.setTraceIP($traces.currentTraceId, event.payload);
	});

	async function startTrace() {
		if (domain_ip === '') {
			$traces.error = 'Please enter a domain or IP address';
			return;
		} else if (
			!/^(25[0-5]|2[0-4][0-9]|[0-1]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[0-1]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[0-1]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[0-1]?[0-9][0-9]?)$/.test(
				domain_ip
			)
		) {
			//is not a valid IP address = domain name
			if (!domain_ip.includes(':')) {
				domain_ip += ':80';
			}
		}
		traces.startTrace({
			dest_ip: domain_ip,
			max_hops: 200,
			name: domain_ip,
			send_rate: 1,
			receive_timeout: 10,
			trace_timeout: 30
		});
		await invoke('trace', { ip: domain_ip, hops: 200 });
	}

	async function cancelTrace() {
		if ($traces.currentTraceId === null) {
			return;
		}
		await invoke('cancel_trace');
	}

	onDestroy(() => {
		listenerHop.then((unlisten) => unlisten());
		listenerError.then((unlisten) => unlisten());
		listenerCancel.then((unlisten) => unlisten());
		listenerDNS.then((unlisten) => unlisten());
		listenerComplete.then((unlisten) => unlisten());
	});

	$inspect($curTrace);
</script>

<Card class="">
	<CardHeader class="">
		<CardTitle>Trace Configuration</CardTitle>
		<CardDescription>Enter a domain or IP address to trace</CardDescription>
	</CardHeader>
	<CardContent>
		<Input
			type="text"
			placeholder="Enter a domain or IP address"
			bind:value={domain_ip}
			class="w-full"
		/>
		{#if $traces.error}
			<p class="mt-2 text-sm text-red-500">{$traces.error}</p>
		{/if}
		{#if $curTrace === undefined || $curTrace.is_finished !== 'Running'}
			<Button class="mt-4 w-full" onclick={startTrace} variant="secondary">Start Trace</Button>
		{:else}
			<div class="flex gap-2">
				<Button class="mt-4 w-full" variant="secondary">Tracing...</Button>
				<Button class="mt-4 w-full" onclick={cancelTrace} variant="destructive">Cancel Trace</Button
				>
			</div>
		{/if}
	</CardContent>
</Card>

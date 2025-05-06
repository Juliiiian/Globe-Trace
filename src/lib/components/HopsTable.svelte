<script lang="ts">
	import type { Trace } from '$src/lib/types';
	import {
		Table,
		TableHeader,
		TableBody,
		TableRow,
		TableHead,
		TableCell
	} from '$lib/components/ui/table/index.js';

	let { trace }: { trace: Trace } = $props();
</script>

<div class="w-full overflow-auto border-t">
	<Table>
		<TableHeader>
			<TableRow>
				<TableHead class="w-12">Hop</TableHead>
				<TableHead>IP Address</TableHead>
				<TableHead>City</TableHead>
				<TableHead>Location</TableHead>
				<TableHead class="text-right">Response Time</TableHead>
			</TableRow>
		</TableHeader>
		<TableBody>
			{#if !trace.is_finished && trace.hops.length === 0}
				<TableRow class="h-12">
					<TableCell colspan={5} class="animate-pulse bg-primary-foreground/30 text-center"
						>Loading...</TableCell
					>
				</TableRow>
			{:else}
				{#each trace.hops as hop}
					<TableRow>
						<TableCell class="font-medium">{hop.seq}</TableCell>
						<TableCell>{hop.ip_addr}</TableCell>
						<TableCell>
							{#if hop.geo}
								{hop.geo.city}
							{:else}
								-
							{/if}
						</TableCell>
						<TableCell
							>{#if hop.geo}
								<div class="text-xs">
									{hop.geo.latitude.toFixed(2)}, {hop.geo.longitude.toFixed(2)}
								</div>
							{:else}
								-
							{/if}</TableCell
						>
						<TableCell class="text-right">
							{hop.rtt} ms
						</TableCell>
					</TableRow>
				{:else}
					<TableRow>
						<TableCell colspan={5} class="text-center py-6 text-muted-foreground">
							No trace data available. Start a trace to see results.
						</TableCell>
					</TableRow>
				{/each}
			{/if}
		</TableBody>
	</Table>
</div>

<script lang="ts">
	import { getStore } from '$src/lib/Store';
	import {
		Card,
		CardContent,
		CardDescription,
		CardHeader,
		CardTitle
	} from '$src/lib/components/ui/card';
	import { cn } from '$src/lib/utils/utils';
	import HopsTable from '$src/lib/components/HopsTable.svelte';
	import Badge from '$src/lib/components/ui/badge/badge.svelte';

	let { activeTab }: { activeTab: string } = $props();

	const traces = getStore();
</script>

<Card
	class={cn(
		activeTab === 'table' || !activeTab ? 'block lg:block  lg:overflow-hidden' : 'hidden lg:block'
	)}
>
	<CardHeader class="flex flex-row items-start justify-between pb-4">
		<div>
			<CardTitle>Hop Information</CardTitle>
			<CardDescription>Details of each network hop</CardDescription>
		</div>

		{#if $traces.api_error}
			<Badge variant="destructive" class="h-6 p-1">
				{$traces.api_error}
			</Badge>
		{/if}
	</CardHeader>
	<CardContent class="p-0">
		<HopsTable />
	</CardContent>
</Card>

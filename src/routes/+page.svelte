<script lang="ts">
	import { getCurrentTrace, setStore } from '$src/lib/Store';
	import {
		Card,
		CardContent,
		CardDescription,
		CardHeader,
		CardTitle
	} from '$src/lib/components/ui/card';
	import { Tabs, TabsContent, TabsList, TabsTrigger } from '$components/ui/tabs';
	import { cn } from '$src/lib/utils/utils';
	import HopsTable from '$src/lib/components/HopsTable.svelte';
	import TraceForm from '$src/lib/components/TraceForm.svelte';

	const curTrace = getCurrentTrace();

	let activeTab = $state('table');
</script>

<main class="grid-bg m-0 min-h-screen w-full p-0">
	<div class="container mx-auto max-w-[1560px] py-8">
		<div class="flex flex-col space-y-6">
			<div class="text-center">
				<h1
					class="mb-2 bg-gradient-to-r from-primary via-primary/80 to-accent bg-clip-text text-4xl font-bold text-transparent"
				>
					Globe Tracer
				</h1>
				<p class="text-muted-foreground">Visual network path tracer with 3D globe visualization</p>
			</div>

			<div class="grid grid-cols-1 gap-6 lg:grid-cols-5">
				<div class="space-y-6 lg:col-span-2">
					<TraceForm></TraceForm>
					<Card class=" lg:hidden">
						<CardHeader class="pb-3">
							<CardTitle>View Options</CardTitle>
						</CardHeader>
						<CardContent>
							<Tabs bind:value={activeTab}>
								<TabsList class="w-full">
									<TabsTrigger value="table" class="flex-1">Table</TabsTrigger>
									<TabsTrigger value="globe" class="flex-1">Globe</TabsTrigger>
								</TabsList>
							</Tabs>
						</CardContent>
					</Card>

					<Card
						class={cn(activeTab === 'table' || !activeTab ? 'block lg:block' : 'hidden lg:block')}
					>
						<CardHeader class="pb-3">
							<CardTitle>Hop Information</CardTitle>
							<CardDescription>Details of each network hop</CardDescription>
						</CardHeader>
						<CardContent class="p-0">
							{#if $curTrace != null}
								<HopsTable trace={$curTrace} />
							{:else}
								<div class="flex h-full items-center justify-center p-6 text-muted-foreground">
									No trace data available. Start a trace to see results.
								</div>
							{/if}
						</CardContent>
					</Card>
				</div>

				<div
					class={cn(
						activeTab === 'globe' || !activeTab ? 'block' : 'hidden lg:block',
						'lg:col-span-3'
					)}
				>
					<Card class="overflow-hidden ">
						<CardHeader class="pb-3">
							<CardTitle>3D Visualization</CardTitle>
							<CardDescription>Trace route path on interactive globe</CardDescription>
						</CardHeader>
						<CardContent class="h-[600px] p-0">
							<!-- <Globe {traceData} isLoading={isTracing} /> -->
						</CardContent>
					</Card>
				</div>
			</div>

			<div class="text-center text-xs text-muted-foreground">
				<p>© {new Date().getFullYear()} Globe Tracer - Visualize network paths across the world</p>
				<p class="mt-1">Drag to rotate | Scroll to zoom | Double-click to reset view</p>
			</div>
		</div>
	</div>
</main>

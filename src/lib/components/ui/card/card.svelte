<script lang="ts">
	import type { WithElementRef } from 'bits-ui';
	import type { HTMLAttributes } from 'svelte/elements';
	import { cn } from '$utils/utils.js';

	let {
		ref = $bindable(null),
		class: className,
		children,
		...restProps
	}: WithElementRef<HTMLAttributes<HTMLDivElement>> = $props();
</script>

<div
	bind:this={ref}
	class={cn(
		' rounded-lg border border-white/10 bg-card text-card-foreground shadow transition-colors dark:border',
		className
	)}
	{...restProps}
>
	{@render children?.()}
</div>

<script lang="ts">
	import '$src/app.postcss';
	import { setStore } from '$src/lib/Store';

	let {
		children
	}: {
		children?: import('svelte').Snippet;
	} = $props();

	setStore([]);
</script>

{@render children?.()}

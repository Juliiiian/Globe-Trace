<script lang="ts">
	import '$src/app.postcss';

	let {
		children
	}: {
		children?: import('svelte').Snippet;
	} = $props();
</script>

{@render children?.()}

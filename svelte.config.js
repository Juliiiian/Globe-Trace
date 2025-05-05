// Tauri doesn't have a Node.js server to do proper SSR
// so we will use adapter-static to prerender the app (SSG)
// See: https://v2.tauri.app/start/frontend/sveltekit/ for more info
import adapter from "@sveltejs/adapter-static";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";
import { sveltePreprocess } from 'svelte-preprocess';

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: [
		vitePreprocess(),
		sveltePreprocess({
			postcss: true
		})
	],
  kit: {
    adapter: adapter(),
    alias: {
			'$src/*': 'src/*',
			$utils: 'src/lib/utils/utils.ts',
			'$utils/*': 'src/lib/utils/*',
			$comp: 'src/lib/components',
			'$comp/*': 'src/lib/components/*'
		}
  },
};

export default config;

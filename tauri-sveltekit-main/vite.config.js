import { sveltekit } from '@sveltejs/kit/vite';

/** @type {import('vite').UserConfig} */
const config = {
	plugins: [sveltekit()],
	resolve: {
		extensions: ['.svelte', '.svelte.ts', '.svelte.js', '.ts', '.js']
	}
};

export default config;

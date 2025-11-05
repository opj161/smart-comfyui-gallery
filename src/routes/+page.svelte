
<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';

	let greetMsg = $state('');
	let name = $state('SvelteKit');

	async function greet() {
		greetMsg = await invoke<string>('greet', { name });
	}
</script>

<div
	class="min-h-screen flex items-center justify-center p-4"
	style="background: linear-gradient(135deg, #acdff0 0%, #ffeec9 50%, #ffdbce 100%);"
>
	<div
		class="max-w-lg w-full bg-white/90 backdrop-blur-sm rounded-lg border border-white/20 p-8 text-center shadow-lg"
	>
		<div class="mb-8">
			<h1 class="text-3xl font-semibold text-gray-900 mb-3">SmartGallery</h1>
			<p class="text-gray-700">
				AI Media Gallery with ComfyUI Workflow Extraction
			</p>
			<p class="text-sm text-gray-500 mt-2">
				Powered by Tauri v2, SvelteKit v2, and Rust
			</p>
		</div>

		<div class="space-y-6">
			<div class="flex justify-center flex-wrap gap-2">
				<span class="bg-orange-100 text-orange-800 px-3 py-1 rounded-md text-sm font-medium">
					SvelteKit v2
				</span>
				<span class="bg-blue-100 text-blue-800 px-3 py-1 rounded-md text-sm font-medium">
					Vite v6
				</span>
				<span class="bg-sky-100 text-sky-800 px-3 py-1 rounded-md text-sm font-medium">
					Tauri v2
				</span>
				<span class="bg-red-100 text-red-800 px-3 py-1 rounded-md text-sm font-medium">
					Rust
				</span>
			</div>

			<!-- Test IPC Bridge -->
			<div class="mt-6 p-4 bg-gray-50 rounded-lg">
				<h2 class="text-lg font-semibold text-gray-900 mb-3">Test Rust ↔️ SvelteKit Bridge</h2>
				<div class="flex gap-2 mb-3">
					<input
						type="text"
						bind:value={name}
						placeholder="Enter your name"
						class="flex-1 px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
					/>
					<button
						onclick={greet}
						class="px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 transition-colors"
					>
						Greet
					</button>
				</div>
				{#if greetMsg}
					<p class="text-green-700 font-medium">{greetMsg}</p>
				{/if}
			</div>

			<div class="text-sm text-gray-600">
				<p>🚀 Phase 1 Complete: Foundation & Scaffolding</p>
				<p class="mt-1">✅ Tauri app running</p>
				<p class="mt-1">✅ IPC bridge functional</p>
				<p class="mt-1">✅ Core data structures defined</p>
			</div>
		</div>
	</div>
</div>

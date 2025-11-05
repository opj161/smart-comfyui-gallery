<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import type { FileEntry } from '$lib/types';

	let greetMessage = $state('');
	let testFile = $state<FileEntry | null>(null);
	let error = $state('');

	async function testGreet() {
		try {
			error = '';
			const message = await invoke<string>('greet', { name: 'SmartGallery' });
			greetMessage = message;
		} catch (e) {
			error = `Error: ${e}`;
		}
	}

	async function testGetFile() {
		try {
			error = '';
			const file = await invoke<FileEntry>('get_test_file');
			testFile = file;
		} catch (e) {
			error = `Error: ${e}`;
		}
	}
</script>

<div
	class="min-h-screen flex items-center justify-center p-4"
	style="background: linear-gradient(135deg, #acdff0 0%, #ffeec9 50%, #ffdbce 100%);"
>
	<div
		class="max-w-2xl w-full bg-white/90 backdrop-blur-sm rounded-lg border border-white/20 p-8 shadow-lg"
	>
		<div class="mb-8 text-center">
			<h1 class="text-3xl font-semibold text-gray-900 mb-3">SmartGallery - Tauri Migration</h1>
			<p class="text-gray-700">Phase 1: Testing Rust-Frontend IPC Bridge</p>
		</div>

		<div class="space-y-6">
			<!-- Badge Display -->
			<div class="flex justify-center flex-wrap gap-2 mb-6">
				<span class="bg-orange-100 text-orange-800 px-3 py-1 rounded-md text-sm font-medium">
					SvelteKit v5
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

			<!-- Test Buttons -->
			<div class="space-y-4">
				<button
					onclick={testGreet}
					class="w-full bg-blue-600 hover:bg-blue-700 text-white font-medium py-3 px-4 rounded-lg transition-colors"
				>
					Test Greet Command
				</button>

				<button
					onclick={testGetFile}
					class="w-full bg-green-600 hover:bg-green-700 text-white font-medium py-3 px-4 rounded-lg transition-colors"
				>
					Test Get File Command
				</button>
			</div>

			<!-- Results Display -->
			{#if error}
				<div class="bg-red-50 border border-red-200 text-red-800 px-4 py-3 rounded-lg">
					<p class="font-medium">Error:</p>
					<p class="text-sm">{error}</p>
				</div>
			{/if}

			{#if greetMessage}
				<div class="bg-green-50 border border-green-200 text-green-800 px-4 py-3 rounded-lg">
					<p class="font-medium">Greet Response:</p>
					<p class="text-sm">{greetMessage}</p>
				</div>
			{/if}

			{#if testFile}
				<div class="bg-blue-50 border border-blue-200 text-blue-900 px-4 py-3 rounded-lg">
					<p class="font-medium mb-2">Test File Entry:</p>
					<pre
						class="text-xs overflow-x-auto bg-blue-100 p-2 rounded">{JSON.stringify(testFile, null, 2)}</pre>
				</div>
			{/if}
		</div>

		<div class="mt-8 text-center text-sm text-gray-600">
			<p>✅ IPC Bridge established between Rust backend and SvelteKit frontend</p>
			<p class="mt-2">✅ Type-safe data structures shared across the stack</p>
		</div>
	</div>
</div>

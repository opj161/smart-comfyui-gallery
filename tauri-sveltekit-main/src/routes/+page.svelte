<script lang="ts">
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';
	import type { FileEntry, PaginatedFiles } from '$lib/types';
	import * as api from '$lib/api';
	import * as store from '$lib/store';

	let greetMessage = $state('');
	let testFile = $state<FileEntry | null>(null);
	let error = $state('');
	let isInitialized = $state(false);
	let galleryFiles = $state<FileEntry[]>([]);
	let stats = $state({ total_files: 0, favorites: 0, with_workflow: 0 });

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

	async function initGallery() {
		try {
			error = '';
			// Initialize with a default path for testing
			const result = await api.initializeGallery('/tmp/smartgallery-test');
			isInitialized = true;
			greetMessage = result;
			
			// Load initial data
			await loadFiles();
			await loadStats();
		} catch (e) {
			error = `Init error: ${e}`;
		}
	}

	async function loadFiles() {
		try {
			error = '';
			const result = await api.getFiles(null, 0, 10);
			galleryFiles = result.files;
			store.setFiles(result.files, result.total_count, result.has_more);
		} catch (e) {
			error = `Load files error: ${e}`;
		}
	}

	async function loadStats() {
		try {
			error = '';
			stats = await api.getStats();
		} catch (e) {
			error = `Load stats error: ${e}`;
		}
	}

	onMount(async () => {
		// Set up event listeners
		const unlistenProgress = await api.listenToSyncProgress((progress) => {
			store.setSyncProgress(progress);
		});

		const unlistenComplete = await api.listenToSyncComplete((syncStats) => {
			store.clearSyncProgress();
			console.log('Sync complete:', syncStats);
		});

		return () => {
			unlistenProgress();
			unlistenComplete();
		};
	});
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
			<p class="text-gray-700">Phase 3: Tauri Commands & State Management</p>
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
					Rust Backend
				</span>
				{#if isInitialized}
					<span class="bg-green-100 text-green-800 px-3 py-1 rounded-md text-sm font-medium">
						✓ Initialized
					</span>
				{/if}
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

				<button
					onclick={initGallery}
					class="w-full bg-purple-600 hover:bg-purple-700 text-white font-medium py-3 px-4 rounded-lg transition-colors"
					disabled={isInitialized}
				>
					Initialize Gallery
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
					<p class="font-medium">Response:</p>
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

			{#if isInitialized}
				<div class="bg-purple-50 border border-purple-200 text-purple-900 px-4 py-3 rounded-lg">
					<p class="font-medium mb-2">Database Stats:</p>
					<div class="text-sm space-y-1">
						<p>Total Files: {stats.total_files}</p>
						<p>Favorites: {stats.favorites}</p>
						<p>With Workflow: {stats.with_workflow}</p>
					</div>
				</div>
			{/if}

			{#if galleryFiles.length > 0}
				<div class="bg-indigo-50 border border-indigo-200 text-indigo-900 px-4 py-3 rounded-lg">
					<p class="font-medium mb-2">Gallery Files ({galleryFiles.length}):</p>
					<div class="text-sm space-y-2 max-h-40 overflow-y-auto">
						{#each galleryFiles as file}
							<div class="bg-white p-2 rounded">
								<p class="font-medium">{file.name}</p>
								<p class="text-xs text-gray-600">{file.path}</p>
							</div>
						{/each}
					</div>
				</div>
			{/if}
		</div>

		<div class="mt-8 text-center text-sm text-gray-600">
			<p>✅ Phase 1: Foundation complete</p>
			<p>✅ Phase 2: Backend complete (1,565 lines of Rust)</p>
			<p>✅ Phase 3: Tauri commands (13 implemented)</p>
			<p class="mt-2">🔄 Next: Complete frontend components</p>
		</div>
	</div>
</div>

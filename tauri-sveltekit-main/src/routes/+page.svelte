<script lang="ts">
	import { onMount } from 'svelte';
	import { listen } from '@tauri-apps/api/event';
	import * as api from '$lib/api';
	import { store } from '$lib/store.svelte';
	import GalleryGrid from '$lib/components/GalleryGrid.svelte';
	import Lightbox from '$lib/components/Lightbox.svelte';
	import FilterPanel from '$lib/components/FilterPanel.svelte';
	import SettingsPanel from '$lib/components/SettingsPanel.svelte';
	import UploadZone from '$lib/components/UploadZone.svelte';
	import { invoke } from '@tauri-apps/api/core';
	import type { AppConfig } from '$lib/types';

	let isInitialized = $state(false);
	let isLoading = $state(true);
	let currentPage = $state(0);
	let totalCount = $state(0);
	let hasMore = $state(false);
	let isFilterPanelOpen = $state(false);
	let isSyncing = $state(false);
	let syncProgress = $state(0);
	let syncTotal = $state(0);
	let showSettings = $state(false);
	let showUpload = $state(false);

	// Initialize gallery on mount
	onMount(async () => {
		try {
			// Try to load config first
			const config = await invoke<AppConfig>('load_config');
			
			if (config && config.output_path) {
				// Initialize with saved config
				await api.initializeGallery(config.output_path, config.input_path || undefined);
				isInitialized = true;

				// Load initial files
				await loadFiles(0);

				// Set up event listeners
				await setupEventListeners();
			} else {
				// Show settings panel to configure
				showSettings = true;
				isLoading = false;
			}
		} catch (error) {
			console.error('Failed to initialize gallery:', error);
			// Show settings panel for first-time configuration
			showSettings = true;
			isLoading = false;
		}
	});

	async function setupEventListeners() {
		// Sync progress
		await listen('sync-progress', (event: any) => {
			const payload = event.payload;
			syncProgress = payload.current;
			syncTotal = payload.total;
		});

		// Sync complete
		await listen('sync-complete', async () => {
			isSyncing = false;
			// Reload files
			await loadFiles(0);
		});
	}

	async function loadFiles(page: number) {
		isLoading = true;
		try {
			const filters = store.filters;
			let result;

			// Check if we have active filters
			const hasActiveFilters =
				filters.search ||
				filters.favorites_only ||
				filters.model ||
				filters.sampler ||
				filters.scheduler ||
				filters.cfg_min !== null ||
				filters.cfg_max !== null ||
				filters.steps_min !== null ||
				filters.steps_max !== null ||
				filters.width !== null ||
				filters.height !== null;

			if (hasActiveFilters) {
				// Use filtered endpoint
				result = await api.getFilesFiltered(filters, page, 50);
			} else if (filters.search) {
				// Use search endpoint
				result = await api.searchFiles(filters.search, page, 50);
			} else {
				// Use regular endpoint
				result = await api.getFiles(null, page, 50);
			}

			if (page === 0) {
				store.setFiles(result.files, result.total_count, result.has_more);
			} else {
				// Append files for pagination
				const currentFiles = store.files;
				store.setFiles([...currentFiles, ...result.files], result.total_count, result.has_more);
			}

			currentPage = page;
			totalCount = result.total_count;
			hasMore = result.has_more;
		} catch (error) {
			console.error('Failed to load files:', error);
		} finally {
			isLoading = false;
		}
	}

	async function handleLoadMore() {
		await loadFiles(currentPage + 1);
	}

	async function handleSync() {
		if (isSyncing) return;

		isSyncing = true;
		syncProgress = 0;
		syncTotal = 0;

		try {
			await api.syncFiles();
		} catch (error) {
			console.error('Sync failed:', error);
			alert('Sync failed. Please check the console for details.');
			isSyncing = false;
		}
	}

	function handleOpenFilters() {
		isFilterPanelOpen = true;
	}

	function handleCloseFilters() {
		isFilterPanelOpen = false;
		// Reload files with new filters
		loadFiles(0);
	}

	// Batch operations
	const selectionCount = $derived(store.selectedFiles.size);
	const hasSelection = $derived(selectionCount > 0);
	let isDeleting = $state(false);
	let isFavoriting = $state(false);

	async function handleBatchDelete() {
		if (!hasSelection || isDeleting) return;

		if (!confirm(`Delete ${selectionCount} selected file(s)?`)) {
			return;
		}

		isDeleting = true;
		try {
			const fileIds: string[] = Array.from(store.selectedFiles);
			await api.batchDelete(fileIds);
			store.clearSelection();
			// Trigger refresh
			await loadFiles(0);
		} catch (error) {
			console.error('Failed to delete files:', error);
			alert('Failed to delete files');
		} finally {
			isDeleting = false;
		}
	}

	async function handleBatchFavorite(favorite: boolean) {
		if (!hasSelection || isFavoriting) return;

		isFavoriting = true;
		try {
			const fileIds: string[] = Array.from(store.selectedFiles);
			await api.batchFavorite(fileIds, favorite);
			store.clearSelection();
			// Trigger refresh
			await loadFiles(0);
		} catch (error) {
			console.error('Failed to update favorites:', error);
			alert('Failed to update favorites');
		} finally {
			isFavoriting = false;
		}
	}
</script>

<div class="app-container">
	<header class="app-header">
		<h1>SmartGallery</h1>
		<div class="header-stats">
			{#if isInitialized}
				<span class="stat-item">{totalCount} files</span>
				{#if isSyncing}
					<span class="stat-item syncing">
						Syncing... {syncProgress}/{syncTotal}
					</span>
				{/if}
			{/if}
		</div>
	</header>

	{#if isInitialized}
		<div class="toolbar">
			<div class="toolbar-left">
				<button class="btn btn-secondary" onclick={handleSync}>
					<span>↻</span>
					<span>Sync</span>
				</button>
				<button class="btn btn-secondary" onclick={handleOpenFilters}>
					<span>⚙</span>
					<span>Filters</span>
				</button>
				<button class="btn btn-secondary" onclick={() => (showUpload = true)}>
					<span>⬆</span>
					<span>Upload</span>
				</button>
				<button class="btn btn-secondary" onclick={() => (showSettings = true)}>
					<span>⚙</span>
					<span>Settings</span>
				</button>
			</div>
			{#if hasSelection}
				<div class="toolbar-center">
					<div class="selection-info">
						<span>{selectionCount} file(s) selected</span>
						<button class="btn-text" onclick={store.clearSelection}>Clear</button>
					</div>
				</div>
				<div class="toolbar-right">
					<div class="selection-actions">
						<button class="btn btn-primary" onclick={() => handleBatchFavorite(true)}>
							Add to Favorites
						</button>
						<button class="btn btn-secondary" onclick={() => handleBatchFavorite(false)}>
							Remove from Favorites
						</button>
						<button class="btn btn-danger" onclick={handleBatchDelete}>
							Delete Selected
						</button>
					</div>
				</div>
			{/if}
		</div>

		<main class="app-main">
			<GalleryGrid onLoadMore={handleLoadMore} {hasMore} {isLoading} />
		</main>

		<Lightbox isOpen={store.isLightboxOpen} />

		<FilterPanel isOpen={isFilterPanelOpen} onClose={handleCloseFilters} />
	{:else}
		<div class="loading-screen">
			<p>Initializing gallery...</p>
			{#if showSettings}
				<p class="hint">Please configure your ComfyUI paths to continue</p>
			{/if}
		</div>
	{/if}

	<!-- Settings Panel -->
	{#if showSettings}
		<SettingsPanel
			onClose={async () => {
				showSettings = false;
				// Reload the app after settings are saved
				try {
					const config = await invoke<AppConfig>('load_config');
					if (config && config.output_path && !isInitialized) {
						await api.initializeGallery(config.output_path, config.input_path || undefined);
						isInitialized = true;
						await loadFiles(0);
						await setupEventListeners();
					}
				} catch (e) {
					console.error('Failed to reload after settings:', e);
				}
			}}
		/>
	{/if}

	<!-- Upload Zone -->
	{#if showUpload}
		<div
			class="modal-overlay"
			onclick={() => (showUpload = false)}
			onkeydown={(e) => e.key === 'Escape' && (showUpload = false)}
			role="button"
			tabindex="0"
			aria-label="Close upload dialog"
		>
			<div
				class="modal-content"
				onclick={(e) => e.stopPropagation()}
				onkeydown={(e) => e.stopPropagation()}
				role="dialog"
				tabindex="-1"
				aria-modal="true"
				aria-labelledby="upload-dialog-title"
			>
				<div class="modal-header">
					<h2 id="upload-dialog-title">Upload Files</h2>
					<button class="close-button" onclick={() => (showUpload = false)}>×</button>
				</div>
				<UploadZone
					onComplete={async () => {
						showUpload = false;
						await loadFiles(0);
					}}
				/>
			</div>
		</div>
	{/if}
</div>

<style>
	:global(body) {
		margin: 0;
		padding: 0;
		font-family:
			'Inter',
			-apple-system,
			BlinkMacSystemFont,
			'Segoe UI',
			Roboto,
			sans-serif;
		background: #121212;
		color: #eaeaea;
	}

	:global(*) {
		box-sizing: border-box;
	}

	:global(:root) {
		--bg-color: #121212;
		--surface-color: #1e1e1e;
		--surface-hover: #2a2a2a;
		--border-color: #333333;
		--text-color: #eaeaea;
		--text-muted: #9e9e9e;
		--accent-yellow: #ffd60a;
		--danger-color: #ff4d4d;
		--danger-hover: #ff1a1a;
		--success-color: #52c41a;
	}

	.app-container {
		width: 100%;
		min-height: 100vh;
		display: flex;
		flex-direction: column;
	}

	.app-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 16px 24px;
		background: var(--surface-color);
		border-bottom: 1px solid var(--border-color);
	}

	.app-header h1 {
		margin: 0;
		font-size: 1.5rem;
		color: var(--text-color);
		font-weight: 700;
	}

	.header-stats {
		display: flex;
		gap: 16px;
		align-items: center;
	}

	.stat-item {
		font-size: 0.875rem;
		color: var(--text-muted);
	}

	.stat-item.syncing {
		color: var(--accent-yellow);
		font-weight: 600;
	}

	.app-main {
		flex: 1;
		overflow-y: auto;
	}

	.loading-screen {
		flex: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--text-muted);
	}

	.loading-screen p {
		font-size: 1.125rem;
	}

	/* Toolbar styles */
	.toolbar {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 12px 24px;
		background: var(--surface-color);
		border-bottom: 1px solid var(--border-color);
		gap: 16px;
		flex-wrap: wrap;
	}

	.toolbar-left,
	.toolbar-center,
	.toolbar-right {
		display: flex;
		align-items: center;
		gap: 12px;
	}

	.toolbar-center {
		flex: 1;
		justify-content: center;
	}

	.selection-info {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 8px 16px;
		background: rgba(255, 214, 10, 0.1);
		border-radius: 6px;
		border: 1px solid rgba(255, 214, 10, 0.3);
	}

	.selection-info span {
		font-size: 0.875rem;
		color: var(--accent-yellow);
		font-weight: 600;
	}

	.selection-actions {
		display: flex;
		gap: 8px;
	}

	.btn {
		padding: 8px 16px;
		border: none;
		border-radius: 6px;
		font-size: 0.875rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
		display: flex;
		align-items: center;
		gap: 6px;
	}

	.btn-primary {
		background: var(--accent-yellow);
		color: #000;
	}

	.btn-primary:hover {
		background: #ffc700;
	}

	.btn-secondary {
		background: var(--surface-hover);
		color: var(--text-color);
		border: 1px solid var(--border-color);
	}

	.btn-secondary:hover {
		background: #363636;
	}

	.btn-danger {
		background: var(--danger-color);
		color: white;
	}

	.btn-danger:hover {
		background: var(--danger-hover);
	}

	.btn-text {
		background: transparent;
		border: none;
		color: var(--text-muted);
		cursor: pointer;
		font-size: 0.875rem;
		text-decoration: underline;
		padding: 0;
	}

	.btn-text:hover {
		color: var(--text-color);
	}

	@media (max-width: 768px) {
		.toolbar {
			flex-direction: column;
			align-items: stretch;
		}

		.toolbar-left,
		.toolbar-center,
		.toolbar-right {
			width: 100%;
			justify-content: center;
		}

		.selection-actions {
			flex-wrap: wrap;
		}
	}

	/* Modal styles */
	.modal-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.75);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
		animation: fadeIn 0.2s ease-out;
	}

	.modal-content {
		background: #1a1a1a;
		border-radius: 12px;
		max-width: 700px;
		width: 90%;
		max-height: 90vh;
		overflow: hidden;
		display: flex;
		flex-direction: column;
		box-shadow: 0 8px 32px rgba(0, 0, 0, 0.5);
		animation: slideUp 0.3s ease-out;
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1.5rem;
		border-bottom: 1px solid #333;
	}

	.modal-header h2 {
		margin: 0;
		font-size: 1.5rem;
		color: #e0e0e0;
	}

	.close-button {
		background: none;
		border: none;
		font-size: 2rem;
		color: #999;
		cursor: pointer;
		padding: 0;
		width: 2rem;
		height: 2rem;
		display: flex;
		align-items: center;
		justify-content: center;
		border-radius: 4px;
		transition: all 0.2s;
	}

	.close-button:hover {
		background: #333;
		color: #fff;
	}

	.hint {
		color: #999;
		font-size: 0.9rem;
		margin-top: 0.5rem;
	}

	@keyframes fadeIn {
		from {
			opacity: 0;
		}
		to {
			opacity: 1;
		}
	}

	@keyframes slideUp {
		from {
			transform: translateY(20px);
			opacity: 0;
		}
		to {
			transform: translateY(0);
			opacity: 1;
		}
	}
</style>

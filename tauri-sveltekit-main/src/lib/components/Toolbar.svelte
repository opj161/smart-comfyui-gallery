<script lang="ts">
	import { store } from '$lib/store.svelte';
	import * as api from '$lib/api';

	interface Props {
		onOpenFilters: () => void;
		onSync: () => void;
	}

	let { onOpenFilters, onSync }: Props = $props();

	let isDeleting = $state(false);
	let isFavoriting = $state(false);

	const selectionCount = $derived(store.selectedFiles.size);
	const hasSelection = $derived(selectionCount > 0);

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
			window.location.reload();
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
			window.location.reload();
		} catch (error) {
			console.error('Failed to update favorites:', error);
			alert('Failed to update favorites');
		} finally {
			isFavoriting = false;
		}
	}
</script>

<div class="toolbar">
	<div class="toolbar-left">
		<button class="btn btn-secondary" onclick={onSync}>
			<span>↻</span>
			<span>Sync</span>
		</button>

		<button class="btn btn-secondary" onclick={onOpenFilters}>
			<span>⚙</span>
			<span>Filters</span>
		</button>
	</div>

	{#if hasSelection}
		<div class="toolbar-center selection-bar">
			<span class="selection-count">{selectionCount} selected</span>

			<div class="selection-actions">
				<button
					class="btn btn-small btn-secondary"
					onclick={() => handleBatchFavorite(true)}
					disabled={isFavoriting}
				>
					<span>★</span>
					<span>Add to Favorites</span>
				</button>

				<button
					class="btn btn-small btn-secondary"
					onclick={() => handleBatchFavorite(false)}
					disabled={isFavoriting}
			>
				<span>☆</span>
				<span>Remove from Favorites</span>
			</button>

			<button class="btn btn-small btn-danger" onclick={handleBatchDelete} disabled={isDeleting}>
				<span>🗑</span>
				<span>{isDeleting ? 'Deleting...' : 'Delete'}</span>
			</button>

			<button class="btn btn-small btn-secondary" onclick={() => store.clearSelection()}>
				<span>✕</span>
				<span>Clear Selection</span>
			</button>
		</div>
	</div>
	{/if}
</div><style>
	.toolbar {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 16px 24px;
		background: var(--surface-color, #1e1e1e);
		border-bottom: 1px solid var(--border-color, #333);
		gap: 16px;
	}

	.toolbar-left {
		display: flex;
		gap: 12px;
	}

	.toolbar-center {
		flex: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 16px;
	}

	.selection-bar {
		background: var(--surface-hover, #2a2a2a);
		padding: 12px 24px;
		border-radius: 4px;
		border: 1px solid var(--border-color, #333);
	}

	.selection-count {
		font-weight: 600;
		color: var(--accent-yellow, #ffd60a);
		margin-right: 16px;
	}

	.selection-actions {
		display: flex;
		gap: 8px;
	}

	.btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		padding: 10px 20px;
		font-size: 0.875rem;
		font-weight: 600;
		border-radius: 4px;
		cursor: pointer;
		transition: all 150ms ease;
		white-space: nowrap;
	}

	.btn-small {
		padding: 6px 12px;
		font-size: 0.8125rem;
	}

	.btn-secondary {
		background: transparent;
		border: 1px solid var(--border-color, #333);
		color: var(--text-color, #eaeaea);
	}

	.btn-secondary:hover:not(:disabled) {
		background: var(--surface-hover, #2a2a2a);
		border-color: var(--text-muted, #9e9e9e);
	}

	.btn-danger {
		background: transparent;
		border: 1px solid var(--danger-color, #ff4d4d);
		color: var(--danger-color, #ff4d4d);
	}

	.btn-danger:hover:not(:disabled) {
		background: var(--danger-color, #ff4d4d);
		color: var(--text-color, #eaeaea);
	}

	.btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	@media (max-width: 1024px) {
		.toolbar {
			flex-direction: column;
			align-items: stretch;
		}

		.toolbar-left,
		.toolbar-center {
			width: 100%;
		}

		.selection-actions {
			flex-wrap: wrap;
		}
	}
</style>

<script lang="ts">
	import type { FileEntry } from '$lib/types';
	import { store } from '$lib/store.svelte';
	import GalleryItem from './GalleryItem.svelte';

	interface Props {
		onLoadMore?: () => void;
		hasMore?: boolean;
		isLoading?: boolean;
	}

	let { onLoadMore, hasMore = false, isLoading = false }: Props = $props();

	function handleOpenLightbox(fileId: string) {
		const index = store.files.findIndex((f: FileEntry) => f.id === fileId);
		if (index !== -1) {
			store.openLightbox(index);
		}
	}

	function isFileSelected(fileId: string): boolean {
		return store.selectedFiles.has(fileId);
	}
</script>

<div class="gallery-grid-container">
	{#if store.files.length === 0 && !isLoading}
		<div class="empty-state">
			<p>No files found</p>
			<p class="empty-hint">Try adjusting your filters or sync your gallery</p>
		</div>
	{:else}
		<div class="gallery-grid">
			{#each store.files as file (file.id)}
				<GalleryItem
					{file}
					isSelected={isFileSelected(file.id)}
					onOpenLightbox={handleOpenLightbox}
				/>
			{/each}
		</div>

		{#if hasMore}
			<div class="load-more-container">
				<button class="btn btn-secondary load-more-btn" onclick={onLoadMore} disabled={isLoading}>
					{isLoading ? 'Loading...' : 'Load More'}
				</button>
			</div>
		{/if}
	{/if}
</div>

<style>
	.gallery-grid-container {
		width: 100%;
		padding: 16px;
	}

	.gallery-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
		gap: 16px;
		width: 100%;
	}

	@media (min-width: 1400px) {
		.gallery-grid {
			grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
		}
	}

	@media (max-width: 768px) {
		.gallery-grid {
			grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
			gap: 12px;
		}

		.gallery-grid-container {
			padding: 12px;
		}
	}

	.empty-state {
		text-align: center;
		padding: 64px 16px;
		color: var(--text-muted, #9e9e9e);
	}

	.empty-state p {
		margin: 8px 0;
	}

	.empty-state p:first-child {
		font-size: 1.25rem;
		font-weight: 600;
		color: var(--text-color, #eaeaea);
	}

	.empty-hint {
		font-size: 0.875rem;
	}

	.load-more-container {
		display: flex;
		justify-content: center;
		margin-top: 32px;
	}

	.btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		padding: 12px 24px;
		font-size: 0.875rem;
		font-weight: 600;
		border-radius: 4px;
		border: 1px solid var(--border-color, #333);
		background: var(--surface-color, #1e1e1e);
		color: var(--text-color, #eaeaea);
		cursor: pointer;
		transition: all 150ms ease;
	}

	.btn:hover:not(:disabled) {
		background: var(--surface-hover, #2a2a2a);
		border-color: var(--text-muted, #9e9e9e);
	}

	.btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.btn-secondary {
		min-width: 150px;
	}
</style>

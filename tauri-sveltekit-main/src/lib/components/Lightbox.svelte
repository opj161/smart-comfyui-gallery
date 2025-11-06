<script lang="ts">
	import { store } from '$lib/store.svelte';
	import * as api from '$lib/api';

	interface Props {
		isOpen: boolean;
	}

	let { isOpen }: Props = $props();

	let workflowMetadata = $state<any>(null);
	let isLoadingMetadata = $state(false);
	let showMetadata = $state(false);

	const currentFile = $derived(store.files[store.currentLightboxIndex] || null);

	const thumbnailUrl = $derived(currentFile ? `/api/thumbnail/${currentFile.id}` : null);

	// Load metadata when file changes
	$effect(() => {
		if (currentFile && showMetadata) {
			loadMetadata();
		}
	});

	async function loadMetadata() {
		if (!currentFile) return;

		isLoadingMetadata = true;
		try {
			workflowMetadata = await api.getWorkflowMetadata(currentFile.id);
		} catch (error) {
			console.error('Failed to load metadata:', error);
			workflowMetadata = null;
		} finally {
			isLoadingMetadata = false;
		}
	}

	function handlePrevious() {
		if (store.currentLightboxIndex > 0) {
			store.previousLightboxImage();
		}
	}

	function handleNext() {
		if (store.currentLightboxIndex < store.files.length - 1) {
			store.nextLightboxImage();
		}
	}

	function handleKeydown(event: KeyboardEvent) {
		if (!isOpen) return;

		switch (event.key) {
			case 'Escape':
				store.closeLightbox();
				break;
			case 'ArrowLeft':
				handlePrevious();
				break;
			case 'ArrowRight':
				handleNext();
				break;
			case 'i':
			case 'I':
				showMetadata = !showMetadata;
				break;
		}
	}

	function handleBackdropClick(event: MouseEvent) {
		if (event.target === event.currentTarget) {
			store.closeLightbox();
		}
	}
</script>

<svelte:window onkeydown={handleKeydown} />

{#if isOpen && currentFile}
	<div class="lightbox-overlay" onclick={handleBackdropClick}>
		<div class="lightbox-container">
			<!-- Close Button -->
			<button class="lightbox-close" onclick={() => store.closeLightbox()} aria-label="Close">
				<span class="close-icon">×</span>
			</button>

			<!-- Navigation Buttons -->
		{#if store.currentLightboxIndex > 0}
			<button class="lightbox-nav lightbox-prev" onclick={handlePrevious} aria-label="Previous">
				<span class="nav-icon">‹</span>
			</button>
		{/if}

		{#if store.currentLightboxIndex < store.lightboxFiles.length - 1}
			<button class="lightbox-nav lightbox-next" onclick={handleNext} aria-label="Next">
				<span class="nav-icon">›</span>
			</button>
		{/if}			<!-- Image -->
			<div class="lightbox-image-container">
				{#if currentFile.type === 'image'}
					<img src={thumbnailUrl} alt={currentFile.name} class="lightbox-image" />
				{:else if currentFile.type === 'video'}
					<video src={currentFile.path} controls class="lightbox-video"><track kind="captions" /></video>
				{:else}
					<div class="unsupported-type">Unsupported file type</div>
				{/if}
			</div>

			<!-- File Info Bar -->
			<div class="lightbox-info-bar">
				<div class="info-left">
					<span class="file-name">{currentFile.name}</span>
					{#if currentFile.dimensions}
						<span class="file-dimensions">{currentFile.dimensions}</span>
					{/if}
			</div>

			<div class="info-right">
				<span class="file-counter">
					{store.currentLightboxIndex + 1} / {store.lightboxFiles.length}
				</span>
				<button
					class="btn-icon"
					onclick={() => (showMetadata = !showMetadata)}
					aria-label="Toggle metadata"
					class:active={showMetadata}
				>
					<span>ℹ</span>
				</button>
			</div>
		</div>			<!-- Metadata Sidebar -->
			{#if showMetadata}
				<div class="metadata-sidebar">
					<h3>Metadata</h3>

					{#if isLoadingMetadata}
						<p class="loading">Loading metadata...</p>
					{:else if workflowMetadata && workflowMetadata.length > 0}
						{#each workflowMetadata as meta, index (index)}
							<div class="metadata-section">
								{#if workflowMetadata.length > 1}
									<h4>Sampler {index + 1}</h4>
								{/if}

								{#if meta.model_name}
									<div class="meta-item">
										<span class="meta-label">Model:</span>
										<span class="meta-value">{meta.model_name}</span>
									</div>
								{/if}

								{#if meta.sampler_name}
									<div class="meta-item">
										<span class="meta-label">Sampler:</span>
										<span class="meta-value">{meta.sampler_name}</span>
									</div>
								{/if}

								{#if meta.scheduler}
									<div class="meta-item">
										<span class="meta-label">Scheduler:</span>
										<span class="meta-value">{meta.scheduler}</span>
									</div>
								{/if}

								{#if meta.cfg}
									<div class="meta-item">
										<span class="meta-label">CFG:</span>
										<span class="meta-value">{meta.cfg}</span>
									</div>
								{/if}

								{#if meta.steps}
									<div class="meta-item">
										<span class="meta-label">Steps:</span>
										<span class="meta-value">{meta.steps}</span>
									</div>
								{/if}

								{#if meta.width && meta.height}
									<div class="meta-item">
										<span class="meta-label">Size:</span>
										<span class="meta-value">{meta.width} × {meta.height}</span>
									</div>
								{/if}

								{#if meta.positive_prompt}
									<div class="meta-item">
										<span class="meta-label">Positive Prompt:</span>
										<span class="meta-value prompt">{meta.positive_prompt}</span>
									</div>
								{/if}

								{#if meta.negative_prompt}
									<div class="meta-item">
										<span class="meta-label">Negative Prompt:</span>
										<span class="meta-value prompt">{meta.negative_prompt}</span>
									</div>
								{/if}
							</div>
						{/each}
					{:else}
						<p class="no-metadata">No workflow metadata available</p>
					{/if}
				</div>
			{/if}
		</div>
	</div>
{/if}

<style>
	.lightbox-overlay {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background: rgba(0, 0, 0, 0.95);
		z-index: 4000;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.lightbox-container {
		position: relative;
		width: 100%;
		height: 100%;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.lightbox-close {
		position: absolute;
		top: 16px;
		right: 16px;
		z-index: 4002;
		background: rgba(18, 18, 18, 0.8);
		border: 1px solid var(--border-color, #333);
		border-radius: 4px;
		padding: 8px 16px;
		cursor: pointer;
		transition: all 150ms ease;
		color: var(--text-color, #eaeaea);
	}

	.lightbox-close:hover {
		background: rgba(18, 18, 18, 0.95);
		border-color: var(--accent-yellow, #ffd60a);
	}

	.close-icon {
		font-size: 2rem;
		line-height: 1;
	}

	.lightbox-nav {
		position: absolute;
		top: 50%;
		transform: translateY(-50%);
		z-index: 4001;
		background: rgba(18, 18, 18, 0.8);
		border: 1px solid var(--border-color, #333);
		border-radius: 4px;
		padding: 16px 12px;
		cursor: pointer;
		transition: all 150ms ease;
		color: var(--text-color, #eaeaea);
	}

	.lightbox-nav:hover {
		background: rgba(18, 18, 18, 0.95);
		border-color: var(--accent-yellow, #ffd60a);
	}

	.lightbox-prev {
		left: 16px;
	}

	.lightbox-next {
		right: 16px;
	}

	.nav-icon {
		font-size: 2.5rem;
		line-height: 1;
		font-weight: 300;
	}

	.lightbox-image-container {
		max-width: 90%;
		max-height: 85vh;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.lightbox-image {
		max-width: 100%;
		max-height: 85vh;
		object-fit: contain;
	}

	.lightbox-video {
		max-width: 100%;
		max-height: 85vh;
	}

	.unsupported-type {
		color: var(--text-muted, #9e9e9e);
		font-size: 1.25rem;
	}

	.lightbox-info-bar {
		position: absolute;
		bottom: 0;
		left: 0;
		right: 0;
		background: rgba(18, 18, 18, 0.9);
		padding: 16px 24px;
		display: flex;
		justify-content: space-between;
		align-items: center;
		border-top: 1px solid var(--border-color, #333);
	}

	.info-left {
		display: flex;
		gap: 16px;
		align-items: center;
		flex: 1;
		min-width: 0;
	}

	.file-name {
		font-weight: 600;
		color: var(--text-color, #eaeaea);
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.file-dimensions {
		color: var(--text-muted, #9e9e9e);
		font-size: 0.875rem;
	}

	.info-right {
		display: flex;
		gap: 16px;
		align-items: center;
	}

	.file-counter {
		color: var(--text-muted, #9e9e9e);
		font-size: 0.875rem;
	}

	.btn-icon {
		background: transparent;
		border: 1px solid var(--border-color, #333);
		border-radius: 4px;
		padding: 6px 12px;
		cursor: pointer;
		transition: all 150ms ease;
		color: var(--text-color, #eaeaea);
		font-size: 1.25rem;
	}

	.btn-icon:hover,
	.btn-icon.active {
		background: var(--surface-color, #1e1e1e);
		border-color: var(--accent-yellow, #ffd60a);
	}

	.metadata-sidebar {
		position: absolute;
		right: 0;
		top: 0;
		bottom: 0;
		width: 400px;
		background: var(--surface-color, #1e1e1e);
		border-left: 1px solid var(--border-color, #333);
		overflow-y: auto;
		padding: 24px;
		z-index: 4001;
	}

	.metadata-sidebar h3 {
		margin: 0 0 16px 0;
		color: var(--text-color, #eaeaea);
		font-size: 1.25rem;
	}

	.metadata-sidebar h4 {
		margin: 16px 0 12px 0;
		color: var(--accent-yellow, #ffd60a);
		font-size: 1rem;
		padding-top: 16px;
		border-top: 1px solid var(--border-color, #333);
	}

	.metadata-sidebar h4:first-of-type {
		margin-top: 0;
		padding-top: 0;
		border-top: none;
	}

	.metadata-section {
		margin-bottom: 16px;
	}

	.meta-item {
		margin-bottom: 12px;
	}

	.meta-label {
		display: block;
		font-size: 0.75rem;
		color: var(--text-muted, #9e9e9e);
		text-transform: uppercase;
		letter-spacing: 0.5px;
		margin-bottom: 4px;
	}

	.meta-value {
		display: block;
		color: var(--text-color, #eaeaea);
		font-size: 0.875rem;
	}

	.meta-value.prompt {
		font-family: var(--font-monospace, monospace);
		font-size: 0.8125rem;
		line-height: 1.5;
		word-break: break-word;
	}

	.loading,
	.no-metadata {
		color: var(--text-muted, #9e9e9e);
		font-size: 0.875rem;
	}
</style>

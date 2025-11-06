<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { listen } from '@tauri-apps/api/event';

	let isDragging = $state(false);
	let isUploading = $state(false);
	let uploadProgress = $state(0);
	let uploadTotal = $state(0);
	let currentFile = $state('');
	let { onComplete }: { onComplete: () => void } = $props();
	let fileInput = $state<HTMLInputElement | undefined>(undefined);

	async function handleDrop(event: DragEvent) {
		event.preventDefault();
		isDragging = false;

		const files = event.dataTransfer?.files;
		if (!files || files.length === 0) return;

		const filePaths: string[] = [];
		for (let i = 0; i < files.length; i++) {
			const file = files[i];
			// @ts-ignore - Tauri adds path property
			if (file.path) {
				// @ts-ignore
				filePaths.push(file.path);
			}
		}

		if (filePaths.length > 0) {
			await uploadFiles(filePaths);
		}
	}

	async function handleFileSelect(event: Event) {
		const target = event.target as HTMLInputElement;
		const files = target.files;
		if (!files || files.length === 0) return;

		const filePaths: string[] = [];
		for (let i = 0; i < files.length; i++) {
			const file = files[i];
			// @ts-ignore - Tauri adds path property
			if (file.path) {
				// @ts-ignore
				filePaths.push(file.path);
			}
		}

		if (filePaths.length > 0) {
			await uploadFiles(filePaths);
		}
	}

	async function uploadFiles(paths: string[]) {
		isUploading = true;
		uploadProgress = 0;
		uploadTotal = paths.length;

		// Listen for progress
		const unlisten = await listen('upload-progress', (event: any) => {
			uploadProgress = event.payload.current;
			currentFile = event.payload.filename || '';
		});

		try {
			await invoke('upload_multiple_files', { sourcePaths: paths });
			onComplete();
		} catch (error) {
			alert(`Upload failed: ${error}`);
		} finally {
			isUploading = false;
			unlisten();
		}
	}
</script>

<div
	class="upload-zone"
	class:dragging={isDragging}
	ondragover={(e) => {
		e.preventDefault();
		isDragging = true;
	}}
	ondragleave={() => {
		isDragging = false;
	}}
	ondrop={handleDrop}
	role="button"
	tabindex="0"
>
	{#if isUploading}
		<div class="upload-progress">
			<div class="spinner"></div>
			<p class="uploading-text">Uploading {currentFile}</p>
			<progress value={uploadProgress} max={uploadTotal}></progress>
			<p class="progress-text">{uploadProgress} / {uploadTotal} files</p>
		</div>
	{:else}
		<div class="upload-prompt">
			<svg
				width="64"
				height="64"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
			>
				<path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
				<polyline points="17 8 12 3 7 8" />
				<line x1="12" y1="3" x2="12" y2="15" />
			</svg>
			<p class="main-text">Drag and drop files here</p>
			<p class="or-text">or</p>
			<input
				bind:this={fileInput}
				type="file"
				multiple
				accept=".png,.jpg,.jpeg,.webp,.mp4,.avi,.mov,.mkv,.webm,.gif"
				onchange={handleFileSelect}
				style="display: none;"
			/>
			<button class="select-button" onclick={() => fileInput?.click()}>Select Files</button>
			<p class="supported-text">Supports: PNG, JPG, MP4, AVI, MOV, MKV, WebM, GIF</p>
		</div>
	{/if}
</div>

<style>
	.upload-zone {
		border: 2px dashed #666;
		border-radius: 12px;
		padding: 3rem 2rem;
		text-align: center;
		transition: all 0.3s ease;
		background: #1a1a1a;
		min-height: 250px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.upload-zone.dragging {
		border-color: #0066cc;
		background: rgba(0, 102, 204, 0.1);
		transform: scale(1.02);
	}

	.upload-prompt svg {
		color: #666;
		margin-bottom: 1.5rem;
	}

	.upload-zone.dragging svg {
		color: #0066cc;
		animation: bounce 0.6s ease-in-out infinite;
	}

	@keyframes bounce {
		0%,
		100% {
			transform: translateY(0);
		}
		50% {
			transform: translateY(-10px);
		}
	}

	.main-text {
		font-size: 1.25rem;
		font-weight: 600;
		color: #e0e0e0;
		margin: 0 0 0.5rem 0;
	}

	.or-text {
		color: #999;
		margin: 1rem 0;
		font-size: 0.9rem;
	}

	.select-button {
		padding: 0.875rem 2rem;
		background: #0066cc;
		color: white;
		border: none;
		border-radius: 6px;
		cursor: pointer;
		font-size: 1rem;
		font-weight: 500;
		transition: all 0.2s;
	}

	.select-button:hover {
		background: #0052a3;
		transform: translateY(-1px);
		box-shadow: 0 4px 12px rgba(0, 102, 204, 0.3);
	}

	.supported-text {
		font-size: 0.85rem;
		color: #666;
		margin-top: 1.5rem;
	}

	.upload-progress {
		width: 100%;
		max-width: 400px;
	}

	.spinner {
		width: 48px;
		height: 48px;
		border: 4px solid #333;
		border-top-color: #0066cc;
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin: 0 auto 1.5rem;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.uploading-text {
		font-size: 1.1rem;
		color: #e0e0e0;
		margin-bottom: 1rem;
		font-weight: 500;
	}

	progress {
		width: 100%;
		height: 24px;
		margin: 1rem 0;
		border-radius: 12px;
		overflow: hidden;
	}

	progress::-webkit-progress-bar {
		background: #2a2a2a;
		border-radius: 12px;
	}

	progress::-webkit-progress-value {
		background: linear-gradient(90deg, #0066cc, #0088ff);
		border-radius: 12px;
		transition: width 0.3s ease;
	}

	progress::-moz-progress-bar {
		background: linear-gradient(90deg, #0066cc, #0088ff);
		border-radius: 12px;
	}

	.progress-text {
		font-size: 0.95rem;
		color: #999;
		margin-top: 0.5rem;
	}
</style>

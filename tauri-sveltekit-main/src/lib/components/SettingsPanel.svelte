<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import type { AppConfig } from '$lib/types';

	let config: AppConfig | null = $state(null);
	let isLoading = $state(true);
	let error = $state<string | null>(null);
	let isSaving = $state(false);
	let { onClose }: { onClose: () => void } = $props();

	// Load config on mount
	async function loadConfig() {
		try {
			config = await invoke('load_config');
			isLoading = false;
		} catch (e) {
			error = `Failed to load config: ${e}`;
			isLoading = false;
		}
	}

	// Save config
	async function saveConfig() {
		if (!config) return;

		isSaving = true;
		error = null;
		try {
			await invoke('save_config', { config });
			// Trigger app reinitialization
			if (config.output_path) {
				await invoke('initialize_gallery', {
					outputPath: config.output_path,
					inputPath: config.input_path || null
				});
			}
			alert('Settings saved! Gallery will reload.');
			onClose();
		} catch (e) {
			error = `Failed to save config: ${e}`;
		} finally {
			isSaving = false;
		}
	}

	// Initialize
	loadConfig();
</script>

<div class="settings-overlay" onclick={onClose} role="button" tabindex="0">
	<div class="settings-panel" onclick={(e) => e.stopPropagation()} role="dialog">
		<div class="settings-header">
			<h2>Settings</h2>
			<button class="close-button" onclick={onClose}>×</button>
		</div>

		<div class="settings-content">
			{#if isLoading}
				<p class="loading">Loading settings...</p>
			{:else if error}
				<p class="error">{error}</p>
			{:else if config}
				<form onsubmit={(e) => { e.preventDefault(); saveConfig(); }}>
					<!-- Output Path -->
					<div class="form-group">
						<label for="output-path">Output Directory *</label>
						<input
							id="output-path"
							type="text"
							bind:value={config.output_path}
							required
							placeholder="e.g., C:\\.ai\\ComfyUI\\output"
						/>
						<p class="hint">ComfyUI output folder (required)</p>
					</div>

					<!-- Input Path -->
					<div class="form-group">
						<label for="input-path">Input Directory</label>
						<input
							id="input-path"
							type="text"
							bind:value={config.input_path}
							placeholder="e.g., C:\\.ai\\ComfyUI\\input (optional)"
						/>
						<p class="hint">ComfyUI input folder (optional)</p>
					</div>

					<!-- Thumbnail Size -->
					<div class="form-group">
						<label for="thumbnail-size">Thumbnail Size</label>
						<input
							id="thumbnail-size"
							type="number"
							bind:value={config.thumbnail_size}
							min="128"
							max="512"
							step="64"
						/>
						<p class="hint">Width in pixels (128-512)</p>
					</div>

					<!-- Theme -->
					<div class="form-group">
						<label for="theme">Theme</label>
						<select id="theme" bind:value={config.theme}>
							<option value="dark">Dark</option>
							<option value="light">Light</option>
						</select>
					</div>

					<!-- Cache Size -->
					<div class="form-group">
						<label for="cache-size">Max Cache Size (MB)</label>
						<input
							id="cache-size"
							type="number"
							bind:value={config.max_cache_size_mb}
							min="100"
							max="10000"
							step="100"
						/>
						<p class="hint">Maximum memory for thumbnails</p>
					</div>

					<!-- Actions -->
					<div class="actions">
						<button type="button" class="cancel-button" onclick={onClose}>Cancel</button>
						<button type="submit" class="save-button" disabled={isSaving}>
							{isSaving ? 'Saving...' : 'Save Settings'}
						</button>
					</div>
				</form>
			{/if}
		</div>
	</div>
</div>

<style>
	.settings-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.75);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
		animation: fadeIn 0.2s ease-out;
	}

	@keyframes fadeIn {
		from {
			opacity: 0;
		}
		to {
			opacity: 1;
		}
	}

	.settings-panel {
		background: #1a1a1a;
		border-radius: 12px;
		max-width: 600px;
		width: 90%;
		max-height: 90vh;
		overflow: hidden;
		display: flex;
		flex-direction: column;
		box-shadow: 0 8px 32px rgba(0, 0, 0, 0.5);
		animation: slideUp 0.3s ease-out;
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

	.settings-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1.5rem;
		border-bottom: 1px solid #333;
	}

	.settings-header h2 {
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

	.settings-content {
		padding: 1.5rem;
		overflow-y: auto;
	}

	.form-group {
		margin-bottom: 1.5rem;
	}

	label {
		display: block;
		margin-bottom: 0.5rem;
		font-weight: 600;
		color: #e0e0e0;
	}

	input,
	select {
		width: 100%;
		padding: 0.75rem;
		border: 1px solid #444;
		border-radius: 6px;
		background: #2a2a2a;
		color: #e0e0e0;
		font-size: 0.95rem;
		transition: border-color 0.2s;
	}

	input:focus,
	select:focus {
		outline: none;
		border-color: #0066cc;
	}

	.path-input {
		display: flex;
		gap: 0.5rem;
	}

	.path-input input {
		flex: 1;
	}

	.browse-button {
		padding: 0.75rem 1.25rem;
		background: #0066cc;
		color: white;
		border: none;
		border-radius: 6px;
		cursor: pointer;
		font-weight: 500;
		transition: background 0.2s;
		white-space: nowrap;
	}

	.browse-button:hover {
		background: #0052a3;
	}

	.hint {
		margin-top: 0.35rem;
		font-size: 0.85rem;
		color: #999;
	}

	.loading,
	.error {
		padding: 1rem;
		border-radius: 6px;
		text-align: center;
	}

	.loading {
		color: #999;
	}

	.error {
		color: #ff5555;
		background: rgba(255, 85, 85, 0.1);
	}

	.actions {
		display: flex;
		gap: 1rem;
		margin-top: 2rem;
		padding-top: 1.5rem;
		border-top: 1px solid #333;
	}

	.cancel-button,
	.save-button {
		flex: 1;
		padding: 0.875rem;
		border: none;
		border-radius: 6px;
		font-size: 1rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.cancel-button {
		background: #333;
		color: #e0e0e0;
	}

	.cancel-button:hover {
		background: #444;
	}

	.save-button {
		background: #00aa00;
		color: white;
	}

	.save-button:hover:not(:disabled) {
		background: #008800;
	}

	.save-button:disabled {
		background: #666;
		cursor: not-allowed;
		opacity: 0.6;
	}
</style>

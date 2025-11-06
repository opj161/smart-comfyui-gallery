<script lang="ts">
	import { toast, type Toast } from '$lib/stores/toast.svelte';
	import { fly, fade } from 'svelte/transition';
	import { flip } from 'svelte/animate';

	function getIcon(type: string): string {
		switch (type) {
			case 'success':
				return '✓';
			case 'error':
				return '✕';
			case 'warning':
				return '⚠';
			case 'info':
				return 'ℹ';
			default:
				return '';
		}
	}

	function handleDismiss(id: string) {
		toast.dismiss(id);
	}

	function handleKeyDown(event: KeyboardEvent, id: string) {
		if (event.key === 'Enter' || event.key === ' ') {
			event.preventDefault();
			handleDismiss(id);
		}
	}
</script>

<div class="toast-container" aria-live="polite" aria-atomic="true">
	{#each toast.list as item (item.id)}
		<div
			class="toast toast-{item.type}"
			role="alert"
			transition:fly={{ x: 300, duration: 300 }}
			animate:flip={{ duration: 200 }}
		>
			<div class="toast-icon">{getIcon(item.type)}</div>
			<div class="toast-message">{item.message}</div>
			{#if item.dismissible}
				<button
					class="toast-close"
					onclick={() => handleDismiss(item.id)}
					onkeydown={(e) => handleKeyDown(e, item.id)}
					aria-label="Dismiss notification"
					type="button"
				>
					×
				</button>
			{/if}
		</div>
	{/each}
</div>

<style>
	.toast-container {
		position: fixed;
		top: 20px;
		right: 20px;
		z-index: 10000;
		display: flex;
		flex-direction: column;
		gap: 12px;
		pointer-events: none;
		max-width: 400px;
	}

	.toast {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 16px;
		border-radius: 8px;
		background: var(--surface);
		border: 1px solid var(--border);
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
		pointer-events: all;
		min-width: 300px;
		backdrop-filter: blur(8px);
	}

	.toast-success {
		background: rgba(34, 197, 94, 0.1);
		border-color: rgb(34, 197, 94);
		color: rgb(34, 197, 94);
	}

	.toast-error {
		background: rgba(239, 68, 68, 0.1);
		border-color: rgb(239, 68, 68);
		color: rgb(239, 68, 68);
	}

	.toast-warning {
		background: rgba(251, 191, 36, 0.1);
		border-color: rgb(251, 191, 36);
		color: rgb(251, 191, 36);
	}

	.toast-info {
		background: rgba(59, 130, 246, 0.1);
		border-color: rgb(59, 130, 246);
		color: rgb(59, 130, 246);
	}

	.toast-icon {
		font-size: 20px;
		font-weight: bold;
		flex-shrink: 0;
		width: 24px;
		height: 24px;
		display: flex;
		align-items: center;
		justify-content: center;
		border-radius: 50%;
		background: currentColor;
		color: var(--surface);
	}

	.toast-message {
		flex: 1;
		font-size: 14px;
		line-height: 1.5;
		color: var(--text);
	}

	.toast-close {
		background: none;
		border: none;
		color: var(--text-secondary);
		font-size: 24px;
		line-height: 1;
		cursor: pointer;
		padding: 0;
		width: 24px;
		height: 24px;
		display: flex;
		align-items: center;
		justify-content: center;
		border-radius: 4px;
		transition: background 0.2s, color 0.2s;
		flex-shrink: 0;
	}

	.toast-close:hover {
		background: rgba(255, 255, 255, 0.1);
		color: var(--text);
	}

	.toast-close:focus {
		outline: 2px solid var(--accent);
		outline-offset: 2px;
	}

	@keyframes slideIn {
		from {
			transform: translateX(100%);
			opacity: 0;
		}
		to {
			transform: translateX(0);
			opacity: 1;
		}
	}

	@keyframes slideOut {
		from {
			transform: translateX(0);
			opacity: 1;
		}
		to {
			transform: translateX(100%);
			opacity: 0;
		}
	}

	/* Mobile responsive */
	@media (max-width: 640px) {
		.toast-container {
			right: 10px;
			left: 10px;
			max-width: none;
		}

		.toast {
			min-width: 0;
		}
	}
</style>

<div class="skeleton-card">
	<div class="skeleton-image shimmer"></div>
	<div class="skeleton-content">
		<div class="skeleton-title shimmer"></div>
		<div class="skeleton-metadata">
			<div class="skeleton-text shimmer"></div>
			<div class="skeleton-text shimmer short"></div>
		</div>
	</div>
</div>

<style>
	.skeleton-card {
		background: var(--surface, #1e1e1e);
		border: 1px solid var(--border, #333);
		border-radius: 8px;
		overflow: hidden;
		height: 320px;
		display: flex;
		flex-direction: column;
	}

	.skeleton-image {
		width: 100%;
		height: 200px;
		background: rgba(255, 255, 255, 0.05);
	}

	.skeleton-content {
		padding: 12px;
		flex: 1;
		display: flex;
		flex-direction: column;
		gap: 12px;
	}

	.skeleton-title {
		height: 20px;
		background: rgba(255, 255, 255, 0.05);
		border-radius: 4px;
		width: 70%;
	}

	.skeleton-metadata {
		display: flex;
		flex-direction: column;
		gap: 8px;
		margin-top: auto;
	}

	.skeleton-text {
		height: 14px;
		background: rgba(255, 255, 255, 0.05);
		border-radius: 4px;
		width: 100%;
	}

	.skeleton-text.short {
		width: 60%;
	}

	.shimmer {
		background: linear-gradient(
			90deg,
			rgba(255, 255, 255, 0.03) 0%,
			rgba(255, 255, 255, 0.08) 50%,
			rgba(255, 255, 255, 0.03) 100%
		);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite ease-in-out;
	}

	@keyframes shimmer {
		0% {
			background-position: -200% 0;
		}
		100% {
			background-position: 200% 0;
		}
	}

	/* Responsive */
	@media (max-width: 768px) {
		.skeleton-card {
			height: 280px;
		}

		.skeleton-image {
			height: 160px;
		}
	}
</style>

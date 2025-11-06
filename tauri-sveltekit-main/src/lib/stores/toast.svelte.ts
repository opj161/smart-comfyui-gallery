// Toast notification store for global toast management
// Supports success, error, warning, and info messages with auto-dismiss

export type ToastType = 'success' | 'error' | 'warning' | 'info';

export interface Toast {
	id: string;
	type: ToastType;
	message: string;
	duration?: number; // milliseconds, 0 = no auto-dismiss
	dismissible?: boolean;
}

class ToastStore {
	private toasts = $state<Toast[]>([]);
	private nextId = 0;

	get list(): Toast[] {
		return this.toasts;
	}

	add(toast: Omit<Toast, 'id'>): string {
		const id = `toast-${this.nextId++}-${Date.now()}`;
		const newToast: Toast = {
			id,
			duration: 5000, // default 5 seconds
			dismissible: true,
			...toast
		};

		this.toasts = [...this.toasts, newToast];

		// Auto-dismiss if duration is set
		if (newToast.duration && newToast.duration > 0) {
			setTimeout(() => {
				this.dismiss(id);
			}, newToast.duration);
		}

		return id;
	}

	success(message: string, duration?: number): string {
		return this.add({ type: 'success', message, duration });
	}

	error(message: string, duration?: number): string {
		return this.add({ type: 'error', message, duration: duration ?? 7000 }); // Errors stay longer
	}

	warning(message: string, duration?: number): string {
		return this.add({ type: 'warning', message, duration });
	}

	info(message: string, duration?: number): string {
		return this.add({ type: 'info', message, duration });
	}

	dismiss(id: string): void {
		this.toasts = this.toasts.filter((t) => t.id !== id);
	}

	clear(): void {
		this.toasts = [];
	}
}

// Export singleton instance
export const toast = new ToastStore();

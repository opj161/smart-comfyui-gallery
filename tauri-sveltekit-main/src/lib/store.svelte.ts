// Global application state using Svelte 5 runes
import type { FileEntry, WorkflowMetadata, FilterOptions, SyncProgress } from '$lib/types';

// Create a reactive store class
class AppStore {
	// Gallery state
	files = $state<FileEntry[]>([]);
	totalCount = $state(0);
	hasMore = $state(false);
	currentPage = $state(0);
	perPage = $state(50);

	// Selection state
	selectedFiles = $state<Set<string>>(new Set());

	// Filter state
	filters = $state({
		search: '',
		favoritesOnly: false,
		favorites_only: false, // Alias for backend compatibility
		fileTypes: [] as string[],
		extensions: [] as string[],
		model: null as string | null,
		sampler: null as string | null,
		scheduler: null as string | null,
		cfg_min: null as number | null,
		cfg_max: null as number | null,
		steps_min: null as number | null,
		steps_max: null as number | null,
		width: null as number | null,
		height: null as number | null
	});

	// Filter options (available values)
	filterOptions = $state<FilterOptions>({
		models: [],
		samplers: [],
		schedulers: [],
		extensions: [],
		prefixes: []
	});

	// Lightbox state
	isLightboxOpen = $state(false);
	currentLightboxIndex = $state(0);
	lightboxFiles = $state<FileEntry[]>([]);

	// Sync state
	isSyncing = $state(false);
	syncProgress = $state<SyncProgress | null>(null);

	// UI state
	isFilterPanelOpen = $state(false);
	viewMode = $state<'grid' | 'list'>('grid');

	// Current file details
	currentFile = $state<FileEntry | null>(null);
	currentWorkflowMetadata = $state<WorkflowMetadata[]>([]);

	// Methods to update state
	setFiles(newFiles: FileEntry[], total: number, more: boolean) {
		this.files = newFiles;
		this.totalCount = total;
		this.hasMore = more;
	}

	appendFiles(newFiles: FileEntry[], total: number, more: boolean) {
		this.files = [...this.files, ...newFiles];
		this.totalCount = total;
		this.hasMore = more;
	}

	clearSelection() {
		this.selectedFiles = new Set();
	}

	toggleFileSelection(fileId: string) {
		const newSelection = new Set(this.selectedFiles);
		if (newSelection.has(fileId)) {
			newSelection.delete(fileId);
		} else {
			newSelection.add(fileId);
		}
		this.selectedFiles = newSelection;
	}

	selectAll() {
		this.selectedFiles = new Set(this.files.map((file) => file.id));
	}

	openLightbox(index: number, fileList?: FileEntry[]) {
		this.currentLightboxIndex = index;
		this.lightboxFiles = fileList || this.files;
		this.isLightboxOpen = true;
	}

	closeLightbox() {
		this.isLightboxOpen = false;
	}

	nextLightboxImage() {
		if (this.currentLightboxIndex < this.lightboxFiles.length - 1) {
			this.currentLightboxIndex++;
		}
	}

	previousLightboxImage() {
		if (this.currentLightboxIndex > 0) {
			this.currentLightboxIndex--;
		}
	}

	setSyncProgress(progress: SyncProgress) {
		this.syncProgress = progress;
		this.isSyncing = progress.status === 'processing';
	}

	clearSyncProgress() {
		this.syncProgress = null;
		this.isSyncing = false;
	}

	setCurrentFile(file: FileEntry | null) {
		this.currentFile = file;
	}

	setCurrentWorkflowMetadata(metadata: WorkflowMetadata[]) {
		this.currentWorkflowMetadata = metadata;
	}
}

// Export singleton instance
export const store = new AppStore();

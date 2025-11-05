// Global application state using Svelte 5 runes
import type { FileEntry, WorkflowMetadata, FilterOptions, SyncProgress } from '$lib/types';

// Gallery state
export let files = $state<FileEntry[]>([]);
export let totalCount = $state(0);
export let hasMore = $state(false);
export let currentPage = $state(0);
export let perPage = $state(50);

// Selection state
export let selectedFiles = $state<Set<string>>(new Set());

// Filter state
export let filters = $state({
	search: '',
	favoritesOnly: false,
	fileTypes: [] as string[],
	extensions: [] as string[],
	model: null as string | null,
	sampler: null as string | null,
	scheduler: null as string | null,
});

// Filter options (available values)
export let filterOptions = $state<FilterOptions>({
	models: [],
	samplers: [],
	schedulers: [],
	extensions: [],
	prefixes: [],
});

// Lightbox state
export let isLightboxOpen = $state(false);
export let currentLightboxIndex = $state(0);
export let lightboxFiles = $state<FileEntry[]>([]);

// Sync state
export let isSyncing = $state(false);
export let syncProgress = $state<SyncProgress | null>(null);

// UI state
export let isFilterPanelOpen = $state(false);
export let viewMode = $state<'grid' | 'list'>('grid');

// Current file details
export let currentFile = $state<FileEntry | null>(null);
export let currentWorkflowMetadata = $state<WorkflowMetadata[]>([]);

// Functions to update state
export function setFiles(newFiles: FileEntry[], total: number, more: boolean) {
	files = newFiles;
	totalCount = total;
	hasMore = more;
}

export function appendFiles(newFiles: FileEntry[], total: number, more: boolean) {
	files = [...files, ...newFiles];
	totalCount = total;
	hasMore = more;
}

export function clearSelection() {
	selectedFiles = new Set();
}

export function toggleFileSelection(fileId: string) {
	const newSelection = new Set(selectedFiles);
	if (newSelection.has(fileId)) {
		newSelection.delete(fileId);
	} else {
		newSelection.add(fileId);
	}
	selectedFiles = newSelection;
}

export function selectAll() {
	selectedFiles = new Set(files.map(file => file.id));
}

export function openLightbox(index: number, fileList?: FileEntry[]) {
	currentLightboxIndex = index;
	lightboxFiles = fileList || files;
	isLightboxOpen = true;
}

export function closeLightbox() {
	isLightboxOpen = false;
}

export function nextLightboxImage() {
	if (currentLightboxIndex < lightboxFiles.length - 1) {
		currentLightboxIndex++;
	}
}

export function previousLightboxImage() {
	if (currentLightboxIndex > 0) {
		currentLightboxIndex--;
	}
}

export function setSyncProgress(progress: SyncProgress) {
	syncProgress = progress;
	isSyncing = progress.status === 'processing';
}

export function clearSyncProgress() {
	syncProgress = null;
	isSyncing = false;
}

export function setCurrentFile(file: FileEntry | null) {
	currentFile = file;
}

export function setCurrentWorkflowMetadata(metadata: WorkflowMetadata[]) {
	currentWorkflowMetadata = metadata;
}

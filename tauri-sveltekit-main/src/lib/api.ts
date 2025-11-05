// API wrapper for Tauri commands
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import type {
	FileEntry,
	WorkflowMetadata,
	PaginatedFiles,
	FilterOptions,
	SyncProgress,
	GalleryFilters
} from '$lib/types';

// Initialize gallery
export async function initializeGallery(
	outputPath: string,
	inputPath?: string
): Promise<string> {
	return await invoke('initialize_gallery', { outputPath, inputPath });
}

// Get paginated files
export async function getFiles(
	folderKey: string | null,
	page: number,
	perPage: number
): Promise<PaginatedFiles> {
	return await invoke('get_files', { folderKey, page, perPage });
}

// Get single file
export async function getFileById(fileId: string): Promise<FileEntry | null> {
	return await invoke('get_file_by_id', { fileId });
}

// Get workflow metadata
export async function getWorkflowMetadata(fileId: string): Promise<WorkflowMetadata[]> {
	return await invoke('get_workflow_metadata', { fileId });
}

// Toggle favorite
export async function toggleFavorite(fileId: string): Promise<boolean> {
	return await invoke('toggle_favorite', { fileId });
}

// Batch favorite
export async function batchFavorite(fileIds: string[], favorite: boolean): Promise<void> {
	return await invoke('batch_favorite', { fileIds, favorite });
}

// Delete file
export async function deleteFile(fileId: string): Promise<void> {
	return await invoke('delete_file', { fileId });
}

// Batch delete
export async function batchDelete(fileIds: string[]): Promise<void> {
	return await invoke('batch_delete', { fileIds });
}

// Sync files
export async function syncFiles(): Promise<string> {
	return await invoke('sync_files');
}

// Get stats
export async function getStats(): Promise<{
	total_files: number;
	favorites: number;
	with_workflow: number;
}> {
	return await invoke('get_stats');
}

// Get thumbnail path
export async function getThumbnailPath(fileId: string): Promise<string | null> {
	return await invoke('get_thumbnail_path', { fileId });
}

// Health check
export async function healthCheck(): Promise<string> {
	return await invoke('health_check');
}

// Get filter options
export async function getFilterOptions(): Promise<FilterOptions> {
	return await invoke('get_filter_options');
}

// Rename file
export async function renameFile(fileId: string, newName: string): Promise<void> {
	return await invoke('rename_file', { fileId, newName });
}

// Move files
export async function moveFiles(fileIds: string[], targetFolder: string): Promise<void> {
	return await invoke('move_files', { fileIds, targetFolder });
}

// Search files
export async function searchFiles(
	query: string,
	page: number,
	perPage: number
): Promise<PaginatedFiles> {
	return await invoke('search_files', { query, page, perPage });
}

// Get files with advanced filtering
export async function getFilesFiltered(
	filters: GalleryFilters,
	page: number,
	perPage: number
): Promise<PaginatedFiles> {
	return await invoke('get_files_filtered', { filters, page, perPage });
}

// Create folder
export async function createFolder(folderPath: string): Promise<string> {
	return await invoke('create_folder', { folderPath });
}

// Get configuration
export async function getConfig(): Promise<{
	output_path: string | null;
	input_path: string | null;
	initialized: boolean;
}> {
	return await invoke('get_config');
}

// Listen to sync progress events
export function listenToSyncProgress(callback: (progress: SyncProgress) => void) {
	return listen<SyncProgress>('sync-progress', (event) => {
		callback(event.payload);
	});
}

// Listen to sync complete events
export function listenToSyncComplete(callback: (stats: any) => void) {
	return listen('sync-complete', (event) => {
		callback(event.payload);
	});
}

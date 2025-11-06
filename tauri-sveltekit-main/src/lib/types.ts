// Type definitions for SmartGallery application

export interface FileEntry {
	id: string;
	path: string;
	name: string;
	type: string; // 'image' | 'video' | 'other'
	extension: string;
	mtime: number;
	size: number;
	has_workflow: boolean;
	is_favorite: boolean;
	prompt_preview: string | null;
	sampler_names: string | null;
	sampler_count: number;
	dimensions: string | null;
}

export interface WorkflowMetadata {
	id: number;
	file_id: string;
	sampler_index: number;
	model_name: string | null;
	sampler_name: string | null;
	scheduler: string | null;
	cfg: number | null;
	steps: number | null;
	positive_prompt: string;
	negative_prompt: string;
	width: number | null;
	height: number | null;
}

export interface PaginatedFiles {
	files: FileEntry[];
	total_count: number;
	has_more: boolean;
}

export interface FilterOptions {
	models: string[];
	samplers: string[];
	schedulers: string[];
	extensions: string[];
	prefixes: string[];
}

export interface GalleryFilters {
	search?: string;
	favorites_only?: boolean;
	model?: string | null;
	sampler?: string | null;
	scheduler?: string | null;
	cfg_min?: number | null;
	cfg_max?: number | null;
	steps_min?: number | null;
	steps_max?: number | null;
	width?: number | null;
	height?: number | null;
}

export interface SyncProgress {
	processed: number;
	total: number;
	current_file: string;
	status: 'processing' | 'complete' | 'error';
	message?: string;
}

export interface AppConfig {
	output_path: string;
	input_path: string | null;
	thumbnail_size: number;
	theme: string;
	max_cache_size_mb: number;
}

export interface UploadProgress {
	current: number;
	total: number;
	filename: string;
}

export interface UploadComplete {
	total: number;
	success: number;
	failed: number;
}

// Input validation and sanitization utilities
// Prevents security vulnerabilities like path traversal, SQL injection, etc.

/**
 * Sanitizes a filename by removing dangerous characters and path traversal attempts
 */
export function sanitizeFilename(filename: string): string {
	return filename
		.replace(/\.\./g, '') // Remove parent directory references
		.replace(/[\/\\]/g, '') // Remove path separators
		.replace(/[<>:"|?*\x00-\x1f]/g, '_') // Remove invalid filename characters
		.replace(/^\.+/, '') // Remove leading dots
		.trim()
		.slice(0, 255); // Limit to max filename length
}

/**
 * Validates a file system path
 */
export function validatePath(path: string): { valid: boolean; error?: string } {
	// Check for path traversal
	if (path.includes('..')) {
		return { valid: false, error: 'Path traversal detected' };
	}

	// Check for null bytes
	if (path.includes('\0')) {
		return { valid: false, error: 'Null byte in path' };
	}

	// Must be absolute path
	const isAbsolutePath = path.startsWith('/') || /^[a-zA-Z]:[/\\]/.test(path);
	if (!isAbsolutePath) {
		return { valid: false, error: 'Path must be absolute' };
	}

	// Check length
	if (path.length > 4096) {
		return { valid: false, error: 'Path too long' };
	}

	return { valid: true };
}

/**
 * Sanitizes search query to prevent injection attacks
 */
export function sanitizeSearchQuery(query: string): string {
	return query
		.replace(/['";]/g, '') // Remove SQL special characters
		.replace(/--|\/\*/g, '') // Remove SQL comment markers
		.replace(/<script>/gi, '') // Remove script tags
		.trim()
		.slice(0, 200); // Limit search length
}

/**
 * Validates an uploaded file
 */
export function validateUploadFile(file: File): { valid: boolean; error?: string } {
	// Check file size (max 100MB)
	const maxSize = 100 * 1024 * 1024;
	if (file.size > maxSize) {
		return { valid: false, error: 'File too large (max 100MB)' };
	}

	// Check file type
	const allowedTypes = [
		'image/png',
		'image/jpeg',
		'image/jpg',
		'image/webp',
		'image/gif',
		'video/mp4',
		'video/webm'
	];

	if (!allowedTypes.includes(file.type)) {
		return { valid: false, error: `Invalid file type: ${file.type}` };
	}

	// Validate filename
	const sanitized = sanitizeFilename(file.name);
	if (sanitized.length === 0) {
		return { valid: false, error: 'Invalid filename' };
	}

	return { valid: true };
}

/**
 * Debounces a function call
 */
export function debounce<T extends (...args: any[]) => any>(
	func: T,
	wait: number
): (...args: Parameters<T>) => void {
	let timeout: ReturnType<typeof setTimeout> | null = null;

	return function (...args: Parameters<T>) {
		if (timeout) clearTimeout(timeout);
		timeout = setTimeout(() => func(...args), wait);
	};
}

/**
 * Validates numeric input within a range
 */
export function validateNumber(
	value: string | number,
	min?: number,
	max?: number
): { valid: boolean; value?: number; error?: string } {
	const num = typeof value === 'string' ? parseFloat(value) : value;

	if (isNaN(num)) {
		return { valid: false, error: 'Invalid number' };
	}

	if (min !== undefined && num < min) {
		return { valid: false, error: `Value must be at least ${min}` };
	}

	if (max !== undefined && num > max) {
		return { valid: false, error: `Value must be at most ${max}` };
	}

	return { valid: true, value: num };
}

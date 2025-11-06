// Error handling utilities
// Provides standardized error handling and user-friendly error messages

export class AppError extends Error {
	constructor(
		message: string,
		public code: string,
		public details?: unknown
	) {
		super(message);
		this.name = 'AppError';
	}
}

/**
 * Converts various error types to user-friendly messages
 */
export function handleApiError(error: unknown): string {
	if (error instanceof AppError) {
		return error.message;
	}

	if (typeof error === 'string') {
		return error;
	}

	if (error instanceof Error) {
		return error.message;
	}

	if (error && typeof error === 'object' && 'message' in error) {
		return String(error.message);
	}

	return 'An unknown error occurred';
}

/**
 * Wraps an async operation with error handling
 */
export async function withErrorHandling<T>(
	operation: () => Promise<T>,
	errorMessage: string
): Promise<T> {
	try {
		return await operation();
	} catch (error) {
		const message = handleApiError(error);
		throw new AppError(`${errorMessage}: ${message}`, 'OPERATION_FAILED', error);
	}
}

/**
 * Error codes for different types of errors
 */
export const ErrorCodes = {
	// File system errors
	FILE_NOT_FOUND: 'FILE_NOT_FOUND',
	FILE_ACCESS_DENIED: 'FILE_ACCESS_DENIED',
	INVALID_PATH: 'INVALID_PATH',

	// Database errors
	DATABASE_ERROR: 'DATABASE_ERROR',
	QUERY_FAILED: 'QUERY_FAILED',

	// Validation errors
	VALIDATION_ERROR: 'VALIDATION_ERROR',
	INVALID_INPUT: 'INVALID_INPUT',

	// Network errors
	NETWORK_ERROR: 'NETWORK_ERROR',
	TIMEOUT: 'TIMEOUT',

	// General errors
	UNKNOWN_ERROR: 'UNKNOWN_ERROR',
	OPERATION_FAILED: 'OPERATION_FAILED'
} as const;

/**
 * Get user-friendly error message based on error code
 */
export function getFriendlyErrorMessage(code: string): string {
	const messages: Record<string, string> = {
		[ErrorCodes.FILE_NOT_FOUND]: 'The requested file could not be found',
		[ErrorCodes.FILE_ACCESS_DENIED]: 'Access to the file was denied',
		[ErrorCodes.INVALID_PATH]: 'The specified path is invalid',
		[ErrorCodes.DATABASE_ERROR]: 'A database error occurred',
		[ErrorCodes.QUERY_FAILED]: 'The database query failed',
		[ErrorCodes.VALIDATION_ERROR]: 'The input data is invalid',
		[ErrorCodes.INVALID_INPUT]: 'Please check your input and try again',
		[ErrorCodes.NETWORK_ERROR]: 'A network error occurred',
		[ErrorCodes.TIMEOUT]: 'The operation timed out',
		[ErrorCodes.UNKNOWN_ERROR]: 'An unexpected error occurred',
		[ErrorCodes.OPERATION_FAILED]: 'The operation failed'
	};

	return messages[code] || 'An error occurred';
}

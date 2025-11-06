# Next Implementation Steps - Priority Optimizations

## Current Status

**Completed (Commit bbea274):**
✅ Comprehensive 61-hour implementation plan created
✅ Toast notification system implemented (store + component)

**Remaining:** 11 improvements across 58 hours of work

---

## Immediate Next Steps (Priority Order)

### 1. Integrate Toast Notifications (2 hours)

**Files to Modify:**
- `src/routes/+page.svelte` - Add ToastManager component and toast calls

**Changes:**
```typescript
import ToastManager from '$lib/components/ToastManager.svelte';
import { toast } from '$lib/stores/toast.svelte';

// In async operations, replace console.log with toast:
try {
  await api.syncFiles();
  toast.success('Files synced successfully');
} catch (error) {
  toast.error(`Sync failed: ${error.message}`);
}
```

**Integration Points:**
- Initialize gallery (success/error)
- Sync files (progress, success, error)
- Upload files (progress, success, error)
- Delete files (success, error)
- Favorite/unfavorite (success)
- Settings save (success, error)

---

### 2. Skeleton Loading States (6 hours)

**Files to Create:**
- `src/lib/components/SkeletonCard.svelte` (skeleton UI component)

**Files to Modify:**
- `src/lib/components/GalleryGrid.svelte` (show skeletons while loading)

**Implementation:**

```svelte
<!-- SkeletonCard.svelte -->
<div class="skeleton-card">
  <div class="skeleton-image shimmer"></div>
  <div class="skeleton-title shimmer"></div>
  <div class="skeleton-metadata shimmer"></div>
</div>

<style>
  .shimmer {
    background: linear-gradient(
      90deg,
      rgba(255, 255, 255, 0.05) 0%,
      rgba(255, 255, 255, 0.1) 50%,
      rgba(255, 255, 255, 0.05) 100%
    );
    background-size: 200% 100%;
    animation: shimmer 1.5s infinite;
  }

  @keyframes shimmer {
    0% { background-position: -200% 0; }
    100% { background-position: 200% 0; }
  }
</style>
```

**In GalleryGrid.svelte:**
```svelte
{#if isLoading}
  {#each Array(12) as _}
    <SkeletonCard />
  {/each}
{:else}
  {#each files as file}
    <GalleryItem {file} />
  {/each}
{/if}
```

---

### 3. Debounce Search Input (2 hours)

**Files to Modify:**
- `src/routes/+page.svelte` (add debounce to search handler)

**Implementation:**
```typescript
let searchTimeout: ReturnType<typeof setTimeout> | null = null;
let searchInput = $state('');

function handleSearchInput(value: string) {
  searchInput = value;
  
  if (searchTimeout) {
    clearTimeout(searchTimeout);
  }
  
  searchTimeout = setTimeout(() => {
    store.filters.search = value;
    loadFiles(0); // Trigger search
  }, 300); // 300ms debounce
}

// In template:
<input 
  type="text" 
  value={searchInput}
  oninput={(e) => handleSearchInput(e.currentTarget.value)}
  placeholder="Search..."
/>
```

---

### 4. Image Lazy Loading (4 hours)

**Files to Modify:**
- `src/lib/components/GalleryItem.svelte` (add IntersectionObserver)

**Implementation:**
```typescript
let isVisible = $state(false);
let imgElement: HTMLElement;

onMount(() => {
  const observer = new IntersectionObserver(
    (entries) => {
      entries.forEach((entry) => {
        if (entry.isIntersecting) {
          isVisible = true;
          observer.disconnect();
        }
      });
    },
    {
      rootMargin: '200px', // Load 200px before viewport
      threshold: 0.01
    }
  );

  if (imgElement) {
    observer.observe(imgElement);
  }

  return () => observer.disconnect();
});

// In template:
<div bind:this={imgElement}>
  {#if isVisible}
    <img src={thumbnailUrl} alt={file.filename} />
  {:else}
    <div class="placeholder"></div>
  {/if}
</div>
```

---

### 5. CSP Implementation (3 hours)

**Files to Modify:**
- `src-tauri/tauri.conf.json` (add security configuration)

**Implementation:**
```json
{
  "tauri": {
    "security": {
      "csp": "default-src 'self'; img-src 'self' asset: https:; style-src 'self' 'unsafe-inline'; script-src 'self' 'unsafe-inline'"
    }
  }
}
```

**Testing:**
- Check browser console for CSP violations
- Verify all images load correctly
- Test all functionality works with CSP active

---

### 6. Input Sanitization Layer (4 hours)

**Files to Create:**
- `src/lib/utils/sanitize.ts` (validation functions)

**Implementation:**
```typescript
export function sanitizeFilename(filename: string): string {
  // Remove path traversal attempts
  return filename
    .replace(/\.\./g, '')
    .replace(/[\/\\]/g, '')
    .replace(/[<>:"|?*]/g, '_')
    .slice(0, 255); // Max filename length
}

export function validatePath(path: string): boolean {
  // Check for path traversal
  if (path.includes('..')) return false;
  
  // Check for null bytes
  if (path.includes('\0')) return false;
  
  // Must be absolute path
  if (!path.startsWith('/') && !path.match(/^[a-zA-Z]:\\/)) return false;
  
  return true;
}

export function sanitizeSearchQuery(query: string): string {
  // Remove SQL injection attempts
  return query
    .replace(/['";]/g, '')
    .replace(/--|\/\*/g, '')
    .slice(0, 200); // Max search length
}

export function validateUploadFile(file: File): { valid: boolean; error?: string } {
  // Check file size (max 100MB)
  if (file.size > 100 * 1024 * 1024) {
    return { valid: false, error: 'File too large (max 100MB)' };
  }
  
  // Check file type
  const allowedTypes = ['image/png', 'image/jpeg', 'image/webp', 'video/mp4'];
  if (!allowedTypes.includes(file.type)) {
    return { valid: false, error: 'Invalid file type' };
  }
  
  return { valid: true };
}
```

**Integration:**
- Use `sanitizeFilename` in upload operations
- Use `validatePath` in settings panel
- Use `sanitizeSearchQuery` in search input
- Use `validateUploadFile` in UploadZone component

---

### 7. Error Handling Standardization (6 hours)

**Files to Create:**
- `src-tauri/src/errors.rs` (Rust error types)
- `src/lib/utils/errors.ts` (TypeScript error handling)

**Rust Implementation:**
```rust
// src-tauri/src/errors.rs
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    DatabaseError(String),
    FileSystemError(String),
    ValidationError(String),
    NetworkError(String),
    NotFound(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            AppError::FileSystemError(msg) => write!(f, "File system error: {}", msg),
            AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            AppError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

// Conversion helpers
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::DatabaseError(err.to_string())
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::FileSystemError(err.to_string())
    }
}
```

**TypeScript Implementation:**
```typescript
// src/lib/utils/errors.ts
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
  
  return 'An unknown error occurred';
}

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
```

---

## Phase 2: Performance Optimizations (32 hours)

### 8. Virtual Scrolling (8 hours)

**Approach:** Use `svelte-virtual` or custom implementation

**Key Changes:**
- Only render visible items (viewport + buffer)
- Calculate visible range based on scroll position
- Maintain scroll position on updates
- Handle dynamic item heights

**Expected Result:**
- 1000 DOM nodes → ~30 DOM nodes
- 60 FPS scrolling with 10,000+ files
- Memory reduction: 80%

---

### 9. Database Query Optimization (6 hours)

**Rust Changes:**
```rust
// Add compound indices
CREATE INDEX idx_favorite_created ON files(favorite, created_at);
CREATE INDEX idx_model_created ON files(model_name, created_at);
CREATE INDEX idx_type_created ON files(file_type, created_at);

// Optimize queries
// Before: SELECT * FROM files WHERE favorite = 1 ORDER BY created_at DESC;
// After: Use index, select only needed columns
SELECT id, filename, thumbnail_path, created_at 
FROM files 
WHERE favorite = 1 
ORDER BY created_at DESC 
LIMIT 50;
```

**Batch Operations:**
```rust
// Use transactions for bulk inserts
let mut tx = pool.begin().await?;
for file in files {
    sqlx::query("INSERT INTO files (...) VALUES (...)")
        .execute(&mut *tx)
        .await?;
}
tx.commit().await?;
```

---

### 10. Logging Infrastructure (6 hours)

**Dependencies:**
```toml
[dependencies]
tracing = "0.1"
tracing-subscriber = "0.3"
```

**Setup:**
```rust
use tracing::{info, warn, error, instrument};

#[instrument(skip(state))]
#[tauri::command]
async fn sync_files(state: State<'_, AppState>) -> Result<ScanStats, String> {
    info!("Starting file sync");
    
    let stats = perform_sync().await?;
    
    info!(
        files_processed = stats.files_processed,
        duration_ms = stats.duration_ms,
        "Sync complete"
    );
    
    Ok(stats)
}
```

---

### 11. Thumbnail Parallelization (4 hours)

**Implementation:**
```rust
use rayon::prelude::*;

// Before: Sequential
for file in files {
    generate_thumbnail(&file)?;
}

// After: Parallel
let results: Vec<_> = files
    .par_iter()
    .map(|file| generate_thumbnail(file))
    .collect();
```

---

### 12. Background Job Queue (8 hours)

**Implementation:**
```rust
use tokio::sync::mpsc;

pub enum Job {
    GenerateThumbnail(String),
    SyncFiles,
    CleanupCache,
}

pub struct JobQueue {
    tx: mpsc::Sender<Job>,
}

impl JobQueue {
    pub async fn enqueue(&self, job: Job) -> Result<(), String> {
        self.tx.send(job).await
            .map_err(|e| format!("Failed to enqueue job: {}", e))
    }
}

// Worker
async fn job_worker(mut rx: mpsc::Receiver<Job>) {
    while let Some(job) = rx.recv().await {
        match job {
            Job::GenerateThumbnail(path) => {
                // Process in background
            },
            // ... other jobs
        }
    }
}
```

---

## Testing Checklist

After each implementation:

**Manual Tests:**
- ✅ Feature works as expected
- ✅ No console errors
- ✅ Performance is improved
- ✅ Accessibility maintained

**Build Tests:**
- ✅ TypeScript compiles with zero errors
- ✅ Rust compiles with zero errors
- ✅ No linting warnings

**Integration Tests:**
- ✅ Feature works with existing functionality
- ✅ No regressions introduced

---

## Priority Recommendations

**Do First (Maximum impact, minimum effort):**
1. Toast integration (2h) - Immediate UX improvement
2. Debounce search (2h) - Quick performance win
3. Skeleton loading (6h) - Perceived performance boost

**Do Second (High impact, moderate effort):**
4. Image lazy loading (4h) - Real performance improvement
5. Input sanitization (4h) - Security critical
6. CSP implementation (3h) - Security critical

**Do Third (Strategic investments):**
7. Error handling (6h) - Code quality foundation
8. Virtual scrolling (8h) - Handles scale
9. Database optimization (6h) - Long-term performance

**Do Fourth (Advanced optimizations):**
10. Logging infrastructure (6h) - Production monitoring
11. Thumbnail parallelization (4h) - Build time improvement
12. Background job queue (8h) - Advanced architecture

---

## Success Metrics

Track these metrics before and after implementation:

**Performance:**
- Initial page load time
- Search response time  
- Scroll FPS (use Chrome DevTools Performance)
- Memory usage (Chrome Task Manager)

**User Experience:**
- Time to first meaningful paint
- Time to interactive
- Perceived loading time (skeleton)
- Error feedback clarity (toasts)

**Code Quality:**
- Lines of code
- Test coverage percentage
- Number of TODO/FIXME comments
- Cyclomatic complexity

---

## Estimated Timeline

**Week 1:**
- Days 1-2: Toast integration + skeleton loading + debounce (10h)
- Days 3-4: Lazy loading + CSP + sanitization (11h)
- Day 5: Error handling (6h)

**Week 2:**
- Days 1-2: Virtual scrolling (8h)
- Days 3-4: Database optimization + logging (12h)
- Day 5: Parallelization + job queue (12h)

**Total: 10 working days (~61 hours)**

---

## Notes

- Each feature is independent and can be implemented separately
- Test thoroughly after each implementation
- Commit frequently with descriptive messages
- Update documentation as you go
- Get user feedback after Phase 1 before continuing to Phase 2

The foundation (toast system) is complete. Everything else builds on top of existing code without major refactoring.

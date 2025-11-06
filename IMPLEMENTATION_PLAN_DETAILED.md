# Detailed Implementation Plan: Priority Optimizations

## Overview
Implementing 12 high-priority improvements across Phase 1 (Quick Wins) and Phase 2 (Performance) optimizations.

**Total Estimated Effort:** 61 hours
**Implementation Order:** Prioritized by impact and dependencies

---

## Phase 1: Quick Wins (29 hours)

### 1. Toast Notification System (4 hours) ✅ NEXT
**Priority:** 🔴 Critical (UX feedback)
**Files to Create:**
- `src/lib/components/ToastManager.svelte` (120 lines)
- `src/lib/stores/toast.svelte.ts` (60 lines)

**Implementation:**
```typescript
// Toast types: success, error, warning, info
// Auto-dismiss after 5s (configurable)
// Stack multiple toasts
// Slide-in animation from top-right
```

**Integration Points:**
- Import in `+page.svelte`
- Use in all async operations (sync, upload, delete, etc.)
- Replace console.log with toast notifications

**Testing:**
- Manual: Trigger sync, upload, delete
- Verify animations, auto-dismiss, stacking

---

### 2. Skeleton Loading States (6 hours)
**Priority:** 🔴 High (perceived performance)
**Files to Modify:**
- `src/lib/components/GalleryGrid.svelte` (add skeleton cards)
- `src/lib/components/GalleryItem.svelte` (loading state)

**Implementation:**
```svelte
<!-- Show 12 skeleton cards while loading -->
{#if loading}
  {#each Array(12) as _}
    <SkeletonCard />
  {/each}
{/if}
```

**CSS:**
- Gradient shimmer animation
- Matches card dimensions
- Smooth fade-in transition

---

### 3. Debounce Search Input (2 hours)
**Priority:** 🔴 High (70% fewer queries)
**Files to Modify:**
- `src/routes/+page.svelte` (add debounce to search handler)

**Implementation:**
```typescript
let searchTimeout: ReturnType<typeof setTimeout>;
function handleSearchInput(value: string) {
  clearTimeout(searchTimeout);
  searchTimeout = setTimeout(() => {
    searchQuery.value = value;
  }, 300); // 300ms debounce
}
```

---

### 4. Image Lazy Loading (4 hours)
**Priority:** 🔴 High (60% faster load)
**Files to Modify:**
- `src/lib/components/GalleryItem.svelte` (add IntersectionObserver)

**Implementation:**
```typescript
// Use IntersectionObserver API
// Load thumbnail when 200px from viewport
// Show placeholder until loaded
// Progressive: blur → full image
```

---

### 5. CSP Implementation (3 hours)
**Priority:** 🔴 Critical (security)
**Files to Modify:**
- `src-tauri/tauri.conf.json` (add CSP headers)

**Implementation:**
```json
"security": {
  "csp": "default-src 'self'; img-src 'self' asset: https:; style-src 'self' 'unsafe-inline'"
}
```

---

### 6. Input Sanitization Layer (4 hours)
**Priority:** 🔴 Critical (security)
**Files to Create:**
- `src/lib/utils/sanitize.ts` (validation functions)

**Files to Modify:**
- All form inputs (search, settings, upload)

**Implementation:**
```typescript
// Sanitize filenames (remove ../, special chars)
// Validate paths (no traversal, exists)
// Sanitize search queries (SQL injection prevention)
// Max length checks
```

---

### 7. Error Handling Standardization (6 hours)
**Priority:** 🟡 High (maintainability)
**Files to Create:**
- `src-tauri/src/errors.rs` (custom error types)
- `src/lib/utils/errors.ts` (frontend error handling)

**Implementation:**
```rust
// Custom error types
pub enum AppError {
    DatabaseError(String),
    FileSystemError(String),
    ValidationError(String),
    NetworkError(String),
}

// Implement Display, Error traits
// Convert to user-friendly messages
```

---

## Phase 2: Performance (32 hours)

### 8. Virtual Scrolling (8 hours)
**Priority:** 🔴 Critical (90% DOM reduction)
**Files to Modify:**
- `src/lib/components/GalleryGrid.svelte` (replace with virtual list)

**Library:** Use `svelte-virtual-list` or custom implementation

**Implementation:**
```typescript
// Only render visible items + buffer
// Calculate visible range based on scroll position
// Dynamically adjust based on window height
// Maintain scroll position on updates
```

**Expected Result:**
- Before: 1000 DOM nodes → After: ~30 DOM nodes
- 60 FPS scrolling with 10,000+ files

---

### 9. Database Query Optimization (6 hours)
**Priority:** 🔴 High (5-10x faster queries)
**Files to Modify:**
- `src-tauri/src/database.rs` (add compound indices, optimize queries)

**Implementation:**
```rust
// Add compound indices:
// - (favorite, created_at)
// - (file_type, created_at)
// - (model_name, created_at)

// Use prepared statements
// Batch inserts in transactions
// Query only needed columns
```

**SQL Examples:**
```sql
-- Before: Full table scan
SELECT * FROM files WHERE favorite = 1 ORDER BY created_at DESC;

-- After: Index scan
CREATE INDEX idx_favorite_created ON files(favorite, created_at);
```

---

### 10. Logging Infrastructure (6 hours)
**Priority:** 🟡 High (debugging, monitoring)
**Files to Modify:**
- `Cargo.toml` (add tracing dependencies)
- `src-tauri/src/lib.rs` (setup logging)
- All command functions (add spans/events)

**Implementation:**
```rust
use tracing::{info, warn, error, instrument};

#[instrument(skip(state))]
#[tauri::command]
async fn sync_files(state: State<'_, AppState>) -> Result<ScanStats, String> {
    info!("Starting file sync");
    // ... operation ...
    info!(files_processed = stats.files_processed, "Sync complete");
    Ok(stats)
}
```

**Levels:**
- ERROR: Critical failures
- WARN: Recoverable issues
- INFO: Major operations
- DEBUG: Detailed traces
- TRACE: Very verbose

---

### 11. Thumbnail Generation Parallelization (4 hours)
**Priority:** 🟡 Medium (3-5x faster)
**Files to Modify:**
- `src-tauri/src/thumbnails.rs` (parallel processing)

**Implementation:**
```rust
use rayon::prelude::*;

// Currently: Sequential processing
// After: Parallel with Rayon

files.par_iter()
    .map(|file| generate_thumbnail(file))
    .collect()
```

---

### 12. Background Job Queue (8 hours)
**Priority:** 🟡 Medium (non-blocking operations)
**Files to Create:**
- `src-tauri/src/job_queue.rs` (async job system)

**Implementation:**
```rust
// Use tokio channels
// Priority queue
// Job types: thumbnail generation, sync, cleanup

pub enum Job {
    GenerateThumbnail(String),
    SyncFiles,
    CleanupCache,
}

// Worker pool
// Progress reporting via Tauri events
```

---

## Implementation Sequence

**Day 1-2: Toast & Error Handling (10 hours)**
1. Create toast notification system
2. Integrate toasts into all operations
3. Standardize error handling (Rust + TS)
4. Replace console.log with proper logging

**Day 3: UI Loading States (8 hours)**
5. Implement skeleton loading
6. Add image lazy loading
7. Debounce search input

**Day 4: Security (7 hours)**
8. Add CSP headers
9. Implement input sanitization layer
10. Add validation throughout

**Day 5-6: Database & Logging (12 hours)**
11. Add compound database indices
12. Optimize queries
13. Setup tracing infrastructure
14. Add logging spans to all commands

**Day 7-8: Virtual Scrolling (8 hours)**
15. Implement virtual list component
16. Test with 10,000+ files
17. Performance benchmarks

**Day 9-10: Parallelization (12 hours)**
18. Parallelize thumbnail generation
19. Implement background job queue
20. Integration tests

**Day 11: Integration Testing (4 hours)**
21. Write integration tests
22. Performance benchmarks
23. Final verification

---

## Success Criteria

**Performance:**
- ✅ Initial load < 1s
- ✅ Search latency < 100ms
- ✅ Virtual scroll at 60 FPS
- ✅ Memory < 200MB with 10k files

**UX:**
- ✅ Toast notifications for all operations
- ✅ Skeleton loading during sync
- ✅ Smooth scrolling with thousands of files
- ✅ No blocking operations

**Security:**
- ✅ CSP headers active
- ✅ All inputs sanitized
- ✅ Path traversal prevented
- ✅ No SQL injection vulnerabilities

**Code Quality:**
- ✅ Standardized error handling
- ✅ Comprehensive logging
- ✅ Integration tests passing
- ✅ Zero TypeScript/Rust errors

---

## Testing Strategy

**Manual Testing:**
1. Load 10,000+ files → verify virtual scrolling
2. Trigger all operations → verify toast notifications
3. Type in search → verify debouncing
4. Scroll gallery → verify lazy loading
5. Check browser console → verify CSP, no errors

**Automated Testing:**
1. Unit tests for validation/sanitization
2. Integration tests for database queries
3. Performance benchmarks with Criterion
4. E2E tests with Playwright (future)

**Performance Benchmarks:**
```bash
# Before optimization
Initial load: 1.5s
Search 1000 files: 200ms
Scroll FPS: 30 (with 500 files)
Memory: 300MB

# After optimization
Initial load: <1s (target)
Search 1000 files: <100ms (target)
Scroll FPS: 60 (target, with 10k files)
Memory: <200MB (target)
```

---

## Risk Assessment

**High Risk:**
- Virtual scrolling: Complex, could break existing UX
  - Mitigation: Extensive testing, fallback to regular list
- Database migration: Could corrupt data
  - Mitigation: Backup before migration, test thoroughly

**Medium Risk:**
- Background jobs: Race conditions possible
  - Mitigation: Proper locking, testing
- CSP: Could break asset loading
  - Mitigation: Test all asset types

**Low Risk:**
- Toast notifications: Pure addition
- Debouncing: Simple, well-tested pattern
- Skeleton loading: Visual only

---

## Rollback Plan

If critical issues arise:
1. Revert specific commit
2. Disable feature via config flag
3. Deploy hotfix
4. Investigate and fix offline

Each feature is isolated, allowing independent rollback.

---

## Next Steps

1. ✅ Create this implementation plan
2. → Start with Toast Notification System
3. → Continue sequentially through the list
4. → Test after each implementation
5. → Document learnings and adjustments

**Estimated Completion:** 11 working days (61 hours total)
**Target Date:** ~2 weeks from start

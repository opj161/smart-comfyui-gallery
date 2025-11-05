<script lang="ts">
import { onMount } from 'svelte';
import { listen } from '@tauri-apps/api/event';
import * as api from '$lib/api';
import * as store from '$lib/store';
import GalleryGrid from '$lib/components/GalleryGrid.svelte';
import Lightbox from '$lib/components/Lightbox.svelte';
import FilterPanel from '$lib/components/FilterPanel.svelte';
import Toolbar from '$lib/components/Toolbar.svelte';

let isInitialized = $state(false);
let isLoading = $state(true);
let currentPage = $state(0);
let totalCount = $state(0);
let hasMore = $state(false);
let isFilterPanelOpen = $state(false);
let isSyncing = $state(false);
let syncProgress = $state(0);
let syncTotal = $state(0);

// Initialize gallery on mount
onMount(async () => {
try {
// Initialize with default paths (can be configured later)
await api.initializeGallery('/tmp/gallery-output', '/tmp/gallery-input');
isInitialized = true;

// Load initial files
await loadFiles(0);

// Set up event listeners
await setupEventListeners();
} catch (error) {
console.error('Failed to initialize gallery:', error);
alert('Failed to initialize gallery. Please check the console for details.');
}
});

async function setupEventListeners() {
// Sync progress
await listen('sync-progress', (event: any) => {
const payload = event.payload;
syncProgress = payload.current;
syncTotal = payload.total;
});

// Sync complete
await listen('sync-complete', async () => {
isSyncing = false;
// Reload files
await loadFiles(0);
});
}

async function loadFiles(page: number) {
isLoading = true;
try {
const filters = store.filters.value;
let result;

// Check if we have active filters
const hasActiveFilters = 
filters.search ||
filters.favorites_only ||
filters.model ||
filters.sampler ||
filters.scheduler ||
filters.cfg_min !== null ||
filters.cfg_max !== null ||
filters.steps_min !== null ||
filters.steps_max !== null ||
filters.width !== null ||
filters.height !== null;

if (hasActiveFilters) {
// Use filtered endpoint
result = await api.getFilesFiltered(filters, page, 50);
} else if (filters.search) {
// Use search endpoint
result = await api.searchFiles(filters.search, page, 50);
} else {
// Use regular endpoint
result = await api.getFiles(null, page, 50);
}

if (page === 0) {
store.setFiles(result.files, result.total_count, result.has_more);
} else {
// Append files for pagination
const currentFiles = store.files.value;
store.setFiles([...currentFiles, ...result.files], result.total_count, result.has_more);
}

currentPage = page;
totalCount = result.total_count;
hasMore = result.has_more;
} catch (error) {
console.error('Failed to load files:', error);
} finally {
isLoading = false;
}
}

async function handleLoadMore() {
await loadFiles(currentPage + 1);
}

async function handleSync() {
if (isSyncing) return;

isSyncing = true;
syncProgress = 0;
syncTotal = 0;

try {
await api.syncFiles();
} catch (error) {
console.error('Sync failed:', error);
alert('Sync failed. Please check the console for details.');
isSyncing = false;
}
}

function handleOpenFilters() {
isFilterPanelOpen = true;
}

function handleCloseFilters() {
isFilterPanelOpen = false;
// Reload files with new filters
loadFiles(0);
}
</script>

<div class="app-container">
<header class="app-header">
<h1>SmartGallery</h1>
<div class="header-stats">
{#if isInitialized}
<span class="stat-item">{totalCount} files</span>
{#if isSyncing}
<span class="stat-item syncing">
Syncing... {syncProgress}/{syncTotal}
</span>
{/if}
{/if}
</div>
</header>

{#if isInitialized}
<Toolbar 
onOpenFilters={handleOpenFilters}
onSync={handleSync}
/>

<main class="app-main">
<GalleryGrid 
onLoadMore={handleLoadMore}
hasMore={hasMore}
isLoading={isLoading}
/>
</main>

<Lightbox isOpen={store.isLightboxOpen.value} />

<FilterPanel 
isOpen={isFilterPanelOpen}
onClose={handleCloseFilters}
/>
{:else}
<div class="loading-screen">
<p>Initializing gallery...</p>
</div>
{/if}
</div>

<style>
:global(body) {
margin: 0;
padding: 0;
font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
background: #121212;
color: #EAEAEA;
}

:global(*) {
box-sizing: border-box;
}

:global(:root) {
--bg-color: #121212;
--surface-color: #1E1E1E;
--surface-hover: #2a2a2a;
--border-color: #333333;
--text-color: #EAEAEA;
--text-muted: #9E9E9E;
--accent-yellow: #FFD60A;
--danger-color: #FF4d4d;
--danger-hover: #ff1a1a;
--success-color: #52c41a;
}

.app-container {
width: 100%;
min-height: 100vh;
display: flex;
flex-direction: column;
}

.app-header {
display: flex;
justify-content: space-between;
align-items: center;
padding: 16px 24px;
background: var(--surface-color);
border-bottom: 1px solid var(--border-color);
}

.app-header h1 {
margin: 0;
font-size: 1.5rem;
color: var(--text-color);
font-weight: 700;
}

.header-stats {
display: flex;
gap: 16px;
align-items: center;
}

.stat-item {
font-size: 0.875rem;
color: var(--text-muted);
}

.stat-item.syncing {
color: var(--accent-yellow);
font-weight: 600;
}

.app-main {
flex: 1;
overflow-y: auto;
}

.loading-screen {
flex: 1;
display: flex;
align-items: center;
justify-content: center;
color: var(--text-muted);
}

.loading-screen p {
font-size: 1.125rem;
}
</style>

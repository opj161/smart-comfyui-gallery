<script lang="ts">
  import type { FileEntry } from '$lib/types';
  import { toggleSelection } from '$lib/store';
  import * as api from '$lib/api';
  
  interface Props {
    file: FileEntry;
    isSelected: boolean;
    onOpenLightbox: (fileId: string) => void;
  }
  
  let { file, isSelected, onOpenLightbox }: Props = $props();
  
  let thumbnailUrl = $state('');
  let isLoading = $state(true);
  let favoriteLoading = $state(false);
  
  // Load thumbnail
  $effect(() => {
    loadThumbnail();
  });
  
  async function loadThumbnail() {
    try {
      thumbnailUrl = await api.getThumbnailPath(file.id);
      isLoading = false;
    } catch (error) {
      console.error('Failed to load thumbnail:', error);
      isLoading = false;
    }
  }
  
  async function handleToggleFavorite(event: MouseEvent) {
    event.stopPropagation();
    if (favoriteLoading) return;
    
    favoriteLoading = true;
    try {
      await api.toggleFavorite(file.id);
      // Update local file state
      file.is_favorite = !file.is_favorite;
    } catch (error) {
      console.error('Failed to toggle favorite:', error);
    } finally {
      favoriteLoading = false;
    }
  }
  
  function handleClick(event: MouseEvent) {
    if (event.shiftKey || event.ctrlKey || event.metaKey) {
      event.preventDefault();
      toggleSelection(file.id);
    } else {
      onOpenLightbox(file.id);
    }
  }
  
  function handleCheckboxChange() {
    toggleSelection(file.id);
  }
</script>

<div 
  class="gallery-item"
  class:selected={isSelected}
  onclick={handleClick}
  role="button"
  tabindex="0"
>
  <!-- Selection Checkbox -->
  <div class="selection-checkbox">
    <input 
      type="checkbox" 
      checked={isSelected}
      onchange={handleCheckboxChange}
      onclick={(e) => e.stopPropagation()}
    />
  </div>
  
  <!-- Favorite Button -->
  <button 
    class="favorite-btn"
    class:is-favorite={file.is_favorite}
    onclick={handleToggleFavorite}
    disabled={favoriteLoading}
    aria-label="Toggle favorite"
  >
    <span class="star-icon">{file.is_favorite ? '★' : '☆'}</span>
  </button>
  
  <!-- Thumbnail -->
  <div class="thumbnail-container">
    {#if isLoading}
      <div class="thumbnail-loading">Loading...</div>
    {:else if thumbnailUrl}
      <img src={thumbnailUrl} alt={file.name} class="thumbnail" />
    {:else}
      <div class="thumbnail-placeholder">No Preview</div>
    {/if}
  </div>
  
  <!-- File Info -->
  <div class="file-info">
    <div class="file-name" title={file.name}>{file.name}</div>
    
    {#if file.dimensions}
      <div class="file-meta">{file.dimensions}</div>
    {/if}
    
    {#if file.has_workflow}
      <div class="workflow-badge">
        <span class="badge">Workflow</span>
        {#if file.sampler_count > 1}
          <span class="badge sampler-count" title={file.sampler_names || ''}>
            {file.sampler_count} samplers
          </span>
        {/if}
      </div>
    {/if}
    
    {#if file.prompt_preview}
      <div class="prompt-preview" title={file.prompt_preview}>
        {file.prompt_preview}
      </div>
    {/if}
  </div>
</div>

<style>
  .gallery-item {
    position: relative;
    background: var(--surface-color, #1E1E1E);
    border: 1px solid var(--border-color, #333);
    border-radius: 4px;
    overflow: hidden;
    cursor: pointer;
    transition: all 150ms ease;
  }
  
  .gallery-item:hover {
    border-color: var(--text-muted, #9E9E9E);
    box-shadow: 0 4px 12px rgba(0,0,0,0.25);
  }
  
  .gallery-item.selected {
    border-color: var(--accent-yellow, #FFD60A);
    box-shadow: 0 0 0 2px var(--accent-yellow, #FFD60A);
  }
  
  .selection-checkbox {
    position: absolute;
    top: 8px;
    left: 8px;
    z-index: 10;
  }
  
  .selection-checkbox input[type="checkbox"] {
    width: 20px;
    height: 20px;
    cursor: pointer;
    accent-color: var(--accent-yellow, #FFD60A);
  }
  
  .favorite-btn {
    position: absolute;
    top: 8px;
    right: 8px;
    z-index: 10;
    background: rgba(18, 18, 18, 0.8);
    border: 1px solid var(--border-color, #333);
    border-radius: 4px;
    padding: 4px 8px;
    cursor: pointer;
    transition: all 150ms ease;
  }
  
  .favorite-btn:hover {
    background: rgba(18, 18, 18, 0.95);
    border-color: var(--accent-yellow, #FFD60A);
  }
  
  .favorite-btn.is-favorite .star-icon {
    color: var(--accent-yellow, #FFD60A);
  }
  
  .star-icon {
    font-size: 1.25rem;
    color: var(--text-muted, #9E9E9E);
  }
  
  .thumbnail-container {
    width: 100%;
    aspect-ratio: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--bg-color, #121212);
    overflow: hidden;
  }
  
  .thumbnail {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }
  
  .thumbnail-loading,
  .thumbnail-placeholder {
    color: var(--text-muted, #9E9E9E);
    font-size: 0.875rem;
  }
  
  .file-info {
    padding: 12px;
  }
  
  .file-name {
    font-size: 0.875rem;
    font-weight: 600;
    color: var(--text-color, #EAEAEA);
    margin-bottom: 4px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  
  .file-meta {
    font-size: 0.75rem;
    color: var(--text-muted, #9E9E9E);
    margin-bottom: 4px;
  }
  
  .workflow-badge {
    display: flex;
    gap: 4px;
    flex-wrap: wrap;
    margin-top: 4px;
  }
  
  .badge {
    display: inline-block;
    font-size: 0.75rem;
    padding: 2px 6px;
    border-radius: 3px;
    background: var(--surface-hover, #2a2a2a);
    color: var(--accent-yellow, #FFD60A);
    font-weight: 600;
  }
  
  .badge.sampler-count {
    background: var(--surface-hover, #2a2a2a);
    color: var(--text-muted, #9E9E9E);
  }
  
  .prompt-preview {
    font-size: 0.75rem;
    color: var(--text-muted, #9E9E9E);
    margin-top: 6px;
    overflow: hidden;
    text-overflow: ellipsis;
    display: -webkit-box;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
    line-height: 1.3;
  }
</style>

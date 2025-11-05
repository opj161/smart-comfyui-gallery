<script lang="ts">
  import { filters, setFilters } from '$lib/store';
  import * as api from '$lib/api';
  
  interface Props {
    isOpen: boolean;
    onClose: () => void;
  }
  
  let { isOpen, onClose }: Props = $props();
  
  let filterOptions = $state<any>({
    models: [],
    samplers: [],
    schedulers: [],
    types: [],
    extensions: []
  });
  
  let isLoading = $state(false);
  
  // Load filter options
  $effect(() => {
    if (isOpen) {
      loadFilterOptions();
    }
  });
  
  async function loadFilterOptions() {
    isLoading = true;
    try {
      filterOptions = await api.getFilterOptions();
    } catch (error) {
      console.error('Failed to load filter options:', error);
    } finally {
      isLoading = false;
    }
  }
  
  function handleApplyFilters() {
    onClose();
  }
  
  function handleClearFilters() {
    setFilters({
      search: '',
      favorites_only: false,
      type: [],
      extension: [],
      model: null,
      sampler: null,
      scheduler: null,
      cfg_min: null,
      cfg_max: null,
      steps_min: null,
      steps_max: null,
      width: null,
      height: null
    });
  }
  
  function handleBackdropClick(event: MouseEvent) {
    if (event.target === event.currentTarget) {
      onClose();
    }
  }
</script>

{#if isOpen}
  <div class="filter-panel-backdrop" onclick={handleBackdropClick}>
    <div class="filter-panel">
      <div class="filter-header">
        <h2>Filters</h2>
        <button class="btn-close" onclick={onClose} aria-label="Close">×</button>
      </div>
      
      <div class="filter-body">
        <!-- Search -->
        <div class="filter-group">
          <label for="search-input">Search</label>
          <input 
            id="search-input"
            type="text" 
            placeholder="Search by name or prompt..." 
            bind:value={filters.value.search}
            class="filter-input"
          />
        </div>
        
        <!-- Favorites -->
        <div class="filter-group">
          <label class="checkbox-label">
            <input 
              type="checkbox" 
              bind:checked={filters.value.favorites_only}
            />
            <span>Favorites Only</span>
          </label>
        </div>
        
        <!-- Model -->
        <div class="filter-group">
          <label for="model-select">Model</label>
          <select 
            id="model-select"
            bind:value={filters.value.model}
            class="filter-select"
          >
            <option value={null}>All Models</option>
            {#each filterOptions.models as model}
              <option value={model}>{model}</option>
            {/each}
          </select>
        </div>
        
        <!-- Sampler -->
        <div class="filter-group">
          <label for="sampler-select">Sampler</label>
          <select 
            id="sampler-select"
            bind:value={filters.value.sampler}
            class="filter-select"
          >
            <option value={null}>All Samplers</option>
            {#each filterOptions.samplers as sampler}
              <option value={sampler}>{sampler}</option>
            {/each}
          </select>
        </div>
        
        <!-- Scheduler -->
        <div class="filter-group">
          <label for="scheduler-select">Scheduler</label>
          <select 
            id="scheduler-select"
            bind:value={filters.value.scheduler}
            class="filter-select"
          >
            <option value={null}>All Schedulers</option>
            {#each filterOptions.schedulers as scheduler}
              <option value={scheduler}>{scheduler}</option>
            {/each}
          </select>
        </div>
        
        <!-- CFG Scale -->
        <div class="filter-group">
          <label>CFG Scale</label>
          <div class="range-inputs">
            <input 
              type="number" 
              placeholder="Min" 
              bind:value={filters.value.cfg_min}
              class="filter-input"
              min="0"
              step="0.5"
            />
            <span>to</span>
            <input 
              type="number" 
              placeholder="Max" 
              bind:value={filters.value.cfg_max}
              class="filter-input"
              min="0"
              step="0.5"
            />
          </div>
        </div>
        
        <!-- Steps -->
        <div class="filter-group">
          <label>Steps</label>
          <div class="range-inputs">
            <input 
              type="number" 
              placeholder="Min" 
              bind:value={filters.value.steps_min}
              class="filter-input"
              min="1"
            />
            <span>to</span>
            <input 
              type="number" 
              placeholder="Max" 
              bind:value={filters.value.steps_max}
              class="filter-input"
              min="1"
            />
          </div>
        </div>
        
        <!-- Dimensions -->
        <div class="filter-group">
          <label>Dimensions</label>
          <div class="dimension-inputs">
            <input 
              type="number" 
              placeholder="Width" 
              bind:value={filters.value.width}
              class="filter-input"
              min="1"
            />
            <span>×</span>
            <input 
              type="number" 
              placeholder="Height" 
              bind:value={filters.value.height}
              class="filter-input"
              min="1"
            />
          </div>
        </div>
      </div>
      
      <div class="filter-footer">
        <button class="btn btn-secondary" onclick={handleClearFilters}>
          Clear All
        </button>
        <button class="btn btn-primary" onclick={handleApplyFilters}>
          Apply Filters
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .filter-panel-backdrop {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.5);
    z-index: 8499;
    display: flex;
    justify-content: flex-end;
  }
  
  .filter-panel {
    width: 400px;
    max-width: 90vw;
    height: 100%;
    background: var(--surface-color, #1E1E1E);
    border-left: 1px solid var(--border-color, #333);
    display: flex;
    flex-direction: column;
    z-index: 8500;
  }
  
  .filter-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 16px 24px;
    border-bottom: 1px solid var(--border-color, #333);
  }
  
  .filter-header h2 {
    margin: 0;
    font-size: 1.25rem;
    color: var(--text-color, #EAEAEA);
  }
  
  .btn-close {
    background: transparent;
    border: none;
    font-size: 2rem;
    color: var(--text-muted, #9E9E9E);
    cursor: pointer;
    line-height: 1;
    padding: 0;
    width: 32px;
    height: 32px;
    display: flex;
    align-items: center;
    justify-content: center;
  }
  
  .btn-close:hover {
    color: var(--text-color, #EAEAEA);
  }
  
  .filter-body {
    flex: 1;
    overflow-y: auto;
    padding: 24px;
  }
  
  .filter-group {
    margin-bottom: 24px;
  }
  
  .filter-group label {
    display: block;
    font-size: 0.875rem;
    font-weight: 600;
    color: var(--text-color, #EAEAEA);
    margin-bottom: 8px;
  }
  
  .filter-input,
  .filter-select {
    width: 100%;
    padding: 10px 12px;
    background: var(--bg-color, #121212);
    border: 1px solid var(--border-color, #333);
    border-radius: 4px;
    color: var(--text-color, #EAEAEA);
    font-size: 0.875rem;
    transition: border-color 150ms ease;
  }
  
  .filter-input:focus,
  .filter-select:focus {
    outline: none;
    border-color: var(--accent-yellow, #FFD60A);
  }
  
  .checkbox-label {
    display: flex;
    align-items: center;
    gap: 8px;
    cursor: pointer;
    user-select: none;
  }
  
  .checkbox-label input[type="checkbox"] {
    width: 20px;
    height: 20px;
    cursor: pointer;
    accent-color: var(--accent-yellow, #FFD60A);
  }
  
  .checkbox-label span {
    font-size: 0.875rem;
    color: var(--text-color, #EAEAEA);
  }
  
  .range-inputs,
  .dimension-inputs {
    display: flex;
    align-items: center;
    gap: 8px;
  }
  
  .range-inputs input,
  .dimension-inputs input {
    flex: 1;
  }
  
  .range-inputs span,
  .dimension-inputs span {
    color: var(--text-muted, #9E9E9E);
    font-size: 0.875rem;
  }
  
  .filter-footer {
    display: flex;
    gap: 12px;
    padding: 16px 24px;
    border-top: 1px solid var(--border-color, #333);
  }
  
  .btn {
    flex: 1;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    padding: 12px 24px;
    font-size: 0.875rem;
    font-weight: 600;
    border-radius: 4px;
    cursor: pointer;
    transition: all 150ms ease;
  }
  
  .btn-secondary {
    background: transparent;
    border: 1px solid var(--border-color, #333);
    color: var(--text-color, #EAEAEA);
  }
  
  .btn-secondary:hover {
    background: var(--surface-hover, #2a2a2a);
    border-color: var(--text-muted, #9E9E9E);
  }
  
  .btn-primary {
    background: var(--accent-yellow, #FFD60A);
    border: 1px solid var(--accent-yellow, #FFD60A);
    color: var(--bg-color, #121212);
  }
  
  .btn-primary:hover {
    filter: brightness(1.1);
  }
</style>

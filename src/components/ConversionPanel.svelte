<script>
  import { invoke } from '@tauri-apps/api/core';
  import { open as openDialog } from '@tauri-apps/plugin-dialog';
  import { homeDir, downloadDir } from '@tauri-apps/api/path';

  let { fileInfo, supportedFormats, isConverting, conversionResult, onconvert } = $props();
  
  let selectedFormat = $state('');
  let outputDir = $state('');
  let quality = $state(85);
  let showAdvanced = $state(false);

  $effect(() => {
    if (supportedFormats.length > 0 && !selectedFormat) {
      // Select first format that's different from input
      selectedFormat = supportedFormats.find(f => f !== fileInfo?.extension) || supportedFormats[0];
    }
    loadDefaultOutputDir();
  });

  async function loadDefaultOutputDir() {
    if (!outputDir) {
      try {
        outputDir = await downloadDir();
      } catch {
        outputDir = await homeDir();
      }
    }
  }

  async function selectOutputDir() {
    const selected = await openDialog({
      directory: true,
      multiple: false,
      defaultPath: outputDir
    });
    
    if (selected) {
      outputDir = selected;
    }
  }

  function handleConvert() {
    const options = fileInfo?.file_type === 'image' ? { quality } : null;
    onconvert({ 
      detail: { 
        format: selectedFormat, 
        outputDir,
        options
      } 
    });
  }

  async function openFileLocation() {
    if (conversionResult?.output_path) {
      try {
        await invoke('open_file_location', { path: conversionResult.output_path });
      } catch (error) {
        console.error('Failed to open location:', error);
      }
    }
  }

  function getFormatDescription(format) {
    const descriptions = {
      // Images
      png: 'Lossless, supports transparency',
      jpg: 'Smaller size, lossy compression',
      jpeg: 'Smaller size, lossy compression',
      gif: 'Supports animation',
      webp: 'Modern format, great compression',
      bmp: 'Uncompressed bitmap',
      ico: 'Icon format',
      tiff: 'High quality, lossless',
      avif: 'Next-gen image format',
      // Documents
      pdf: 'Portable document format',
      txt: 'Plain text',
      md: 'Markdown format',
      html: 'Web page format',
      docx: 'Word document',
      epub: 'E-book format',
      rtf: 'Rich text format',
      // Data
      json: 'Structured data format',
      csv: 'Spreadsheet compatible',
      xml: 'Extensible markup',
      yaml: 'Human-readable data',
      toml: 'Config file format',
      // Spreadsheets
      xlsx: 'Excel spreadsheet',
      xls: 'Legacy Excel format',
      ods: 'OpenDocument spreadsheet',
      // Audio
      mp3: 'Universal audio format',
      wav: 'Uncompressed audio',
      ogg: 'Open audio format',
      flac: 'Lossless audio',
      aac: 'Advanced audio codec',
      m4a: 'Apple audio format',
      // Video
      mp4: 'Universal video format',
      avi: 'Windows video format',
      mkv: 'Matroska video',
      mov: 'Apple video format',
      webm: 'Web video format',
      // Archives
      zip: 'Compressed archive',
      tar: 'Tape archive',
      gz: 'Gzip compressed',
      // Extract
      extract: 'Extract archive contents',
      list: 'List archive contents',
    };
    return descriptions[format] || '';
  }
</script>

<div class="conversion-panel">
  <div class="panel-header">
    <h3>Convert To</h3>
  </div>

  <div class="format-selector">
    <div class="format-grid">
      {#each supportedFormats as format}
        <button 
          class="format-btn"
          class:selected={selectedFormat === format}
          onclick={() => selectedFormat = format}
          disabled={isConverting}
        >
          <span class="format-name">.{format}</span>
          <span class="format-desc">{getFormatDescription(format)}</span>
        </button>
      {/each}
    </div>
  </div>

  {#if fileInfo?.file_type === 'image'}
    <div class="advanced-section">
      <button class="advanced-toggle" onclick={() => showAdvanced = !showAdvanced}>
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class:rotated={showAdvanced}>
          <polyline points="6 9 12 15 18 9"></polyline>
        </svg>
        Advanced Options
      </button>
      
      {#if showAdvanced}
        <div class="advanced-options">
          {#if selectedFormat === 'jpg' || selectedFormat === 'jpeg'}
            <div class="option-group">
              <label>
                Quality: {quality}%
                <input 
                  type="range" 
                  min="1" 
                  max="100" 
                  bind:value={quality}
                  disabled={isConverting}
                />
              </label>
            </div>
          {/if}
        </div>
      {/if}
    </div>
  {/if}

  <div class="output-section">
    <label class="output-label">Output Location</label>
    <div class="output-path">
      <input 
        type="text" 
        bind:value={outputDir} 
        readonly 
        class="path-input"
      />
      <button class="browse-btn" onclick={selectOutputDir} disabled={isConverting}>
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"></path>
        </svg>
      </button>
    </div>
  </div>

  <button 
    class="convert-btn"
    onclick={handleConvert}
    disabled={isConverting || !selectedFormat || !outputDir}
  >
    {#if isConverting}
      <div class="spinner"></div>
      Converting...
    {:else}
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
        <polyline points="7 10 12 15 17 10"></polyline>
        <line x1="12" y1="15" x2="12" y2="3"></line>
      </svg>
      Convert & Download
    {/if}
  </button>

  {#if conversionResult}
    <div class="result" class:success={conversionResult.success} class:error={!conversionResult.success}>
      {#if conversionResult.success}
        <div class="result-content">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"></path>
            <polyline points="22 4 12 14.01 9 11.01"></polyline>
          </svg>
          <div class="result-text">
            <strong>Conversion Complete!</strong>
            <span class="result-path">{conversionResult.output_path}</span>
          </div>
        </div>
        <button class="open-btn" onclick={openFileLocation}>
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"></path>
            <polyline points="15 3 21 3 21 9"></polyline>
            <line x1="10" y1="14" x2="21" y2="3"></line>
          </svg>
          Open Folder
        </button>
      {:else}
        <div class="result-content">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="12" cy="12" r="10"></circle>
            <line x1="15" y1="9" x2="9" y2="15"></line>
            <line x1="9" y1="9" x2="15" y2="15"></line>
          </svg>
          <div class="result-text">
            <strong>Conversion Failed</strong>
            <span class="result-error">{conversionResult.error}</span>
          </div>
        </div>
      {/if}
    </div>
  {/if}
</div>

<style>
  .conversion-panel {
    height: 100%;
    display: flex;
    flex-direction: column;
  }

  .panel-header {
    margin-bottom: 1rem;
  }

  .panel-header h3 {
    font-size: 1rem;
    font-weight: 600;
    color: #a5b4fc;
  }

  .format-selector {
    flex: 1;
    overflow-y: auto;
    margin-bottom: 1rem;
  }

  .format-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(140px, 1fr));
    gap: 0.75rem;
  }

  .format-btn {
    background: rgba(0, 0, 0, 0.2);
    border: 2px solid rgba(255, 255, 255, 0.08);
    border-radius: 12px;
    padding: 1rem;
    cursor: pointer;
    transition: all 0.2s ease;
    text-align: left;
  }

  .format-btn:hover:not(:disabled) {
    border-color: rgba(99, 102, 241, 0.4);
    background: rgba(99, 102, 241, 0.1);
  }

  .format-btn.selected {
    border-color: #6366f1;
    background: rgba(99, 102, 241, 0.15);
  }

  .format-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .format-name {
    display: block;
    font-size: 1.1rem;
    font-weight: 600;
    color: #e2e8f0;
    text-transform: uppercase;
  }

  .format-desc {
    display: block;
    font-size: 0.7rem;
    color: #64748b;
    margin-top: 0.25rem;
    line-height: 1.3;
  }

  .advanced-section {
    margin-bottom: 1rem;
  }

  .advanced-toggle {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    background: none;
    border: none;
    color: #64748b;
    font-size: 0.875rem;
    cursor: pointer;
    padding: 0.5rem 0;
    transition: color 0.2s ease;
  }

  .advanced-toggle:hover {
    color: #a5b4fc;
  }

  .advanced-toggle svg {
    width: 18px;
    height: 18px;
    transition: transform 0.2s ease;
  }

  .advanced-toggle svg.rotated {
    transform: rotate(180deg);
  }

  .advanced-options {
    padding: 1rem;
    background: rgba(0, 0, 0, 0.2);
    border-radius: 12px;
    margin-top: 0.5rem;
  }

  .option-group label {
    display: block;
    color: #94a3b8;
    font-size: 0.875rem;
  }

  .option-group input[type="range"] {
    width: 100%;
    margin-top: 0.5rem;
    accent-color: #6366f1;
  }

  .output-section {
    margin-bottom: 1rem;
  }

  .output-label {
    display: block;
    font-size: 0.75rem;
    color: #64748b;
    margin-bottom: 0.5rem;
  }

  .output-path {
    display: flex;
    gap: 0.5rem;
  }

  .path-input {
    flex: 1;
    background: rgba(0, 0, 0, 0.3);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 8px;
    padding: 0.75rem 1rem;
    color: #94a3b8;
    font-size: 0.8rem;
    font-family: monospace;
  }

  .browse-btn {
    background: rgba(99, 102, 241, 0.15);
    border: 1px solid rgba(99, 102, 241, 0.3);
    border-radius: 8px;
    padding: 0.75rem;
    cursor: pointer;
    color: #a5b4fc;
    transition: all 0.2s ease;
  }

  .browse-btn:hover:not(:disabled) {
    background: rgba(99, 102, 241, 0.25);
  }

  .browse-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .browse-btn svg {
    width: 20px;
    height: 20px;
  }

  .convert-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.75rem;
    width: 100%;
    padding: 1rem;
    background: linear-gradient(135deg, #6366f1 0%, #8b5cf6 100%);
    border: none;
    border-radius: 12px;
    color: white;
    font-size: 1rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .convert-btn:hover:not(:disabled) {
    transform: translateY(-2px);
    box-shadow: 0 8px 25px rgba(99, 102, 241, 0.4);
  }

  .convert-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
    transform: none;
  }

  .convert-btn svg {
    width: 20px;
    height: 20px;
  }

  .spinner {
    width: 20px;
    height: 20px;
    border: 2px solid rgba(255, 255, 255, 0.3);
    border-top-color: white;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  .result {
    margin-top: 1rem;
    padding: 1rem;
    border-radius: 12px;
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .result.success {
    background: rgba(16, 185, 129, 0.1);
    border: 1px solid rgba(16, 185, 129, 0.3);
  }

  .result.error {
    background: rgba(239, 68, 68, 0.1);
    border: 1px solid rgba(239, 68, 68, 0.3);
  }

  .result-content {
    display: flex;
    align-items: flex-start;
    gap: 0.75rem;
  }

  .result-content svg {
    width: 24px;
    height: 24px;
    flex-shrink: 0;
  }

  .result.success svg {
    color: #10b981;
  }

  .result.error svg {
    color: #ef4444;
  }

  .result-text {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    min-width: 0;
  }

  .result-text strong {
    color: #e2e8f0;
    font-size: 0.9rem;
  }

  .result-path, .result-error {
    font-size: 0.75rem;
    color: #64748b;
    word-break: break-all;
  }

  .open-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    background: rgba(16, 185, 129, 0.15);
    border: 1px solid rgba(16, 185, 129, 0.3);
    border-radius: 8px;
    padding: 0.6rem 1rem;
    color: #10b981;
    font-size: 0.85rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .open-btn:hover {
    background: rgba(16, 185, 129, 0.25);
  }

  .open-btn svg {
    width: 16px;
    height: 16px;
  }
</style>

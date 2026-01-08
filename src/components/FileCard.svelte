<script>
  import { invoke } from '@tauri-apps/api/core';

  let { fileInfo, filePath, onclear } = $props();
  
  let imagePreview = $state(null);
  let imageInfo = $state(null);
  let pdfInfo = $state(null);
  let textPreview = $state(null);

  $effect(() => {
    loadPreview();
  });

  async function loadPreview() {
    imagePreview = null;
    imageInfo = null;
    pdfInfo = null;
    textPreview = null;

    if (!fileInfo) return;

    try {
      if (fileInfo.file_type === 'image') {
        const preview = await invoke('get_image_preview_cmd', { 
          inputPath: filePath, 
          maxSize: 300 
        });
        imagePreview = `data:image/png;base64,${preview}`;
        
        const info = await invoke('get_image_info_cmd', { inputPath: filePath });
        imageInfo = info;
      } else if (fileInfo.file_type === 'pdf') {
        const info = await invoke('get_pdf_info_cmd', { inputPath: filePath });
        pdfInfo = info;
      } else if (['text', 'config', 'code', 'script', 'style', 'html', 'data'].includes(fileInfo.file_type)) {
        const content = await invoke('read_text_file', { path: filePath });
        textPreview = content.slice(0, 500) + (content.length > 500 ? '...' : '');
      }
    } catch (error) {
      console.error('Preview error:', error);
    }
  }

  function formatBytes(bytes) {
    if (bytes === 0) return '0 Bytes';
    const k = 1024;
    const sizes = ['Bytes', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
  }

  function getFileIcon(type) {
    const icons = {
      image: `<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect>
        <circle cx="8.5" cy="8.5" r="1.5"></circle>
        <polyline points="21 15 16 10 5 21"></polyline>
      </svg>`,
      pdf: `<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path>
        <polyline points="14 2 14 8 20 8"></polyline>
        <line x1="16" y1="13" x2="8" y2="13"></line>
        <line x1="16" y1="17" x2="8" y2="17"></line>
        <polyline points="10 9 9 9 8 9"></polyline>
      </svg>`,
      text: `<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path>
        <polyline points="14 2 14 8 20 8"></polyline>
        <line x1="16" y1="13" x2="8" y2="13"></line>
        <line x1="16" y1="17" x2="8" y2="17"></line>
      </svg>`,
      code: `<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <polyline points="16 18 22 12 16 6"></polyline>
        <polyline points="8 6 2 12 8 18"></polyline>
      </svg>`,
      data: `<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <ellipse cx="12" cy="5" rx="9" ry="3"></ellipse>
        <path d="M21 12c0 1.66-4 3-9 3s-9-1.34-9-3"></path>
        <path d="M3 5v14c0 1.66 4 3 9 3s9-1.34 9-3V5"></path>
      </svg>`,
      default: `<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path>
        <polyline points="14 2 14 8 20 8"></polyline>
      </svg>`
    };
    return icons[type] || icons.default;
  }

  function getTypeColor(type) {
    const colors = {
      image: '#10b981',
      pdf: '#ef4444',
      text: '#3b82f6',
      code: '#f59e0b',
      script: '#f59e0b',
      config: '#8b5cf6',
      data: '#06b6d4',
      html: '#ec4899',
      style: '#ec4899',
    };
    return colors[type] || '#6366f1';
  }
</script>

<div class="file-card">
  <div class="card-header">
    <h3>Selected File</h3>
    <button class="clear-btn" onclick={onclear}>
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <line x1="18" y1="6" x2="6" y2="18"></line>
        <line x1="6" y1="6" x2="18" y2="18"></line>
      </svg>
    </button>
  </div>

  <div class="preview-area">
    {#if fileInfo?.file_type === 'image' && imagePreview}
      <img src={imagePreview} alt="Preview" class="image-preview" />
    {:else if fileInfo?.file_type === 'pdf'}
      <div class="pdf-preview">
        <div class="pdf-icon" style="color: {getTypeColor('pdf')}">
          {@html getFileIcon('pdf')}
        </div>
        {#if pdfInfo}
          <div class="pdf-details">
            <span>{pdfInfo.page_count} pages</span>
            <span>PDF {pdfInfo.version}</span>
          </div>
        {/if}
      </div>
    {:else if textPreview}
      <div class="text-preview">
        <pre>{textPreview}</pre>
      </div>
    {:else}
      <div class="generic-preview">
        <div class="file-icon" style="color: {getTypeColor(fileInfo?.file_type)}">
          {@html getFileIcon(fileInfo?.file_type)}
        </div>
      </div>
    {/if}
  </div>

  <div class="file-info">
    <div class="file-name" title={fileInfo?.name}>
      {fileInfo?.name}
    </div>
    
    <div class="file-meta">
      <span class="meta-item">
        <span class="label">Type:</span>
        <span class="type-badge" style="background: {getTypeColor(fileInfo?.file_type)}20; color: {getTypeColor(fileInfo?.file_type)}; border-color: {getTypeColor(fileInfo?.file_type)}40">
          {fileInfo?.extension.toUpperCase()}
        </span>
      </span>
      
      <span class="meta-item">
        <span class="label">Size:</span>
        <span class="value">{formatBytes(fileInfo?.size)}</span>
      </span>
      
      {#if imageInfo}
        <span class="meta-item">
          <span class="label">Dimensions:</span>
          <span class="value">{imageInfo.width} × {imageInfo.height}</span>
        </span>
      {/if}
    </div>
    
    <div class="file-path" title={filePath}>
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"></path>
      </svg>
      <span>{filePath}</span>
    </div>
  </div>
</div>

<style>
  .file-card {
    height: 100%;
    display: flex;
    flex-direction: column;
    min-width: 0;
    overflow: hidden;
  }

  .card-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
  }

  .card-header h3 {
    font-size: 1rem;
    font-weight: 600;
    color: #a5b4fc;
  }

  .clear-btn {
    width: 32px;
    height: 32px;
    border: none;
    background: rgba(239, 68, 68, 0.1);
    border-radius: 8px;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    color: #ef4444;
    transition: all 0.2s ease;
  }

  .clear-btn:hover {
    background: rgba(239, 68, 68, 0.2);
  }

  .clear-btn svg {
    width: 18px;
    height: 18px;
  }

  .preview-area {
    flex: 1;
    min-height: 200px;
    background: rgba(0, 0, 0, 0.2);
    border-radius: 12px;
    display: flex;
    align-items: center;
    justify-content: center;
    overflow: hidden;
    margin-bottom: 1rem;
  }

  .image-preview {
    max-width: 100%;
    max-height: 100%;
    object-fit: contain;
    border-radius: 8px;
  }

  .pdf-preview, .generic-preview {
    text-align: center;
    padding: 2rem;
  }

  .pdf-icon, .file-icon {
    width: 80px;
    height: 80px;
    margin: 0 auto 1rem;
  }

  .pdf-icon :global(svg), .file-icon :global(svg) {
    width: 100%;
    height: 100%;
  }

  .pdf-details {
    display: flex;
    gap: 1rem;
    justify-content: center;
    color: #94a3b8;
    font-size: 0.875rem;
  }

  .text-preview {
    width: 100%;
    height: 100%;
    overflow: auto;
    padding: 1rem;
  }

  .text-preview pre {
    font-family: 'Monaco', 'Menlo', monospace;
    font-size: 0.75rem;
    color: #94a3b8;
    white-space: pre-wrap;
    word-break: break-word;
    margin: 0;
    line-height: 1.5;
  }

  .file-info {
    background: rgba(0, 0, 0, 0.2);
    border-radius: 12px;
    padding: 1rem;
    overflow: hidden;
    min-width: 0;
  }

  .file-name {
    font-size: 1rem;
    font-weight: 600;
    color: #e2e8f0;
    margin-bottom: 0.75rem;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .file-meta {
    display: flex;
    flex-wrap: wrap;
    gap: 1rem;
    margin-bottom: 0.75rem;
  }

  .meta-item {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .label {
    font-size: 0.75rem;
    color: #64748b;
  }

  .value {
    font-size: 0.875rem;
    color: #e2e8f0;
  }

  .type-badge {
    padding: 0.25rem 0.5rem;
    border-radius: 6px;
    font-size: 0.75rem;
    font-weight: 600;
    border: 1px solid;
  }

  .file-path {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.7rem;
    color: #64748b;
    overflow: hidden;
    min-width: 0;
  }

  .file-path span {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    min-width: 0;
  }

  .file-path svg {
    width: 14px;
    height: 14px;
    flex-shrink: 0;
  }
</style>

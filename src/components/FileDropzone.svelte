<script>
  import { open } from '@tauri-apps/plugin-dialog';

  let { onfileselected } = $props();
  
  let isDragging = $state(false);

  async function handleBrowse() {
    const file = await open({
      multiple: false,
      filters: [
        {
          name: 'All Supported Files',
          extensions: [
            'png', 'jpg', 'jpeg', 'gif', 'bmp', 'ico', 'webp', 'tiff', 'tif', 'avif',
            'pdf',
            'txt', 'md', 'markdown', 'rst', 'log',
            'json', 'xml', 'yaml', 'yml', 'toml', 'ini', 'cfg', 'conf',
            'csv', 'tsv',
            'html', 'htm', 'xhtml',
            'css', 'scss', 'sass', 'less',
            'js', 'ts', 'jsx', 'tsx', 'mjs', 'cjs',
            'py', 'rb', 'php', 'java', 'c', 'cpp', 'h', 'hpp', 'rs', 'go', 'swift'
          ]
        },
        { name: 'Images', extensions: ['png', 'jpg', 'jpeg', 'gif', 'bmp', 'ico', 'webp', 'tiff', 'avif'] },
        { name: 'Documents', extensions: ['pdf', 'txt', 'md', 'html'] },
        { name: 'Data Files', extensions: ['json', 'csv', 'xml', 'yaml', 'yml'] },
        { name: 'All Files', extensions: ['*'] }
      ]
    });
    
    if (file) {
      onfileselected({ detail: { path: file } });
    }
  }

  function handleDragOver(e) {
    e.preventDefault();
    isDragging = true;
  }

  function handleDragLeave(e) {
    e.preventDefault();
    isDragging = false;
  }

  async function handleDrop(e) {
    e.preventDefault();
    isDragging = false;
    
    const files = e.dataTransfer?.files;
    if (files && files.length > 0) {
      const file = files[0];
      // For drag and drop, we need to use the file picker
      // since we can't get the full path from the browser API
      await handleBrowse();
    }
  }
</script>

<div 
  class="dropzone"
  class:dragging={isDragging}
  role="button"
  tabindex="0"
  ondragover={handleDragOver}
  ondragleave={handleDragLeave}
  ondrop={handleDrop}
  onclick={handleBrowse}
  onkeydown={(e) => e.key === 'Enter' && handleBrowse()}
>
  <div class="dropzone-content">
    <div class="icon-container">
      <svg class="upload-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
        <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
        <polyline points="17 8 12 3 7 8"></polyline>
        <line x1="12" y1="3" x2="12" y2="15"></line>
      </svg>
    </div>
    
    <h2>Drop your file here</h2>
    <p>or click to browse</p>
    
    <div class="supported-formats">
      <span class="format-group">
        <strong>Images:</strong> PNG, JPG, GIF, WebP, BMP, ICO, TIFF
      </span>
      <span class="format-group">
        <strong>Documents:</strong> PDF, TXT, MD, HTML
      </span>
      <span class="format-group">
        <strong>Data:</strong> JSON, CSV, XML, YAML
      </span>
      <span class="format-group">
        <strong>Code:</strong> JS, TS, PY, RS, and more
      </span>
    </div>
  </div>
  
  <div class="glow"></div>
</div>

<style>
  .dropzone {
    position: relative;
    height: calc(100vh - 200px);
    min-height: 400px;
    border: 2px dashed rgba(99, 102, 241, 0.4);
    border-radius: 24px;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: all 0.3s ease;
    background: rgba(99, 102, 241, 0.03);
    overflow: hidden;
  }

  .dropzone:hover, .dropzone.dragging {
    border-color: #6366f1;
    background: rgba(99, 102, 241, 0.08);
  }

  .dropzone.dragging {
    transform: scale(1.01);
  }

  .glow {
    position: absolute;
    top: 50%;
    left: 50%;
    width: 300px;
    height: 300px;
    background: radial-gradient(circle, rgba(99, 102, 241, 0.15) 0%, transparent 70%);
    transform: translate(-50%, -50%);
    pointer-events: none;
    opacity: 0;
    transition: opacity 0.3s ease;
  }

  .dropzone:hover .glow {
    opacity: 1;
  }

  .dropzone-content {
    text-align: center;
    z-index: 1;
    padding: 2rem;
  }

  .icon-container {
    width: 100px;
    height: 100px;
    margin: 0 auto 1.5rem;
    background: linear-gradient(135deg, rgba(99, 102, 241, 0.2) 0%, rgba(139, 92, 246, 0.2) 100%);
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    border: 2px solid rgba(99, 102, 241, 0.3);
  }

  .upload-icon {
    width: 48px;
    height: 48px;
    color: #a5b4fc;
  }

  h2 {
    font-size: 1.75rem;
    font-weight: 600;
    margin-bottom: 0.5rem;
    color: #e2e8f0;
  }

  p {
    color: #64748b;
    margin-bottom: 2rem;
    font-size: 1rem;
  }

  .supported-formats {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    max-width: 400px;
    margin: 0 auto;
  }

  .format-group {
    font-size: 0.8rem;
    color: #64748b;
    padding: 0.4rem 0.8rem;
    background: rgba(255, 255, 255, 0.03);
    border-radius: 8px;
  }

  .format-group strong {
    color: #a5b4fc;
  }
</style>

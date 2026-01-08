<script>
  import FileDropzone from './components/FileDropzone.svelte';
  import FileCard from './components/FileCard.svelte';
  import ConversionPanel from './components/ConversionPanel.svelte';
  import Toast from './components/Toast.svelte';
  import Header from './components/Header.svelte';
  import { invoke } from '@tauri-apps/api/core';

  let selectedFile = $state(null);
  let fileInfo = $state(null);
  let supportedFormats = $state([]);
  let isConverting = $state(false);
  let conversionResult = $state(null);
  let toast = $state({ show: false, message: '', type: 'success' });

  async function handleFileSelected(event) {
    const path = event.detail.path;
    
    try {
      fileInfo = await invoke('get_file_info', { path });
      selectedFile = path;
      
      supportedFormats = await invoke('get_supported_formats', {
        fileType: fileInfo.file_type,
        extension: fileInfo.extension
      });
      
      conversionResult = null;
    } catch (error) {
      showToast('Failed to load file: ' + error, 'error');
    }
  }

  async function handleConvert(event) {
    const { format, outputDir, options } = event.detail;
    
    isConverting = true;
    conversionResult = null;
    
    try {
      const result = await invoke('convert_file', {
        inputPath: selectedFile,
        outputFormat: format,
        outputDir: outputDir,
        options: options
      });
      
      conversionResult = result;
      
      if (result.success) {
        showToast('File converted successfully!', 'success');
      } else {
        showToast('Conversion failed: ' + result.error, 'error');
      }
    } catch (error) {
      showToast('Conversion error: ' + error, 'error');
      conversionResult = { success: false, error: error.toString() };
    } finally {
      isConverting = false;
    }
  }

  function showToast(message, type = 'success') {
    toast = { show: true, message, type };
    setTimeout(() => {
      toast = { ...toast, show: false };
    }, 4000);
  }

  function clearFile() {
    selectedFile = null;
    fileInfo = null;
    supportedFormats = [];
    conversionResult = null;
  }
</script>

<main>
  <Header />
  
  <div class="container">
    {#if !selectedFile}
      <FileDropzone onfileselected={handleFileSelected} />
    {:else}
      <div class="conversion-layout">
        <div class="left-panel">
          <FileCard 
            {fileInfo} 
            filePath={selectedFile}
            onclear={clearFile}
          />
        </div>
        
        <div class="right-panel">
          <ConversionPanel 
            {fileInfo}
            {supportedFormats}
            {isConverting}
            {conversionResult}
            onconvert={handleConvert}
          />
        </div>
      </div>
    {/if}
  </div>
  
  {#if toast.show}
    <Toast message={toast.message} type={toast.type} />
  {/if}
</main>

<style>
  :global(*) {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
  }

  :global(body) {
    font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
    background: linear-gradient(135deg, #0f0f23 0%, #1a1a2e 50%, #16213e 100%);
    min-height: 100vh;
    color: #e2e8f0;
  }

  main {
    min-height: 100vh;
    display: flex;
    flex-direction: column;
  }

  .container {
    flex: 1;
    max-width: 1200px;
    margin: 0 auto;
    padding: 2rem;
    width: 100%;
  }

  .conversion-layout {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 2rem;
    height: calc(100vh - 180px);
  }

  .left-panel, .right-panel {
    background: rgba(255, 255, 255, 0.03);
    border-radius: 20px;
    border: 1px solid rgba(255, 255, 255, 0.08);
    backdrop-filter: blur(10px);
    padding: 1.5rem;
    display: flex;
    flex-direction: column;
    min-width: 0;
    overflow: hidden;
  }

  @media (max-width: 900px) {
    .conversion-layout {
      grid-template-columns: 1fr;
      height: auto;
    }
  }
</style>

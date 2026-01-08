<script>
</script>

<header>
  <div class="logo">
    <div class="logo-icon">
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path>
        <polyline points="14 2 14 8 20 8"></polyline>
        <path d="M12 18v-6"></path>
        <path d="M9 15l3 3 3-3"></path>
      </svg>
    </div>
    <div class="logo-text">
      <h1>File Converter</h1>
      <span class="tagline">Local • Fast • Private</span>
    </div>
  </div>
  
  <div class="badges">
    <span class="badge">Images</span>
    <span class="badge">PDFs</span>
    <span class="badge">Text</span>
    <span class="badge">Data</span>
    <span class="badge">Code</span>
  </div>
</header>

<style>
  header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1.5rem 2rem;
    background: rgba(0, 0, 0, 0.2);
    border-bottom: 1px solid rgba(255, 255, 255, 0.05);
  }

  .logo {
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  .logo-icon {
    width: 48px;
    height: 48px;
    background: linear-gradient(135deg, #6366f1 0%, #8b5cf6 100%);
    border-radius: 14px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: white;
  }

  .logo-icon svg {
    width: 28px;
    height: 28px;
  }

  .logo-text h1 {
    font-size: 1.5rem;
    font-weight: 700;
    background: linear-gradient(135deg, #fff 0%, #a5b4fc 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
  }

  .tagline {
    font-size: 0.75rem;
    color: #64748b;
    letter-spacing: 0.1em;
    text-transform: uppercase;
  }

  .badges {
    display: flex;
    gap: 0.5rem;
  }

  .badge {
    padding: 0.4rem 0.8rem;
    background: rgba(99, 102, 241, 0.15);
    border: 1px solid rgba(99, 102, 241, 0.3);
    border-radius: 20px;
    font-size: 0.75rem;
    color: #a5b4fc;
    font-weight: 500;
  }

  @media (max-width: 700px) {
    header {
      flex-direction: column;
      gap: 1rem;
    }
    
    .badges {
      flex-wrap: wrap;
      justify-content: center;
    }
  }
</style>

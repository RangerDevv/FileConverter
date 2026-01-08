<script>
  let { message, type = 'success' } = $props();
</script>

<div class="toast" class:success={type === 'success'} class:error={type === 'error'}>
  {#if type === 'success'}
    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
      <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"></path>
      <polyline points="22 4 12 14.01 9 11.01"></polyline>
    </svg>
  {:else}
    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
      <circle cx="12" cy="12" r="10"></circle>
      <line x1="12" y1="8" x2="12" y2="12"></line>
      <line x1="12" y1="16" x2="12.01" y2="16"></line>
    </svg>
  {/if}
  <span>{message}</span>
</div>

<style>
  .toast {
    position: fixed;
    bottom: 2rem;
    left: 50%;
    transform: translateX(-50%);
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 1rem 1.5rem;
    border-radius: 12px;
    font-weight: 500;
    animation: slideUp 0.3s ease;
    z-index: 1000;
    box-shadow: 0 10px 40px rgba(0, 0, 0, 0.3);
  }

  .toast.success {
    background: linear-gradient(135deg, #059669 0%, #10b981 100%);
    color: white;
  }

  .toast.error {
    background: linear-gradient(135deg, #dc2626 0%, #ef4444 100%);
    color: white;
  }

  .toast svg {
    width: 22px;
    height: 22px;
  }

  @keyframes slideUp {
    from {
      opacity: 0;
      transform: translateX(-50%) translateY(20px);
    }
    to {
      opacity: 1;
      transform: translateX(-50%) translateY(0);
    }
  }
</style>

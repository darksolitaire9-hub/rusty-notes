<!-- src/lib/components/onboarding/OnboardingSubmit.svelte -->
<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';

  let {
    notesFolder,
    autoSaveInterval,
    useTrash,
    disabled = false,
    onComplete
  } = $props<{
    notesFolder: string;
    autoSaveInterval: number;
    useTrash: boolean;
    disabled?: boolean;
    onComplete?: () => void;
  }>();

  let saving = $state(false);
  let errorMsg = $state<string | null>(null);

  async function handleSubmit(event: SubmitEvent) {
    event.preventDefault();

    if (disabled || saving) return;

    saving = true;
    errorMsg = null;

    try {
      await invoke('complete_onboarding'); // Rust only needs to flip flag
      onComplete?.();                     // tells layout to close modal
    } catch (err) {
      console.error(err);
      errorMsg = 'Failed to save settings. Please try again.';
    } finally {
      saving = false;
    }
  }
</script>

<form onsubmit={handleSubmit} class="space-y-6">
  {#if errorMsg}
    <p class="text-sm text-red-500">{errorMsg}</p>
  {/if}

  <button
    type="submit"
    class="inline-flex w-full items-center justify-center rounded-lg bg-primary px-4 py-2 text-sm font-medium text-primary-foreground shadow-sm transition-colors disabled:cursor-not-allowed disabled:opacity-60 hover:bg-primary/90"
    disabled={disabled || saving}
  >
    {saving ? 'Saving…' : 'Start using Rusty Notes'}
  </button>
</form>

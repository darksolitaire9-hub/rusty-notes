<!-- src/lib/components/onboarding/OnboardingForm.svelte -->
<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import StorageLocationField from './StorageLocationField.svelte';
  import AutoSaveIntervalField from './AutoSaveIntervalField.svelte';
  import DeleteBehaviorField from './DeleteBehaviorField.svelte';
  import ErrorBanner from './ErrorBanner.svelte';

  let {
    onComplete
  } = $props<{
    onComplete?: () => void;
  }>();

  let notesFolder = $state<string | null>(null);
  let autoSaveInterval = $state(30);
  let useTrash = $state(true);
  let confirmedRisk = $state(false);
  let saving = $state(false);
  let errorMsg = $state<string | null>(null);

  const canContinue = () =>
    !saving &&
    notesFolder !== null &&
    (useTrash || confirmedRisk);

  async function handleSubmit(event: SubmitEvent) {
    event.preventDefault();

    if (!canContinue()) return;
    saving = true;
    errorMsg = null;

    try {
      await invoke('complete_onboarding', {
        notesFolder,
        autoSaveInterval,
        useTrash
      });

      onComplete?.();
    } catch (err) {
      console.error(err);
      errorMsg = 'Failed to save settings. Please try again.';
    } finally {
      saving = false;
    }
  }
</script>

<form onsubmit={handleSubmit} class="space-y-6">
  <header class="space-y-2">
    <h1 class="text-2xl font-semibold tracking-tight">Welcome to Rusty Notes</h1>
    <p class="text-sm text-muted-foreground">
      Rusty Notes saves your notes on this device, and encryption is always on by default.
    </p>
  </header>

  <StorageLocationField
    value={notesFolder}
    disabled={saving}
    onChange={(folder) => (notesFolder = folder)}
  />

  <AutoSaveIntervalField
    bind:value={autoSaveInterval}
    disabled={saving}
  />

  <DeleteBehaviorField
    bind:useTrash={useTrash}
    bind:confirmedRisk={confirmedRisk}
    disabled={saving}
  />

  {#if errorMsg}
    <ErrorBanner {errorMsg} />
  {/if}

  <button
    type="submit"
    class="inline-flex w-full items-center justify-center rounded-lg bg-primary px-4 py-2 text-sm font-medium text-primary-foreground shadow-sm transition-colors disabled:cursor-not-allowed disabled:opacity-60 hover:bg-primary/90"
    disabled={!canContinue()}
  >
    {saving ? 'Saving…' : 'Start using Rusty Notes'}
  </button>
</form>

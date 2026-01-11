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
      // 1) Load current settings (to preserve unknown fields / future versions)
      const current = await invoke<{
        version: number;
        notes_folder: string;
        auto_save_interval_secs: number;
        delete_behavior: 'MoveToTrash' | 'Permanent';
        onboarding_completed: boolean;
      }>('get_settings');

      // 2) Build updated settings
      const newSettings = {
        ...current,
        notes_folder: notesFolder,
        auto_save_interval_secs: autoSaveInterval,
        delete_behavior: useTrash ? 'MoveToTrash' : 'Permanent',
        onboarding_completed: true,
      };

      // 3) Persist settings
      await invoke('update_settings', { newSettings : newSettings });

      // 4) (Optional) flip onboarding flag separately if you want to keep that command
      // await invoke('complete_onboarding');

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

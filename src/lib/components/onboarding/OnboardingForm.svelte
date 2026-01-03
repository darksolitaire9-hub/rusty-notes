<!-- src/lib/components/onboarding/OnboardingForm.svelte -->
<script lang="ts">
  import StorageLocationField from './StorageLocationField.svelte';
  import AutoSaveIntervalField from './AutoSaveIntervalField.svelte';
  import DeleteBehaviorField from './DeleteBehaviorField.svelte';
  import ErrorBanner from './ErrorBanner.svelte';
  import OnboardingSubmit from './OnboardingSubmit.svelte';

  let {
    onComplete
  } = $props<{
    onComplete?: () => void;
  }>();

  let notesFolder = $state<string | null>(null);
  let autoSaveInterval = $state(30);
  let useTrash = $state(true);
  let confirmedRisk = $state(false);
  let errorMsg = $state<string | null>(null);

  const canContinue = () =>
    notesFolder !== null &&
    (useTrash || confirmedRisk);
</script>

<div class="space-y-6">
  <header class="space-y-2">
    <h1 class="text-2xl font-semibold tracking-tight">Welcome to Rusty Notes</h1>
    <p class="text-sm text-muted-foreground">
      Rusty Notes saves your notes on this device, and encryption is always on by default.
    </p>
  </header>

  <StorageLocationField
    value={notesFolder}
    onChange={(folder) => (notesFolder = folder)}
  />

  <AutoSaveIntervalField
    bind:value={autoSaveInterval}
  />

  <DeleteBehaviorField
    bind:useTrash={useTrash}
    bind:confirmedRisk={confirmedRisk}
  />

  {#if errorMsg}
    <ErrorBanner {errorMsg} />
  {/if}

  <!-- Delegate submission to OnboardingSubmit -->
  <OnboardingSubmit
    notesFolder={notesFolder ?? ''}
    autoSaveInterval={autoSaveInterval}
    useTrash={useTrash}
    disabled={!canContinue()}
    onComplete={onComplete}
  />
</div>

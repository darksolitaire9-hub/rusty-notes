<!-- src/routes/+layout.svelte -->
<script lang="ts">
  import "../app.css";

  import NavBar from "$lib/shared/components/layout/Header.svelte";
  import { handleGlobalShortcut } from "$lib/shared/shortcuts/handlers";
  import OnboardingModal from "$lib/features/settings/components/onboarding/OnboardingModal.svelte";
  import OnboardingForm from "$lib/features/settings/components/onboarding/OnboardingForm.svelte";
  import { browser } from "$app/environment";
  import { invoke } from "@tauri-apps/api/core";

  // layout children snippet
  let { children } = $props();

  // reactive state in runes mode
  let showOnboarding = $state(false);
  let settingsLoaded = $state(false);

  // Load settings from Rust backend (browser only)
  $effect(() => {
    if (!browser) return;

    (async () => {
      try {
        const settings = await invoke<{
          version: number;
          notes_folder: string;
          auto_save_interval_secs: number;
          delete_behavior: string;
          onboarding_completed: boolean;
        }>("get_settings");

        showOnboarding = !settings.onboarding_completed;
        settingsLoaded = true;
      } catch (err) {
        console.error("Failed to load settings:", err);
        settingsLoaded = true; // show UI anyway to avoid blank screen
      }
    })();
  });

  async function completeOnboarding() {
    if (!browser) return;

    try {
      // Optional extra persist; your submit component already calls this
      await invoke("complete_onboarding");
      showOnboarding = false; // ✅ hide modal
    } catch (err) {
      console.error("Failed to complete onboarding:", err);
    }
  }
</script>

<svelte:window onkeydown={handleGlobalShortcut} />

<!-- App Container -->
<div
  class={`app-shell flex h-screen flex-col overflow-hidden bg-background text-foreground ${
    showOnboarding ? "blurred" : ""
  }`}
>
  <NavBar />

  <div class="flex flex-1 overflow-hidden">
    {#if settingsLoaded}
      {@render children()}
    {:else}
      <div class="flex items-center justify-center flex-1">
        <p>Loading…</p>
      </div>
    {/if}
  </div>
</div>

{#if showOnboarding && settingsLoaded}
  <OnboardingModal open={showOnboarding} onComplete={completeOnboarding}>
    {#snippet children({ onComplete }: { onComplete: () => void })}
      <!-- ✅ use callback prop, not on:complete -->
      <OnboardingForm {onComplete} />
    {/snippet}
  </OnboardingModal>
{/if}

<style>
  .app-shell {
    transition: filter 0.25s ease;
  }

  .blurred {
    filter: blur(8px) brightness(0.8);
    pointer-events: none;
  }
</style>

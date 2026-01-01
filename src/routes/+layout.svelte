<script lang="ts">
  import NavBar from '$lib/components/layout/NavBar.svelte';
  import '../app.css';
  import { handleGlobalShortcut } from '$lib/shortcuts/handlers';

  // Svelte 5: children come from $props()
  let { children } = $props();
</script>

<!-- Global keyboard listener for the entire app -->
<svelte:window on:keydown={handleGlobalShortcut} />

<!-- The App Container -->
<div class="flex h-screen flex-col overflow-hidden bg-background text-foreground">
  <!-- Top Navigation -->
  <NavBar />

  <!-- Main Content Area -->
  <div class="flex flex-1 overflow-hidden">
    {@render children()}
  </div>
</div>

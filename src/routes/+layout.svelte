<script lang="ts">
  import NavBar from '$lib/components/layout/NavBar.svelte';
  import { handleGlobalShortcut } from '$lib/shortcuts/handlers';
</script>

<!-- Global keyboard listener for the entire app -->
<svelte:window on:keydown={handleGlobalShortcut} />

<div class="app">
  <NavBar />
  <main>
    {@render children()}
  </main>
</div>

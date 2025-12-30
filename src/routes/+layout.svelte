<script lang="ts">
  import NavBar from '$lib/components/layout/NavBar.svelte';
  import '../app.css';

  // In Svelte 5, we receive children as a prop instead of using <slot>
  let { children } = $props();
</script>

<!-- The App Container -->
<div class="flex h-screen flex-col overflow-hidden bg-background text-foreground">
  
  <!-- Top Navigation -->
  <NavBar />

  <!-- Main Content Area -->
  <div class="flex flex-1 overflow-hidden">
    {@render children()}
  </div>

</div>

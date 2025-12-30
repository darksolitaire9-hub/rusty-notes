<script lang="ts">
  import { Button } from '$lib/components/ui/button';
  import { Sun, Moon } from 'lucide-svelte';
  import { themeStore } from '$lib/state/theme.svelte'; // Import the store
</script>

<header class="sticky top-0 z-50 w-full border-b border-border/40 bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60">
  <div class="container mx-auto flex h-16 max-w-screen-2xl items-center justify-between px-6">
    
    <!-- Brand -->
    <div class="flex items-center">
      <a href="/" class="flex items-center gap-2">
        <span class="font-mono text-2xl font-bold tracking-tight text-foreground transition-colors hover:text-primary">
          Rusty Notes
        </span>
      </a>
    </div>

    <!-- Actions -->
    <div class="flex items-center gap-2">
      <Button
        variant="ghost"
        size="icon"
        onclick={() => themeStore.toggle()} 
        class="h-9 w-9 rounded-full" 
      >
        {#if themeStore.isDark}
           <Sun class="h-5 w-5 transition-all" />
        {:else}
           <Moon class="h-5 w-5 transition-all" />
        {/if}
        <span class="sr-only">Toggle theme</span>
      </Button>
    </div>

  </div>
</header>

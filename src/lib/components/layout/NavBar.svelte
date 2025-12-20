<script lang="ts">
  import { onMount } from 'svelte';
  import { Button } from '$lib/components/ui/button';
  import { Sun, Moon } from 'lucide-svelte';

  let isDark = $state(true);

  function toggleTheme() {
    isDark = !isDark;
    const root = document.documentElement;
    if (isDark) {
      root.classList.add('dark');
      localStorage.setItem('theme', 'dark');
    } else {
      root.classList.remove('dark');
      localStorage.setItem('theme', 'light');
    }
  }

  onMount(() => {
    const stored = localStorage.getItem('theme');
    if (stored) {
      isDark = stored === 'dark';
    } else {
      isDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
    }
    
    const root = document.documentElement;
    if (isDark) root.classList.add('dark');
    else root.classList.remove('dark');
  });
</script>

<header class="sticky top-0 z-50 w-full border-b border-border/40 bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60">
  
  <div class="container mx-auto flex h-16 max-w-screen-2xl items-center justify-between px-6">
    
    <!-- Left: Brand -->
    <div class="flex items-center">
      <a href="/" class="flex items-center gap-2">
        <span class="font-mono text-2xl font-bold tracking-tight text-foreground transition-colors hover:text-primary">
          Rusty Notes
        </span>
      </a>
    </div>

    <!-- Right: Actions -->
    <div class="flex items-center gap-2">
      <Button
        variant="ghost"
        size="icon"
        onclick={toggleTheme}
        class="h-9 w-9 rounded-full" 
      >
        {#if isDark}
           <Sun class="h-5 w-5 transition-all" />
        {:else}
           <Moon class="h-5 w-5 transition-all" />
        {/if}
        <span class="sr-only">Toggle theme</span>
      </Button>
    </div>

  </div>
</header>

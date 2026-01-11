<script lang="ts">
  import { Button } from '$lib/shared/components/ui/button';
  import { Sun, Moon } from 'lucide-svelte';
  import { themeStore } from '$lib/shared/state/theme.svelte';
 
</script>



<header
  class="sticky top-0 z-50 w-full border-b border-border/40 
         bg-background/90 dark:bg-background/100 
         backdrop-blur supports-[backdrop-filter]:bg-background/60"
>
  <div class="flex h-16 w-full items-center justify-between">

    <!-- Brand -->
    <a href="/" class="flex items-center pl-3">
      <img
        src="/logo.png"
        alt="Rusty Notes"
        class="h-12 w-12 object-contain mix-blend-normal"
        loading="lazy"
      />  
    </a> 

    <!-- Actions -->
    <div class="flex items-center gap-2 pr-6">
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

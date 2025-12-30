<script lang="ts">
  import { Button } from '$lib/components/ui/button';
  import type { ComponentType } from 'svelte';
  
  let { 
    icon, 
    title, 
    isActive = false, 
    onclick 
  } = $props<{ 
    icon: ComponentType, 
    title: string, 
    isActive?: boolean, 
    onclick: () => void 
  }>();
</script>

<Button 
  variant={isActive ? "secondary" : "ghost"} 
  size="icon" 
  class="h-8 w-8 shrink-0" 
  {onclick} 
  {title}
>
  <svelte:component this={icon} class="h-4 w-4" />
</Button>

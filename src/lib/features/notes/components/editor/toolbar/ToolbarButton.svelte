<script lang="ts">
  import { Button } from '$lib/shared/components/ui/button';

  let {
    icon,
    title,
    isActive = false,
    onclick
  } = $props();
</script>

<Button
  variant={isActive ? "secondary" : "ghost"}
  size="icon"
  class="h-8 w-8 shrink-0"
  onclick={onclick}
  title={title}
>
  <icon class="h-4 w-4"></icon>
</Button>

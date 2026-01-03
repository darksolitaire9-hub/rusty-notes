<!-- src/lib/onboarding/ErrorBanner.svelte -->
<script lang="ts">
  export let errorMsg: string;
</script>

<p
  class="rounded-md border border-destructive/40 bg-destructive/10 px-3 py-2 text-sm text-destructive"
>
  {errorMsg}
</p>

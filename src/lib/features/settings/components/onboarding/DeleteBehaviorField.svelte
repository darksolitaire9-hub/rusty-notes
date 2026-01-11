<!-- src/lib/components/onboarding/DeleteBehaviorField.svelte -->
<script lang="ts">
  export let useTrash: boolean;
  export let confirmedRisk: boolean;
  export let disabled = false;
</script>

<section class="space-y-2">
  <h2 class="text-sm font-medium text-foreground">Delete behavior</h2>
  <p class="text-xs text-muted-foreground">
    What happens when you delete a note. Trash keeps deleted notes for 30 days, permanent delete removes them immediately.
  </p>

  <div class="space-y-1 text-sm text-foreground">
    <label class="flex items-center gap-2">
      <input
        type="radio"
        class="h-4 w-4 border-input text-primary focus:ring-ring"
        bind:group={useTrash}
        value={true}
        {disabled}
      />
      <span>Move to trash for 30 days (recommended)</span>
    </label>

    <label class="flex items-center gap-2">
      <input
        type="radio"
        class="h-4 w-4 border-input text-primary focus:ring-ring"
        bind:group={useTrash}
        value={false}
        {disabled}
      />
      <span>Delete permanently</span>
    </label>
  </div>

  {#if !useTrash}
    <label class="mt-2 flex items-start gap-2 text-sm text-destructive">
      <input
        type="checkbox"
        class="mt-[2px] h-4 w-4 border-input text-destructive focus:ring-ring"
        bind:checked={confirmedRisk}
        {disabled}
      />
      <span>I understand permanent deletion cannot be undone</span>
    </label>
  {/if}
</section>

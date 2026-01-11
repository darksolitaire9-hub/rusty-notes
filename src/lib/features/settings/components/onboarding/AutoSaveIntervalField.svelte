<!-- src/lib/components/onboarding/AutoSaveIntervalField.svelte -->
<script lang="ts">
  export let value: number;
  export let disabled = false;
</script>

<section class="space-y-2">
  <h2 class="text-sm font-medium text-foreground">Auto-save interval</h2>
  <p class="text-xs text-muted-foreground">
    How often your notes are automatically saved while you type.
  </p>

  <div class="space-y-1 text-sm text-foreground">
    <label class="flex items-center gap-2">
      <input
        type="radio"
        class="h-4 w-4 border-input text-primary focus:ring-ring"
        bind:group={value}
        value={30}
        {disabled}
      />
      <span>Every 30 seconds</span>
    </label>

    <label class="flex items-center gap-2">
      <input
        type="radio"
        class="h-4 w-4 border-input text-primary focus:ring-ring"
        bind:group={value}
        value={60}
        {disabled}
      />
      <span>Every 1 minute</span>
    </label>

    <label class="flex items-center gap-2">
      <input
        type="radio"
        class="h-4 w-4 border-input text-primary focus:ring-ring"
        bind:group={value}
        value={300}
        {disabled}
      />
      <span>Every 5 minutes</span>
    </label>

    <label class="flex items-center gap-2">
      <input
        type="radio"
        class="h-4 w-4 border-input text-primary focus:ring-ring"
        bind:group={value}
        value={0}
        {disabled}
      />
      <span>Manual only (Ctrl+S)</span>
    </label>
  </div>
</section>

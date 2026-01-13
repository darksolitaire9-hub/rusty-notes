<script lang="ts">
  import { noteService } from "$lib/features/notes/store/notes-service.svelte";
  import { Button } from "$lib/shared/components/ui/button";
  import { Save } from '@lucide/svelte';

  let note = $derived(noteService.activeNote);

  function focusTitle(node: HTMLInputElement, shouldFocus: boolean) {
    if (shouldFocus) {
      setTimeout(() => {
        node.focus();
        node.select();
        noteService.clearTitleFocus();
      }, 50);
    }

    return {
      update(newShouldFocus: boolean) {
        if (newShouldFocus) {
          setTimeout(() => {
            node.focus();
            node.select();
            noteService.requestTitleFocus();
          }, 50);
        }
      },
    };
  }

  function onInput(e: Event) {
    noteService.updateTitle((e.currentTarget as HTMLInputElement).value);
  }
</script>

{#if note}
  <div class="border-b border-border px-8 py-8 z-20 relative bg-muted/30 backdrop-blur">
    <div class="flex items-center gap-4">
      <div class="flex-1">
        {#key note.id}
          <input
            use:focusTitle={noteService.shouldFocusTitle}
            type="text"
            value={note.title}
            oninput={onInput}
            placeholder="Untitled"
            class="w-full bg-transparent text-4xl font-extrabold text-foreground focus:outline-none placeholder:text-muted-foreground/40 tracking-tight leading-relaxed py-2"
          />
        {/key}
      </div>

      {#if note.isDirty}
        <Button
          onclick={() => noteService.saveActive()}
          variant="default"
          size="sm"
          class="shrink-0 gap-2"
          title="Save changes (Ctrl+S)"
        >
          <Save class="h-4 w-4" />
          Save
        </Button>
      {/if}
    </div>
  </div>
{/if}

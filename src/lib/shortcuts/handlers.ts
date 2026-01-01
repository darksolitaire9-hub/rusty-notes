import { noteStore } from '$lib/state/notes.svelte';

export function handleGlobalShortcut(event: KeyboardEvent) {
  // Ctrl+N → New note (works everywhere, even in editor)
  if (
    event.ctrlKey &&
    !event.shiftKey &&
    event.key.toLowerCase() === 'n'
  ) {
    event.preventDefault();
    event.stopPropagation();  // Stop editor from processing it
    noteStore.create();
    return;
  }

  // For other shortcuts that should NOT interrupt typing:
  const target = event.target as HTMLElement | null;
  const inTextField = target?.matches('input, textarea, [contenteditable]');

 
}

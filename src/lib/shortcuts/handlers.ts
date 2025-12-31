// src/lib/shortcuts/handlers.ts
import { noteStore } from '$lib/state/notes.svelte';

export function handleGlobalShortcut(event: KeyboardEvent) {
  const target = event.target as HTMLElement | null;
  const inTextField = target?.matches('input, textarea, [contenteditable]');

  // Ctrl+N → New note (no Shift required)
  if (
    event.ctrlKey &&
    !event.shiftKey &&                      // ensure it's NOT Ctrl+Shift+N
    event.key.toLowerCase() === 'n' &&
    !inTextField
  ) {
    event.preventDefault();
    noteStore.create();
    return;
  }
}

import { noteStore } from '$lib/state/notes.svelte';
import { modalOpen } from '$lib/state/ui';
import { get } from 'svelte/store';

export function handleGlobalShortcut(event: KeyboardEvent) {
  // 1. If onboarding/modal is open, ignore global shortcuts
  if (get(modalOpen)) {
    // Do nothing so background shortcuts are effectively blocked
    return;
  }

  // 2. Ctrl+N → New note (exact combo: Ctrl + n, no Shift)
  if (
    event.ctrlKey &&
    !event.shiftKey &&
    event.key.toLowerCase() === 'n'
  ) {
    event.preventDefault();
    event.stopPropagation();
    noteStore.create();
    return;
  }

  // 3. You can keep this for future shortcuts that should not interrupt typing
  const target = event.target as HTMLElement | null;
  const inTextField = target?.matches('input, textarea, [contenteditable]');
}

import { noteService } from '$lib/features/notes/notes-index';
import { modalOpen } from '$lib/shared/state/ui';
import { get } from 'svelte/store';

export function handleGlobalShortcut(event: KeyboardEvent) {
  // 1. If onboarding/modal is open, ignore global shortcuts
  if (get(modalOpen)) {
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
    noteService.create();
    return;
  }

  // 3. Keep for future shortcuts that should not interrupt typing
  const target = event.target as HTMLElement | null;
  const inTextField = target?.matches('input, textarea, [contenteditable]');
}

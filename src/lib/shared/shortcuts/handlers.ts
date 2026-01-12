import { modalOpen } from '$lib/shared/state/ui';
import { get } from 'svelte/store';

export function handleGlobalShortcut(event: KeyboardEvent) {
  // 1. If onboarding/modal is open, ignore global shortcuts
  if (get(modalOpen)) {
    return;
  }

  // 2. Don't handle shortcuts in text fields
  const target = event.target as HTMLElement | null;
  const inTextField = target?.matches('input, textarea, [contenteditable]');
  
  if (inTextField) {
    return;
  }

  // Note: Ctrl+N is now handled by Rust global shortcuts
  // Add other DOM-specific shortcuts here if needed
  
  // Example future shortcuts:
  // if (event.key === 'Escape') {
  //   // Handle escape
  // }
}
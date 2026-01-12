import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { noteService } from '$lib/features/notes/store/notes-service.svelte';

let unlisteners: UnlistenFn[] = [];

/**
 * Register all shortcut event listeners from Rust
 */
export async function registerShortcutListeners(): Promise<void> {
  // Clean up existing listeners
  await unregisterShortcutListeners();

  // Create new note (Ctrl+N)
  const unlistenCreate = await listen('create-new-note', () => {
    console.log('📝 Shortcut triggered: Create new note');
    noteService.create();
  });
  unlisteners.push(unlistenCreate);

  // Future: Add more listeners here
  // const unlistenDelete = await listen('delete-note', () => { ... });
  // unlisteners.push(unlistenDelete);

  console.log('✓ Shortcut listeners registered');
}

/**
 * Unregister all shortcut event listeners
 */
export async function unregisterShortcutListeners(): Promise<void> {
  for (const unlisten of unlisteners) {
    unlisten();
  }
  unlisteners = [];
}
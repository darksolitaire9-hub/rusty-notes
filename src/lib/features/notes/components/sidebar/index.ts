
export { default as NotesSidebar } from './NotesSidebar.svelte';
export { default as SidebarHeader } from './SidebarHeader.svelte';
export { default as NotesList } from './NotesList.svelte';
export { default as NoteItem } from './NoteItem.svelte';
export { default as SidebarToggleButton } from './SidebarToggleButton.svelte';


// ============================================================================
// FILE: $lib/features/notes/types.ts
// ============================================================================
// Type definitions for Note entity
// (Assuming this file exists - adjust path as needed)

export interface Note {
  id: string;
  title: string;
  body: string;
  isDirty: boolean;      
  updatedAt: Date;
  createdAt: Date;
}


// ============================================================================
// USAGE EXAMPLE
// ============================================================================
// In your main layout or page:

/*
<script lang="ts">
  import { NotesSidebar } from '$lib/features/notes/components/sidebar';
</script>

<div class="flex h-screen">
  <NotesSidebar />
  <main class="flex-1 p-8">
    <!-- Your main content here -->
  </main>
</div>
*/

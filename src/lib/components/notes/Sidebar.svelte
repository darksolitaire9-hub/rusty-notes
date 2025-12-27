<script lang="ts">
  import { Button } from '$lib/components/ui/button';
  import { Plus, Trash2, PanelLeftClose, PanelLeftOpen, FileText } from 'lucide-svelte';
  
  type Note = { id: string; title: string; body: string };

  let { 
    notes = [], 
    selectedId = null,
    onCreate,
    onSelect,
    onDelete 
  } = $props<{
    notes: Note[];
    selectedId: string | null;
    onCreate: () => void;
    onSelect: (id: string) => void;
    onDelete: (id: string) => void;
  }>();

  // Internal state for "Accordion/Collapse" behavior
  let isCollapsed = $state(false);

  function toggleSidebar() {
    isCollapsed = !isCollapsed;
  }
</script>

<!-- 
  transition-all duration-300: Animates the width change 
  w-64 vs w-16: The widths for open vs collapsed states
-->
<aside class="flex-none border-r border-border bg-muted/10 flex flex-col h-full transition-all duration-300 ease-in-out {isCollapsed ? 'w-16' : 'w-64'}">
  
  <!-- Sidebar Header (Toggle & Create) -->
  <div class="p-4 border-b border-border flex items-center justify-between gap-2">
    <!-- Collapse Toggle -->
    <Button variant="ghost" size="icon" class="h-8 w-8 shrink-0" onclick={toggleSidebar}>
      {#if isCollapsed}
        <PanelLeftOpen class="h-4 w-4" />
      {:else}
        <PanelLeftClose class="h-4 w-4" />
      {/if}
    </Button>

    <!-- Create Button (Text hides when collapsed) -->
    <Button 
      onclick={onCreate} 
      variant={isCollapsed ? "ghost" : "default"}
      size={isCollapsed ? "icon" : "default"}
      class="{isCollapsed ? 'h-8 w-8' : 'flex-1'} transition-all"
    >
      <Plus class="h-4 w-4 {isCollapsed ? '' : 'mr-2'}" />
      {#if !isCollapsed}
        New Note
      {/if}
    </Button>
  </div>
  
  <!-- Note List -->
  <div class="flex-1 overflow-y-auto p-2 space-y-1">
    {#each notes as note (note.id)}
      <button
        onclick={() => onSelect(note.id)}
        class="group flex w-full items-center gap-3 rounded-lg p-3 text-left transition-all hover:bg-accent 
        {selectedId === note.id ? 'bg-accent text-accent-foreground' : ''} 
        {isCollapsed ? 'justify-center' : 'justify-start'}"
        title={note.title} 
      >
        <!-- Icon always shows -->
        <FileText class="h-4 w-4 shrink-0 opacity-70" />

        <!-- Text details hide when collapsed -->
        {#if !isCollapsed}
          <div class="min-w-0 flex-1 overflow-hidden">
            <div class="flex items-center justify-between font-semibold">
              <span class="truncate text-sm">{note.title || 'Untitled'}</span>
              
              {#if selectedId === note.id}
                <div 
                  role="button" 
                  tabindex="0"
                  class="opacity-0 group-hover:opacity-100 transition-opacity p-1 rounded-sm hover:bg-background/20"
                  onclick={(e) => { e.stopPropagation(); onDelete(note.id); }}
                  onkeydown={(e) => { if(e.key === 'Enter') onDelete(note.id); }}
                >
                   <Trash2 class="h-3 w-3 text-destructive hover:text-destructive/80" />
                </div>
              {/if}
            </div>
            <div class="line-clamp-1 text-xs text-muted-foreground truncate">
              {note.body || 'No additional text'}
            </div>
          </div>
        {/if}
      </button>
    {/each}
  </div>
</aside>

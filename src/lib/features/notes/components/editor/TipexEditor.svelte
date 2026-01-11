<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { Editor } from '@tiptap/core';
  import StarterKit from '@tiptap/starter-kit';
  import TextAlign from '@tiptap/extension-text-align';
  import Underline from '@tiptap/extension-underline';
  import Link from '@tiptap/extension-link';
  import TaskList from '@tiptap/extension-task-list';
  import TaskItem from '@tiptap/extension-task-item';
  
  import Toolbar from './toolbar/Toolbar.svelte';

  let { value, onChange } = $props<{ 
    value: string, 
    onChange: (html: string) => void 
  }>();

  let element: HTMLDivElement;
  let editor: Editor | null = $state(null);

  onMount(() => {
    editor = new Editor({
      element: element,
      extensions: [
        StarterKit, // ✅ Markdown shortcuts (# H1, ## H2, etc.) still work
        Underline,
        TextAlign.configure({
          types: ['heading', 'paragraph'],
        }),
        Link.configure({
          openOnClick: false,
        }),
        TaskList,
        TaskItem.configure({
          nested: true,
        }),
      ],
      content: value,
      editorProps: {
        attributes: {
          // ✅ Theme-aware: prose adapts to light/dark
          class: 'prose prose-stone dark:prose-invert max-w-none focus:outline-none min-h-[50vh] px-8 py-6'
        }
      },
      onUpdate: ({ editor }) => {
        onChange(editor.getHTML());
      }
    });
  });

  onDestroy(() => {
    editor?.destroy();
  });

  $effect(() => {
    if (editor && value !== editor.getHTML()) {
       editor.commands.setContent(value, false);
    }
  });
</script>

<div class="flex flex-col h-full w-full bg-background text-foreground">
  
  <!-- Toolbar Component - Theme aware -->
  {#if editor}
    <div class="sticky top-0 z-20 border-b border-border bg-muted px-4 py-3 shadow-sm">
      <Toolbar {editor} />
    </div>
  {/if}

  <!-- Editor Content -->
  <div class="flex-1 overflow-y-auto bg-background" bind:this={element}></div>
</div>

<style>
  /* Base Editor Styles - Theme Aware */
  :global(.ProseMirror) {
    color: oklch(var(--foreground));
    min-height: 100%;
    outline: none;
  }
  
  /* Heading Hierarchy - Theme Aware */
  :global(.ProseMirror h1) { 
    color: oklch(var(--foreground)); 
    font-weight: 800; 
    font-size: 2.5rem; 
    margin-top: 2rem; 
    margin-bottom: 1rem; 
  }
  :global(.ProseMirror h2) { 
    color: oklch(var(--foreground)); 
    opacity: 0.95;
    font-weight: 700; 
    font-size: 2rem; 
    margin-top: 1.5rem; 
    margin-bottom: 0.75rem; 
  }
  :global(.ProseMirror h3) { 
    color: oklch(var(--foreground)); 
    opacity: 0.9;
    font-weight: 600; 
    font-size: 1.5rem; 
    margin-top: 1.25rem; 
    margin-bottom: 0.5rem; 
  }
  :global(.ProseMirror h4) { 
    color: oklch(var(--muted-foreground)); 
    font-weight: 600; 
    font-size: 1.25rem; 
    margin-top: 1rem; 
    margin-bottom: 0.5rem; 
  }
  :global(.ProseMirror h5) { 
    color: oklch(var(--muted-foreground)); 
    opacity: 0.9;
    font-weight: 600; 
    font-size: 1.1rem; 
    margin-top: 0.75rem; 
    margin-bottom: 0.5rem; 
  }
  :global(.ProseMirror h6) { 
    color: oklch(var(--muted-foreground)); 
    opacity: 0.8;
    font-weight: 600; 
    font-size: 1rem; 
    margin-top: 0.5rem; 
    margin-bottom: 0.5rem; 
  }
  
  /* Content Styles */
  :global(.ProseMirror p) { 
    margin-bottom: 1rem; 
  }
  :global(.ProseMirror ul) { 
    list-style-type: disc; 
    padding-left: 2rem; 
    margin-bottom: 1rem; 
  }
  :global(.ProseMirror ol) { 
    list-style-type: decimal; 
    padding-left: 2rem; 
    margin-bottom: 1rem; 
  }
  :global(.ProseMirror blockquote) { 
    border-left: 4px solid oklch(var(--primary)); 
    padding-left: 1.5rem; 
    color: oklch(var(--muted-foreground)); 
    margin: 1.5rem 0; 
  }
  :global(.ProseMirror code) { 
    background: oklch(var(--muted)); 
    color: oklch(var(--primary)); 
    padding: 0.2rem 0.4rem; 
    border-radius: 4px; 
    font-size: 0.9em; 
  }
  :global(.ProseMirror pre) { 
    background: oklch(var(--muted)); 
    color: oklch(var(--foreground)); 
    padding: 1rem; 
    border-radius: 8px; 
    overflow-x: auto; 
    margin: 1rem 0; 
  }
  :global(.ProseMirror a) { 
    color: oklch(var(--primary)); 
    text-decoration: underline; 
  }
  :global(.ProseMirror hr) { 
    border: none; 
    border-top: 2px solid oklch(var(--border)); 
    margin: 2rem 0; 
  }
  
  /* Task List Styles */
  :global(.ProseMirror ul[data-type="taskList"]) { 
    list-style: none; 
    padding-left: 0; 
  }
  :global(.ProseMirror ul[data-type="taskList"] li) { 
    display: flex; 
    align-items: flex-start; 
  }
  :global(.ProseMirror ul[data-type="taskList"] li > label) { 
    margin-right: 0.5rem; 
    user-select: none; 
    margin-top: 0.35em;
  }
  :global(.ProseMirror ul[data-type="taskList"] li > div) { 
    flex: 1; 
  }
</style>

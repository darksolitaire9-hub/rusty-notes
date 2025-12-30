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
        StarterKit,
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
          class: 'prose prose-invert max-w-none focus:outline-none min-h-[50vh] px-8 py-6'
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

<div class="flex flex-col h-full w-full bg-slate-950 text-slate-200">
  
  <!-- Toolbar Component -->
  {#if editor}
    <Toolbar {editor} />
  {/if}

  <!-- Editor Content -->
  <div class="flex-1 overflow-y-auto" bind:this={element}></div>
</div>

<style>
  :global(.ProseMirror) {
    color: #cbd5e1;
    min-height: 100%;
    outline: none;
  }
  :global(.ProseMirror h1) { color: #fff; font-weight: 800; font-size: 2.5rem; margin-top: 2rem; margin-bottom: 1rem; }
  :global(.ProseMirror h2) { color: #f1f5f9; font-weight: 700; font-size: 2rem; margin-top: 1.5rem; margin-bottom: 0.75rem; }
  :global(.ProseMirror h3) { color: #e2e8f0; font-weight: 600; font-size: 1.5rem; margin-top: 1.25rem; margin-bottom: 0.5rem; }
  :global(.ProseMirror h4) { color: #cbd5e1; font-weight: 600; font-size: 1.25rem; margin-top: 1rem; margin-bottom: 0.5rem; }
  :global(.ProseMirror h5) { color: #94a3b8; font-weight: 600; font-size: 1.1rem; margin-top: 0.75rem; margin-bottom: 0.5rem; }
  :global(.ProseMirror h6) { color: #64748b; font-weight: 600; font-size: 1rem; margin-top: 0.5rem; margin-bottom: 0.5rem; }
  :global(.ProseMirror p) { margin-bottom: 1rem; }
  :global(.ProseMirror ul) { list-style-type: disc; padding-left: 2rem; margin-bottom: 1rem; }
  :global(.ProseMirror ol) { list-style-type: decimal; padding-left: 2rem; margin-bottom: 1rem; }
  :global(.ProseMirror blockquote) { border-left: 4px solid #3b82f6; padding-left: 1.5rem; color: #94a3b8; margin: 1.5rem 0; }
  :global(.ProseMirror code) { background: #1e293b; color: #f472b6; padding: 0.2rem 0.4rem; border-radius: 4px; font-size: 0.9em; }
  :global(.ProseMirror pre) { background: #0f172a; color: #e2e8f0; padding: 1rem; border-radius: 8px; overflow-x: auto; margin: 1rem 0; }
  :global(.ProseMirror a) { color: #60a5fa; text-decoration: underline; }
  :global(.ProseMirror hr) { border: none; border-top: 2px solid #334155; margin: 2rem 0; }
  
  /* Task List Styles */
  :global(.ProseMirror ul[data-type="taskList"]) { list-style: none; padding-left: 0; }
  :global(.ProseMirror ul[data-type="taskList"] li) { display: flex; align-items: flex-start; }
  :global(.ProseMirror ul[data-type="taskList"] li > label) { margin-right: 0.5rem; user-select: none; }
  :global(.ProseMirror ul[data-type="taskList"] li > div) { flex: 1; }
</style>

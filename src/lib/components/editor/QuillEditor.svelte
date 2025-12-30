<script lang="ts">
  import { onMount } from 'svelte';
  import Quill from 'quill';
  import 'quill/dist/quill.snow.css'; // "Snow" is the nice top-toolbar theme

  let { value, onChange } = $props<{ 
    value: string, 
    onChange: (html: string) => void 
  }>();

  let editorDiv: HTMLDivElement;
  let quill: Quill;

  onMount(() => {
    quill = new Quill(editorDiv, {
      theme: 'snow',
      placeholder: 'Write something...',
      modules: {
        toolbar: [
          [{ 'header': [1, 2, 3, false] }],
          ['bold', 'italic', 'underline', 'strike'],
          ['blockquote', 'code-block'],
          [{ 'list': 'ordered'}, { 'list': 'bullet' }],
          ['clean']
        ]
      }
    });

    // Set initial content
    // We use clipboard.dangerouslyPasteHTML to handle HTML content correctly
    if (value) {
        quill.clipboard.dangerouslyPasteHTML(value);
    }

    quill.on('text-change', () => {
      onChange(quill.root.innerHTML);
    });
  });
  
  // Sync changes from outside
  $effect(() => {
      // Prevent cursor jumping by checking if content actually changed content-wise
      // (Quill comparison is tricky, simple length check or strict equality often suffices for simple apps)
      if (quill && quill.root.innerHTML !== value && !quill.hasFocus()) {
          quill.clipboard.dangerouslyPasteHTML(value);
      }
  });
</script>

<div class="quill-wrapper h-full flex flex-col bg-slate-950 text-slate-200">
  <div bind:this={editorDiv} class="flex-1 overflow-y-auto border-none"></div>
</div>

<style>
  /* Dark Mode Overrides for Quill */
  :global(.ql-toolbar) {
    background: #0f172a;
    border-color: #334155 !important;
    border-top: none !important;
    border-left: none !important;
    border-right: none !important;
  }
  
  :global(.ql-container) {
    border: none !important;
    font-size: 1rem;
    font-family: inherit;
  }
  
  :global(.ql-editor) {
    color: #e2e8f0;
    min-height: 100%;
  }

  /* Toolbar Icons */
  :global(.ql-stroke) {
    stroke: #e2e8f0 !important;
  }
  :global(.ql-fill) {
    fill: #e2e8f0 !important;
  }
  :global(.ql-picker) {
    color: #e2e8f0 !important;
  }
</style>

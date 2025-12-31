<script lang="ts">
  import type { Editor } from '@tiptap/core';
  
  let { editor } = $props<{ editor: Editor | null }>();
  
  function handleChange(e: Event) {
    const target = e.currentTarget as HTMLSelectElement;
    const level = target.value;
    
    if (level === 'p') {
      editor?.chain().focus().setParagraph().run();
    } else {
      editor?.chain().focus().toggleHeading({ level: parseInt(level) }).run();
    }
  }
</script>

<select 
  class="h-8 px-3 bg-muted border border-border rounded text-sm text-foreground cursor-pointer hover:bg-muted/80 transition-colors focus:outline-none focus:ring-2 focus:ring-primary"
  onchange={handleChange}
  title="Text Style"
>
  <option value="p">Normal</option>
  <option value="1">Heading 1</option>
  <option value="2">Heading 2</option>
  <option value="3">Heading 3</option>
  <option value="4">Heading 4</option>
  <option value="5">Heading 5</option>
  <option value="6">Heading 6</option>
</select>

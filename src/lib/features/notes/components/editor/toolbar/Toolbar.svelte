<script lang="ts">
  import type { Editor } from '@tiptap/core';
  import ToolbarButton from './ToolbarButton.svelte';
  import ToolbarDivider from './ToolbarDivider.svelte';
  import HeadingSelect from './HeadingSelect.svelte';
  
  import { 
    Bold, Italic, Strikethrough, Code, Underline as UnderlineIcon,
    List, ListOrdered, ListTodo, Quote, Minus,
    AlignLeft, AlignCenter, AlignRight, AlignJustify,
    Link as LinkIcon, Undo, Redo, RemoveFormatting
  } from 'lucide-svelte';

  let { editor } = $props<{ editor: Editor | null }>();
  
  function addLink() {
    const url = prompt('Enter URL:');
    if (url) {
      editor?.chain().focus().setLink({ href: url }).run();
    }
  }
</script>

<div class="flex items-center gap-1 px-4 py-2 border-b border-border bg-muted sticky top-0 z-20 overflow-x-auto">

  
  <!-- Text Formatting -->
  <ToolbarButton 
    icon={Bold} 
    title="Bold (Ctrl+B)" 
    isActive={editor?.isActive('bold')} 
    onclick={() => editor?.chain().focus().toggleBold().run()} 
  />
  
  <ToolbarButton 
    icon={Italic} 
    title="Italic (Ctrl+I)" 
    isActive={editor?.isActive('italic')} 
    onclick={() => editor?.chain().focus().toggleItalic().run()} 
  />

  <ToolbarButton 
    icon={UnderlineIcon} 
    title="Underline (Ctrl+U)" 
    isActive={editor?.isActive('underline')} 
    onclick={() => editor?.chain().focus().toggleUnderline().run()} 
  />

  <ToolbarButton 
    icon={Strikethrough} 
    title="Strikethrough" 
    isActive={editor?.isActive('strike')} 
    onclick={() => editor?.chain().focus().toggleStrike().run()} 
  />

  <ToolbarButton 
    icon={Code} 
    title="Inline Code" 
    isActive={editor?.isActive('code')} 
    onclick={() => editor?.chain().focus().toggleCode().run()} 
  />

  <ToolbarDivider />

  <!-- Headings -->
  <HeadingSelect {editor} />

  <ToolbarDivider />

  <!-- Alignment -->
  <ToolbarButton 
    icon={AlignLeft} 
    title="Align Left" 
    isActive={editor?.isActive({ textAlign: 'left' })} 
    onclick={() => editor?.chain().focus().setTextAlign('left').run()} 
  />

  <ToolbarButton 
    icon={AlignCenter} 
    title="Align Center" 
    isActive={editor?.isActive({ textAlign: 'center' })} 
    onclick={() => editor?.chain().focus().setTextAlign('center').run()} 
  />

  <ToolbarButton 
    icon={AlignRight} 
    title="Align Right" 
    isActive={editor?.isActive({ textAlign: 'right' })} 
    onclick={() => editor?.chain().focus().setTextAlign('right').run()} 
  />

  <ToolbarButton 
    icon={AlignJustify} 
    title="Justify" 
    isActive={editor?.isActive({ textAlign: 'justify' })} 
    onclick={() => editor?.chain().focus().setTextAlign('justify').run()} 
  />

  <ToolbarDivider />

  <!-- Lists -->
  <ToolbarButton 
    icon={List} 
    title="Bullet List" 
    isActive={editor?.isActive('bulletList')} 
    onclick={() => editor?.chain().focus().toggleBulletList().run()} 
  />
  
  <ToolbarButton 
    icon={ListOrdered} 
    title="Numbered List" 
    isActive={editor?.isActive('orderedList')} 
    onclick={() => editor?.chain().focus().toggleOrderedList().run()} 
  />

  <ToolbarButton 
    icon={ListTodo} 
    title="Task List" 
    isActive={editor?.isActive('taskList')} 
    onclick={() => editor?.chain().focus().toggleTaskList().run()} 
  />

  <ToolbarDivider />

  <!-- Blocks -->
  <ToolbarButton 
    icon={Quote} 
    title="Quote" 
    isActive={editor?.isActive('blockquote')} 
    onclick={() => editor?.chain().focus().toggleBlockquote().run()} 
  />

  <ToolbarButton 
    icon={Minus} 
    title="Horizontal Line" 
    onclick={() => editor?.chain().focus().setHorizontalRule().run()} 
  />

  <ToolbarButton 
    icon={LinkIcon} 
    title="Add Link" 
    isActive={editor?.isActive('link')} 
    onclick={addLink} 
  />

  <ToolbarButton 
    icon={RemoveFormatting} 
    title="Clear Formatting" 
    onclick={() => editor?.chain().focus().clearNodes().unsetAllMarks().run()} 
  />

  <div class="flex-1"></div>

  <!-- Undo/Redo -->
  <ToolbarButton 
    icon={Undo} 
    title="Undo (Ctrl+Z)" 
    onclick={() => editor?.chain().focus().undo().run()} 
  />

  <ToolbarButton 
    icon={Redo} 
    title="Redo (Ctrl+Y)" 
    onclick={() => editor?.chain().focus().redo().run()} 
  />

</div>

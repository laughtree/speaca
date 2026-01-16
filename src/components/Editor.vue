<script setup lang="ts">
import { EditorContent, useEditor } from '@tiptap/vue-3';
import StarterKit from '@tiptap/starter-kit';
import { invoke } from '@tauri-apps/api/core';

const editor = useEditor({
    extensions: [
        StarterKit,
    ],
    editorProps: {
        attributes: {

        },
    },
    content: '',
    autofocus: true,
});

async function save() {
    const content = editor.value?.getHTML();
    if (!content) return;
    await invoke('save_content', { uid: 'test', content: content });
}
</script>

<template>
    <div id="editor" class="container" v-if="editor">
        <div id="editor-header" class="container">
            <!-- <h2>Rich Text Editor</h2> -->
        </div>
        <div id="editor-toolbar" class="container">
            <button @click="editor.chain().focus().toggleBold().run()" :class="{ 'is-active': editor.isActive('bold') }">Bold</button>
            <button @click="editor.chain().focus().toggleItalic().run()" :class="{ 'is-active': editor.isActive('italic') }">Italic</button>
            <button @click="editor.chain().focus().toggleUnderline().run()" :class="{ 'is-active': editor.isActive('underline') }">Underline</button>
            <button @click="editor.chain().focus().toggleStrike().run()" :class="{ 'is-active': editor.isActive('strike') }">Strike</button>
        </div>
        <div id="editor-content" class="container">
            <EditorContent :editor="editor" />
        </div>
        <div id="editor-footer" class="container">
            <button @click="save()">Save Version</button>
            <button @click="editor.chain().focus().setContent('').run()">Reset</button>
        </div>
    </div>
</template>

<style scoped>
#editor {
    border: 1px solid #ccc;
    border-radius: 4px;
    padding: 16px;
    background-color: #cfcec9;
}

#editor-toolbar {
    flex-direction: row;
}

button {
    margin-right: 8px;
    padding: 8px 12px;
    border: none;
    border-radius: 4px;
    background-color: #e0e0e0;
    cursor: pointer;
}

button.is-active {
    background-color: #b0b0b0;
}
</style>
<script setup lang="ts">
import { EditorContent, useEditor } from '@tiptap/vue-3';
import StarterKit from '@tiptap/starter-kit';

const editor = useEditor({
    extensions: [
        StarterKit,
    ],
    editorProps: {
        attributes: {

        },
    },
    content: '<p>幹!</p>',
    autofocus: true,
});
</script>

<template>
    <div id="editor" class="container">
        <div id="editor-header" class="container">
            <h2>Rich Text Editor</h2>
        </div>
        <div id="editor-toolbar" class="container">
        
        </div>
        <div id="editor-content" class="container">
            <EditorContent :editor="editor" />
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
</style>
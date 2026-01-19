<script setup lang="ts">
import { EditorContent, useEditor } from '@tiptap/vue-3';
import StarterKit from '@tiptap/starter-kit';
import { saveContent, loadContent } from '../apis/document';

const editor = useEditor({
    extensions: [
        StarterKit,
    ],
    editorProps: {
        attributes: {
            class: 'tiptap-editor',
        },
    },
    content: '',
    autofocus: true,
});

let uid : string = '';

async function save() {
    const content = editor.value?.getJSON();
    if (!content) return;
    const contentStr = JSON.stringify(content, null, 2);
    await saveContent(uid, contentStr);
}

async function load(target_uid : string = 'test') {
    uid = target_uid;
    const contentStr = await loadContent(uid);
    console.log('Loaded content ', uid, ' : ', contentStr)
    if (!contentStr) return;
    const content = JSON.parse(contentStr);
    editor.value?.commands.setContent(content);
}

defineExpose({
    load,
    save,
});
</script>

<template>
    <div id="editor" class="container" v-if="editor">
        <div id="editor-header" class="container">
            <!-- <h2>Rich Text Editor</h2> -->
        </div>
        <div id="editor-toolbar" class="container toolbar">
            <button @click="editor.chain().focus().toggleBold().run()" :class="{ 'is-active': editor.isActive('bold') }">Bold</button>
            <button @click="editor.chain().focus().toggleItalic().run()" :class="{ 'is-active': editor.isActive('italic') }">Italic</button>
            <button @click="editor.chain().focus().toggleUnderline().run()" :class="{ 'is-active': editor.isActive('underline') }">Underline</button>
            <button @click="editor.chain().focus().toggleStrike().run()" :class="{ 'is-active': editor.isActive('strike') }">Strike</button>
        </div>
        <div id="editor-content" class="container">
            <EditorContent :editor="editor" />
        </div>
        <div id="editor-footer" class="container toolbar align-right">
            <button @click="save()">Save</button>
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

.toolbar {
    flex-direction: row;
}

.align-right {
    justify-content: flex-end;
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

:deep(.tiptap) {
    outline: none;
}

:deep(.tiptap p) {
    text-indent: 2em;
    line-height: 1.8;
    margin-top: 0.9em;
    margin-bottom: 0.9em;
}

:deep(.tiptap p.is-empty::before) {
    content: attr(data-placeholder);
    float: left;
    color: #adb5bd;
    pointer-events: none;
    height: 0;
    text-indent: 0;
}

:deep(.tiptap h1), :deep(.tiptap h2), :deep(.tiptap h3), :deep(.tiptap h4), :deep(.tiptap h5), :deep(.tiptap h6) {
    text-indent: 0;
    margin-top: 16px;
    margin-bottom: 8px;
    font-weight: bold;
}


</style>
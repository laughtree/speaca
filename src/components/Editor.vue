<script setup lang="ts">
import StarterKit from '@tiptap/starter-kit';
import { EditorContent, isActive, useEditor } from '@tiptap/vue-3';

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

defineExpose({
    getJSON: () => editor.value?.getJSON(),
    getHTML: () => editor.value?.getHTML(),
    setContent: (content: any) => editor.value?.commands.setContent(content),
    instance: editor,
    isActive: (type: string) => editor.value?.isActive(type),
});
</script>
<template>
    <EditorContent :editor="editor"/>
</template>
<style scoped>
:deep(.tiptap) {
    outline: none;
    background-color: #ececec;

    width: 100%;
    height: 100%;
    overflow-y: auto;
    overflow-x: hidden;

    padding: 20px;
    box-sizing: border-box;
}

:deep(.tiptap p) {
    text-indent: 2em;
    line-height: 1.8;
    margin-top: 0.9em;
    margin-bottom: 0.9em;
    font-size: 24px;
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
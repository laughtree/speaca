<script setup lang="ts">
import { EditorContent, useEditor } from '@tiptap/vue-3';
import StarterKit from '@tiptap/starter-kit';
import { useRouter, useRoute } from 'vue-router';
import { NButton, NSpace } from 'naive-ui';
import { ref, watch } from 'vue'

import EditingPath from '../components/EditingPath.vue';

import { editingWork, editingChapter } from '../store';
import { loadContent, saveContent } from '../apis/document';

const router = useRouter();
const route = useRoute();

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

const uid = ref<string>('');

const loadChapter = async () => {
    editingChapter.value = await loadContent(uid.value);
}

const saveChapter = async () => {
    await saveContent(uid.value, editingChapter.value);
}

watch(
    () => route.params.uid, 
    async (newUid) => {
        if(newUid) {
            uid.value = newUid as string;
            editingWork.value = await loadContent(uid.value);
            editor.value?.commands.setContent(editingWork.value.body);
        }
    },
    { immediate: true }
);

</script>
<template>
  <div>
    <div id="header" class="container">
      <editing-path />
      <n-button id="back-btn" @click="router.back()">回首頁</n-button>
    </div>
    <div id="editor" class="container" v-if="editor">
        <n-space id="editor-toolbar" class="container toolbar">
            <n-button @click="editor.chain().focus().toggleBold().run()" :class="{ 'is-active': editor.isActive('bold') }" >Bold</n-button>
            <n-button @click="editor.chain().focus().toggleItalic().run()" :class="{ 'is-active': editor.isActive('italic') }">Italic</n-button>
            <n-button @click="editor.chain().focus().toggleUnderline().run()" :class="{ 'is-active': editor.isActive('underline') }">Underline</n-button>
            <n-button @click="editor.chain().focus().toggleStrike().run()" :class="{ 'is-active': editor.isActive('strike') }">Strike</n-button>
        </n-space>
        <div id="editor-content" class="container">
            <EditorContent :editor="editor" />
        </div>
        <n-space id="editor-footer" class="container toolbar align-right">
            <n-button @click="saveChapter">Save</n-button>
        </n-space>
    </div>
  </div>
</template>
<style scoped>
#back-btn {
  margin-left: auto;
}

#editor {
    border: 1px solid #ccc;
    border-radius: 4px;
    padding: 16px;
    background-color: #cfcec9;
    
    height: 100%;
}

.toolbar {
    flex-direction: row;
    align-items: center;
}

.align-right {
    justify-content: flex-end;
}

button {
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
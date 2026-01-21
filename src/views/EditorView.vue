<script setup lang="ts">
import { EditorContent, useEditor } from '@tiptap/vue-3';
import StarterKit from '@tiptap/starter-kit';
import { useRouter, useRoute } from 'vue-router';
import { NButton, NSpace, NLayout, NLayoutSider, NLayoutContent, NLayoutHeader, NLayoutFooter } from 'naive-ui';
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
const loading = ref<boolean>(false);

const loadChapter = async () => {
    editingChapter.value = await loadContent(uid.value);
}

const saveChapter = async () => {
    editingChapter.value.body = editor.value?.getJSON();
    await saveContent(uid.value, editingChapter.value);
}

watch(
    () => route.params.uid, 
    async (newUid) => {
        if(newUid) {
          loading.value = true;
          uid.value = newUid as string;

          console.log("uid updated! : ", uid.value)

          try {
            const content = await loadContent(uid.value);
            editingChapter.value = content;
            console.log("Loaded Chapter: ", editingChapter.value);

            if(editor.value) {
              editor.value.commands.setContent(editingChapter.value.body);
            }
          } catch(e) {
              console.log("Load Chapter Failed! : ", e);
          } finally {
              loading.value = false;
          }
        }
    },
    {
      immediate: true,
    }
);

</script>
<template>
  <div id="editorview-outer">
    <div id="sider-left"></div>
    <div id="editor-area">
        <div id="header" class="container">
          <editing-path />
        </div>
        <div id="editor" class="container" v-if="editor">
          <n-space id="editor-toolbar" class="container toolbar" justify="space-between">
            <div class="toolbar">
              <n-button @click="editor.chain().focus().toggleBold().run()" :class="{ 'is-active': editor.isActive('bold') }" >Bold</n-button>
              <n-button @click="editor.chain().focus().toggleItalic().run()" :class="{ 'is-active': editor.isActive('italic') }">Italic</n-button>
              <n-button @click="editor.chain().focus().toggleUnderline().run()" :class="{ 'is-active': editor.isActive('underline') }">Underline</n-button>
              <n-button @click="editor.chain().focus().toggleStrike().run()" :class="{ 'is-active': editor.isActive('strike') }">Strike</n-button>
            </div>
            <n-button id="back-btn" @click="router.back()">回首頁</n-button>
          </n-space>
          <n-layout :has-sider=true style="height: 100%; width: 100%; flex: 1;">
            
            <n-layout-content style="overflow: hidden;">
              <EditorContent :editor="editor" />
            </n-layout-content>
          </n-layout>
          <n-space id="editor-footer" class="container toolbar">
            <n-button @click="saveChapter">Save</n-button>
          </n-space>
        </div>
    </div>
    <div id="sider-right">
      <n-layout :has-sider="true" style="height: 100%;">
        <n-layout-sider
          placement="right"
          collapse-mode="width"
          :collapsed-width="0"
          :width="240"
          show-trigger="bar"
          content-style="padding: 24px;"
        >
          <n-space>

          </n-space>
        </n-layout-sider>
        <n-layout-content>

        </n-layout-content>
      </n-layout>
    </div>
  </div>
</template>
<style scoped>
#editorview-outer {
  display: grid;
  grid-template-columns: 1fr auto 1fr;
  height: 100vh;
  width: 100vw;
  gap: 0;
}

#sider-left {
  background-color: transparent;
}

#sider-right {
  background-color: transparent;
}

#editor-area {
  display: grid;
  grid-template-rows: auto 1fr auto;
  overflow: hidden;
  min-width: 800px;
}

#back-btn {
  margin-left: auto;
}

#editor {
    border: 1px solid #ccc;
    border-radius: 4px;
    padding: 8px;
    background-color: #cfcec9;
    display: flex;
    flex-direction: column;
    flex: 1;
    overflow: hidden;
    height: 100%;
}

#editor-content {
  overflow-y: auto;
  padding: 8px;
  align-items: center;
}

.toolbar nav {
  height: 100%;
}

.toolbar {
    flex-direction: row;
    align-items: center;
    height: auto;
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
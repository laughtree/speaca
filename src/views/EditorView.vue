<script setup lang="ts">
import StarterKit from '@tiptap/starter-kit';
import { useRouter, useRoute } from 'vue-router';
import { NButton, NSpace, NLayout, NLayoutSider, NLayoutContent, NButtonGroup } from 'naive-ui';
import { ref, watch } from 'vue'

import EditingPath from '../components/EditingPath.vue';

import { editingWork, editingChapter, loadingbar } from '../store';
import { loadContent, saveContent, saveMeta } from '../apis/document';
import Editor from '../components/Editor.vue';

const router = useRouter();
const route = useRoute();

const uid = ref<string>('');
const loading = ref<boolean>(false);
const editor = ref<any>(null);

const saveChapter = async () => {
    editingChapter.value.body = editor.value?.getJSON();
    await saveContent(uid.value, editingChapter.value);
    await saveMeta(editingWork.value.title, JSON.stringify(editingWork.value));
}

watch(
    () => route.params.uid, 
    async (newUid) => {
        if(newUid) {
          loadingbar.value?.start();
          loading.value = true;
          uid.value = newUid as string;

          console.log("uid updated! : ", uid.value)

          try {
            const content = await loadContent(uid.value);
            editingChapter.value = content;
            console.log("Loaded Chapter: ", editingChapter.value);

            editor.value?.setContent(editingChapter.value?.body);
          } catch(e) {
              console.log("Load Chapter Failed! : ", e);
              loadingbar.value?.error();
          } finally {
              loading.value = false;
              loadingbar.value?.finish();
          }
        }
    },
    {
      immediate: true,
    }
);

</script>
<template>
  <n-flex id="editorview-outer">
    <n-flex 
      id="sider-left"
      style="flex: 1;"
    >
    </n-flex>
    <n-flex 
      id="editor-area"
      style="flex: 1;"
    >
        <n-flex id="header" class="container">
          <editing-path />
        </n-flex>
        <n-flex id="editor" class="container">
          <n-flex 
            id="editor-toolbar" 
            class="container toolbar" 
            justify="space-between"
            v-if="editor && editor?.instance"
          >
            <n-button-group>
              <n-button 
                @click="editor?.instance?.chain().focus().toggleBold().run()" 
                :class="{ 'is-active': editor.isActive('bold') }" 
              >
                Bold
              </n-button>
              <n-button 
                @click="editor?.instance?.chain().focus().toggleItalic().run()" 
                :class="{ 'is-active': editor.isActive('italic') }"
              >
                Italic
              </n-button>
              <n-button 
                @click="editor?.instance?.chain().focus().toggleUnderline().run()" 
                :class="{ 'is-active': editor.isActive('underline') }"
              >
                Underline
              </n-button>
              <n-button 
                @click="editor?.instance?.chain().focus().toggleStrike().run()" 
                :class="{ 'is-active': editor.isActive('strike') }"
              >
                Strike
              </n-button>
            </n-button-group>
            <n-button 
              id="back-btn" 
              @click="router.back()"
            >
              回首頁
            </n-button>
          </n-flex>
          <n-flex
            style="flex: 0.98; overflow-y: auto;"
          >
            <Editor 
              ref="editor"
              style="height: 100%;"
            />
          </n-flex>
          <n-flex id="editor-footer" class="container toolbar">
            <n-button
              @click="saveChapter"
            >
              Save
            </n-button>
          </n-flex>
        </n-flex>
      </n-flex>
    <n-flex 
      id="sider-right"
      style="flex: 1;"
    >
      <n-layout :has-sider="true" style="height: 100%;">
        <n-layout-sider
          placement="right"
          collapse-mode="width"
          :collapsed-width="0"
          :width="240"
          show-trigger="bar"
          content-style="padding: 24px;"
        >
          <n-flex>
            
          </n-flex>
        </n-layout-sider>
        <n-layout-content>

        </n-layout-content>
      </n-layout>
    </n-flex>
  </n-flex>
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
  max-height: 100vh;
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


</style>
<script setup lang="ts">
import { ref } from 'vue';
import Editor from './components/Editor.vue';
import FloatButton from './components/FloatButton.vue';
import Menu from './components/Menu.vue';
import { createNewWork, loadMeta, saveMeta } from './apis/document';

import { NBreadcrumb, NBreadcrumbItem } from 'naive-ui';

const showMenu = ref<boolean>(false);
const EditorRef = ref<InstanceType<typeof Editor> | null>(null);
const workRef = ref<Object>(null)

function toggleMenu() {
  showMenu.value = !showMenu.value;
  console.log('Toggled menu:', showMenu.value);
}

function closeMenu() {
  showMenu.value = false;
}

async function newWork() {
  const work = await createNewWork("New Work", "Unknown Author");
  console.log('Created new work: ', work);
  workRef.value = work;
  await loadEditing();
}

async function load() {
  const work = await loadMeta("New Work");
  console.log('Loaded work: ', work);
  workRef.value = work;
  await loadEditing();
}

async function loadEditing() {
  const work = workRef.value;
  let editing_chap = work.latest_edited_chapter;
  let editing_branch = work.latest_edited_branch;
  await EditorRef.value?.load(work.chapters[editing_chap][editing_branch].uid);
}
</script>

<template>
  <main class="container">
    <div id="header" class="container">
     <n-breadcrumb separator=">" v-if="workRef">
        <n-breadcrumb-item v-if="workRef">{{ workRef.title }}</n-breadcrumb-item>
        <n-breadcrumb-item v-if="workRef">{{ workRef.latest_edited_chapter }} | {{workRef.chapters[workRef.latest_edited_chapter][workRef.latest_edited_branch].title}}</n-breadcrumb-item>
      </n-breadcrumb>
    </div>
    <Editor ref="EditorRef" />
    <router-view v-slot="{ Component }">
      <transition name="fade" mode="out-in">
        <component :is="Component" />
      </transition>
    </router-view>
    <FloatButton @click="toggleMenu">
      +
    </FloatButton>
    <Menu v-if="showMenu" @close="closeMenu" @new-work="newWork" @option-2="load" @option-3="closeMenu" />
  </main>
</template>

<style>
:root {
  font-family: "Iansui", "Noto Serif TC", Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  display: flex;
  flex-direction: column;
}
</style>
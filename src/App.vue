<script setup lang="ts">
import { ref } from 'vue';
import Editor from './components/Editor.vue';
import FloatButton from './components/FloatButton.vue';
import Menu from './components/Menu.vue';

const showMenu = ref<boolean>(false);

function toggleMenu() {
  showMenu.value = !showMenu.value;
  console.log('Toggled menu:', showMenu.value);
}
</script>

<template>
  <main class="container">
    <Editor />
    <FloatButton @click="toggleMenu">
      +
    </FloatButton>
    <Menu v-if="showMenu" />
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
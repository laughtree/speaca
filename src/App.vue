<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import Editor from './components/Editor.vue';

const greetMsg = ref("");
const name = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg.value = await invoke("greet", { name: name.value });
}

window.addEventListener('keydown', (e) => {
  console.log('鍵碼:', e.key, '組字中 (isComposing):', e.isComposing);
});
window.addEventListener('compositionstart', () => console.log('✅ 觸發組字開始'));
</script>

<template>
  <main class="container">
    <h1>嗨 Tauri + Vue</h1>
    垃圾
    <input type="text" inputmode="text" style="font-family: 'Noto Sans TC' !important;" placeholder="強迫正黑體" />
    <textarea placeholder="原始 textarea" inputmode="text" style="font-family: 'Iansui' !important;"></textarea>
    
    <Editor />
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
  gap: 16px;
  padding: 16px;
}
</style>
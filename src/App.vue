<script setup lang="ts">
import { useRoute } from 'vue-router';
import { NMessageProvider } from 'naive-ui';

const route = useRoute();
</script>
<template>
  <main class="container">
    <router-view v-slot="{ Component }">
      <transition name="fade" mode="out-in">
        <n-message-provider>
          <component :is="Component"/>
        </n-message-provider>
      </transition>
    </router-view>
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

* {
  font-family: "Iansui", "Noto Serif TC", Inter, Avenir, Helvetica, Arial, sans-serif !important;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}

.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s ease;
}

.fade-enter-to,
.fade-leave-from {
  opacity: 1;
}

.container {
  display: flex;
  flex-direction: column;
  height: 100%;
  /* width: 100%; */
  overflow: hidden;
}

#header {
  align-items: center;
  flex-direction: row;
  width: 100%;
  height: auto;
}

html, body, #app {
  height: 100%;
  margin: 0;
  padding: 0;
}
</style>
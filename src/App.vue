<script setup lang="ts">

</script>
<template>
  <main class="container">
    <router-view v-slot="{ Component }">
      <transition name="fade" mode="out-in">
        <component :is="Component" />
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

.container {
  display: flex;
  flex-direction: column;
  height: 100%;
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
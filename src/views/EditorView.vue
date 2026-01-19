<script setup lang="ts">
import { useRouter } from 'vue-router';
import { NButton } from 'naive-ui';

const router = useRouter();
</script>

<template>
  <div>
    <n-button @click="router.back()">回首頁</n-button>
    <hr />
    <h2>編輯器頁面</h2>
  </div>
</template>
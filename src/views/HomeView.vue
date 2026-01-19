<script setup lang="ts">
import { useRouter } from 'vue-router';
import { NButton, NSpace } from 'naive-ui';

const router = useRouter();

// 點擊後跳轉到編輯頁面
const goEditor = () => {
  router.push('/editor');
};

// 模擬建立新作品後跳轉
const handleNewWork = () => {
  console.log('Creating new work...');
  router.push('/editor');
};
</script>

<template>
  <div style="padding: 50px;">
    <h1>我的作品集</h1>
    
    <n-space>
      <n-button type="primary" @click="handleNewWork">
        新建作品
      </n-button>
      
      <n-button @click="goEditor">
        開啟最後編輯
      </n-button>
    </n-space>
  </div>
</template>
<script setup lang="ts">
import { useRouter } from 'vue-router';
import { NButton } from 'naive-ui';
import { ref } from 'vue'

import Editor from '../components/Editor.vue';
import EditingPath from '../components/EditingPath.vue';

const router = useRouter();

const editor = ref<InstanceType<typeof Editor> | null>(null);

</script>
<template>
  <div>
    <div id="header" class="container">
      <editing-path />
      <n-button id="back-btn" @click="router.back()">回首頁</n-button>
    </div>
    <Editor ref="editor"/>
  </div>
</template>
<style scoped>
#back-btn {
  margin-left: auto;
}
editing-path {
  
}
</style>
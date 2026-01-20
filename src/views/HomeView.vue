<script setup lang="ts">
import { useRouter } from 'vue-router';
import { NButton, NSpace } from 'naive-ui';

import { createNewWork, loadMeta } from '../apis/document';
import { editingWork, currentUser } from '../store';

const router = useRouter();

const handleLoadWork = async () => {
  editingWork.value = await loadMeta("New Work");
  console.log("Loaded work: ", editingWork.value);
  router.push(`editor/${editingWork.value.chapters[editingWork.value.latest_edited_chapter][editingWork.value.latest_edited_branch].uid}`);
};

const handleNewWork = async () => {
  editingWork.value = await createNewWork("New Work", currentUser.value);
  console.log('Created new work: ', editingWork.value);
  router.push(`editor/${editingWork.value.chapters[editingWork.value.latest_edited_chapter][editingWork.value.latest_edited_branch].uid}`);
};
</script>

<template>
  <div style="padding: 50px;">
    <h1>我的作品集</h1>
    <n-space>
      <n-button type="primary" @click="handleNewWork">
        新建作品
      </n-button>
      <n-button @click="handleLoadWork">
        開啟最後編輯
      </n-button>
    </n-space>
  </div>
</template>
<script setup lang="ts">
import { useRouter } from 'vue-router';
import { NButton, NFlex } from 'naive-ui';
import { ref } from 'vue';

import { loadMeta } from '../apis/document';
import { editingWork, message } from '../store';

import WorkList from '../components/WorkList.vue';
import WorkCreationModal from '../components/WorkCreationModal.vue';

const router = useRouter();

const workCreationModalRef = ref<any | null>(null);

const handleLoadWork = async () => {
  try {
    editingWork.value = await loadMeta("New Work");
    console.log("Loaded work: ", editingWork.value);
    router.push(`editor/${editingWork.value.chapters[editingWork.value.latest_edited_chapter][editingWork.value.latest_edited_branch].uid}`);
  } catch (e) {
    message.value?.error(e as string);
  }
};
</script>

<template>
  <n-flex vertical>
    <h1>我的作品集</h1>
    <n-flex>
      <n-button type="primary" @click="workCreationModalRef?.toggleShowing">
        新建作品
      </n-button>
      <n-button @click="handleLoadWork">
        開啟最後編輯
      </n-button>
      <WorkList />
    </n-flex>
    <WorkCreationModal ref="workCreationModalRef" />
  </n-flex>
</template>
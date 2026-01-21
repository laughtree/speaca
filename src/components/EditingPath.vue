<script setup lang="ts">
import { NBreadcrumb, NBreadcrumbItem, NSpace } from 'naive-ui';

import { editingWork, editingChapter } from '../store';
import { watch, ref } from 'vue';

const workTitle = ref("");
const chapNum = ref(null);
const chapTitle = ref("");

watch (
    editingWork, (newWork) => {
        if (newWork) {
            workTitle.value = newWork.title;
        }
    },
    { 
        immediate: true ,
        deep: true,
    }
);

watch(
    editingChapter, (newChap) => {
        if(newChap) {
            workTitle.value = newChap.header.parent_work;
            chapNum.value = newChap.header.chapter_number;
            chapTitle.value = newChap.header.title;
        }
    },
    { 
        immediate: true,
        deep: true,
    }
);
</script>
<template>
    <n-space id="breadcrumb-container" class="container">
        <n-breadcrumb separator=">" v-if="editingWork || editingChapter">
            <n-breadcrumb-item>{{ workTitle }}</n-breadcrumb-item>
            <n-breadcrumb-item>{{ chapNum }} | {{chapTitle}}</n-breadcrumb-item>
        </n-breadcrumb>
    </n-space>
</template>
<style scoped>

</style>
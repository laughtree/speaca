<script setup lang="ts">
import { NBreadcrumb, NBreadcrumbItem, NSpace } from 'naive-ui';

import { editingWork } from '../store';
</script>
<template>
    <n-space id="breadcrumb-container" class="container">
        <n-breadcrumb separator=">" v-if="editingWork">
            <n-breadcrumb-item>{{ editingWork.title }}</n-breadcrumb-item>
            <n-breadcrumb-item>{{ editingWork.latest_edited_chapter }} | {{editingWork.chapters[editingWork.latest_edited_chapter][editingWork.latest_edited_branch].title}}</n-breadcrumb-item>
        </n-breadcrumb>
    </n-space>
</template>
<style scoped>

</style>
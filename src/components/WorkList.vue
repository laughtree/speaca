<script lang="ts">
import { NEmpty, NButton } from 'naive-ui';

const works = [];
</script>
<template>
    <n-empty size="huge" description="Waiting for creation...">
        <template #extra>
            <n-button size="small">
                Create New
            </n-button>
        </template>
    </n-empty>
</template>
<style scoped>

</style>
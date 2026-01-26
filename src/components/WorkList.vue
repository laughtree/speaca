<script setup lang="ts">
import { NEmpty, NVirtualList, NFlex, NButton, NSpin } from 'naive-ui';
import { ref, onMounted, computed } from 'vue';
import { useRouter } from 'vue-router';

import { get_works, loadMeta } from '../apis/document';
import { message, loadingbar, editingWork } from '../store';

const router = useRouter();

const works = ref<null | string[]>(null);
const loaded = ref<boolean>(false);

const workItems = computed(() => {
    return works.value?.map(name => ({
        key: name,
        label: name
    }));
});

const get_work_list = async () => {
    loadingbar.value?.start();
    try {
        works.value = await get_works();
        console.log("Found works: ", works);
    } catch (e) {
        message.value?.error(e as string);
        loadingbar.value?.error();
    } finally {
        loadingbar.value?.finish();
        loaded.value = true;
    }
}

const handleLoadWork = async (title: string) => {
    loadingbar.value?.start();
    try {
        editingWork.value = await loadMeta(title);
        console.log("Loaded work: ", editingWork.value);
        router.push(`editor/${editingWork.value.chapters[editingWork.value.latest_edited_chapter][editingWork.value.latest_edited_branch].uid}`);
    } catch (e) {
        message.value?.error(e as string);
        loadingbar.value?.error();
    } finally {
        loadingbar.value?.finish();
    }
}

onMounted(() => {
    get_work_list();
});

</script>
<template>
    <n-flex>
        <n-spin
            v-if="!loaded"
            size="large"
        >

        </n-spin>
        <n-empty 
            v-else-if="works?.length === 0"
            description="Waiting for creating..."
        >

        </n-empty>
        <n-virtual-list
            style="max-height: 600px; padding-right: 12px; width: 100%;"
            :item-size="60"
            :items="workItems"
            v-else
        >
            <template #default="{ item }">
                <n-flex :key="item.key" class="work-item-card">
                    <n-button
                        tertiaty
                        block
                        @click="handleLoadWork(item.label)"
                    >
                        <n-flex>
                            <span class="work-title">{{item.label}}</span>
                        </n-flex>
                    </n-button>
                </n-flex>
            </template>
        </n-virtual-list>
    </n-flex>
</template>
<style scoped>

</style>
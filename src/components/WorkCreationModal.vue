<script setup lang="ts">
import { NModal, NCard, NInput, NDynamicTags, NFlex, NButton } from 'naive-ui';
import { ref } from 'vue';
import { useRouter } from 'vue-router';

import { editingWork, currentUser, message } from '../store';
import { createNewWork, saveMeta } from '../apis/document';

const router = useRouter();

const show = ref<boolean>(false);
const title = ref<string>("");
const tags = ref<string[]>([]);

const toggleShowing = () => {
    show.value = !show.value;
}

const handleNewWork = async () => {
  try {
    editingWork.value = await createNewWork(title.value, currentUser.value);

    if(tags.value.length > 0) {
        editingWork.value.tags = tags.value;
        try {
            await saveMeta(editingWork.value.title, JSON.stringify(editingWork.value));
        } catch (e) {
            message.value?.error(e as string);
        }
        
    }

    console.log('Created new work: ', editingWork.value);
    router.push(`editor/${editingWork.value.chapters[editingWork.value.latest_edited_chapter][editingWork.value.latest_edited_branch].uid}`);
  } catch (e) { 
    message.value?.error(e as string);
  }
};

defineExpose({ toggleShowing });

</script>
<template>
    <n-modal v-model:show="show">
        <n-card
            style="width: 600px"
            title="開始新作品"
            :bordered="false"
            size="huge"
            role="dialog"
            aria-modal="true"
        >
            <!-- <template #header-extra>
            header
            </template> -->

            <n-flex vertical>
                <n-input
                    v-model:value="title"
                    type="text"
                    placeholder="Title"
                    size="large"
                />
            </n-flex>

            <template #footer>
                <n-flex vertical>
                    <span>
                        Tags
                    </span>
                    <n-dynamic-tags v-model:value="tags"/>
                    <n-flex justify="end">
                        <span>
                            You can setup these later
                        </span>
                    </n-flex>
                </n-flex>
            </template>

            <template #action>
                <n-flex justify="end">
                    <n-button
                        type="primary"
                        @click="handleNewWork"
                    >
                        Create
                    </n-button>
                </n-flex>
            </template>
        </n-card>
    </n-modal>
</template>
<style scoped>
    
</style>
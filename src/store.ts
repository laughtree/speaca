import { ref } from 'vue';

const editingWork = ref<any>(null);
const editingChapter = ref<any>(null);
const currentUser = ref<string>("anonymous");

export { editingWork, editingChapter, currentUser };
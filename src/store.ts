import { DialogApi, LoadingBarApi, MessageApi, ModalApi, NotificationApi } from 'naive-ui';
import { ref } from 'vue';

const editingWork = ref<any>(null);
const editingChapter = ref<any>(null);
const currentUser = ref<string>("anonymous");

export { editingWork, editingChapter, currentUser }; // global valuables

const message = ref<MessageApi | null>(null);
const notification = ref<NotificationApi | null>(null);
const dialog = ref<DialogApi | null>(null);
const modal = ref<ModalApi | null>(null);
const loadingbar = ref<LoadingBarApi | null>(null);

export { message, notification, dialog, modal, loadingbar }; // frontend api


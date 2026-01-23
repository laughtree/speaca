<script setup lang="ts">
import { useLoadingBar, useDialog, useMessage, useModal, useNotification, useDialogReactiveList, useModalReactiveList, useThemeVars, useOsTheme } from 'naive-ui';

import { message, notification, dialog, modal, loadingbar } from '../store';

message.value = useMessage();
notification.value = useNotification();
dialog.value = useDialog();
modal.value = useModal();
loadingbar.value = useLoadingBar();

window.$dialogList = useDialogReactiveList();
window.$modalList = useModalReactiveList();
window.$themeVars = useThemeVars();
window.$osTheme = useOsTheme();

</script>
<template>

</template>
<style scoped>

</style>
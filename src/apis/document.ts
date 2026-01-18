import { invoke } from '@tauri-apps/api/core';

async function loadContent(uid: string): Promise<string> {
  return await invoke('load_content', { uid });
}

async function saveContent(uid: string, content: string): Promise<string> {
  return await invoke('save_content', { uid, content });
}

export { loadContent, saveContent };
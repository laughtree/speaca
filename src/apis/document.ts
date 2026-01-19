import { invoke } from '@tauri-apps/api/core';

async function loadContent(uid: string): Promise<object> {
  return await invoke('load_content', { uid });
}

async function saveContent(uid: string, content: object): Promise<string> {
  return await invoke('save_content', { uid, content });
}

async function loadMeta(name: string): Promise<string> {
  return await invoke('load_meta', { name });
}

async function saveMeta(name: string, content: string): Promise<string> {
  return await invoke('save_meta', { name, content });
}

async function createNewWork(name: string, author: string): Promise<string> {
  return await invoke('create_new_work', { name, author });
}

export { loadContent, saveContent, loadMeta, saveMeta, createNewWork };
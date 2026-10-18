import { defineStore } from 'pinia';

export const useClipboardStore = defineStore('clipboard', {
  state: () => ({ additions: [] as string[] }),
});

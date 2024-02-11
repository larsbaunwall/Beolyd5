import { defineStore } from "pinia";
import { ref } from "vue";

import { invoke } from "@tauri-apps/api";

export const useUIStore = defineStore('ui', () => {
  const volume = ref(50);
  const wheelPointerAngle = ref(180);

  const tick = () => {
    invoke('click');
  }

  return {volume, wheelPointerAngle, tick}
})
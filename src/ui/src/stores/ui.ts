import { defineStore } from "pinia";
import { ref } from "vue";

export const useUIStore = defineStore('ui', () => {
  const volume = ref(0);
  const wheelPointerAngle = ref(180);

  return {volume, wheelPointerAngle}
})
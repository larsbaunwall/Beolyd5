import { defineStore } from "pinia";
import { ref } from "vue";

//import { invoke } from "@tauri-apps/api";

export const useUIStore = defineStore('ui', () => {
  const volume = ref(50);
  const wheelPointerAngle = ref(180);
  const topWheelPosition = ref(0);
  const isNowPLayingOverlayActive = ref(false);

  const tick = () => {
    //invoke('tick');
  }

  return {volume, wheelPointerAngle, topWheelPosition, isNowPLayingOverlayActive, tick}
})
import { defineStore } from "pinia";
import { ref } from "vue";
import {Subject} from "rxjs";
import {HardwareEvent} from "../hardware/events.ts";

//import { invoke } from "@tauri-apps/api";

export const useUIStore = defineStore('ui', () => {
  const volume = ref(50);
  const wheelPointerAngle = ref(180);
  const topWheelPosition = ref(0);
  const isNowPLayingOverlayActive = ref(false);
  const hardwareEvents = new Subject<HardwareEvent>();

  const tick = () => {
    //invoke('tick');
  }

  const nextHardwareEvent = (event: HardwareEvent) => {
    hardwareEvents.next(event);
  }

  return {hardwareEvents, volume, wheelPointerAngle, topWheelPosition, isNowPLayingOverlayActive, tick, nextHardwareEvent}
})
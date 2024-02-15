<script setup lang="ts">
import arcs from "../utils/arcs.ts";
import {useSlots, watch, onMounted, onUnmounted, reactive} from "vue";
import { useUIStore } from "../stores/ui.ts";

const props = withDefaults(defineProps<{
      radius: number
      angleStep?: number
      itemHeight?: number
      itemWidth?: number
    }>(), {
      angleStep: 10,
      itemHeight: 50,
      itemWidth: 200,
    });
const state = reactive({ scrollPosition: 0, isScrolling: false });

const slots = useSlots();

function startItemAngle(): number {
  const totalSpan = props.angleStep * (3 - 1);
  return 180 - totalSpan / 2;
}

function itemStyle(index: number) {
  const itemAngle = startItemAngle() + index * props.angleStep - state.scrollPosition;
  const position = arcs.getArcPoint(props.radius, 20, itemAngle);
  return {
    position: 'absolute',
    left: `${position.x - props.itemWidth}px`,
    top: `${position.y - props.itemHeight / 2}px`,
    width: `${props.itemWidth}px`,
    height: `${props.itemHeight}px`,
  };
}

function isSelectedItem(index: number) {
  const itemAngle = startItemAngle() + index * props.angleStep - state.scrollPosition;
  return Math.abs(itemAngle - 180) < 1;
}

const uiStore = useUIStore();

watch(() => uiStore.topWheelPosition, (newVal) => {
  if(newVal == 0)
    return;

  const totalSpan = props.angleStep * (1);
  if (newVal > 0) {
    if (state.scrollPosition <= totalSpan - 1) {
      state.isScrolling = true;
      state.scrollPosition += props.angleStep;
      setTimeout(() => state.isScrolling = false, 200);
    }
  } else if (newVal < 0) {
    if (state.scrollPosition >= 0) {
      state.isScrolling = true;
      state.scrollPosition -= props.angleStep;
      setTimeout(() => state.isScrolling = false, 200);
    }
  }

  uiStore.topWheelPosition = 0;
});

const handleKeyDown = (event) => {
  switch (event.key) {
    case "ArrowDown":
      uiStore.topWheelPosition = 1;
      break;
    case "ArrowUp":
      uiStore.topWheelPosition = -1;
      break;
  }
};

onMounted(() => {
  window.addEventListener('keydown', handleKeyDown);
});

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeyDown);
});
</script>

<template>
  <div>
    <svg style="height:0; width:0; position:absolute;">
      <filter id="motionBlur">
        <feGaussianBlur in="SourceGraphic" stdDeviation="0 10" />
      </filter>
    </svg>
    <div v-for="(slot, index) in slots" :key="index" class="list-item" :style="itemStyle(index)"
         :class="{ selectedItem: isSelectedItem(index), scrolling: state.isScrolling }">
      <slot :name="`${index}`"></slot>
    </div>
  </div>
</template>

<style scoped>
.list-item {
  z-index: 1000;
  font-weight: 100;
  font-size: 14px;
  color: white;
  display: flex;
  justify-content: right;
  align-items: center;
  transition: filter 200ms ease;
}

.list-item.selectedItem {
  font-weight: bold;
}
.list-item.scrolling:not(.selectedItem) {
  filter: url(#motionBlur);
}

/* Add transition and transformation effects */
.list-item {
  transition: all 200ms ease;
  transform-origin: center;
}

/* Style for the SVG filter */
svg {
  position: absolute;
  width: 0;
  height: 0;
}
</style>
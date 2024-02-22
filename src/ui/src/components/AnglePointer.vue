<script setup lang="ts">
import { useUIStore } from '../stores/ui.ts';
import arcs from "../utils/arcs.ts";

const uiStore = useUIStore();

const props = withDefaults(defineProps<{
  radius: number
}>(), {
  radius: 1000,
});
</script>

<template>
  <svg id="anglePointer" width="1024" height="768" style="position: absolute; z-index: 100;">
    <defs>
      <radialGradient id="lineGradient">
        <stop offset="0%" stop-color="rgba(102,153,255,0.45)" />
        <stop offset="90%" stop-color="rgba(0,0,0,0)" />
      </radialGradient>
      <radialGradient id="dotGradient">
        <stop offset="0%" stop-color="rgba(128,204,255,0.9)" />
        <stop offset="40%" stop-color="rgba(128,204,255,0.4)" />
        <stop offset="100%" stop-color="rgba(128,204,255,0)" />
      </radialGradient>
      <filter id="exposureFilter" x="0" y="0" width="100%" height="100%">
        <feComponentTransfer>
          <feFuncR type="linear" slope="2" />
          <feFuncG type="linear" slope="2" />
          <feFuncB type="linear" slope="2" />
        </feComponentTransfer>
      </filter>
      <filter id="blurPointerFilter">
        <feGaussianBlur in="SourceGraphic" stdDeviation="10" />
      </filter>
      <filter id="blurPointFilter">
        <feGaussianBlur in="SourceGraphic" stdDeviation="5" />
      </filter>
    </defs>
    <ellipse id="pointerDot"
             :style="{
               '--cx': `${arcs.getArcPoint(radius, 0, uiStore.wheelPointerAngle).x}px`,
               '--cy': `${arcs.getArcPoint(radius, 0, uiStore.wheelPointerAngle).y}px`,
               'transform': `rotate(${uiStore.wheelPointerAngle - 90}deg)`
             }"
             :rx="45"
             :ry="25"
             fill="url(#dotGradient)"
             filter="url(#blurPointFilter)"
    />
    <ellipse id="pointerLine"
             :style="{
               '--cx': `${arcs.getArcPoint(radius, 0, uiStore.wheelPointerAngle).x}px`,
               '--cy': `${arcs.getArcPoint(radius, 0, uiStore.wheelPointerAngle).y}px`,
               'transform': `rotate(${uiStore.wheelPointerAngle - 90}deg)`
             }"
             :rx="40"
             :ry="400"
             fill="url(#lineGradient)"
             filter="url(#exposureFilter) url(#blurPointerFilter)"
    />
  </svg>
</template>

<style scoped>

#pointerDot, #pointerLine {
  transition: all 100ms ease;
  transform-origin: var(--cx) var(--cy);
  cx: var(--cx);
  cy: var(--cy);
}

</style>
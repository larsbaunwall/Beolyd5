<script setup lang="ts">
import {computed} from 'vue';
import arcs from '../utils/arcs';
import { useUIStore } from '../stores/ui';

const uiStore = useUIStore();

const radius = 270;
const startArcAngle = 95;
const endArcAngle = 265;

const translateVolume = computed(() => ((uiStore.volume - 0) * (endArcAngle - startArcAngle)) / (100 - 0) + startArcAngle);
</script>

<template>
  <svg width="1024" height="768" style="position: absolute; z-index: 90; opacity: 0.5;">
    <defs>
      <linearGradient id="gradient">
        <stop offset="5%" stop-color="rgba(102,153,255,0.3)" />
        <stop offset="95%" stop-color="rgba(102,153,204,0.3)" />
      </linearGradient>
    </defs>
    <!-- Arc path for reference -->
    <path :d="arcs.describeArc(arcs.cx, arcs.cy, radius, startArcAngle, translateVolume)" fill="none" stroke="url(#gradient)"
          stroke-width="10" stroke-linecap="round" />
  </svg>
</template>

<style scoped>
</style>
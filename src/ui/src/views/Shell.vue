<script setup lang="ts">
import { defineProps } from 'vue';
import { onMounted, onUnmounted, computed } from 'vue';
import BS5Shell from "./FullscreenContainer.vue";

const props = defineProps({
  component: {
    type: Object,
    default: () => BS5Shell,
  },
  shell: {
    type: String,
    default: 'default',
  },
});

const isDefaultShell = computed(() => props.shell === 'default');

onMounted(() => {
  document.documentElement.style.overflow = isDefaultShell.value ? 'hidden' : 'auto';
});

onUnmounted(() => {
  document.documentElement.style.overflow = 'auto';
});
</script>

<template>
  <component :is="component"></component>
</template>

<style scoped>

</style>
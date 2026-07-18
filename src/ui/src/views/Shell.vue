<script setup lang="ts">
import { computed, watch } from 'vue';
import BS5Shell from "./FullscreenContainer.vue";
import type { Component } from 'vue';

interface Props {
  component?: Component;
  shell?: string;
}

const props = withDefaults(defineProps<Props>(), {
  component: () => BS5Shell as Component,
  shell: 'default',
});

const isDefaultShell = computed(() => props.shell === 'default');

// Update overflow whenever the shell type changes (component is reused by Vue Router).
watch(isDefaultShell, (isDefault) => {
  document.documentElement.style.overflow = isDefault ? 'hidden' : 'auto';
}, { immediate: true });
</script>

<template>
  <component :is="component"></component>
</template>

<style scoped>

</style>
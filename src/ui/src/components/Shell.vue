<script setup lang="ts">
import { defineProps } from 'vue';
import Bs5Shell from "./bs5-shell.vue";

defineProps({
  component: {
    type: Object,
    default: () => Bs5Shell,
  },
});
</script>

<template>
  <component :is="component"></component>
</template>

<style scoped>

</style>
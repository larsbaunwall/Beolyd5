<script setup lang="ts">
import {computed, watch} from "vue";
import {useUIStore} from "../stores/ui.ts";
import {useRoute, useRouter} from "vue-router";

const uiStore = useUIStore();
const router = useRouter();
const route = useRoute();

// Derived reactively so it stays correct if shell param ever changes on this component instance.
const shellPrefix = computed(() => route.params.shell === 'sim' ? '/sim' : '');

watch(() => uiStore.wheelPointerAngle, () => {
  if (uiStore.wheelPointerAngle > 203 && !uiStore.isNowPLayingOverlayActive) {
    router.push(`${shellPrefix.value}/playing`);
    uiStore.isNowPLayingOverlayActive = true;
  } else if (uiStore.wheelPointerAngle < 155 && !uiStore.isNowPLayingOverlayActive) {
    router.push(`${shellPrefix.value}/playing`);
    uiStore.isNowPLayingOverlayActive = true;
  } else if(uiStore.wheelPointerAngle > 155 && uiStore.wheelPointerAngle < 203 && uiStore.isNowPLayingOverlayActive) {
    router.push(`${shellPrefix.value}/menu`);
    uiStore.isNowPLayingOverlayActive = false;
  }
});
</script>

<template>
  <div id="viewport">
  <router-view v-slot="{ Component }">
    <transition name="fade" mode="out-in">
      <component :is="Component"/>
    </transition>
  </router-view>
  </div>
</template>

<style scoped>

#viewport {
  background-color: black;
}

.fade-enter-active,
.fade-leave-active {
  transition: opacity 200ms;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
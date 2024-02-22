<script setup lang="ts">
import {ref, watch} from "vue";
import {useUIStore} from "../stores/ui.ts";
import {useRoute, useRouter} from "vue-router";

const uiStore = useUIStore();
const router = useRouter();
const route = useRoute();

watch(() => uiStore.wheelPointerAngle, () => {
  if (uiStore.wheelPointerAngle > 203 && !uiStore.isNowPLayingOverlayActive) {
    router.push(`/${route.params.shell}/playing`);
    uiStore.isNowPLayingOverlayActive = true;
  } else if (uiStore.wheelPointerAngle < 155 && !uiStore.isNowPLayingOverlayActive) {
    router.push(`/${route.params.shell}/playing`);
    uiStore.isNowPLayingOverlayActive = true;
  } else if(uiStore.wheelPointerAngle > 155 && uiStore.wheelPointerAngle < 203 && uiStore.isNowPLayingOverlayActive) {
    router.push(`/${route.params.shell}/menu`);
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
  background-color: red;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
<template>
  <div class="container">
    <MainCircleArc :radius="radius"/>
    <ArcContentFlow :radius=300>
      <template v-slot:0>
        <div>Test 1</div>
      </template>
      <template v-slot:1>
        <div>Test 2</div>
      </template>
      <template v-slot:2>
        <div>Test 3</div>
      </template>
    </ArcContentFlow>
    <VolumeArc/>
    <AnglePointer :radius="radius"/>
    <div style="position: absolute; top: 20px; left: 180px; z-index: 1; width: 820px; height: 700px;">
      <router-view v-slot="{ Component }">
        <transition name="slide-up" mode="out-in">
          <component :is="Component"/>
        </transition>
      </router-view>
    </div>

    <div v-for="(item, index) in menuItems" :key="index" class="list-item" :style="menuItemStyle(index)"
         :class="{ selectedItem: isSelectedItem(index) }">
      {{ item.title }}
    </div>
  </div>
</template>

<script setup lang="ts">
import {computed, CSSProperties, ref} from 'vue';
import {useRoute, useRouter} from 'vue-router';
import {useUIStore} from '../stores/ui.ts';
import arcs from '../utils/arcs.ts';
import MainCircleArc from '../components/MainArc.vue';
import VolumeArc from '../components/VolumeArc.vue';
import ArcContentFlow from '../components/ArcContentFlow.vue';
import AnglePointer from "../components/AnglePointer.vue";

const router = useRouter();
const uiStore = useUIStore();

const route = useRoute();

const menuItems = ref([
  {title: 'SETTINGS', path: ''},
  {title: 'SOURCES', path: ''},
  {title: 'N.RADIO', path: 'radio'},
  {title: 'N.MUSIC', path: 'music'}
]);
const radius = ref(1000);
const angleStep = ref(7);

const startItemAngle = computed(() => {
  const totalSpan = angleStep.value * (menuItems.value.length - 1);
  return 180 - totalSpan / 2;
});

function menuItemStyle(index: number): CSSProperties {
  const itemAngle = startItemAngle.value + index * angleStep.value;
  const position = arcs.getArcPoint(radius.value, 20, itemAngle);
  const divWidth = 100;
  const divHeight = 50;
  return {
    position: 'absolute',
    left: `${position.x - divWidth}px`,
    top: `${position.y - divHeight / 2}px`,
    width: `${divWidth}px`,
    height: `${divHeight}px`,
  };
}

const selectedMenuItem = ref(-1);

function isSelectedItem(index: number) {
  const itemAngle = startItemAngle.value + index * angleStep.value;
  const diff = Math.abs(uiStore.wheelPointerAngle - itemAngle);
  if (diff <= 2) {
    if (selectedMenuItem.value !== index) {
      uiStore.tick();
      selectedMenuItem.value = index;
      router.push({path: `/${route.params.shell ?? 'default'}/${menuItems.value[index].path}`});
    }
    return true;
  }
  return false;
}

</script>

<style scoped>
.container {
  background-color: black;
  width: 100%;
  height: 1000px;
  cursor: none;
  overflow: hidden;
}

.list-item {
  z-index: 1000;
  font-weight: 100;
  font-size: 14px;
  color: white;
  display: flex;
  justify-content: right;
  align-items: center;
  transition: font-weight 0.5s ease-in-out;
}

.list-item.selectedItem {
  font-weight: bold;
  transition: font-weight 0.5s ease-in-out;
}

.slide-up-enter-active,
.slide-up-leave-active {
  transition: all 0.25s ease-out;
}

.slide-up-enter-from {
  opacity: 0;
  transform: translateY(30px);
}

.slide-up-leave-to {
  opacity: 0;
  transform: translateY(-30px);
}

</style>
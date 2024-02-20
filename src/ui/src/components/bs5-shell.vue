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
  <VolumeArc />
  <div style="position: absolute; top: 20px; left: 180px; z-index: 1; width: 820px; height: 700px;">
    <router-view v-slot="{ Component }">
      <transition name="slide-up" mode="out-in">
        <component :is="Component" />
      </transition>
    </router-view>
  </div>
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
               '--cx': `${getArcPoint(radius, 0, uiStore.wheelPointerAngle).x}px`,
               '--cy': `${getArcPoint(radius, 0, uiStore.wheelPointerAngle).y}px`,
               'transform': `rotate(${uiStore.wheelPointerAngle - 90}deg)`
             }"
             :rx="45"
      :ry="25"
      fill="url(#dotGradient)"
      filter="url(#blurPointFilter)"
    />
    <ellipse id="pointerLine"
             :style="{
               '--cx': `${getArcPoint(radius, 0, uiStore.wheelPointerAngle).x}px`,
               '--cy': `${getArcPoint(radius, 0, uiStore.wheelPointerAngle).y}px`,
               'transform': `rotate(${uiStore.wheelPointerAngle - 90}deg)`
             }"
             :rx="40"
      :ry="400"
      fill="url(#lineGradient)"
      filter="url(#exposureFilter) url(#blurPointerFilter)"
    />
  </svg>
  <div v-for="(item, index) in menuItems" :key="index" class="list-item" :style="menuItemStyle(index)" :class="{ selectedItem: isSelectedItem(index) }">
    {{ item.title }}
  </div>
    </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { useRouter } from 'vue-router';
import { useUIStore } from '../stores/ui';
import arcs from '../utils/arcs';
import BS5DebugOverlay from './debug-overlay.vue';
import MainCircleArc from './main-circle-arc.vue';
import VolumeArc from './volume-arc.vue';
import ArcContentFlow from './ArcContentFlow.vue';

export default defineComponent({
  name: 'BS5Shell',
  components: {
    MainCircleArc, VolumeArc, ArcContentFlow
  },
  setup() {
    const router = useRouter();
    const uiStore = useUIStore();

    return { router, uiStore };
  },
  data() {
    return {
      menuItems: [{title: 'SETTINGS', path: '/'}, {title: 'SOURCES', path: '/'}, {title: 'N.RADIO', path: '/radio'}, {title: 'N.MUSIC', path: '/music'} ],
      radius: 1000, // Adjusted radius to fit within the viewport
      angleStep: 7, // Adjust this value to change the spacing between menu items
    };
  },
  computed: {
    startItemAngle(): number {
      const totalSpan = this.angleStep * (this.menuItems.length - 1);
      return 180 - totalSpan / 2;
    },
  },
  methods: {
    menuItemStyle(index: number) {
      const itemAngle = this.startItemAngle + index * this.angleStep;
      const position = this.getArcPoint(this.radius, 20, itemAngle);
      const divWidth = 100;
      const divHeight = 50;
      return {
        position: 'absolute',
        left: `${position.x - divWidth}px`,
        top: `${position.y - divHeight / 2}px`,
        width: `${divWidth}px`,
        height: `${divHeight}px`,
      };
    },
    isSelectedItem(index: number) {
      const itemAngle = this.startItemAngle + index * this.angleStep;
      const diff = Math.abs(this.uiStore.wheelPointerAngle - itemAngle);
      if (diff <= 1) {
        this.uiStore.tick();
        this.router.push(this.menuItems[index].path);
        return true;
      }
      return false;
    },
    polarToCartesian(radius: number, angleInDegrees: number) {
      const angleInRadians = (angleInDegrees * Math.PI) / 180.0;
      return {
        x: arcs.cx + radius * Math.cos(angleInRadians),
        y: arcs.cy + radius * Math.sin(angleInRadians),
      };
    },
    getArcPoint(radius: number, radiusPadding: number, angle: number) {
      return this.polarToCartesian(radius + radiusPadding, angle);
    },
  },
});
</script>

<style scoped>
.container {
  background-color: black;
  width:100%;
  height: 1000px;
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

/* Add transition and transformation effects */
#pointerDot, #pointerLine {
  transition: all 300ms ease;
  transform-origin: var(--cx) var(--cy);
  cx: var(--cx);
  cy: var(--cy);
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
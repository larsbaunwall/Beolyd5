<template>  
<input type="number" v-model="uiStore.wheelPointerAngle" />
  <div style="z-index: 200; position: fixed; top: 50%; right: 10px; opacity: 0.2; transform: translateY(-50%) rotate(270deg); ">
    <input type="range" min="155" max="207" step="0.001" v-model="uiStore.wheelPointerAngle" />
  </div>
  <div style="position: absolute; top: 20px; left: 180px; z-index: 1; width: 820px; height: 700px;">
    <router-view v-slot="{ Component }">
      <transition name="slide-up" mode="out-in">
        <component :is="Component" />
      </transition>
    </router-view>
  </div>
  <svg width="1024" height="768" style="position: relative; z-index: 100;">
    <defs>
    <linearGradient id="gradient" gradientTransform="rotate(90)">
      <stop offset="0%" stop-color="rgba(102,153,255,0)" />
      <stop offset="5%" stop-color="rgba(102,153,255,1)" />
      <stop offset="95%" stop-color="rgba(0,255,204,1)" />
      <stop offset="100%" stop-color="rgba(0,255,204,0)" />
    </linearGradient>
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
  </defs>
    <!-- Arc path for reference -->

    <ellipse
      :cx="getArcPoint(cx, cy, radius, 0, uiStore.wheelPointerAngle).x"
      :cy="getArcPoint(cx, cy, radius, 0,  uiStore.wheelPointerAngle).y"
      :rx="35"
      :ry="25"
      fill="url(#dotGradient)"
      :transform="`rotate(${uiStore.wheelPointerAngle - 90}, ${getArcPoint(cx, cy, radius, 0,  uiStore.wheelPointerAngle).x}, ${getArcPoint(cx, cy, radius, 0,  uiStore.wheelPointerAngle).y})`"
    />
    <ellipse
      :cx="getArcPoint(cx, cy, radius, 0, uiStore.wheelPointerAngle).x"
      :cy="getArcPoint(cx, cy, radius, 0,  uiStore.wheelPointerAngle).y"
      :rx="40"
      :ry="400"
      fill="url(#lineGradient)"
      filter="url(#exposureFilter)"
      :transform="`rotate(${uiStore.wheelPointerAngle - 90}, ${getArcPoint(cx, cy, radius, 0,  uiStore.wheelPointerAngle).x}, ${getArcPoint(cx, cy, radius, 0,  uiStore.wheelPointerAngle).y})`"
    />
    <path
      :d="describeArc(cx, cy, radius, startArcAngle, endArcAngle)"
      fill="none"
      stroke="url(#gradient)" 
      stroke-width="3"
    />
    <!-- Menu items positioned along the arc -->
    <g v-for="(item, index) in menuItems" :key="index">
      <text
        :x="getArcPoint(cx, cy, radius, 20,  startItemAngle + index * (angleStep)).x"
        :y="getArcPoint(cx, cy, radius, 20,  startItemAngle + index * (angleStep)).y"
        dominant-baseline="middle"
        text-anchor="end"
        font-size="14"
        :font-weight="boldness(index)"
        fill="white" 
        class="menu-item"
      >
        {{ item.title }}
      </text>
    </g>
  </svg>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { useRouter } from 'vue-router'
import { useUIStore } from '../stores/ui';

export default defineComponent({
  name: 'BS5Shell',
  setup() {
    const router = useRouter();
    const uiStore = useUIStore();

    return { router, uiStore };
  },
  data() {
    return {
      menuItems: [{title: 'SETTINGS', path: '/'}, {title: 'SOURCES', path: '/'}, {title: 'N.RADIO', path: '/radio'}, {title: 'N.MUSIC', path: '/music'} ],
      cx: 1300, // Center x coordinate
      cy: 384, // Center y coordinate
      radius: 1150, // Adjusted radius to fit within the viewport
      startArcAngle: 162, // Starting angle for the first menu item
      endArcAngle: 199, // Ending angle for the last menu item
      startItemAngle: 172, // Angle for the first menu item
    };
  },
  computed: {
    angleStep(): number {
      return (this.endArcAngle - this.startArcAngle - 20) / (this.menuItems.length - 1);
    },
    boldness() {
      return (index: number) => {
        const itemAngle = this.startItemAngle + index * this.angleStep;
        const diff = Math.abs(this.uiStore.wheelPointerAngle - itemAngle);

        if (diff < 0.5) {
          return 800;
        } else if (diff < 1) {
          this.uiStore.tick();
          this.router.push(this.menuItems[index].path);
          return 400;
        } else if (diff < 4) {
          return 200;
        } 
          
        return 100;
      };
    },
  },
  methods: {
    polarToCartesian(centerX: number, centerY: number, radius: number, angleInDegrees: number) {
      const angleInRadians = (angleInDegrees * Math.PI) / 180.0;
      return {
        x: centerX + radius * Math.cos(angleInRadians),
        y: centerY + radius * Math.sin(angleInRadians),
      };
    },
    describeArc(x: number, y: number, radius: number, startAngle: number, endAngle: number) {
      const start = this.polarToCartesian(x, y, radius, endAngle);
      const end = this.polarToCartesian(x, y, radius, startAngle);
      const largeArcFlag = endAngle - startAngle <= 180 ? '0' : '1';
      const d = [
        'M', start.x, start.y,
        'A', radius, radius, 0, largeArcFlag, 0, end.x, end.y
      ].join(' ');
      return d;
    },
    getArcPoint(centerX: number, centerY: number, radius: number, radiusPadding: number, angle: number) {
      return this.polarToCartesian(centerX, centerY, radius + radiusPadding, angle);
    },
  },
});
</script>

<style scoped>
.menu-item {
  transition: transform 0.3s ease;
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
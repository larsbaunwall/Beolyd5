<template>  
<div style="position: fixed; top: 50%; right: 10px; opacity: 0.2; transform: translateY(-50%) rotate(270deg); ">
  <input type="range" min="155" max="207" step="0.001" v-model="lineAngle" />
</div>
  <svg width="1024" height="768">
    <defs>
    <linearGradient id="gradient" gradientTransform="rotate(90)">
      <stop offset="0%" stop-color="black" />
      <stop offset="5%" stop-color="red" />
      <stop offset="95%" stop-color="blue" />
      <stop offset="100%" stop-color="black" />
    </linearGradient>
    <radialGradient id="lineGradient"> 
      <stop offset="0%" stop-color="rgba(255,255,255,0.3)" />
      <stop offset="90%" stop-color="rgba(0,0,0,0)" />
    </radialGradient>
  </defs>
    <!-- Arc path for reference -->
    <path
      :d="describeArc(cx, cy, radius, startArcAngle, endArcAngle)"
      fill="none"
      stroke="url(#gradient)" 
      stroke-width="2"
    />
    <ellipse
      :cx="getArcPoint(cx, cy, radius, 0, lineAngle).x"
      :cy="getArcPoint(cx, cy, radius, 0,  lineAngle).y"
      :rx="50"
      :ry="350"
      fill="url(#lineGradient)"
      :transform="`rotate(${lineAngle - 90}, ${getArcPoint(cx, cy, radius, 0,  lineAngle).x}, ${getArcPoint(cx, cy, radius, 0,  lineAngle).y})`"
    />
    <!-- Menu items positioned along the arc -->
    <g v-for="(item, index) in menuItems" :key="index">
      <text
        :x="getArcPoint(cx, cy, radius, 40,  startItemAngle + index * (angleStep)).x"
        :y="getArcPoint(cx, cy, radius, 40,  startItemAngle + index * (angleStep)).y"
        dominant-baseline="middle"
        text-anchor="end"
        font-size="14"
        :font-weight="boldness(index)"
        fill="white" 
        class="menu-item"
      >
        {{ item }}
      </text>
    </g>
  </svg>
</template>

<style scoped>
.menu-item {
  transition: transform 0.3s ease;
}
</style>
<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({
  name: 'ArcMenu',
  data() {
    return {
      menuItems: ['SETTINGS', 'SOURCES', 'N.RADIO', 'N.MUSIC', ],
      cx: 1300, // Center x coordinate
      cy: 384, // Center y coordinate
      radius: 1100, // Adjusted radius to fit within the viewport
      startArcAngle: 161, // Starting angle for the first menu item
      endArcAngle: 200, // Ending angle for the last menu item
      startItemAngle: 170, // Angle for the first menu item
      lineAngle: 180, // Angle for the line pointing to the selected menu item
    };
  },
  computed: {
    angleStep(): number {
      return (this.endArcAngle - this.startArcAngle - 20) / (this.menuItems.length - 1);
    },
    boldness() {
      return (index: number) => {
        const itemAngle = this.startItemAngle + index * this.angleStep;
        const diff = Math.abs(this.lineAngle - itemAngle);

        if (diff < 0.5) {
          return 800;
        } else if (diff < 1) {
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
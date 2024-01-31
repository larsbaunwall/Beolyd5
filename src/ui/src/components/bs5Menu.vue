<template>
    <div class="circle-container">
      <svg class="circle-svg">
        <circle cx="150" cy="150" r="100" stroke="black" stroke-width="2" fill="transparent"/>
        <line :x1="150" :y1="150" :x2="lineX" :y2="lineY" stroke="red" stroke-width="2"/>
        <g v-for="(item, index) in positionedItems" :key="index">
          <circle :cx="item.x" :cy="item.y" r="10" fill="blue"/>
          <text :x="item.x" :y="item.y" text-anchor="end" dy=".3em" fill="white">{{ item.label }}</text>
        </g>
      </svg>
      <input type="range" min="0" max="120" v-model="scrollValue" />
    </div>
  </template>
  
<script lang="ts">
import { defineComponent, computed, ref, watch } from 'vue';

export default defineComponent({
  name: 'CircleList',
  props: {
    startAngle: {
      type: Number,
      default: 90 // Math.PI / 2, // 90 degrees
    },
    endAngle: {
      type: Number,
      default: 100 //(3 * Math.PI) / 2, // 270 degrees
    },
  },
  setup(props) {
    const scrollValue = ref(0);
    const items = ref([
      { label: 'SOURCES' },
      { label: 'OTHER' },
      { label: 'YET OTHER' },
      { label: 'SETTINGS' },
      // ... more items
    ]);
    const totalItems = items.value.length;
    const selectedIndex = ref(0);

    watch(scrollValue, (newValue) => {
      selectedIndex.value = Math.floor((newValue / 120) * totalItems) % totalItems;
    });

    const positionedItems = computed(() =>
      items.value.map((item, index) => {
        const angleRange = props.endAngle - props.startAngle;
        const angle = props.startAngle + (index / totalItems) * angleRange;
        const x = 150 + 100 * Math.cos(angle);
        const y = 150 + 100 * Math.sin(angle);
        return { ...item, x, y };
      })
    );

    const lineX = computed(() => {
      const angleRange = props.endAngle - props.startAngle;
      const angle = props.startAngle + (selectedIndex.value / totalItems) * angleRange;
      return 150 + 100 * Math.cos(angle);
    });

    const lineY = computed(() => {
      const angleRange = props.endAngle - props.startAngle;
      const angle = props.startAngle + (selectedIndex.value / totalItems) * angleRange;
      return 150 + 100 * Math.sin(angle);
    });

    return {
      scrollValue,
      positionedItems,
      lineX,
      lineY,
    };
  },
});
</script>

<style scoped>
.circle-container {
  position: relative;
  width: 300px;
  height: 300px;
}

.circle-svg {
  width: 300px;
  height: 300px;
}

input[type="range"] {
  position: absolute;
  bottom: -50px; /* Adjust as needed */
  left: 50%;
  transform: translateX(-50%);
}
</style>
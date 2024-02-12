<template>
    <svg width="1024" height="768" style="position: absolute; z-index: 90;">
        <defs>
            <linearGradient id="gradient" gradientTransform="rotate(90)">
                <stop offset="0%" stop-color="rgba(102,153,255,0)" />
                <stop offset="5%" stop-color="rgba(102,153,255,1)" />
                <stop offset="95%" stop-color="rgba(0,255,204,1)" />
                <stop offset="100%" stop-color="rgba(0,255,204,0)" />
            </linearGradient>
        </defs>
        <!-- Arc path for reference -->
        <path :d="describeArc(startArcAngle, endArcAngle)" fill="none" stroke="url(#gradient)"
            stroke-width="3" stroke-linecap="round" />
    </svg>
</template>
  
<script lang="ts">
import { defineComponent } from 'vue';
import arcs from '../utils/arcs';

export default defineComponent({
    name: 'MainCircleArc',
    props: {
      radius: {
        type: Number,
        required: true,
        default: 1000,
      }
    },
    setup() {
        return {};
    },
    data() {
        return {
            startArcAngle: 158,
            endArcAngle: 202,
        };
    },
    computed: {
    },
    methods: {
        describeArc(startAngle: number, endAngle: number) {
            return arcs.describeArc(arcs.cx, arcs.cy, this.radius, startAngle, endAngle);
        },
    },
});
</script>
  
<style scoped>
</style>../utils/arcs
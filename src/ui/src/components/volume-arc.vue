<template>
    <svg width="1024" height="768" style="position: absolute; z-index: 90; opacity: 0.5;">
        <defs>
            <linearGradient id="gradient">
                <stop offset="5%" stop-color="rgba(102,153,255,0.3)" />
                <stop offset="95%" stop-color="rgba(102,153,204,0.3)" />
            </linearGradient>
        </defs>
        <!-- Arc path for reference -->
        <path :d="describeArc(radius, startArcAngle, translateVolume)" fill="none" stroke="url(#gradient)"
            stroke-width="10" stroke-linecap="round" />
    </svg>
</template>
  
<script lang="ts">
import { defineComponent } from 'vue';
import arcs from '../utils/arcs';
import { useUIStore } from '../stores/ui';

export default defineComponent({
    name: 'VolumeArc',
    setup() {
        const uiStore = useUIStore();
        return { uiStore };
    },
    data() {
        return {
            radius: 270,
            startArcAngle: 95,
            endArcAngle: 265,
        };
    },
    computed: {
        translateVolume(): number {
            // Map the input value from 0-100 to 90-270
            return ((this.uiStore.volume - 0) * (this.endArcAngle - this.startArcAngle)) / (100 - 0) + this.startArcAngle;
        }
    },
    methods: {
        describeArc(radius: number, startAngle: number, endAngle: number) {
            return arcs.describeArc(arcs.cx, arcs.cy, radius, startAngle, endAngle);            
        }
    },
});
</script>
  
<style scoped>
</style>
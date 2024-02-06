<template>
    <div style="z-index: 1000; position: fixed; top: 0%; right: 0%; opacity: 0.1; ">
        <input type="number" v-model="uiStore.wheelPointerAngle" />
        <input type="number" v-model="uiStore.volume" />
    </div>
    <div
        style="z-index: 200; position: fixed; top: 50%; right: 50px; opacity: 0.99; ">
        <input style="transform: translateY(-50%) rotate(270deg);" type="range" min="150" max="210" step="0.001" v-model="uiStore.wheelPointerAngle" />
    </div>
    <div
        style="z-index: 200; position: fixed; top: 50%; right: 0px; opacity: 0.99; ">
        <input style="transform: translateY(-50%) rotate(270deg);" type="range" min="0" max="100" step="1" v-model="uiStore.volume" />
    </div>
</template>
    
<script lang="ts">
import { defineComponent } from 'vue';
import { useUIStore } from '../stores/ui';

export default defineComponent({
    name: 'BS5DebugOverlay',
    setup() {
        const uiStore = useUIStore();
        return { uiStore };
    },
    data() {
        return {
        };
    },
    computed: {
    }
});
</script>
    
<style scoped></style>
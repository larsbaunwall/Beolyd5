<script setup lang="ts">
import BS5Shell from "./bs5.vue";
import BS5DebugOverlay from "../components/debug-overlay.vue";
import {useUIStore} from "../stores/ui.ts";

const uiStore = useUIStore();
</script>

<template>
  <div style="z-index: 10000; position: fixed; top: 0%; right: 0%; ">
    <input type="number" v-model="uiStore.wheelPointerAngle" />
    <input type="number" v-model="uiStore.volume" />
  </div>
  <div
      style="z-index: 10000; position: fixed; top: 50%; right: 50px; ">
    <input style="transform: translateY(-50%) rotate(270deg);" type="range" min="150" max="210" step="0.001" v-model="uiStore.wheelPointerAngle" />
  </div>
  <div
      style="z-index: 10000; position: fixed; top: 50%; right: 0px; ">
    <input style="transform: translateY(-50%) rotate(270deg);" type="range" min="0" max="100" step="1" v-model="uiStore.volume" />
  </div>
<img id="wheel" src="../assets/wheel.png">
  <div class="controls">
    <button class="middle-button"><</button> <!-- Add this line -->
    <button class="middle-button">></button> <!-- Add this line -->
    <button class="middle-button">GO</button> <!-- Add this line -->
  </div>
  <div id="simulator">
    <div class="debug-container">
      <BS5Shell />
    </div>
  </div>
</template>
<style scoped>
#wheel {
  position: absolute;
  top: 238px;
  left: 1004px;
  width: 500px;
  height: 500px;
  z-index: 1000;
}

.controls {
  display: flex;
  justify-content: space-between;
  position: absolute;
  top: 469px; /* Position at the middle of the parent element */
  left: 1057px; /* Position at the middle of the parent element */
  width: 390px; /* Adjust as needed */
  z-index: 2500;
}
.middle-button {
  flex: 1; /* This will make the buttons take up equal space */
  margin: 2px; /* Add some space between the buttons */
  height: 32px;
  background-color: black;
  color: white;
  border: none;
  cursor: pointer;
}

.middle-button:active {
  animation: flashGray 300ms;
}

@keyframes flashGray {
  0% { background-color: black; }
  50% { background-color: #333; }
  100% { background-color: black; }
}


.debug-container {
  border: 80px black solid;
  border-radius: 5px;
  width: 1024px; /* Adjust as needed */
  height: 768px; /* Adjust as needed */
  position: absolute; /* or relative, depending on your needs */
  overflow: hidden;
}

#simulator {
  position: relative;
  width: 1400px; /* Adjust as needed */
  height: 948px; /* Adjust as needed */
  overflow: auto;
  top: 20px; /* Adjust as needed */
  left: 20px; /* Adjust as needed */
}
</style>
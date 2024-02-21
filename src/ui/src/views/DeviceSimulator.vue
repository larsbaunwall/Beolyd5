<script setup lang="ts">
import BS5Shell from "../components/bs5-shell.vue";
import BS5DebugOverlay from "../components/debug-overlay.vue";
</script>

<template>
  <svg class="circle-svg">
    <circle cx="1247" cy="487" r="240" fill="lightgray" />
    <line x1="1017" y1="437" x2="1477" y2="437" stroke="gray" /> <!-- Adjusted this line -->
    <line x1="1017" y1="537" x2="1477" y2="537" stroke="gray" /> <!-- Adjusted this line -->
  </svg>
  <div class="controls">
    <button class="middle-button"><</button> <!-- Add this line -->
    <button class="middle-button">></button> <!-- Add this line -->
    <button class="middle-button">GO</button> <!-- Add this line -->
  </div>
  <div class="debug-container">
    <BS5DebugOverlay />
    <BS5Shell />
  </div>
</template>

<style scoped>
.circle-svg {
  position: absolute;
  width: 100%;
  height: 100%;
  z-index: 2000; /* To ensure the circle is on top */
}
.controls {
  display: flex;
  justify-content: space-between;
  position: absolute;
  top: 452px; /* Position at the middle of the parent element */
  left: 1047px; /* Position at the middle of the parent element */
  width: 400px; /* Adjust as needed */
  z-index: 2500;
}
.middle-button {
  flex: 1; /* This will make the buttons take up equal space */
  margin: 10px; /* Add some space between the buttons */
  height: 50px;
}
.debug-container {
  border: 80px black solid;
  border-radius: 5px;
  width: 1024px; /* Adjust as needed */
  height: 768px; /* Adjust as needed */
  position: absolute; /* or relative, depending on your needs */
  top: 20px; /* Adjust as needed */
  left: 20px; /* Adjust as needed */
  overflow: hidden;
}
</style>
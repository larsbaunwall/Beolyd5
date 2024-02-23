<script setup lang="ts">
import {useUIStore} from "../stores/ui.ts";
import FullscreenContainer from "./FullscreenContainer.vue";
import {Wheel} from "../hardware/events.ts";

const uiStore = useUIStore();

const handleAngularWheelChange = e => {
  console.log('Wheel change', e.target.value);
  uiStore.nextHardwareEvent({payload: {kind: 'wheel', source: Wheel.Angular, value: e.target.value}})
};
</script>

<template>
  <a href="https://github.com/larsbaunwall/Beolyd5" class="github-corner" aria-label="View source on GitHub">
    <svg width="80" height="80" viewBox="0 0 250 250" style="fill:#151513; color:#fff; position: absolute; top: 0; border: 0; right: 0;" aria-hidden="true">
      <path d="M0,0 L115,115 L130,115 L142,142 L250,250 L250,0 Z"></path>
      <path d="M128.3,109.0 C113.8,99.7 119.0,89.6 119.0,89.6 C122.0,82.7 120.5,78.6 120.5,78.6 C119.2,72.0 123.4,76.3 123.4,76.3 C127.3,80.9 125.5,87.3 125.5,87.3 C122.9,97.6 130.6,101.9 134.4,103.2" fill="currentColor" style="transform-origin: 130px 106px;" class="octo-arm"></path>
      <path d="M115.0,115.0 C114.9,115.1 118.7,116.5 119.8,115.4 L133.7,101.6 C136.9,99.2 139.9,98.4 142.2,98.6 C133.8,88.0 127.5,74.4 143.8,58.0 C148.5,53.4 154.0,51.2 159.7,51.0 C160.3,49.4 163.2,43.6 171.4,40.1 C171.4,40.1 176.1,42.5 178.8,56.2 C183.1,58.6 187.2,61.8 190.9,65.4 C194.5,69.0 197.7,73.2 200.1,77.6 C213.8,80.2 216.3,84.9 216.3,84.9 C212.7,93.1 206.9,96.0 205.4,96.6 C205.1,102.4 203.0,107.8 198.3,112.5 C181.9,128.9 168.3,122.5 157.7,114.1 C157.9,116.9 156.7,120.9 152.7,124.9 L141.0,136.5 C139.8,137.7 141.6,141.9 141.8,141.8 Z" fill="currentColor" class="octo-body"></path>
    </svg>
  </a>
  <div style="padding: 20px; background-color: white">
    <h1>Beolyd5 UI simulation</h1>
    <p>This is a simulation of the custom-built UI for the Beosound 5 sound system. The software is written for the Raspberry PI using the Beosound 5 rotation controller and a 1024x768px screen.</p>
    <p>Read more at <a href="https://github.com/larsbaunwall/Beolyd5">the project repo on Github.</a></p>
    <h2>How to use</h2>
    <p>Use the sliders to control the wheel pointer angle and the volume. The buttons are not functional yet.</p>
    <p>For now, you can use up and down arrow keys to cycle through the menu option list.</p>
    <h2>Contribute</h2>
    <p>Come help out - open an <a href="https://github.com/larsbaunwall/Beolyd5/issues">issue</a> or start a <a href="https://github.com/larsbaunwall/Beolyd5/discussions">discussion</a>!</p>
    <div id="simulator">
      <div style="z-index: 10000">
        <span style="position: absolute; left: 1540px; top: 340px; color: gray">Wheels</span>
        <div id="wheel-bars">
          <div>
            <input type="range" min="150" max="210" step="0.1" disabled/>
          </div>
          <div>
            <input
                type="range" value="60" min="0" max="120" step="0.1"
                @input="handleAngularWheelChange"/>
          </div>
          <div>
            <input type="range" min="0" max="100"
                   step="0.1"
                   v-model="uiStore.volume"/>
          </div>
        </div>
        <img id="wheel" src="../assets/wheel.png">
        <div class="controls">
          <button class="middle-button"><</button>
          <button class="middle-button">></button>
          <button class="middle-button">GO</button>
        </div>
      </div>
      <div class="debug-container">
        <FullscreenContainer/>
      </div>
    </div>
    <div id="store-props">
      <div><strong>Debug values</strong></div>
      <div><span>Wheel pointer angle: </span><input type="number" step="0.5" min="150" max="210"
                                                    v-model="uiStore.wheelPointerAngle"/></div>
      <div><span>Volume: </span><input type="number" step="0.5" min="0" max="100" v-model="uiStore.volume"/></div>
    </div>
  </div>
</template>
<style scoped>
#wheel {
  position: absolute;
  top: 218px;
  left: 1004px;
  width: 500px;
  height: 500px;
  z-index: 1000;
}

.controls {
  display: flex;
  justify-content: space-between;
  position: absolute;
  top: 449px; /* Position at the middle of the parent element */
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
  0% {
    background-color: black;
  }
  50% {
    background-color: #333;
  }
  100% {
    background-color: black;
  }
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
  width: 1700px; /* Adjust as needed */
  height: 948px; /* Adjust as needed */
  overflow: auto;
  top: 20px; /* Adjust as needed */
  left: 20px; /* Adjust as needed */
}

#store-props {
  margin: 20px;
  display: flex;
  flex-direction: column;
}

#store-props div {
  margin: 2px;
}

#wheel-bars {
  display: flex;
  justify-content: space-between;
  flex-direction: row;
  position: absolute;
  top: 459px;
  left: 1520px;
  width: 100px;
  z-index: 111111;
}

#wheel-bars div {
  transform: rotate(270deg);
  width: 100px;
}

#wheel-bars input {
  width: 200px;
  margin: -90px;
}
.github-corner:hover .octo-arm{animation:octocat-wave 560ms ease-in-out}@keyframes octocat-wave{0%,100%{transform:rotate(0)}20%,60%{transform:rotate(-25deg)}40%,80%{transform:rotate(10deg)}}@media (max-width:500px){.github-corner:hover .octo-arm{animation:none}.github-corner .octo-arm{animation:octocat-wave 560ms ease-in-out}}
</style>
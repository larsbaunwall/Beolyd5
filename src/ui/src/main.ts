import { createApp } from "vue";
import { createRouter, createWebHistory } from "vue-router";
import { createPinia } from "pinia";

import "./styles.css";
import App from "./App.vue";

import DefaultView from "./components/default.vue";
import MusicView from "./components/music.vue";
import RadioView from "./components/radio.vue";
import { listen } from "@tauri-apps/api/event";
import { useUIStore } from "./stores/ui";
import { translateToRange } from "./utils/arcs";

const router = createRouter({
    history: createWebHistory(),
    routes: [
        {
            path: "/",
            children: [ 
                {path: '', component: DefaultView, meta: {title: 'HOME'}},
                {path: 'music', component: MusicView, meta: {title: 'N.MUSIC'}},
                {path: 'radio', component: RadioView, meta: {title: 'N.RADIO'}},
            ],
        },
    ],
});

const pinia = createPinia();

createApp(App)
    .use(pinia)
    .use(router)
    .mount("#app");

const uiStore = useUIStore();

const unlisten = listen('wheelEvent', (event) => {
    console.log({event});
    if(event.payload.wheel == 'Angular') {
        
        uiStore.wheelPointerAngle = translateToRange(event.payload.position, 152, 195);
    }
    if(event.payload.wheel == 'Back') {
        console.log(wheelSpinDifference(event.payload.position));
        let newVolume = uiStore.volume + wheelSpinDifference(event.payload.position);
        uiStore.volume = Math.max(0, Math.min(newVolume, 100));
    }
});

export function wheelSpinDifference(value: number): number {
    return value <= 125 ? value : (256 - value) * -1;
}

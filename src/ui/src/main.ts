import { createApp } from "vue";
import {createRouter, createWebHashHistory} from "vue-router";
import { createPinia } from "pinia";

import "./styles.css";
import App from "./App.vue";

import {Subject, filter, bufferCount} from "rxjs";

import DefaultView from "./components/default.vue";
import MusicView from "./components/music.vue";
import RadioView from "./components/radio.vue";
import { listen } from "@tauri-apps/api/event";
import { useUIStore } from "./stores/ui";
import { translateToRange } from "./utils/arcs";
import DeviceSim from "./views/DeviceSimulator.vue";
import Bs5Shell from "./views/bs5.vue";
import Shell from "./views/Shell.vue";

const router = createRouter({
    history: createWebHashHistory(),
    routes: [
        {
            path: "/",
            component: Shell,
            props: route => {
                const shell = route.params.shell;

                if(shell === 'sim') {
                    return { component: DeviceSim, shell: 'sim' };
                } else {
                    return { component: Bs5Shell, shell: 'default' };
                }
            },
            children: [ 
                {path: ':shell?', component: DefaultView, meta: {title: 'HOME'}, props: true},
                {path: ':shell?/music', component: MusicView, meta: {title: 'N.MUSIC'}, props: true},
                {path: ':shell?/radio', component: RadioView, meta: {title: 'N.RADIO'}, props: true},
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

interface WheelEvent {
    payload: {
        position: number;
        wheel: string;
    };
}

const wheelEvents = new Subject<WheelEvent>();

const unlisten = listen('wheelEvent', (event:WheelEvent) => {
    wheelEvents.next(event);
});

const diags = listen('diagnostics', (event) => {
    console.log({event});
});

const wheelSub$ = wheelEvents.subscribe((event) => {
    if(event.payload.wheel == 'Angular') {
        uiStore.wheelPointerAngle = translateToRange(event.payload.position, 152, 195);
    }
    if(event.payload.wheel == 'Back') {
        console.log(wheelSpinDifference(event.payload.position));
        let newVolume = uiStore.volume + wheelSpinDifference(event.payload.position);
        uiStore.volume = Math.max(0, Math.min(newVolume, 100));
    }
});

// Filter events where wheel is 'Front'
const frontWheelEvents$ = wheelEvents.pipe(
    filter(event => event.payload.wheel === 'Front')
).pipe(bufferCount(10));

frontWheelEvents$.subscribe((events) => {
    const event = events[events.length - 1];
    console.log(wheelSpinDifference(event.payload.position));
    uiStore.topWheelPosition = wheelSpinDifference(event.payload.position);
});

function wheelSpinDifference(value: number): number {
    return value <= 125 ? value : (256 - value) * -1;
}

import { createApp } from "vue";
import { createRouter, createWebHistory } from "vue-router";
import { createPinia } from "pinia";
import { emit, listen } from '@tauri-apps/api/event'

import "./styles.css";
import App from "./App.vue";

import DefaultView from "./components/default.vue";
import MusicView from "./components/music.vue";
import RadioView from "./components/radio.vue";

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

const unlisten = await listen('wheelEvent', (event) => {
    console.log('Received event', event);
    });



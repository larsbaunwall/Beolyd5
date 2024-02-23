import { createApp } from "vue";
import { createPinia } from "pinia";

import "./styles.css";
import App from "./App.vue";

import {createRoutes} from "./routing/routes.ts";
import {startHardwareBridge} from "./hardware/events.ts";


const pinia = createPinia();

createApp(App)
    .use(pinia)
    .use(createRoutes)
    .mount("#app");

startHardwareBridge();
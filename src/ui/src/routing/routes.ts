import {createRouter, createWebHashHistory} from "vue-router";
import Shell from "../views/Shell.vue";
import DeviceSim from "../views/DeviceSimulator.vue";
import MainMenuShell from "../views/MainMenuShell.vue";
import DefaultView from "../components/Default.vue";
import MusicView from "../views/Music.vue";
import RadioView from "../views/Radio.vue";

export const createRoutes = createRouter({
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
                    return { component: MainMenuShell, shell: 'default' };
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

import {createRouter, createWebHashHistory} from "vue-router";
import Shell from "../views/Shell.vue";
import DeviceSim from "../views/DeviceSimulator.vue";
import MainMenu from "../views/MainMenu.vue";
import DefaultView from "../components/Default.vue";
import MusicView from "../views/Music.vue";
import RadioView from "../views/Radio.vue";
import FullscreenContainer from "../views/FullscreenContainer.vue";
import NowPlaying from "../views/NowPlaying.vue";

export const createRoutes = createRouter({
    history: createWebHashHistory(),
    routes: [
        {
            path: "/:shell?",
            component: Shell,
            props: route => {
                const shell = route.params.shell;

                if(shell === 'sim') {
                    return { component: DeviceSim, shell: 'sim' };
                } else {
                    return { component: FullscreenContainer, shell: 'default' };
                }
            },
            redirect: route => `/${route.params.shell}/menu`,
            children: [
                {
                    path: "menu",
                    component: MainMenu,
                    props: true,
                    children: [
                        {path: '', component: DefaultView, meta: {title: 'HOME'}, props: true},
                        {path: 'music', component: MusicView, meta: {title: 'N.MUSIC'}, props: true},
                        {path: 'radio', component: RadioView, meta: {title: 'N.RADIO'}, props: true},
                    ]
                },
                {
                    path: "playing",
                    component: NowPlaying,
                    props: true,
                },

            ],
        },
    ],
});

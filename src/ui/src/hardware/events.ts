import {bufferCount, filter, Subject} from "rxjs";
import {listen} from "@tauri-apps/api/event";
import arcs from "../utils/arcs.ts";
import {useUIStore} from "../stores/ui.ts";

export interface WheelEvent {
    payload: {
        position: number;
        wheel: string;
    };
}

export const startHardwareBridge = () => {

    const uiStore = useUIStore();
    const allWheelEvents$ = new Subject<WheelEvent>();
    const diagnostics$ = new Subject<any>();
    const frontWheelEvents$ = allWheelEvents$.pipe(
        filter(event => event.payload.wheel === 'Front')
    ).pipe(bufferCount(10));
    const backWheelEvents$ = allWheelEvents$.pipe(
        filter(event => event.payload.wheel === 'Back')
    ).pipe(bufferCount(10));

    const unlisten = listen('wheelEvent', (event: WheelEvent) => {
        allWheelEvents$.next(event);
    });
    const diags = listen('diagnostics', (event) => {
        diagnostics$.next(event);
        console.log({event});
    });

    allWheelEvents$.subscribe((event) => {
        if (event.payload.wheel == 'Angular') {
            uiStore.wheelPointerAngle = arcs.translateToRange(event.payload.position, 152, 195);
        }
    });

    frontWheelEvents$.subscribe((events) => {
        const event = events[events.length - 1];
        uiStore.topWheelPosition = wheelSpinDifference(event.payload.position);
    });

    backWheelEvents$.subscribe((events) => {
        const event = events[events.length - 1];
        let newVolume = uiStore.volume + wheelSpinDifference(event.payload.position);
        uiStore.volume = Math.max(0, Math.min(newVolume, 100));
    });

    return {wheelEvents: allWheelEvents$, diagnostics: diagnostics$};
}

function wheelSpinDifference(value: number): number {
    return value <= 125 ? value : (256 - value) * -1;
}
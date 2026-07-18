import {bufferCount, filter} from "rxjs";
import {listen} from "@tauri-apps/api/event";
import arcs from "../utils/arcs.ts";
import {useUIStore} from "../stores/ui.ts";

export enum Wheel {
    Front = "Front",
    Angular = "Angular",
    Back = "Back"
}

export enum Button {
    Left = "Left",
    Right = "Right",
    Go = "Go",
    Standby = "Standby"
}

export interface HardwareEvent {
    payload: {
        kind: 'wheel' | 'button' | string,
        source: Wheel | Button | string,
        value: number
    },
}

export const startHardwareBridge = () => {
    const uiStore = useUIStore();

    // listen() returns a Promise<UnlistenFn>. We await it so any rejection is
    // surfaced and the unlisten handle is available for cleanup if needed.
    listen('hardwareEvent', (event: HardwareEvent) => {
        uiStore.nextHardwareEvent(event);
    }).catch((err) => console.error('hardwareEvent listener failed:', err));

    const wheelEvents$ = uiStore.hardwareEvents.pipe(
        filter(event => event.payload.kind === 'wheel')
    );

    wheelEvents$.pipe(
            filter(event => event.payload.source === Wheel.Back),
            bufferCount(10)
        ).subscribe((events) => {
            const event = events[events.length - 1];
            const newVolume = uiStore.volume + wheelSpinDifference(event.payload.value);
            uiStore.volume = Math.max(0, Math.min(newVolume, 100));
        });

    wheelEvents$.pipe(
            filter(event => event.payload.source === Wheel.Front),
            bufferCount(10)
        ).subscribe((events) => {
            const event = events[events.length - 1];
            uiStore.topWheelPosition = wheelSpinDifference(event.payload.value);
        });

    wheelEvents$.pipe(
            filter(event => event.payload.source === Wheel.Angular)
        ).subscribe((event) => {
            uiStore.wheelPointerAngle = arcs.translateToRange(event.payload.value, 0,120,152, 205);
        });

    listen('diagnostics', (event) => {
        console.log('diagnostics:', event);
    }).catch((err) => console.error('diagnostics listener failed:', err));
}

export function wheelSpinDifference(value: number): number {
    return value <= 125 ? value : (256 - value) * -1;
}
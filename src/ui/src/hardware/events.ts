import {bufferCount, filter, Subject} from "rxjs";
import {listen} from "@tauri-apps/api/event";
import arcs from "../utils/arcs.ts";
import {useUIStore} from "../stores/ui.ts";

export enum Wheel {
    Front = "Front",
    Angle = "Angle",
    Back = "Back"
}

export enum Button {
    Left = "Left",
    Right = "Right",
    Go = "Go",
    Standby = "Standby"
}

export interface WheelEvent {
    payload: {
        position: number,
        wheel: string,
    },
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

    const allWheelEventsSubject = new Subject<WheelEvent>();
    const diagnosticsSubject = new Subject<any>();

    const frontWheelEvents$ = allWheelEventsSubject.pipe(
        filter(event => event.payload.wheel === 'Front')
    ).pipe(bufferCount(10));
    const backWheelEvents$ = allWheelEventsSubject.pipe(
        filter(event => event.payload.wheel === 'Back')
    ).pipe(bufferCount(10));

    const unlisten = listen('wheelEvent', (event: WheelEvent) => {
        allWheelEventsSubject.next(event);
    });
    const diags = listen('diagnostics', (event) => {
        diagnosticsSubject.next(event);
        console.log({event});
    });

    const unlistenAllEvents = listen('hardwareEvent', (event: HardwareEvent) => {
        uiStore.hardwareEvents.next(event);
    });

    allWheelEventsSubject.subscribe((event) => {
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

    return {wheelEvents: allWheelEventsSubject, diagnostics: diagnosticsSubject};
}

function wheelSpinDifference(value: number): number {
    return value <= 125 ? value : (256 - value) * -1;
}
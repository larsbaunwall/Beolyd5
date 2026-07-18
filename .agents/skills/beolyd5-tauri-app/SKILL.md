---
name: beolyd5-tauri-app
description: >-
  Use this skill to act as the Vue/Frontend Expert to build the full UI-replacement for
  the BeoSound 5. Covers the Tauri 2 (Rust) desktop shell, the Vue 3 + Pinia + RxJS
  frontend, mapping MPRIS state to the UI, the B&O minimalist aesthetic, the Rust↔JS
  event bridge, arc geometry, and the device simulator. Use when acting as an autonomous
  agent to flesh out the missing Views (Music, Radio, NowPlaying), wire the hardware
  bridge, or build out the frontend architecture.
license: Apache-2.0
---

# Beolyd5 UI (Tauri + Vue) development

The app in `src/ui` is the on-device interface. A **Tauri v2** (Rust) host wraps a Vue 3
frontend rendered via WebKitGTK on Linux. The Rust side owns the HID device and pushes
events; the Vue side draws the arc UI and reacts.

Load `beosound5-hardware` for protocol facts and `beolyd5-audio-backend` for MPRIS facts.

## Architecture — control surface, not audio engine

**Core principle** (owned canonically by `beolyd5-architecture`): the Tauri app is a
*control surface*. It must NOT contain an audio engine, decoder, or streaming client.
Playback lives in headless daemons (see `beolyd5-audio-backend`); the app only renders state
and sends control intents. Putting playback in the webview is the wrong architecture.

## Layout

- `src/ui/src-tauri/src/main.rs` — Tauri entry; registers `tick` command, sets up
  `HWController`.
- `src/ui/src-tauri/src/hw_controller.rs` — wraps the crate; forwards wheel and button
  events to JS via `app_handle.emit("hardwareEvent", payload)` (Tauri 2 `Emitter` trait)
  and `emit("diagnostics", payload)`. Keeps the async task alive with
  `std::future::pending::<()>().await` — no busy-loop.
- `src/ui/src/hardware/events.ts` — JS side of the bridge; `listen(...)` + RxJS pipelines
  turn raw events into store mutations (volume, wheel pointer angle, top-wheel scroll).
- `src/ui/src/stores/ui.ts` — Pinia store: `volume`, `wheelPointerAngle`,
  `topWheelPosition`, `hardwareEvents` Subject, `tick()`.
- `src/ui/src/utils/arcs.ts` — polar/cartesian arc math; screen-aligned centre
  (`cx: 1147, cy: 387`).
- `src/ui/src/views/*` — `Shell`, `MainMenu`, `Music`, `Radio`, `NowPlaying`,
  `DeviceSimulator`, `FullscreenContainer`.

## The event bridge (Rust → JS)

Rust emits:
- `"hardwareEvent"` — for **both** wheel and button events, carrying
  `HardwareEvent { kind, source, value }`. Uses Tauri 2 `app_handle.emit(event, payload)`
  (import `tauri::Emitter`).
- `"diagnostics"` — device open/error status.

`kind: "wheel" | "button"`, `source: Front|Angular|Back` or `Left|Right|Go|Standby`, `value: u8`.

JS consumes it in `events.ts`. Mapping currently used:

- **Back wheel** → volume (buffered, `bufferCount(10)`, clamped 0–100).
- **Front wheel** → `topWheelPosition` (list scroll, also buffered `bufferCount(10)`).
- **Angular wheel** → `wheelPointerAngle` via `translateToRange(value, 0,120,152,205)`.
- **Buttons** → no JS handler exists yet; `listen('hardwareEvent', ...)` receives them but
  the pipeline only filters on `kind === 'wheel'`. Adding button handling means filtering
  `kind === 'button'` on the same `"hardwareEvent"` stream.

Convert relative wheel values with `wheelSpinDifference` (the 256-wrap → signed delta).

### Wiring the click ("tick")

Use Tauri v2 imports and guard simulator mode with `isTauri()` (not `window.__TAURI__`,
which is unreliable in v2):

```ts
import { invoke, isTauri } from "@tauri-apps/api/core";
const tick = () => {
  if (!isTauri()) return;
  invoke('tick').catch(() => {});
};
```

The `useUIStore` in `stores/ui.ts` already implements this pattern.

## Building the UI Replacement: Aesthetic and Views

Load `beolyd5-ux-design` for all visual/interaction/aesthetic decisions.

The placeholder views still to complete:
- **`NowPlaying.vue` / `Music.vue` / `Radio.vue`** — empty/stub. Connect to MPRIS broker
  data via a Pinia `useAudioStore`.
- **Cover Art** — large album art with fading/reflections per the BS5 look.
- `ArcContentFlow` — populate with real library tracks so front-wheel scroll navigates
  artists/albums from MPD.

## Tauri 2 baseline

This project is on **Tauri v2**. The migration from v1 is complete. Key v2 conventions
already in place:

- **Events:** `app_handle.emit(event, payload)` with `use tauri::Emitter` imported.
- **JS imports:** `@tauri-apps/api/core` for `invoke`/`isTauri`; `@tauri-apps/api/event`
  for `listen`.
- **Config:** `tauri.conf.json` uses the v2 **capabilities/permissions** model
  (`src-tauri/capabilities/*.json`). Grant only what's used.
- **Plugins:** shell is `tauri-plugin-shell` (crate + `@tauri-apps/plugin-shell` npm pkg).
- **Async keep-alive:** `hw_controller.rs::init` ends with `std::future::pending::<()>().await`
  — no `loop {}` busy-wait.
- **Package manager:** npm-only. `yarn.lock` is gone; use `npm install`.

## Frontend conventions

- **Vue 3 Composition API + `<script setup>` only** (the codebase standardised on this).
- **State via Pinia**, cross-cutting event streams via **RxJS** Subjects.
- **Vue Router** with hash history (`createWebHashHistory`) — required for GitHub Pages
  simulator.
- The **device simulator** (`/sim` route, `DeviceSimulator.vue`) drives the same store
  with sliders/keys so the UI runs in a plain browser without hardware. Keep any new
  hardware-driven feature simulator-drivable, and guard Tauri-only calls.

## Arc geometry

Menu/list items are positioned along an arc via `arcs.ts` (`getArcPoint`,
`polarToCartesian`). The centre (`cx/cy`) and radii are tuned to the physical
screen/wheel offset. **Do not scatter magic numbers**: when touching layout, thread
values through props/config so the same component works in the 1024×768 device shell and
the simulator. Selection = an item's arc angle within a small threshold of the pointer
angle; fire `tick()` on the transition.

## Build & run

```bash
cd src/ui
npm install
npm run dev            # browser-only (simulator), http://localhost:1421/Beolyd5/
npm run tauri dev      # full app with HID bridge (Linux/WebKitGTK)
npm run build          # vue-tsc --noEmit (strict + noUnusedLocals) + vite build
```

## Gotchas

- `startHardwareBridge()` runs unconditionally in `main.ts`; Tauri `listen` is a no-op in
  a plain browser, which is fine for the sim — but any `invoke` will reject, so guard it
  with `isTauri()` from `@tauri-apps/api/core`. Do **not** use `window.__TAURI__` —
  it is unreliable in Tauri v2.
- **Base path** is conditional: `TAURI_ENV_PLATFORM ? "/" : "/Beolyd5/"` (set in
  `vite.config.ts`). The env var is injected by Tauri dev/build; absent in plain `npm run dev`.
  Keep the `homepage` in `package.json` and the `base` in sync.
- TypeScript is now `ES2022` + `noUnusedLocals: true` + `noUnusedParameters: true`.
  `npm run build` fails on unused imports — remove them.
- WebKitGTK on the Pi may need `WEBKIT_DISABLE_COMPOSITING_MODE=1` (see the boot script)
  to avoid rendering glitches. CI uses `libwebkit2gtk-4.1-dev` (Tauri 2 requirement).
- Keep the crate types `Serialize`-compatible; the bridge serialises `Button`/`Wheel`.
- `FullscreenContainer.vue` `shellPrefix` is reactive (`computed`) — do not make it a
  plain `ref` or `let`.

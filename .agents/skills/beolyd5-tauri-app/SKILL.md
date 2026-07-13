---
name: beolyd5-tauri-app
description: >-
  Use this skill to act as the Vue/Frontend Expert to build the full UI-replacement for
  the BeoSound 5. Covers the Tauri (Rust) desktop shell, the Vue 3 + Pinia + RxJS
  frontend, mapping MPRIS state to the UI, the B&O minimalist aesthetic, the Rust↔JS
  event bridge, arc geometry, and the device simulator. Use when acting as an autonomous
  agent to flesh out the missing Views (Music, Radio, NowPlaying), wire the hardware
  bridge, or build out the frontend architecture.
license: Apache-2.0
---

# Beolyd5 UI (Tauri + Vue) development

As the Vue/Frontend expert, your primary goal is to turn `src/ui` into a complete,
authentic replica of the Bang & Olufsen BeoSound 5 interface.

The app in `src/ui` is the on-device interface. A Tauri (Rust) host wraps a Vue 3 frontend
rendered via WebKitGTK on Linux. The Rust side owns the HID device and pushes events;
the Vue side draws the arc UI and reacts.

Load `beosound5-hardware` for protocol facts and `beolyd5-audio-backend` for MPRIS facts.

## Architecture — control surface, not audio engine

**Core principle** (owned canonically by `beolyd5-architecture`): the Tauri app is a
*control surface*. It must NOT contain an audio engine, decoder, or streaming client.
Playback lives in headless daemons (see `beolyd5-audio-backend`); the app only renders state
and sends control intents. Putting playback in the webview is the wrong architecture.

## Layout

- `src/ui/src-tauri/src/main.rs` — Tauri entry; registers `tick` command, sets up
  `HWController`.
- `src/ui/src-tauri/src/hw_controller.rs` — wraps the crate; forwards wheel/button events
  to JS via `emit_all("hardwareEvent" | "buttonEvent" | "diagnostics", payload)`.
- `src/ui/src/hardware/events.ts` — JS side of the bridge; `listen(...)` + RxJS pipelines
  turn raw events into store mutations (volume, wheel pointer angle, top-wheel scroll).
- `src/ui/src/stores/ui.ts` — Pinia store: `volume`, `wheelPointerAngle`,
  `topWheelPosition`, `hardwareEvents` Subject, `tick()`.
- `src/ui/src/utils/arcs.ts` — polar/cartesian arc math; screen-aligned centre
  (`cx: 1147, cy: 387`).
- `src/ui/src/views/*` — `Shell`, `MainMenu`, `Music`, `Radio`, `NowPlaying`,
  `DeviceSimulator`, `FullscreenContainer`.

## The event bridge (Rust → JS)

Rust emits **two** event names:
- `"hardwareEvent"` — for both wheel and button events, carrying `HardwareEvent { kind, source, value }`.
- `"buttonEvent"` — also emitted for buttons in `hw_controller.rs`, but **never listened to
  in the JS frontend**. All button handling goes through `"hardwareEvent"` with `kind: "button"`.
  The `"buttonEvent"` emission is dead code on the JS side.
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

`ui.ts`'s `tick()` currently has the `invoke('tick')` call commented out. The file also
has the import commented out as `//import { invoke } from "@tauri-apps/api";` (the v1
generic import). To restore haptic feedback on Tauri v1 (current):

```ts
import { invoke } from "@tauri-apps/api/tauri"; // v1 path
const tick = () => { invoke('tick').catch(() => {}); };
```

On Tauri v2 after migration, the import path changes to `@tauri-apps/api/core`.
Guard both against the simulator: `window.__TAURI__` is defined only inside Tauri, so
check it before calling `invoke`.

## Building the UI Replacement: Aesthetic and Views

For all visual, interaction, hierarchy, motion, and B&O-aesthetic decisions, load the
`beolyd5-ux-design` skill — it is the design authority. Summary of what it mandates:

The legacy BeoSound 5 interface is defined by **minimalism, pure black backgrounds, and
crisp white text** using Helvetica-like fonts. 

To finish the UI replacement, an autonomous agent needs to build out the placeholders:
- **`NowPlaying.vue` / `Music.vue` / `Radio.vue`**: Currently empty or just Unsplash 
  images. Connect these to the MPRIS broker data (which should feed a Pinia store like
  `useAudioStore`).
- **Cover Art**: Display large album art with fading/reflections per the original BS5
  look.
- MPRIS logic needs to populate tracks into `ArcContentFlow` so scrolling the wheel 
  rotates through real artists/albums from MPD.

## Tauri v1 → v2 migration (recommended)

The app is **currently on the Tauri v1 line** (v1 is end-of-life). Migrate to the v2 line for
the current security model and better ARM/Linux support. Prefer the official tool, then fix
the known breakages:

```bash
cd src/ui
npm install
npx @tauri-apps/cli@latest migrate
```

Manual points the migrator won't fully handle:

- **Events:** v1 `app_handle.emit_all(event, payload)` → v2 `app_handle.emit(event, payload)`
  (import `tauri::Emitter`). Update `hw_controller.rs`.
- **JS imports:** `@tauri-apps/api/tauri` → `@tauri-apps/api/core` (`invoke`); `event`
  module stays but re-verify `listen` paths.
- **Config:** `tauri.conf.json` `allowlist` → the v2 **capabilities/permissions** model
  (`src-tauri/capabilities/*.json`). Grant only what's used (shell-open, event).
- **Plugins:** shell/dialog etc. become separate `tauri-plugin-*` crates + JS packages.
- Replace the `loop {}` busy-wait in `hw_controller.rs::init` with a blocking join/park
  (same bug called out in the rust-controller skill).

Verify with `npm run tauri dev` on Linux (WebKitGTK) before considering the migration done.

## Frontend conventions

- **Vue 3 Composition API + `<script setup>` only** (the codebase standardised on this).
- **State via Pinia**, cross-cutting event streams via **RxJS** Subjects.
- **Vue Router** with hash history (`createWebHashHistory`) so it also works on GitHub
  Pages for the simulator.
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
npm run build          # vue-tsc typecheck + vite build
```

## Gotchas

- `startHardwareBridge()` runs unconditionally in `main.ts`; Tauri `listen` is a no-op in
  a plain browser, which is fine for the sim — but any `invoke` will reject, so guard it.
- Hash router base is `/Beolyd5/` for GH Pages; keep it consistent with `homepage` in
  `package.json` and the Vite `base`.
- WebKitGTK on the Pi may need `WEBKIT_DISABLE_COMPOSITING_MODE=1` (see the boot script)
  to avoid rendering glitches.
- Keep the crate types `Serialize`-compatible; the bridge serialises `Button`/`Wheel`.

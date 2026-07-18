# AGENTS.md

Context for Beolyd5: an open-source rebuild of the Bang & Olufsen **BeoSound 5** targeting Raspberry Pi + HiFiBerry DAC.

## Project overview

Repo has **two decoupled components** under `src/`:

1. **`src/rust/`**: `beolyd5_controller` crate (USB-HID abstraction for the physical controller). Published Cargo package.
2. **`src/ui/`**: Vue 3 frontend + **Tauri v2** (Rust) shell. The Tauri host bridges hardware events to the webview. Also runs as a browser-only simulator.

> **CRITICAL**: `src/ui/src-tauri/Cargo.toml` points to the **published** `beolyd5_controller = "1.0.2"`, *not* local `src/rust/`. Local crate edits have no effect on the app without a `[patch]` directive.

Note: No `.NET`/`src/dotnet` files exist despite README mentions. Ignore MAUI tooling.

## Repository layout

```text
/
├─ raspberry-os-*            Pi scripts (Wayland/cage kiosk preferred over X11 matchbox)
├─ .agents/skills/           Agent skills with deep domain knowledge (read these!)
└─ src/
   ├─ rust/                  `beolyd5_controller` crate (standalone Cargo package)
   │  └─ bs5-controller/     STALE nested scaffold (build artifacts only). Do NOT edit.
   └─ ui/                    Vue 3 frontend + Tauri v2 Rust host
      ├─ src/                Vue app (Pinia stores, RxJS hardware bridge, arc math)
      ├─ src-tauri/          Tauri v2 setup (main.rs, hw_controller.rs)
      └─ vite.config.ts      Dev server on port 1421, base `/` in Tauri, `/Beolyd5/` for GH Pages
```

## Setup & build

```bash
# Frontend/simulator (browser only)
cd src/ui
npm install && npm run build
npm run dev          # http://localhost:1421/Beolyd5/sim

# Full desktop app (Linux needs WebKitGTK headers)
npm run tauri dev

# Rust crate
cd ../rust
cargo build
cargo run --example listen  # Needs physical device + udev rules
```

## Testing

- **No automated test suite** in either component.
- The only check is `src/ui/`'s `npm run build` which runs `vue-tsc --noEmit` and **fails on type errors**.
- Crate decode logic (`get_wheel_moved`, `get_button_pressed`) are pure functions; add table-driven `#[test]`s when changing them.

## Conventions

### Rust

- Edition 2021. Keep existing Apache-2.0 headers.
- `beolyd5_controller` public API is a semver contract.
- Keep `Button`, `Wheel`, `SystemEvent` both `Serialize + Deserialize` (required by Tauri bridge).

### Frontend (Vue 3 / TypeScript)

- **`<script setup>` only.** State via **Pinia**, events via **RxJS**.
- **Vue Router** uses `createWebHashHistory` (required for GH pages).
- **TypeScript `strict: true`** + `noUnusedLocals/Parameters: true`. Fix errors to pass `npm run build`.
- Local imports use explicit `.ts` extensions (`allowImportingTsExtensions: true`). Match this.
- Guard Tauri calls with `isTauri()` from `@tauri-apps/api/core` so the browser **simulator** does not throw. Do **not** use `window.__TAURI__` — unreliable in Tauri v2.

## CI & Deployment

- `deploy.yml` (on `main` push): Node 21, `npm run build`, deploys `/sim` to **GitHub Pages** (<https://larsbaunwall.github.io/Beolyd5>).
- `build-ui.yml` (manual): Cross-builds ARM64 `.deb` via DietPi runner (cargo tauri build).

## Gotchas

- **Package manager: npm-only.** `yarn.lock` has been removed. Use `npm install` everywhere.
- **WebKitGTK versions:** `raspberry-os-prereqs.sh` uses 4.1; CI uses `libwebkit2gtk-4.1-dev` (Tauri 2 requirement).
- **`tick()` active:** `src/ui/src/stores/ui.ts` calls `invoke('tick')` guarded by `isTauri()`.
- **Unified hardware events:** Rust emits **all** HW events (wheel and button) on `"hardwareEvent"`. There is no separate `buttonEvent`. Add button handling by filtering `kind === 'button'` in `events.ts`.
- **Linux HID access:** needs udev rule for `0cd4:1112` or `HidApi::open` throws "BS5 controller not found".

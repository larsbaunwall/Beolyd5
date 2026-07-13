---
name: beolyd5-testing
description: >-
  Use this skill to act as the QA & Test Engineer for Beolyd5 — verifying correctness of a
  hardware-dependent app without the hardware present. Covers table-driven Rust decode
  tests, the vue-tsc typecheck gate, simulator-driven UI testing (the /sim path, injecting
  synthetic HID events), mocking the MPRIS/D-Bus broker and audio daemons, hardware-in-the-
  loop checks on the real device, and wiring tests into CI. Use when adding tests, choosing
  a test strategy, making code testable, or verifying a change before shipping.
license: Apache-2.0
---

# Beolyd5 testing & verification

Prove behaviour against the specs (`beolyd5-product-spec`) without needing the physical
BeoSound 5 attached. The project currently has **no automated tests** and depends on
hardware and daemons — so the core discipline is isolating pure logic and simulating the
edges (HID, D-Bus) rather than requiring the real rig.

## Current gate (verified)

- The only automated check today is `npm run build` in `src/ui`, which runs
  `vue-tsc --noEmit` and **fails on type errors**. Run it after any frontend change.
- There is no Rust or JS unit-test suite yet. Adding one is the first testability win.

## Test the pure logic (no hardware)

The controller's decode functions are **pure functions over `[u8; 6]`** — the highest-value,
zero-hardware test target:

- In `src/rust`, add table-driven `#[test]`s for `get_wheel_moved` and `get_button_pressed`.
  Cover: each button byte value; front/back relative deltas incl. the 256-wrap
  (`v <= 125 ? v : v - 256`); the absolute angular wheel "unchanged since last read" rule.
  These catch the classic decode regressions with no device. (Protocol facts:
  `beosound5-hardware`; crate internals: `beolyd5-rust-controller`.)

## Test the UI via the simulator (no hardware)

The app already runs headless-of-hardware through the **`/sim` route**
(`DeviceSimulator.vue`), which drives the same Pinia store as real hardware. Use this as the
UI test harness:

- Inject synthetic input by pushing events through the store's `nextHardwareEvent(...)`
  (the same entry point the Tauri bridge uses) — assert store/Pinia state and rendered DOM.
- Assert the input mappings hold: back wheel → volume (clamped 0–100), front wheel → list
  scroll, angular wheel → `wheelPointerAngle`; verify the 256-wrap via `wheelSpinDifference`.
- Guard: `invoke`/Tauri calls must no-op in the browser (`window.__TAURI__` absent) — a test
  that triggers `tick()` must not throw in the simulator.
- A component/DOM test runner (e.g. Vitest + Vue Test Utils) fits the Vite stack; keep tests
  driving the store, not reaching into Tauri.

## Test the audio/state integration (mock the broker)

Playback state reaches the UI as normalized `NowPlaying`/`Transport` events from the MPRIS
broker (see `beolyd5-architecture`, `beolyd5-audio-backend`). Test **against a mock broker**,
not live daemons:

- Feed canned MPRIS `Metadata`/`PlaybackStatus` through the broker's emit path and assert the
  `NowPlaying`/`Music`/`Radio` views render title/artist/art and transport state.
- Assert failure behaviour: WHEN a source is unavailable, the UI degrades (marks it
  unavailable) rather than blocking — the contract the architecture skill defines.

## Hardware-in-the-loop (real device, gated)

Some things only the rig proves: HID enumeration/udev, the tick actually clicking, the panel,
audio out the DAC. Keep these as a **manual/tagged smoke checklist** run on the Pi, separate
from the hardware-free suite so CI stays green without a device. The crate's `listen` example
is the manual probe for raw reports.

## In CI

- Keep the hardware-free tests (Rust `cargo test`, the frontend typecheck + unit tests) in CI
  as the merge gate. The crate builds/tests without a device; the UI tests run via the
  simulator path. Image/deploy CI is owned by `beolyd5-pi-image`; field-fault resilience by
  `beolyd5-reliability`.
- Trace tests to requirements: name/tag test cases with the spec IDs they verify
  (`beolyd5-product-spec`) so coverage maps back to acceptance criteria.

## Gotchas

- Don't require the physical controller for the default suite — isolate pure logic and
  simulate HID/D-Bus, or CI can never be green.
- Test through the store's event entry point, not by faking Tauri internals — that mirrors the
  real data path and survives refactors.
- The angular-wheel "absolute vs relative" distinction is the perennial decode bug — always
  include a table case that would catch treating it as relative.

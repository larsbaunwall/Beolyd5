---
name: beolyd5-rust-controller
description: >-
  Use this skill to act as the Rust Systems Expert for the `beolyd5_controller` crate
  (src/rust) — the USB HID abstraction for the BeoSound 5 controller. Covers the read-loop
  threading model, event/callback design, robustness fixes (read timeouts, removing
  busy-loops), the blocking-read-vs-tick deadlock, and cross-compiling for aarch64.
  Use when acting as an autonomous agent editing the Rust crate, decoding HID reports,
  fixing thread panics/CPU spikes, or extending the public API.
license: Apache-2.0
---

# beolyd5_controller crate development

The Rust crate at [src/rust](../../../src/rust) is the crown jewel of this project: a
working, published (`beolyd5_controller` on crates.io) HID abstraction for the BS5
controller. Protect its correctness and API stability.

**Prerequisite:** the wire protocol (VID/PID, byte layout, command bytes, wheel decoding)
lives in the `beosound5-hardware` skill. Load that first for any protocol question.

## Layout

- `src/rust/src/lib.rs` — `Beolyd5Controller`: open device, spawn read thread, decode,
  dispatch callbacks, send commands (`tick`, `send`, `close`).
- `src/rust/src/types.rs` — `Button`, `Wheel`, `SystemEvent` (all `Serialize`).
- `src/rust/examples/listen.rs` — reference consumer.

## Current design (and its sharp edges)

The controller stores three callback vectors (`device`/`wheel`/`button`) as
`Vec<Arc<Mutex<dyn Fn(...) -> Result<(), Box<dyn Error + Send>> + Send>>>`. `open()`
clones the whole struct into an `Arc` and spawns a thread that blocks on
`HidDevice::read`.

### Known bugs — fix these deliberately, do not paper over them

1. **Blocking read holds the device `Mutex`, starving `tick()`/`send()`.**
   The read loop locks the device, calls the *blocking* `read`, and only unlocks after a
   report arrives. Any other thread calling `tick()`/`send()` blocks until the user next
   touches a wheel/button — so the click feedback is delayed or stalls.
   **Fix:** use `read_timeout(&mut buf, N_ms)`, or split writes onto their own path (e.g.
   a command channel drained inside the read loop, or a second handle). Never hold the
   read lock across a blocking read if writes share that lock.

2. **`device.read(...).unwrap()` in the loop panics the thread on any HID error.**
   A transient USB error kills the listener silently. **Fix:** match on the `Result`, log
   and continue on recoverable errors, break on fatal ones. Propagate via a diagnostics
   callback rather than `unwrap`/`expect`.

3. **`loop {}` busy-waits pin a CPU core at 100%.** Present in `src/ui/src-tauri/src/hw_controller.rs::init`
   (the `tauri::async_runtime::spawn` block ends with a bare `loop {}`). Note: `examples/listen.rs`
   does **not** do this — it uses `std::thread::sleep(Duration::from_millis(1))`, which is low-CPU
   but still blocks the thread unnecessarily. **Fix the Tauri host** by blocking on a channel
   `recv()` or `thread::park()`; the example is acceptable as-is.

4. **`Clone` resets `threads: Vec::new()` and deep-clones callback vectors every event.**
   `handle_device_event` clones the callback `Vec` on the hot path (allocations per
   report). **Preferred refactor:** hold shared state in a single `Arc<Inner>` and emit
   decoded events over a `crossbeam-channel`/`std::sync::mpsc` (or `tokio` channel)
   instead of the clone-self-into-Arc pattern. Keep the public API
   (`register_*_callback`, `open`, `tick`, `send`, `close`) source-compatible if possible.

## Decoding rules (must match the hardware skill exactly)

- Front (`event[0]`) and back (`event[1]`) wheels: relative; `0` = untouched.
- Angular (`event[2]`): absolute; untouched only when equal to the previous read.
- Convert relative values to signed deltas with the 256-wrap rule
  (`v <= 125 ? v : v - 256`).
- Buttons decode from `event[3]` per the hardware table.
- `tick()` sends `[0x00, 0x31]`; other LED/backlight commands via `send([u8; 2])`.

## Working on the crate

`cargo run --example listen` (in `src/rust`) is the fastest way to exercise the crate,
but it needs the **physical controller** connected and udev access to `0cd4:1112`
(otherwise it errors "BS5 controller not found"). The decode logic has effectively no
test coverage: `get_wheel_moved` and `get_button_pressed` are pure functions over
`[u8; 6]` — add table-driven tests for them (no hardware needed) when you change decoding.

### Cross-compiling for Raspberry Pi (aarch64)

`hidapi` links native libs, so cross-compilation needs a proper toolchain. Prefer
[`cross`](https://github.com/cross-rs/cross):

```bash
cargo install cross
cross build --release --target aarch64-unknown-linux-gnu
```

On the Pi, ensure `libudev`/`libusb` HID backends are present and a udev rule grants
access to `0cd4:1112`.

## API-stability rules

- The crate is **published**; treat public items as a semver contract. Additive changes
  (new methods, new `Wheel`/`Button` variants marked carefully) are minor; signature
  changes to existing callbacks are breaking — bump major and update `src/ui/src-tauri`
  which depends on it.
- Keep `Button`, `Wheel`, and `SystemEvent` all `Serialize + Deserialize` — the Tauri
  bridge serialises them across the JS boundary. (All three currently derive both.)

## Verify after a concurrency/refactor change

- [ ] `tick()` fires within ~1 frame of a selection change (the Mutex-starvation regression).
- [ ] Idle CPU stays low — no core pinned at 100% (the busy-loop regression).
- [ ] Unplug/replug the controller mid-run: the listener must not panic; surface an error/diagnostic instead.
- [ ] Wheel deltas stay correct across the 256 wrap (spin slowly past 0 in both directions).
- [ ] `src/ui/src-tauri` still builds against the crate (no unintended breaking API change).

## Gotchas

- Do **not** change the VID/PID or byte offsets without cross-checking the
  `beosound5-hardware` skill; they are physical facts, not preferences.
- `serde_json` is in `Cargo.toml` as a dependency but is **not used anywhere** in
  `src/rust/src/`. It is unused dead weight — safe to remove when touching the manifest.
- Errors surface to users as "BS5 controller not found" — usually a missing udev rule or
  the device not enumerated, not an actual code bug.
- After any refactor, verify `tick()` still fires promptly on selection change (this is
  the regression the Mutex bug causes).

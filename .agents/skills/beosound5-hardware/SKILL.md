---
name: beosound5-hardware
description: >-
  Use this skill to act as the Hardware Integration Expert for the Bang & Olufsen
  BeoSound 5 (BS5) and the Beolyd5 target hardware. Covers the USB HID protocol of the
  aluminium controller (wheels, buttons, LED/backlight/click commands), the legacy two-box
  product architecture, the "n.Music" MOTS UX, and the Raspberry Pi + HiFiBerry DAC2 HD +
  PowerLink runtime. Use when decoding HID reports, working on the physical signal path,
  or replicating original BeoSound 5 behaviour as an autonomous agent.
license: Apache-2.0
---

# BeoSound 5 hardware & domain reference

Source of truth for facts an agent cannot infer about the BS5 and the Beolyd5 rebuild.
Load before touching the HID protocol, the physical signal path, or any code reproducing
original BS5 behaviour.

## Product architecture (this is why the project exists)

The legacy product is **two boxes**, not one:

- **BeoSound 5 controller** — the aluminium unit with the 1024×768 screen and the
  physical wheel/button cluster. This is the input + display surface. Beolyd5 rebuilds
  the software that drives it.
- **BeoMaster 5** — a separate Windows XP–based music engine box that did playback,
  library, and streaming. Beolyd5 **replaces this entirely** with a Raspberry Pi.

When someone says "close the gap with the legacy BeoSound 5," they almost always mean
rebuilding BeoMaster 5 functionality (audio engine, library, streaming, MOTS), because
the controller half already works in this repo.

## The controller USB HID protocol

The controller enumerates as a USB HID device. These are the load-bearing constants —
get them wrong and nothing works:

- **Vendor ID:** `0x0cd4`
- **Product ID:** `0x1112`
- **Input report:** 6 bytes, read via `hidapi`.
- **Output/command report:** 2 bytes, written via `hidapi`.

### Input report byte layout (6 bytes)

| Byte | Meaning | Notes |
|------|---------|-------|
| `[0]` | Front (top) wheel position | Relative encoder. `0` = untouched this frame. |
| `[1]` | Back wheel position | Relative encoder. `0` = untouched this frame. |
| `[2]` | Angular wheel position | **Absolute** position. Untouched = unchanged vs. last read. |
| `[3]` | Button bitmask | See table below. |
| `[4]`,`[5]` | Unused / reserved | Not decoded today. |

Button byte (`[3]`) values:

| Value | Button |
|-------|--------|
| `0x00` | None |
| `0x20` | Left |
| `0x10` | Right |
| `0x40` | Go |
| `0x80` | Standby |

### Wheel decoding gotchas

- **Front and back wheels are relative encoders.** A non-zero value is a delta for that
  frame; `0` means "not touched." Detect movement by `value != 0`.
- **The angular wheel is absolute.** You cannot use `!= 0` to detect motion — it is only
  "untouched" when its value equals the previous read (`last_read[2] == event[2]`).
  Confusing these two models is the classic bug (see commit "Fix the angular wheel detection").
- **Relative values wrap at 256.** Convert to a signed delta:
  `delta = value <= 125 ? value : (value - 256)`. Values just under 256 are small
  negative movements, not huge positive jumps.

### Output command report byte layout (2 bytes)

Byte `[0]` is a bitfield controlling LED / backlight / click; byte `[1]` is a modifier.
Verified bits (from toresby/neomaster and this repo):

| Byte 0 | Effect |
|--------|--------|
| `0x80` | LED solid on |
| `0x40` | LCD backlight on |
| `0x20` | Unknown |
| `0x10` | LED blink |
| `0x0X` | Any low-nibble bit triggers the mechanical **click** ("tick") |

Common command values:

- `[0x00, 0x00]` — everything off
- `[0x40, 0x00]` — backlight on
- `[0xc0, 0x00]` — LED on (solid) + backlight
- `[0xd0, 0x00]` — LED blinking
- `[0x00, 0x31]` — the **tick / click sound** used for haptic-style feedback on selection

The click is the signature BS5 feedback. Fire it when the selection changes (menu item
crosses the pointer), not on every raw HID frame.

### Observing raw reports

To decode the unknown bytes (`[4]`, `[5]`) or verify any decoding change, run the crate's
`listen` example and watch `SystemEvent.event_bytes` — the 6 raw bytes of each frame.

## Signature UX to replicate

- **Arc-based navigation.** Menu items and lists are laid out along a circular arc whose
  centre sits off-screen, aligned to the physical wheel's position relative to the panel.
  The angular wheel moves a pointer along the arc; items near the pointer are "selected."
- **"n.Music" MOTS ("More Of The Same").** The defining BeoSound 5 feature: pick a track
  and the system builds an endless, evolving queue of similar music. Reproducing this
  (via library similarity / audio features / a recommendation source) is what makes a
  rebuild *feel* like a BS5 rather than a generic player.
- **Standby, Left/Right, Go** map to source/navigation actions, not media transport keys.

## Runtime hardware (Raspberry Pi target)

- **Compute:** Raspberry Pi (5 or 4). Runtime target for the shipping product.
- **DAC:** HiFiBerry **DAC2 HD**. This is an **I²S** HAT on the Pi's 40-pin GPIO header —
  **not USB**. Enable it with `dtoverlay=hifiberry-dacplushd` and disable onboard audio
  (`dtparam=audio=off`). The README block diagram labelling the HAT link as "USB" is
  misleading; audio comes off the GPIO header.
- **Speakers:** Bang & Olufsen BeoLab active speakers over **PowerLink**. The DAC's
  analog out needs a PowerLink (DIN/RJ45) converter/adaptor to reach BeoLab inputs.
- **Controller link:** USB (HID input + command output) between the BS5 controller and
  the Pi.

## Gotchas

- The **display connector is the biggest open hardware risk.** The BS5 aluminium unit's
  internal 1024×768 panel is **not** a plain HDMI monitor. Do not assume HDMI works;
  confirm the actual panel interface (toresby/neomaster documents the original wiring)
  before promising an integrated display.
- On Linux, opening the HID device may require a udev rule granting access to
  `0cd4:1112` (otherwise `HidApi::open` fails with a permission error, surfaced here as
  "BS5 controller not found").
- This project builds on and credits [toresby/neomaster](https://github.com/toresbe/neomaster);
  consult it for protocol edge cases and the display path.

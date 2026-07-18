---
name: beolyd5-architecture
description: >-
  Use this skill to act as the System Architect for Beolyd5 — the authority on how the
  independent processes (HID controller crate, Tauri host, Vue UI, MPRIS broker, audio
  daemons) compose into one coherent system. Owns the end-to-end data flow, the IPC/event
  contracts, the "control surface not audio engine" boundary, the process/threading model,
  component decoupling, and the v1→v2 evolution invariants. Use when defining interfaces
  between components, changing the event bridge, adding a source/domain, or reasoning about
  cross-cutting behaviour, versioning, and partial failure.
license: Apache-2.0
---

# Beolyd5 system architecture

Own the **contracts and data flow between components**, not the components themselves.
A Beolyd5 device is a distributed system in miniature — a kiosk UI, a HID daemon, an MPRIS
broker, and several audio daemons — so clarity here makes it debuggable and safely
upgradeable.

Component experts: `beolyd5-rust-controller`, `beolyd5-tauri-app`, `beolyd5-audio-backend`,
`beosound5-hardware`. Field-robustness: `beolyd5-reliability`.

## The one rule: control surface, not audio engine

**The UI never decodes or streams audio.** Playback lives in headless daemons; the UI
observes state and sends intents. This is the load-bearing boundary of the system — every
other skill defers here for it. If audio decoding/streaming logic appears in the Tauri host
or webview, the architecture is being violated.

## End-to-end data flow

```mermaid
flowchart LR
  HID[BS5 controller\nUSB HID 0cd4:1112] -->|6-byte reports| CRATE[beolyd5_controller\ncrate]
  CRATE -->|wheel/button callbacks| HOST[Tauri host\nhw_controller.rs]
  HOST -->|emit hardwareEvent| VUE[Vue UI\nPinia + RxJS]
  VUE -->|invoke: tick / transport intents| HOST
  HOST -->|tick command| CRATE
  CRATE -->|LED/click 2-byte cmd| HID

  subgraph Playback (daemons)
    MPD[MPD] & SPOT[spotifyd] & AIR[shairport-sync]
  end
  Playback -->|MPRIS signals over D-Bus| BROKER[MPRIS broker\nzbus in Tauri host]
  BROKER -->|emit nowPlaying / transport| VUE
  VUE -->|transport intents| BROKER
  BROKER -->|MPRIS methods| Playback
```

Two event streams converge on the UI: **hardware** (input) and **playback state**
(NowPlaying/transport). The UI is a pure function of those two; it originates intents but
holds no authoritative state.

## IPC contracts (treat every boundary as a versioned remote API)

Even though everything runs on one device, IPC is *not* an internal detail — it is a
contract between independently-failing peers. Design each boundary explicitly:

1. **HID → UI bridge (Tauri events).** Today: `hardwareEvent { kind, source, value }`
   (see `beolyd5-tauri-app` for the live schema). Treat the payload shape as a versioned
   interface: additive fields only; never repurpose `kind`/`source` meanings.
2. **Playback ↔ UI (MPRIS over D-Bus).** MPRIS is already a formal D-Bus contract
   (`org.mpris.MediaPlayer2.Player`). Define **one normalized model** the broker exposes to
   the UI — `NowPlaying { title, artist, album, artUrl, status, position }` and a
   `Transport` intent set — so the UI never sees MPD/spotifyd/shairport quirks. The broker
   owns the translation; sources are swappable behind it.
3. **Broker ownership.** Exactly one component (the broker, preferably in the Tauri host via
   `zbus`) is authoritative for aggregating players and picking the "active" source.
   The UI talks only to the broker, never to a daemon directly.

### Contract rules

- **Single owner per resource.** One process owns the HID device (the crate, surfaced by the
  host); one owns source-arbitration (the broker). No component opens HID or a daemon socket
  behind another's back.
- **Define failure behaviour explicitly.** For every boundary, decide what the UI does on
  timeout / bus error / missing peer: grey out the control, show a diagnostic, or retry.
  Undefined failure behaviour is the main source of "distributed weirdness."
- **Independent restart.** Any component may crash and restart without corrupting another's
  state. The UI must re-derive state from events after a broker or daemon restart — never
  assume a peer has been alive the whole session.
- **High-level messages only.** Buses carry logical events (knob-turn, source-changed,
  now-playing), not bulk data or low-level frames.

## Process & threading model

- **Crate read loop:** one blocking HID reader thread; writes (`tick`) must not be starved by
  it (see the `beolyd5-rust-controller` deadlock note). The host must not busy-loop.
- **Tauri host:** owns the HID handle *and* the D-Bus/zbus broker; forwards both event
  streams to the webview via `emit`. Async runtime, not spin loops.
- **Daemons:** independent systemd services, each targeting the same ALSA device.
- **UI:** single-threaded reactive; holds only derived/ephemeral state.

## Evolution invariants (v1 audio → v2 information hub)

The interaction grammar and the contracts are the product; protect them as you grow:

- **New domains are new Sources, not new subsystems.** Home automation, weather, intercom
  each plug in as another MPRIS-like *provider behind the broker* + another Source on the arc
  (see `beolyd5-ux-design`). The UI's Source→Dimension→Item contract does not change.
- **Version interfaces, don't mutate them.** If the bridge or broker schema must change
  incompatibly, introduce a v2 interface alongside v1 (e.g. `BeoController1` → `BeoController2`)
  so a UI and backend updated at different times still interoperate — essential once OTA can
  update parts independently.
- **Keep the crate decoupled.** `src/ui/src-tauri` depends on the *published* crate, not local
  `src/rust`. Cross-component changes need a publish or a `[patch]`; never assume a local edit
  propagates.

## Gotchas

- Don't let the UI become authoritative for playback state — it is a view; the broker/daemons
  are the source of truth.
- Don't add a second input channel parallel to `hardwareEvent`; extend the one contract.

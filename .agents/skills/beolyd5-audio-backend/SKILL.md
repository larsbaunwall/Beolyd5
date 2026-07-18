---
name: beolyd5-audio-backend
description: >-
  Use this skill to act as the Audio Integration Expert to build the software replacing
  the legacy BeoMaster 5. Covers running headless playback daemons on Raspberry Pi (MPD,
  spotifyd, shairport-sync), unifying transport/metadata via MPRIS over D-Bus, and
  reproducing the "n.Music" MOTS experience. Use as an autonomous agent when connecting
  audio sources, implementing Now Playing logic, volume/transport control, or designing
  music library browsing.
license: Apache-2.0
---

# Beolyd5 audio backend

Defines the playback engine the Tauri UI controls — the software replacement for the
legacy BeoMaster 5. Daemons do playback; the UI observes and commands. System-wide
broker-ownership contract: `beolyd5-architecture`.

Related: `beolyd5-tauri-app` (the client), `beolyd5-pi-image` (bakes these daemons into
`stage-audio`), `beosound5-hardware` (DAC2 HD / PowerLink output path, n.Music MOTS UX).

## Daemon stack (defaults)

Run as systemd services on Pi OS Lite, all outputting to the HiFiBerry DAC2 HD
(ALSA device `hw:0` after `dtoverlay=hifiberry-dacplushd`):

| Concern | Default daemon | Notes |
|---------|----------------|-------|
| Local library + web radio | **MPD** (Music Player Daemon) | Primary source; rich control protocol, exposes metadata + cover art. |
| Spotify | **spotifyd** (or librespot) | Spotify Connect (Premium). Prefer **spotifyd** built with `dbus_mpris` for MPRIS control; plain librespot exposes no MPRIS. |
| AirPlay | **shairport-sync** | AirPlay 2 receiver. |
| Multiroom (optional) | **snapcast** | Synchronised multi-device playback; add only if needed. |

Prefer this set over building anything custom. If you want a single higher-level backend
instead of wiring daemons individually, **Music Assistant** is a reasonable alternative to
evaluate — but MPD-centric is the simpler, more controllable default.

## Control & metadata: unify via MPRIS (D-Bus)

Do not write a bespoke client per source. Expose every player on **MPRIS2** over D-Bus and
drive one code path for Now Playing + transport:

- **MPD** → full MPRIS via a bridge (`mpd-mpris`, or `mpDris2`): metadata, artwork, and
  `Play/Pause/Next/Previous/SetPosition`. This is the richest control surface.
- **Spotify** → **spotifyd** with its `dbus_mpris` feature exposes MPRIS control; plain
  **librespot** does not (you'd need a custom event bridge).
- **shairport-sync** → a *partial, MPRIS-like* interface plus native D-Bus, with metadata
  and artwork. **Remote control works only for Classic AirPlay, not AirPlay 2** — for AP2
  you can show Now Playing but cannot transport-control the source.
- The broker subscribes to `org.mpris.MediaPlayer2.Player` (`Metadata`, `PlaybackStatus`,
  `Position`) and calls the transport methods where the source supports them.

This gives a single, source-agnostic "Now Playing" model — exactly what the empty
`NowPlaying.vue` / `Music.vue` / `Radio.vue` views need.

### Where to put the broker

Two options — pick per complexity:

- **In the Tauri host (Rust):** subscribe to D-Bus (`zbus`), forward normalized
  `NowPlaying`/`Transport` events to the webview via the same `emit` bridge used for
  hardware events. Keeps the webview free of D-Bus. **Preferred.**
- **In the frontend:** only if a local websocket/HTTP MPRIS gateway exists. Avoid pulling
  D-Bus into JS directly.

## Mapping controls to sources

- **Volume:** route the back-wheel volume to the ALSA/hardware mixer (or MPRIS `Volume`),
  and ultimately to BeoLab over PowerLink. Prefer a single system volume so all sources
  behave consistently.
- **Standby / Left / Right / Go:** source switch + navigate/select, not raw media keys
  (see hardware skill). Switching source = activating the corresponding MPRIS player.
- **Cover art:** MPD exposes embedded/`cover.*` art; surface the art URL through the
  broker for the arc/Now Playing views.

## n.Music MOTS ("More Of The Same")

The signature BeoSound 5 feature — an endless queue of similar music from a seed track.
Reproduce it on top of the library rather than faking it:

1. Start from a seed track's metadata/audio features.
2. Build a similarity queue. Options, cheapest first:
   - MPD "similar"/sticker-based or genre/tag clustering over the local library.
   - Audio-feature similarity (analyze the library once; nearest-neighbour on features).
   - An external recommendation source (e.g. Spotify recommendations when in Spotify mode).
3. Continuously top up the MPD queue as tracks play, so it never ends and evolves.

Treat MOTS as a queue-population strategy feeding MPD, not a separate player.

## Building it into the image

In `beolyd5-pi-image`'s `stage-audio`: install the daemons, drop systemd units + ALSA
config pointing at the DAC, enable the MPRIS bridges, and pre-create the MPD music dir.
Keep credentials (Spotify) out of the image — collect them in the first-boot wizard and
write to config at runtime.

## Gotchas

- All daemons must target the **same ALSA device** (the DAC2 HD); leaving onboard audio
  enabled or misrouting outputs is the usual "no sound" cause.
- shairport-sync metadata / D-Bus / MPRIS interfaces are **build-time options** — a
  stock package may omit them. Verify (or build with them enabled) before relying on them.
- Spotify Connect and AirPlay each grab exclusive audio; coordinate so switching sources
  pauses the others (drive this through the MPRIS broker).
- MPD needs the library indexed (`mpc update`) before browsing works.
- Web radio = just URLs in MPD playlists — no extra daemon needed.

---
name: beolyd5-reliability
description: >-
  Use this skill to act as the Reliability & Security Engineer for Beolyd5 — making the
  appliance survive years of unattended use in a cabinet. Covers power-loss safety
  (read-only / overlay rootfs, tmpfs, SD-card wear), watchdogs and crash self-healing,
  safe shutdown and factory reset, security hardening (kiosk lockdown, systemd sandboxing,
  SSH, firewall, secret storage), observability/diagnostics on a headless device, and
  network resilience / headless Wi-Fi onboarding. Use when designing for faults, hardening
  the device, adding logging/recovery, or making the appliance robust on real networks.
license: Apache-2.0
---

# Beolyd5 reliability, security & field operations

A demo "works on the bench." An **appliance** survives abrupt power cuts, flaky Wi-Fi, SD
wear, daemon hangs, and rare panics — unattended, inside a BeoSound cabinet, for years.
This skill owns everything about behaviour **when conditions are not ideal**. It is distinct
from `beolyd5-pi-image` (which owns *building/shipping* the image) — that skill wires the
services; this one makes the running system fault-tolerant and secure.

## Power-loss safety & flash wear (do this first)

An SD card mid-write during a power cut is the #1 cause of a bricked hobby appliance. The Pi
in a cabinet has no clean-shutdown guarantee, so design for sudden power removal:

- **Read-only (or read-mostly) root filesystem.** A read-only rootfs cannot be corrupted by
  power loss and makes updates/factory-reset trivial. Raspberry Pi OS ships an overlay option
  (`raspi-config` → Performance → Overlay File System), or use `overlayfs` manually with a
  RAM upper layer.
- **Split state into volatile vs persistent.** Mount `/run`, `/var/tmp`, caches, and logs on
  **tmpfs** (RAM). Put the few things that must persist (`/etc/beolyd5.conf`, Spotify token,
  MPD database/stickers) on a **separate writable data partition** (e.g. mounted at `/data`).
  Audit every writer before locking the root.
- **Minimise writes** to persistent flash (batch, avoid chatty logging to disk) to extend
  card life. Consider `log2ram` if keeping journald on disk.
- No hardware UPS is assumed — so the filesystem design *is* your power-loss protection.

## Watchdogs & self-healing

Detect and recover from hangs without a human:

- **Service-level (systemd):** for long-lived services (the Tauri host, the broker), use
  `Type=notify` + `WatchdogSec=` and emit `sd_notify(WATCHDOG=1)` heartbeats **only while
  healthy**; pair with `Restart=on-watchdog`/`on-failure`. This catches deadlocks/infinite
  loops, not just crashes.
- **Prevent restart storms:** set `StartLimitIntervalSec` + `StartLimitBurst`.
- **System-level (hardware):** the Pi has a hardware watchdog; enable it and let systemd kick
  it (`RuntimeWatchdogSec=` in `/etc/systemd/system.conf`) so the board resets if systemd
  itself wedges — the final safety net against kernel deadlock.
- Tune timeouts: long enough to tolerate heavy IO / network stalls, short enough to catch a
  real hang.

## Safe shutdown, recovery & factory reset

- Provide a **documented reset path** (hidden key sequence / long-press Standby) that restores
  a known-good state — restore golden defaults from the read-only root into `/data`, then
  reboot. With a stateless root this is trivial; without it, it's ad-hoc and fragile.
- Keep **golden config** in the read-only root; copy to `/data` on first boot / reset (mind
  directory structure so you don't leave dangling symlinks).
- After a panic/watchdog reset, the device should boot back to a usable state on its own
  (watchdog reboot + stateless root + persistent logs = self-recovery).

## Security hardening (LAN appliance holding credentials)

Threat model: it sits on the user's LAN with Wi-Fi + streaming credentials; the audio daemons
(MPD/spotifyd/shairport-sync) and the kiosk are attack surface.

- **Kiosk lockdown:** `cage` launching a single app already limits escape; ensure the webview
  cannot navigate to arbitrary URLs or open devtools in production, and cannot spawn a shell.
- **Least privilege via systemd sandboxing:** run each daemon as a dedicated non-root user
  with `ProtectSystem=strict`, `ProtectHome=true`, `NoNewPrivileges=true`, and minimal
  `CapabilityBoundingSet`. Limit filesystem/network reach per service.
- **SSH:** off by default (or key-only). **Regenerate SSH host keys on first boot** — flashed
  images share keys otherwise — and never ship a default password.
- **Network exposure:** bind services to `localhost` wherever the UI talks to them over local
  socket/D-Bus (MPD's TCP port, any HTTP). Default-deny inbound with `nftables`; open only
  what a companion app truly needs.
- **Secrets:** store tokens/Wi-Fi creds in a restricted dir on `/data` (root-only perms), never
  in the image and never in logs. Scrub provisioning/auth logs of plaintext credentials.
- **Updates are a trust boundary:** verify signatures + TLS on OTA (see `beolyd5-pi-image`).

## Observability on a headless device

You cannot attach a debugger to a cabinet. Design for post-hoc diagnosis:

- **journald** as the spine. On a read-only root use `Storage=volatile` (RAM) or point
  persistent logs at `/data`; cap size. Optionally forward to a LAN host with
  `systemd-journal-remote` (open only its port in the firewall).
- **Surface diagnostics to the UI.** The Tauri host already emits a `diagnostics` event —
  route device-open failures, D-Bus/broker errors, and network state to a hidden debug view so
  a user can report what's wrong without SSH.
- **Persist crash evidence** across the watchdog reboot (logs on `/data`) so a hang leaves a
  trail.

## Network resilience & headless onboarding

The device is headless-in-a-cabinet: the user has no keyboard.

- **Onboarding patterns (best → simplest):** temporary **AP mode** (device hosts its own
  Wi-Fi for setup) or **BLE provisioning** via a companion app; fallback = pre-seed
  `wpa_supplicant.conf` (with `country=`) on the boot partition. Show the device IP / a QR code
  on the panel on first boot. (Mechanics of writing config live in `beolyd5-pi-image`.)
- **Discovery:** advertise via **mDNS/Avahi** (`beolyd5.local`) — but treat it as unreliable
  (blocked on many corporate/complex LANs); always provide an IP/QR fallback.
- **Degraded behaviour:** daemons must tolerate Spotify/AirPlay endpoints dropping — bounded
  retries with backoff, no resource leaks, and **never block the whole UI** on one dead source.
  Surface "offline"/"reconnecting" states clearly and auto-recover when connectivity returns.

## Priority (essential vs later)

- **Essential v1:** read-only/overlay rootfs + tmpfs, watchdogs, safe shutdown/reset, basic
  hardening (non-root daemons, SSH off, localhost binding, host-key regen), journald + UI
  diagnostics, headless onboarding + IP/QR fallback.
- **Later:** remote log aggregation, RAUC A/B rollback (see `beolyd5-pi-image`), BLE
  provisioning, cert rotation.

## Gotchas

- Don't lock the root read-only before auditing writers — a daemon that needs to write will
  silently fail. Find every writable path first, then redirect to tmpfs or `/data`.
- A watchdog that heartbeats unconditionally (even when hung) defeats itself — beat only from
  a healthy code path.
- mDNS "works on my network" is a trap — always ship an IP/QR fallback.

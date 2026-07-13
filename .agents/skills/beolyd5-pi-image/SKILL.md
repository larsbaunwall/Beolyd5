---
name: beolyd5-pi-image
description: >-
  Use this skill to act as the Embedded Linux / DevOps Expert to package and deploy
  Beolyd5 as a Raspberry Pi appliance. Covers building a custom Pi OS image with pi-gen,
  the cage/Wayland single-app kiosk, config.txt for the HiFiBerry DAC2 HD and BS5 panel,
  systemd service wiring, aarch64 native-compilation in CI, and OTA update strategy.
  Use as an autonomous agent when asked how to install, package, ship, flash, configure
  Wayland/systemd, or auto-update the device.
license: Apache-2.0
---

# Packaging Beolyd5 as a Raspberry Pi appliance

Goal: a user downloads one `.img.xz`, flashes it with **Raspberry Pi Imager**, boots, and
lands directly in the BS5 arc UI with audio daemons running. This skill defines that
build and boot path.

Related: `beolyd5-tauri-app` (the app being deployed), `beolyd5-audio-backend` (the
daemons baked in), `beosound5-hardware` (DAC/panel/PowerLink facts). **Scope boundary:**
this skill builds & ships the image; *field robustness* of the running device (read-only
rootfs, watchdogs, hardening, network resilience) is owned by `beolyd5-reliability`.

## Decision: pi-gen, not HifiBerryOS

- **Do NOT fork HifiBerryOS / Beocreate.** It assumes a phone-as-remote, headless-renderer
  model with its own init/webserver/UI. Embedding a display-driven kiosk means fighting
  those assumptions and creating an un-upstreamable fork.
- **Use [pi-gen](https://github.com/RPi-Distro/pi-gen)** — the tool the Raspberry Pi
  Foundation uses to build Pi OS. Output is a standard flashable image; it's just shell
  stages layered on Pi OS **Lite** (64-bit). You own it end-to-end. Reuse HifiBerry's
  *ideas* (MPD, librespot, shairport-sync) without their image.
- Buildroot/Yocto are overkill for this project; only revisit if you need a
  sub-10-second, read-only, minimal appliance later.

## Image stage structure

Layer custom stages on top of upstream Pi OS Lite stages:

```
stage0-2        → Pi OS Lite (upstream, untouched)
stage-audio     → MPD, librespot, shairport-sync, HiFiBerry overlay  (audio-backend skill)
stage-kiosk     → cage + WebKitGTK deps, the beolyd5 app binary, systemd units
stage-config    → first-boot wizard, /etc/beolyd5.conf, udev rule for 0cd4:1112
```

Each custom stage is a directory with a `NN-run.sh` script and a `NN-packages` list;
pi-gen copies files from the stage's `files/` tree into the rootfs.

## Kiosk stack (cage on Wayland — not X11/matchbox)

The repo's `raspberry-os-*.sh` scripts use `startx` + matchbox. **Replace with `cage`**,
a purpose-built single-app Wayland kiosk compositor: faster boot, no cursor artifacts,
clean crash-restart under systemd. WebKitGTK (hence Tauri) runs fine under it.

Boot chain:

```
systemd → beolyd5-ui.service → cage → beolyd5-app (Tauri/WebKitGTK)
                    ↑ After= mpd, librespot, shairport-sync
```

Example unit (`stage-kiosk/files/etc/systemd/system/beolyd5-ui.service`):

```ini
[Unit]
Description=Beolyd5 kiosk UI
After=systemd-user-sessions.service mpd.service
Wants=mpd.service

[Service]
Type=simple
User=beolyd5
Environment=WEBKIT_DISABLE_COMPOSITING_MODE=1
Environment=WLR_LIBINPUT_NO_DEVICES=1
ExecStart=/usr/bin/cage -- /usr/bin/beolyd5-app
Restart=always
RestartSec=2
TTYPath=/dev/tty1

[Install]
WantedBy=multi-user.target
```

Enable at build time in the stage script:

```bash
on_chroot << EOF
systemctl enable beolyd5-ui.service
EOF
```

## config.txt (`/boot/firmware/config.txt`)

```ini
# HiFiBerry DAC2 HD (I2S HAT on GPIO — NOT USB)
dtoverlay=hifiberry-dacplushd
dtparam=audio=off          # disable onboard audio (conflicts with the HAT)

# Fast, clean boot to kiosk
disable_splash=1
boot_delay=0

# BS5 1024x768 panel: add the correct DSI/DPI/HDMI-timing overlay here.
# The panel is NOT guaranteed plain HDMI — confirm the connector first
# (see beosound5-hardware skill). Do not hardcode HDMI blindly.
```

## udev rule for the controller

`stage-config/files/etc/udev/rules.d/99-beolyd5.rules`:

```
SUBSYSTEM=="usb", ATTRS{idVendor}=="0cd4", ATTRS{idProduct}=="1112", MODE="0660", GROUP="plugdev"
```

Add the `beolyd5` user to `plugdev`, or `HidApi::open` fails ("BS5 controller not found").

## CI: build the app on arm64, then bake it into the image

The Tauri app links **WebKitGTK**, which makes `cross`-compiling to aarch64 painful (it
needs an arm64 sysroot with `libwebkit2gtk` and friends). Build **natively on an arm64
runner** (`ubuntu-24.04-arm`) instead — simpler, and it matches the Pi's libraries. (The
standalone Rust crate *can* use `cross`; the app effectively can't.)

```yaml
# .github/workflows/image.yml
jobs:
  build-app:
    runs-on: ubuntu-24.04-arm         # native aarch64 — no cross toolchain
    steps:
      - uses: actions/checkout@v4
      - run: sudo apt-get update && sudo apt-get install -y libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev
      - run: cd src/ui && npm ci && npm run build
      - run: cd src/ui/src-tauri && cargo build --release
      - uses: actions/upload-artifact@v4
        with: { name: beolyd5-app, path: src/ui/src-tauri/target/release/bs5-controller-ui }

  build-image:
    needs: build-app
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/download-artifact@v4
        with: { name: beolyd5-app, path: image/stage-kiosk/files/usr/bin/ }
      - uses: usimd/pi-gen-action@v1
        with:
          image-name: beolyd5
          stage-list: stage0 stage1 stage2 ./image/stage-audio ./image/stage-kiosk ./image/stage-config
```

Publish the resulting `.img.xz` to GitHub Releases so users can flash it via Raspberry Pi
Imager's "Use custom" (URL) option.

## OTA updates

Pick based on maturity:

- **Simple (start here):** ship a `.deb` on GitHub Releases; a systemd timer checks the
  latest release tag on boot and `apt-get install`s the new package. Low effort, good
  enough for a hobby appliance.
- **Robust (later):** [RAUC](https://rauc.io/) A/B partition images with automatic
  rollback if the new slot fails to boot. This is the appliance-grade path; adopt once the
  product stabilises.

## First-boot experience

A `stage-config` first-boot service should collect Wi-Fi credentials and Spotify login
(driven by the rotary dial where possible), write `/etc/beolyd5.conf`, then hand off to
`beolyd5-ui.service`. Keep it minimal — flash, boot, works.

## Gotchas

- Base on Pi OS **Lite 64-bit** (`arm64`); the app and daemons are aarch64.
- **Test the display path on real hardware early** — it's the top project risk (panel is
  not confirmed plain HDMI). Get pi-gen producing a bootable image *before* the app is
  feature-complete, so you can iterate on the panel/boot on-device.
- `dtparam=audio=off` is required — leaving onboard audio on collides with the HAT.
- Don't `cross`-compile the Tauri app — WebKitGTK linkage makes it fragile; build it
  natively on arm64 (an `ubuntu-24.04-arm` runner, or an arm64 container/QEMU).
- Keep custom stages idempotent and file-copy based; avoid interactive `apt` prompts
  (`DEBIAN_FRONTEND=noninteractive`).

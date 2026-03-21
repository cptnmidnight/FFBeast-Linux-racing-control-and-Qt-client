# FFBeast Linux Racing Control

Linux-first FFBeast wheel configuration for sim racing.

This project now uses:
- a Rust backend/service for device I/O, protocol parsing, telemetry, and wheel control
- a Qt desktop client implemented with Python/PySide6

The current product focus is narrow by design:
- driving simulators only
- FFBeast wheel setup first
- pedal and peripheral tuning only as advanced options
- no flight-sim UI or flight-specific settings

## Workspace Layout

- `apps/ffbeast-service`
  - Rust stdio JSON service process
- `apps/ffbeast-qt`
  - Python/PySide6 Qt client
- `libs/controller`
  - FFBeast HID/device access and protocol parsing
- `libs/input_manager`
  - input and profile-related backend logic
- `libs/backend_api`
  - shared request/response/event contract
- `docs/`
  - migration notes and project documentation

## Prerequisites

- Linux
- Rust stable with `cargo`
- Python 3
- PySide6
- `libudev` development files for the Rust HID stack

If you use Nix, the included shell is the simplest way to get a working dev environment:

```bash
nix-shell
```

## Quick Start

1. Enter the dev shell:

```bash
nix-shell
```

2. Verify the backend crates:

```bash
cargo test -p ffbeast-controller
cargo check -p ffbeast-backend-api -p ffbeast-service
```

3. Launch the Qt client:

```bash
python3 apps/ffbeast-qt/main.py
```

The Qt client starts `ffbeast-service` automatically by default.

## Run The Service Directly

Run the backend service on its own:

```bash
cargo run -p ffbeast-service
```

The service uses line-delimited JSON over stdio.

## Current UI Scope

The Qt client is structured around a sim-racing workflow:

- `Drive`
  - steering range
  - total force strength
  - soft stop settings
  - dampening settings
- `Wheel Setup`
  - force enablement
  - direction controls
  - power and braking limits
  - encoder and motor setup
- `Profiles`
  - save, load, and apply sim-specific profiles
- `Maintenance`
  - save settings
  - reset center
  - reboot
  - DFU
- `Advanced`
  - calibration
  - control-loop tuning
  - GPIO/ADC and other peripheral settings

## Useful Commands

Backend checks:

```bash
cargo test -p ffbeast-controller
cargo check -p ffbeast-backend-api -p ffbeast-service
```

Qt syntax check:

```bash
python3 -m py_compile \
  apps/ffbeast-qt/main.py \
  apps/ffbeast-qt/service_client.py \
  apps/ffbeast-qt/settings_tabs.py \
  apps/ffbeast-qt/profile_store.py
```

## Environment Overrides

Override the service command used by the Qt client:

```bash
FFBEAST_SERVICE_CMD="cargo run -p ffbeast-service --quiet" python3 apps/ffbeast-qt/main.py
```

Run the Qt client offscreen for a smoke test:

```bash
QT_QPA_PLATFORM=offscreen python3 apps/ffbeast-qt/main.py
```

## Troubleshooting

If `cargo test` or `cargo check` fails with `linker cc not found`:

```bash
nix-shell
```

If the wheel is not detected:
- confirm the device appears in `lsusb`
- confirm your user can access `/dev/hidraw*`
- check `udev` permissions
- try launching from the provided Nix shell first

If the Qt client starts but cannot talk to the backend:
- run `cargo run -p ffbeast-service` directly to confirm the service starts
- check the log pane in the Qt app
- verify `FFBEAST_SERVICE_CMD` is not pointing at an invalid command

## Status

This repo has moved away from the previous Tauri frontend. The active architecture is:

- Rust backend/service
- Python/PySide6 Qt client

The next validation step is real hardware testing against an FFBeast wheel on Linux.

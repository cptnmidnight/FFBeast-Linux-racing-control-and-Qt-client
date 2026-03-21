# FFBeast Qt Client

Qt desktop client for the FFBeast Linux racing-control stack.

This frontend is intentionally focused on sim racing:
- wheel setup first
- profile management for driving sims
- maintenance and firmware actions
- advanced peripheral tuning only when needed

## What It Does

- starts `ffbeast-service` as a subprocess by default
- talks to the service over line-delimited JSON on stdio
- loads handshake data from the Rust backend
- streams live telemetry
- edits wheel settings through the shared backend contract
- stores local sim profiles in `~/.config/ffbeast-qt/profiles.json`

## Main Pages

- `Drive`
  - motion range
  - total strength
  - soft stop controls
  - dampening controls
- `Wheel Setup`
  - force enablement
  - force and joystick direction
  - power limit
  - braking limit
  - encoder CPR
  - pole pairs
- `Profiles`
  - save, load, and apply sim-racing profiles
- `Maintenance`
  - save settings
  - reset center
  - reboot
  - enter DFU
- `Advanced`
  - calibration
  - control-loop tuning
  - GPIO/ADC/peripheral settings

## Run

From the repo root:

```bash
nix-shell
python3 apps/ffbeast-qt/main.py
```

If you already have Python, PySide6, and the Rust toolchain available, you can run it directly without `nix-shell`.

## Service Override

Override the backend command if needed:

```bash
FFBEAST_SERVICE_CMD="cargo run -p ffbeast-service --quiet" python3 apps/ffbeast-qt/main.py
```

## Smoke Test

Run a headless/offscreen launch check:

```bash
QT_QPA_PLATFORM=offscreen python3 apps/ffbeast-qt/main.py
```

## Common Issues

If the app opens but the service does not start:
- confirm `cargo` is installed
- run `cargo run -p ffbeast-service` manually
- check the Qt log panel for stderr output

If the service starts but the wheel does not connect:
- verify the device appears in `lsusb`
- check `hidraw` permissions
- confirm you are testing on Linux

If profile changes do not appear on hardware:
- load the wheel setup first
- apply page changes
- use `Save Settings` if you want values persisted to the device

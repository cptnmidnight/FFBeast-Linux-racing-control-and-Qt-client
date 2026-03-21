# AGENTS.md

## Project Direction

FFBeast is moving to a Linux-first architecture with:

- Rust backend for device I/O, protocol parsing, telemetry, and input/profile logic
- Qt frontend as the long-term desktop UI
- Python/PySide6 as the current Qt client implementation layer
- Tauri/Vue removed from the target architecture
- Driving simulators are the only supported product focus for the first build; do not add flight-sim UI or terminology

When making changes, prefer strengthening the Rust backend and shared API boundary. Do not invest in Tauri-specific code beyond what is strictly required to extract and delete it.

## Workspace Layout

- `libs/controller`
  - Core FFBeast hardware access
  - HID protocol constants, parsing, reconnect behavior, Linux device handling
- `libs/input_manager`
  - Input mapping, keyboard/profile logic
- `libs/backend_api`
  - Shared request/response/event contract intended for future Qt and service integration
- `apps/ffbeast-service`
  - Rust stdio JSON service process for frontend integration
- `apps/ffbeast-qt`
  - Python/PySide6 Qt client targeting `ffbeast-service`
  - Sim-racing-first UI for wheels and racing pedal setups
- `docs/qt-migration-plan.md`
  - Source of truth for the completed Tauri-to-Qt migration path and remaining cleanup

## Priorities

1. Keep device I/O trustworthy.
2. Keep backend interfaces UI-agnostic.
3. Prefer reusable Rust crates over app-local glue.
4. Avoid deepening Tauri coupling. Prefer extraction and deletion.
5. Optimize for the `ffbeast-service` + `ffbeast-qt` split already in progress.
6. Keep the UI focused on racing wheel and pedal workflows; hide peripheral or experimental controls behind `Advanced`.

## Build & Verification

Backend-focused checks:

- `cargo test -p ffbeast-controller`
- `cargo check -p ffbeast-backend-api`
- `cargo check -p ffbeast-service`

Qt client checks:

- `python3 -m py_compile apps/ffbeast-qt/main.py apps/ffbeast-qt/service_client.py apps/ffbeast-qt/settings_tabs.py apps/ffbeast-qt/profile_store.py`

Full workspace Rust check:

- `cargo check`

If Linux native dependencies are missing, use a temporary shell with the required packages. A minimal backend shell that worked here was:

```bash
nix-shell -p systemd pkg-config gcc
```

Tauri/UI builds currently require additional GTK/WebKit packages. Do not assume `cargo check` for the full workspace will pass outside a properly provisioned Linux desktop build environment.

## Coding Rules

- Rust first: backend logic belongs in Rust crates, not in the UI shell.
- Shared contracts belong in `libs/backend_api`.
- Qt UI logic belongs in `apps/ffbeast-qt`; hardware logic does not.
- Default visible UI should center on `Drive`, `Wheel Setup`, `Profiles`, and `Maintenance`.
- `GPIO`, `ADC`, calibration, and other peripheral tuning belong under `Advanced`, not the main flow.
- Do not surface flight-only wording or features in the Qt UI.
- Protocol parsing must be explicit and length-checked. Avoid layout-coupled unsafe parsing.
- Linux diagnostics should be actionable, especially around `hidraw`, `udev`, and missing native libraries.
- Keep code and technical docs in English.
- Use structured logging with `tracing`.
- Prefer simple, composable modules over large mixed-responsibility files.

## Tauri Removal Rules

- Do not add new Tauri-owned DTOs if they can live in `libs/backend_api`.
- Do not add new frontend polling paths for telemetry; backend-owned status events are the source of truth.
- Do not add new Tauri features.
- Only touch Tauri code to extract backend logic, preserve short-term buildability when necessary, or delete it.
- If a change would increase Tauri lock-in, do not make it.

## Migration Guidance

When adding features, ask:

1. Should this live in `libs/controller`?
2. Should this live in `libs/input_manager`?
3. Should this become part of `libs/backend_api`?
4. Should this live in `apps/ffbeast-service` as process orchestration?
5. Is this deprecated Tauri code that should instead be removed?

If the answer is "deprecated Tauri code", keep it isolated only long enough to replace or delete it.

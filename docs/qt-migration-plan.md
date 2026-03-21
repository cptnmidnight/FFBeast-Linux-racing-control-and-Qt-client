# Qt Migration Plan

## Goal

Retire the Tauri/Vue frontend and keep Rust as the long-lived backend for FFBeast device I/O, protocol parsing, telemetry, and input services.

## Keep

- `libs/controller`
  - Device I/O, HID protocol, evdev fallback, report parsing.
- `libs/input_manager`
  - Keyboard/profile logic that is independent of the UI shell.
- `libs/backend_api`
  - Shared frontend/backend contract for requests, responses, and telemetry.
- `docs/specifications`
  - Protocol and product notes worth carrying into the Qt app.

## Removed Layer

- `apps/ffbeast/src-tauri`
  - Removed after service extraction and Qt client scaffolding.
- `apps/ffbeast/src`
  - Removed with the Vue/Tauri frontend.

## Replace

- `apps/ffbeast-qt/`
  - Replaces the former Vue/Tauri frontend.
- `apps/ffbeast-service`
  - Replaces the former Tauri-owned backend adapter layer.

## Recommended New Shape

1. `libs/controller`
   - Pure hardware and protocol crate.
2. `libs/input_manager`
   - Pure input mapping crate.
3. `libs/backend_api`
   - Stable app-facing request/response/event contract.
4. `apps/ffbeast-service`
   - Rust process exposing the backend API over line-delimited JSON on stdio today.
   - Can later move to Unix socket or JSON-RPC without changing the core backend crates.
   - Owns live telemetry/event streaming via `ServiceMessage::Event`, so UI shells do not need to invent their own emitter contracts.
5. `apps/ffbeast-qt`
   - Qt desktop UI consuming the backend API.

## Deleted

These are no longer part of the active architecture:

- `apps/ffbeast/src`
- `apps/ffbeast/src-tauri`
- Tauri-specific build and release paths

## Immediate Next Steps

1. Expand `apps/ffbeast-service` from extracted service logic into a real transport process.
2. Expand the Qt client against the stdio service contract from `libs/backend_api`.
3. Harden the service protocol and cover more device workflows with tests.
4. Define and finish the remaining Qt screens against `libs/backend_api`:
   - connection state
   - handshake/settings load
   - live telemetry
   - effect settings
   - hardware settings

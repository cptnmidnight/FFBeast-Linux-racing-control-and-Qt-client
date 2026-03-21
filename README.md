# FFBeast Configurator

## Overview

This project is a centralized workspace for the **FFBeast Steering Wheel Configurator**. It now uses a Rust backend/service architecture with a Qt desktop client for Linux-focused wheel configuration and telemetry.

## Project Structure

The workspace is organized as follows:

- **`apps/`**: Contains the user-facing applications.
  - **`ffbeast-service/`**: Rust stdio service exposing the FFBeast backend contract.
  - **`ffbeast-qt/`**: Qt desktop client implemented with Python/PySide6.
- **`libs/`**: Shared Rust libraries.
  - **`controller/`**: A driver library for communicating with the FFBeast hardware via HID/USB.
  - **`backend_api/`**: Shared request/response/event contract for frontend integration.
- **`docs/`**: Project documentation and known issues.

## Prerequisites

- **Rust**: Latest stable version.
- **Python**: 3.13+ recommended.
- **PySide6**: for the Qt client.
- Linux native dependencies for HID access (`libudev`) and Qt runtime support.

## Setup & Build

1.  **Run backend verification**:
    ```bash
    cargo test -p ffbeast-controller
    cargo check -p ffbeast-backend-api -p ffbeast-service
    ```

2.  **Run the Qt client**:
    ```bash
    python3 apps/ffbeast-qt/main.py
    ```

3.  **Run the Rust service directly**:
    ```bash
    cargo run -p ffbeast-service
    ```

## Features

- **Linux-first Architecture**: Rust service + Qt client.
- **Modular Backend**: separate crates for hardware, input management, and API contract.
- **Qt Client Workflow**: handshake loading, telemetry, effects/hardware/GPIO/ADC editing, maintenance actions, and local profiles.
- **Real-time Monitoring**: High-frequency telemetry for wheel position, torque, and IO states.

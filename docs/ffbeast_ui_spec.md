# General Specification: FFBeast UI

**FFBeast UI** is the configuration and monitoring application for the "FFBeast" Force Feedback hardware. Built on the **Tauri v2** platform, it uses **Vue 3**, **TypeScript**, and **Vite** for a modern, reactive, and typed interface.

![Logo](../img/logo_app.png)

## Architecture

- **Frontend**: Reactive Single Page Application (SPA).
  - **Framework**: Vue 3 (Composition API).
  - **Language**: TypeScript (Strict Mode).
  - **State**: Pinia (Centralized Store `hardware.ts`).
  - **Components**: Modular and reusable SFC (Single File Components).
  - **Styling**: Native CSS following the project's design system.
  - **Internationalization**: Vue I18n with EN and PT-BR support.
  - **Communication**: IPC via Tauri Commands.

- **Backend (Tauri/Rust)**: 
  - Manages the native window and direct HID communication with the hardware.
  - Exposes commands such as `connect`, `get_hardware_settings`, `save_settings`.

## Key Features

### 1. Monitoring (Dashboard)
- Real-time steering wheel position visualization (60FPS SVG Animation).
- Live Torque/Force Feedback bar.
- Firmware and Connection Status.
- Visual grid Button Status.
- Analog Axis Monitoring (ADC).

### 2. Hardware Configuration
- **Motor and Encoder**: Definition of CPR, Power Limit, Poles.
- **Calibration**: Center Reset, ADC Calibration (Min/Max/Invert).
- **PID Tuning**: Adjustment of P and I gains for position/force control.

### 3. Force Effects (FFB)
- Global gain adjustment and specific effects (Spring, Damper, Friction, Inertia).
- "Motion Range" configuration (Rotation degrees).
- Force Reversal.

### 4. Inputs and GPIO
- Pin mapping for buttons and axes.
- Button matrix configuration.
- Expansion protocol selection (DirectHID, CAN-Bus, etc).

### 5. Tools and System
- **FFB Test**: Manual effect tests (Constant Force, Sine, etc).
- **Logs**: Internal console for command debugging.
- **Settings**: UI Customization (Accent color, language).
- **License**: Activation system and hardware ID.

## Folder Structure (src)

```
apps/ffbeast/src
├── assets/          # Static assets
├── components/      # Vue SFC Components
│   ├── common/      # Base UI (BaseCard, BaseSlider, etc)
│   ├── monitor/     # Monitoring widgets
│   └── tabs/        # Application main tabs
├── locales/         # Translations (en.json, pt-BR.json)
├── models/          # TypeScript Interfaces and Types (1 per file)
├── services/        # Tauri command abstraction
├── stores/          # Reactive state (Pinia)
└── styles/          # CSS variables and global styles
```

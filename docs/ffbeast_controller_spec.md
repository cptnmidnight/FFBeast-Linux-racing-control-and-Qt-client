# General Specification: FFBeast Controller (Backend Library)

The **FFBeast Controller** library (`libs/controller`) is the logical core written in **Rust** responsible for all low-level interaction with the wheel hardware. It abstracts the complexity of the USB/HID protocol and provides a safe API for the application layer (UI or Launcher).

## Objectives
- Ensure stable and thread-safe communication with the HID device.
- Process Input Reports at high frequency (>1kHz desirable).
- Manage persistent state and device settings.
- Provide "Trait-based" abstraction to support different hardware backends in the future.

## Key Features

### 1. HID Communication
- **Protocol**: Human Interface Device (USB HID).
- **Vendor ID**: `0x045E` (Example/Placeholder) or Configured.
- **Product ID**: FFBeast Specific.
- Uses the `hidapi` library for cross-platform access (Windows/Linux).

### 2. Sensor Reading
- **Read Loop**: Dedicated thread for polling HID Reports.
- **Processed Data**:
  - Encoder Position (16-bit or 32-bit).
  - Current Torque (Motor feedback).
  - Button State (up to 32/64 buttons).
  - Analog Inputs (ADC) - Pedals, Handbrake.

### 3. Cross-Platform Integration (Gamepad)
- Uses the `gilrs` crate to read inputs from generic controllers as fallback or complementation.
- Unification of controller events (Real wheel buttons + Virtual Gamepad).

### 4. Settings Management
Serializable data structures (interchangeable with Frontend):
- `HardwareSettings`: Physical and electrical limits.
- `EffectSettings`: FFB effect parameters.
- `GpioSettings`: Pin mapping.
- `AdcSettings`: Axis calibration.

The backend sends and receives this data via HID **Feature Reports**.

### 5. Force Control (Force Feedback)
- Sending direct force commands (`DirectX Spring`, `Constant`, etc.) to the Firmware.
- Support for dynamic update of gain parameters.

### 6. Licensing System
- Secure reading of processor ID (Unique ID via Feature Report).
- Validation of encrypted activation key.
- Secure storage of activation state.

## Code Structure

```
libs/controller/src
├── lib.rs              # Exports
├── hardware_service.rs # Main implementation (HardwareService struct)
├── key_map.rs          # vJoy/Keyboard key mapping
├── gamepad_reader.rs   # Gilrs integration
├── models/             # Data Structs (Settings, Status)
└── wheel_interface.rs  # WheelInterface Trait (API Contract)
```

## Logs and Debug
- Uses the `tracing` crate for structured logging.
- Low-level HID packet monitoring (raw bytes) for diagnostics.

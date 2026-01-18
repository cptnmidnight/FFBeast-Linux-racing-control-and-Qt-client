/// Cross-platform gamepad input reader
/// Simplified implementation that doesn't require Sync
use anyhow::{Result, anyhow};

#[derive(Debug, Clone, Default)]
pub struct GamepadState {
    /// Button states as a 32-bit bitmask (1 = pressed, 0 = released)
    pub buttons: u32,
    /// Analog axes values (0-4095 for 12-bit ADC compatibility)
    pub axes: [u16; 6],
}

/// Cross-platform gamepad input reader
/// Note: Removed Sync requirement because gilrs is not Sync on Windows
pub trait GamepadReader: Send {
    /// Read current gamepad state
    fn read_state(&mut self) -> Result<GamepadState>;

    /// Check if gamepad is connected
    fn is_connected(&self) -> bool;

    /// Get gamepad name/description
    fn name(&self) -> &str;
}

/// Gilrs-based gamepad reader (cross-platform)
pub struct GilrsGamepadReader {
    gilrs: gilrs::Gilrs,
    gamepad_id: Option<gilrs::GamepadId>,
    device_name: String,
    last_state: GamepadState,
}

impl GilrsGamepadReader {
    pub fn new() -> Result<Self> {
        tracing::info!("[GamepadReader] Initializing gilrs...");

        let gilrs =
            gilrs::Gilrs::new().map_err(|e| anyhow!("Failed to initialize gilrs: {}", e))?;

        // Find FFBeast controller or any connected gamepad
        let mut gamepad_id = None;
        let mut device_name = "No gamepad found".to_string();

        for (_id, gamepad) in gilrs.gamepads() {
            let name = gamepad.name();
            tracing::info!("[GamepadReader] Found gamepad: {} (ID: {:?})", name, _id);

            // Prefer FFBeast, but accept any gamepad
            tracing::info!(
                "[GamepadReader] Found device: {} (uuid: {:?})",
                name,
                gamepad.uuid()
            );

            // Prioritize FFBeast, but accept any stick/wheel if FFBeast is not explicitly named
            let is_likely_target = name.to_lowercase().contains("ffbeast")
                || name.to_lowercase().contains("wheel")
                || name.to_lowercase().contains("simucube")
                || name.to_lowercase().contains("vjoy");

            if is_likely_target || gamepad_id.is_none() {
                gamepad_id = Some(_id);
                device_name = name.to_string();

                if name.to_lowercase().contains("ffbeast") {
                    tracing::info!("[GamepadReader] Exact match found!");
                    break;
                }
            }
        }

        if let Some(id) = gamepad_id {
            tracing::info!(
                "[GamepadReader] Selected gamepad: {} (ID: {:?})",
                device_name,
                id
            );
        } else {
            tracing::warn!("[GamepadReader] No suitable gamepad found! Buttons will not work.");
        }

        Ok(Self {
            gilrs,
            gamepad_id,
            device_name,
            last_state: GamepadState::default(),
        })
    }

    /// Convert gilrs axis value (-1.0 to 1.0) to ADC value (0-4095)
    fn axis_to_adc(value: f32) -> u16 {
        let clamped = value.clamp(-1.0, 1.0);
        let normalized = (clamped + 1.0) / 2.0;
        (normalized * 4095.0) as u16
    }
}

impl GamepadReader for GilrsGamepadReader {
    fn read_state(&mut self) -> Result<GamepadState> {
        // Process events to update gamepad state
        while let Some(_event) = self.gilrs.next_event() {
            // Events are processed automatically by gilrs
        }

        let Some(gamepad_id) = self.gamepad_id else {
            return Ok(self.last_state.clone());
        };

        let gamepad = self.gilrs.gamepad(gamepad_id);
        let mut state = GamepadState::default();

        // Read buttons using is_pressed() directly
        use gilrs::Button;
        let button_map = [
            (Button::South, 0),
            (Button::East, 1),
            (Button::North, 2),
            (Button::West, 3),
            (Button::LeftTrigger, 4),
            (Button::RightTrigger, 5),
            (Button::LeftTrigger2, 6),
            (Button::RightTrigger2, 7),
            (Button::Select, 8),
            (Button::Start, 9),
            (Button::Mode, 10),
            (Button::LeftThumb, 11),
            (Button::RightThumb, 12),
            (Button::DPadUp, 13),
            (Button::DPadDown, 14),
            (Button::DPadLeft, 15),
            (Button::DPadRight, 16),
        ];

        for (button, bit_pos) in button_map.iter() {
            let pressed = gamepad.is_pressed(*button);
            if pressed {
                state.buttons |= 1 << bit_pos;
            }
        }

        // Read axes using axis_data()
        use gilrs::Axis;
        if let Some(axis) = gamepad.axis_data(Axis::LeftStickX) {
            state.axes[0] = Self::axis_to_adc(axis.value());
        }
        if let Some(axis) = gamepad.axis_data(Axis::LeftStickY) {
            state.axes[1] = Self::axis_to_adc(axis.value());
        }
        if let Some(axis) = gamepad.axis_data(Axis::RightStickX) {
            state.axes[2] = Self::axis_to_adc(axis.value());
        }
        if let Some(axis) = gamepad.axis_data(Axis::RightStickY) {
            state.axes[3] = Self::axis_to_adc(axis.value());
        }
        if let Some(axis) = gamepad.axis_data(Axis::LeftZ) {
            state.axes[4] = Self::axis_to_adc(axis.value());
        }
        if let Some(axis) = gamepad.axis_data(Axis::RightZ) {
            state.axes[5] = Self::axis_to_adc(axis.value());
        }

        // DEBUG: Trace inputs to identify mapping issues on Linux
        static mut LOG_SKIP: usize = 0;
        unsafe {
            if LOG_SKIP % 60 == 0 { // Log ~1Hz
                // Check all axes
                let active_axes: Vec<_> = (0..6).filter(|&i| state.axes[i] > 10 && state.axes[i] < 4085).collect();
                
                // Check all buttons
                let mut active_btns = Vec::new();
                for (btn, _) in button_map.iter() {
                    if gamepad.is_pressed(*btn) {
                        active_btns.push(format!("{:?}", btn));
                    }
                }

                if !active_axes.is_empty() || !active_btns.is_empty() {
                    tracing::info!("[GamepadReader] Active Inputs - Axes: {:?} (indices), Buttons: {:?}", active_axes, active_btns);
                }
            }
            LOG_SKIP += 1;
        }

        self.last_state = state.clone();
        Ok(state)
    }

    fn is_connected(&self) -> bool {
        self.gamepad_id.is_some()
    }

    fn name(&self) -> &str {
        &self.device_name
    }
}

/// Factory function to create platform-specific gamepad reader
pub fn create_gamepad_reader() -> Result<Box<dyn GamepadReader>> {
    GilrsGamepadReader::new().map(|r| Box::new(r) as Box<dyn GamepadReader>)
}

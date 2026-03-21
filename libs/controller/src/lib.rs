pub mod gamepad_reader;
pub mod hardware_service;
pub mod models;
pub mod protocol;
pub mod wheel_interface;
#[cfg(target_os = "windows")]
pub mod windows_gamepad;

pub use gamepad_reader::{GamepadReader, GamepadState, create_gamepad_reader};
pub use hardware_service::HardwareService;
pub use models::{
    AdcSettings, EffectSettings, GpioSettings, HardwareSettingId, HardwareSettings, WheelStatus,
};
pub use protocol::{
    CMD_DFU_MODE, CMD_FIRMWARE_ACTIVATION_DATA, CMD_OVERRIDE_DATA, CMD_REBOOT, CMD_RESET_CENTER,
    CMD_SAVE_SETTINGS, CMD_SETTINGS_FIELD_DATA, REPORT_ADC_SETTINGS_FEATURE,
    REPORT_EFFECT_SETTINGS_FEATURE, REPORT_FIRMWARE_LICENSE_FEATURE,
    REPORT_GENERIC_INPUT_OUTPUT, REPORT_GPIO_SETTINGS_FEATURE, REPORT_HARDWARE_SETTINGS_FEATURE,
    REPORT_JOYSTICK_INPUT, USB_VID, WHEEL_PID,
};
pub use wheel_interface::WheelInterface;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

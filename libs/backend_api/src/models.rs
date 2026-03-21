use ffbeast_controller::{
    AdcSettings, EffectSettings, GpioSettings, HardwareSettings, WheelStatus,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandshakeResponse {
    pub status: WheelStatus,
    pub fx: EffectSettings,
    pub hw: HardwareSettings,
    pub gpio: GpioSettings,
    pub adc: AdcSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppVersions {
    pub app: String,
    pub controller: String,
}

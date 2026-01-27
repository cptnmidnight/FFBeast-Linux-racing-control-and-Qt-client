use serde::{Deserialize, Serialize};
use ffbeast_controller::{AdcSettings, EffectSettings, GpioSettings, HardwareSettings, WheelStatus};

#[derive(Serialize, Deserialize)]
pub struct HandshakeResponse {
    pub status: WheelStatus,
    pub fx: EffectSettings,
    pub hw: HardwareSettings,
    pub gpio: GpioSettings,
    pub adc: AdcSettings,
}

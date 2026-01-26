use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum HardwareSettingId {
    // Effect Settings
    DirectXConstantDirection = 0,
    DirectXSpring = 1,
    DirectXConstant = 2,
    DirectXPeriodic = 3,
    TotalEffectStrength = 4,
    MotionRange = 5,
    SoftStopStrength = 6,
    SoftStopRange = 7,
    StaticDampening = 8,
    SoftStopDampening = 9,
    DynamicDampening = 10,
    IntegratedSpring = 43,

    // Hardware Limit Settings
    ForceEnabled = 11,
    DebugTorque = 12,
    AmplifierGain = 13,
    CalibrationMagnitude = 15,
    CalibrationSpeed = 16,
    PowerLimit = 17,
    BrakingLimit = 18,
    PositionSmoothing = 19,
    SpeedBufferSize = 20,
    EncoderDirection = 21,
    ForceInvert = 22,
    PolePairs = 23,
    EncoderCPR = 24,
    PGain = 25,
    IGain = 26,
    ResetCenterOnZ0 = 41,

    // Change modes
    ExtensionMode = 27,
    PinMode = 28,
    ButtonMode = 29,

    // SPI
    SpiMode = 30,
    SpiLatchMode = 31,
    SpiLatchDelay = 32,
    SpiClkPulseLength = 33,

    // ADC
    AdcMin = 34,
    AdcMax = 35,
    AdcButtonLow = 36,
    AdcButtonHigh = 37,
    AdcSmoothing = 38,
    AdcInvert = 39,

    // Fallback
    Unknown(u8),
}

impl From<u8> for HardwareSettingId {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::DirectXConstantDirection,
            1 => Self::DirectXSpring,
            2 => Self::DirectXConstant,
            3 => Self::DirectXPeriodic,
            4 => Self::TotalEffectStrength,
            5 => Self::MotionRange,
            6 => Self::SoftStopStrength,
            7 => Self::SoftStopRange,
            8 => Self::StaticDampening,
            9 => Self::SoftStopDampening,
            10 => Self::DynamicDampening,
            11 => Self::ForceEnabled,
            12 => Self::DebugTorque,
            13 => Self::AmplifierGain,
            15 => Self::CalibrationMagnitude,
            16 => Self::CalibrationSpeed,
            17 => Self::PowerLimit,
            18 => Self::BrakingLimit,
            19 => Self::PositionSmoothing,
            20 => Self::SpeedBufferSize,
            21 => Self::EncoderDirection,
            22 => Self::ForceInvert,
            23 => Self::PolePairs,
            24 => Self::EncoderCPR,
            25 => Self::PGain,
            26 => Self::IGain,
            41 => Self::ResetCenterOnZ0,
            43 => Self::IntegratedSpring,
            27 => Self::ExtensionMode,
            28 => Self::PinMode,
            29 => Self::ButtonMode,
            30 => Self::SpiMode,
            31 => Self::SpiLatchMode,
            32 => Self::SpiLatchDelay,
            33 => Self::SpiClkPulseLength,
            34 => Self::AdcMin,
            35 => Self::AdcMax,
            36 => Self::AdcButtonLow,
            37 => Self::AdcButtonHigh,
            38 => Self::AdcSmoothing,
            39 => Self::AdcInvert,
            v => Self::Unknown(v),
        }
    }
}

impl From<HardwareSettingId> for u8 {
    fn from(value: HardwareSettingId) -> Self {
        match value {
            HardwareSettingId::DirectXConstantDirection => 0,
            HardwareSettingId::DirectXSpring => 1,
            HardwareSettingId::DirectXConstant => 2,
            HardwareSettingId::DirectXPeriodic => 3,
            HardwareSettingId::TotalEffectStrength => 4,
            HardwareSettingId::MotionRange => 5,
            HardwareSettingId::SoftStopStrength => 6,
            HardwareSettingId::SoftStopRange => 7,
            HardwareSettingId::StaticDampening => 8,
            HardwareSettingId::SoftStopDampening => 9,
            HardwareSettingId::DynamicDampening => 10,
            HardwareSettingId::ForceEnabled => 11,
            HardwareSettingId::DebugTorque => 12,
            HardwareSettingId::AmplifierGain => 13,
            HardwareSettingId::CalibrationMagnitude => 15,
            HardwareSettingId::CalibrationSpeed => 16,
            HardwareSettingId::PowerLimit => 17,
            HardwareSettingId::BrakingLimit => 18,
            HardwareSettingId::PositionSmoothing => 19,
            HardwareSettingId::SpeedBufferSize => 20,
            HardwareSettingId::EncoderDirection => 21,
            HardwareSettingId::ForceInvert => 22,
            HardwareSettingId::PolePairs => 23,
            HardwareSettingId::EncoderCPR => 24,
            HardwareSettingId::PGain => 25,
            HardwareSettingId::IGain => 26,
            HardwareSettingId::ResetCenterOnZ0 => 41,
            HardwareSettingId::IntegratedSpring => 43,
            HardwareSettingId::ExtensionMode => 27,
            HardwareSettingId::PinMode => 28,
            HardwareSettingId::ButtonMode => 29,
            HardwareSettingId::SpiMode => 30,
            HardwareSettingId::SpiLatchMode => 31,
            HardwareSettingId::SpiLatchDelay => 32,
            HardwareSettingId::SpiClkPulseLength => 33,
            HardwareSettingId::AdcMin => 34,
            HardwareSettingId::AdcMax => 35,
            HardwareSettingId::AdcButtonLow => 36,
            HardwareSettingId::AdcButtonHigh => 37,
            HardwareSettingId::AdcSmoothing => 38,
            HardwareSettingId::AdcInvert => 39,
            HardwareSettingId::Unknown(v) => v,
        }
    }
}

export enum SettingsField {
    // Effects
    DirectXConstantDirection = 0,
    DirectXSpringStrength = 1,
    DirectXConstantStrength = 2,
    DirectXPeriodicStrength = 3,
    TotalEffectStrength = 4,
    MotionRange = 5,
    SoftStopStrength = 6,
    SoftStopRange = 7,
    StaticDampeningStrength = 8,
    SoftStopDampeningStrength = 9,
    DynamicDampeningStrength = 10,

    // Hardware
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
    ForceDirection = 22,
    PolePairs = 23,
    EncoderCPR = 24,
    PGain = 25,
    IGain = 26,

    // GPIO / Extensions
    ExtensionMode = 27,
    PinMode = 28,
    ButtonMode = 29,
    SpiMode = 30,
    SpiLatchMode = 31,
    SpiLatchDelay = 32,
    SpiClkPulseLength = 33,

    // ADC
    AdcMinDeadZone = 34,
    AdcMaxDeadZone = 35,
    AdcToButtonLow = 36,
    AdcToButtonHigh = 37,
    AdcSmoothing = 38,
    AdcInvert = 39,

    // Other
    ResetCenterOnZ0 = 41,
    IntegratedSpringStrength = 43,
}

export enum PinMode {
    None = 0,
    Gpio = 1,
    Analog = 2,
    SpiCs = 3,
    SpiSck = 4,
    SpiMiso = 5,
    EnableEffects = 6,
    CenterReset = 7,
    BrakingPwm = 8,
    EffectLed = 9,
    Reboot = 10,
}

export enum ButtonMode {
    None = 0,
    Normal = 1,
    Inverted = 2,
    Pulse = 3,
    PulseInverted = 4,
}

export enum ExtensionMode {
    None = 0,
    SpiCustom = 1,
    Spi3xCD4021 = 2,
    Spi3xSN74HC165 = 3,
    SpiTMStyle = 4,
    SpiVPC = 5,
}

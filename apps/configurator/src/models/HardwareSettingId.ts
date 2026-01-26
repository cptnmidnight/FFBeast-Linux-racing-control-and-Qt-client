export enum HardwareSettingId {
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
}

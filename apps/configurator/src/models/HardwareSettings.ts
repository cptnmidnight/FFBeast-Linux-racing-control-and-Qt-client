export interface HardwareSettings {
    power_limit: number;
    braking_limit: number;
    amplifier_gain: number;
    pole_pairs: number;
    calibration_speed: number;
    calibration_magnitude: number;
    force_enabled: number;
    encoder_direction: number;
    force_direction: number;
    debug_torque: number;
    proportional_gain: number;
    integral_gain: number;
}

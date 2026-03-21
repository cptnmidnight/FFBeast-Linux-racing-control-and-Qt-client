use crate::models::{
    AdcSettings, EffectSettings, GpioSettings, HardwareSettings, WheelStatus,
    wheel_status::FirmwareVersion,
};
use anyhow::{Result, anyhow, bail};

pub type LicenseInfo = ([u32; 3], [u32; 3]);

pub const USB_VID: u16 = 0x045B;
pub const WHEEL_PID: u16 = 0x59D7;

pub const REPORT_JOYSTICK_INPUT: u8 = 0x01;
pub const REPORT_HARDWARE_SETTINGS_FEATURE: u8 = 0x21;
pub const REPORT_EFFECT_SETTINGS_FEATURE: u8 = 0x22;
pub const REPORT_FIRMWARE_LICENSE_FEATURE: u8 = 0x25;
pub const REPORT_GPIO_SETTINGS_FEATURE: u8 = 0xA1;
pub const REPORT_ADC_SETTINGS_FEATURE: u8 = 0xA2;
pub const REPORT_GENERIC_INPUT_OUTPUT: u8 = 0xA3;

pub const CMD_REBOOT: u8 = 0x01;
pub const CMD_SAVE_SETTINGS: u8 = 0x02;
pub const CMD_DFU_MODE: u8 = 0x03;
pub const CMD_RESET_CENTER: u8 = 0x04;
pub const CMD_OVERRIDE_DATA: u8 = 0x10;
pub const CMD_FIRMWARE_ACTIVATION_DATA: u8 = 0x13;
pub const CMD_SETTINGS_FIELD_DATA: u8 = 0x14;

const EFFECT_SETTINGS_LEN: usize = 16;
const HARDWARE_SETTINGS_LEN: usize = 17;
const GPIO_SETTINGS_LEN: usize = 47;
const ADC_SETTINGS_LEN: usize = 24;
const LICENSE_LEN: usize = 29;
const STATUS_MIN_LEN: usize = 9;

fn payload_for_report<'a>(buf: &'a [u8], report_id: u8, min_len: usize) -> Result<&'a [u8]> {
    if buf.is_empty() {
        bail!("Empty report for 0x{report_id:02X}");
    }

    let payload = if buf[0] == report_id {
        &buf[1..]
    } else if buf.len() == min_len {
        buf
    } else {
        bail!(
            "Unexpected report ID for 0x{report_id:02X}: got 0x{:02X}",
            buf[0]
        );
    };

    if payload.len() < min_len {
        bail!(
            "Report 0x{report_id:02X} too short: expected at least {min_len} bytes, got {}",
            payload.len()
        );
    }

    Ok(payload)
}

fn read_u16(payload: &[u8], offset: usize) -> Result<u16> {
    let end = offset + 2;
    let bytes = payload
        .get(offset..end)
        .ok_or_else(|| anyhow!("Missing u16 at offset {offset}"))?;
    Ok(u16::from_le_bytes([bytes[0], bytes[1]]))
}

fn read_i16(payload: &[u8], offset: usize) -> Result<i16> {
    let end = offset + 2;
    let bytes = payload
        .get(offset..end)
        .ok_or_else(|| anyhow!("Missing i16 at offset {offset}"))?;
    Ok(i16::from_le_bytes([bytes[0], bytes[1]]))
}

fn read_u32(payload: &[u8], offset: usize) -> Result<u32> {
    let end = offset + 4;
    let bytes = payload
        .get(offset..end)
        .ok_or_else(|| anyhow!("Missing u32 at offset {offset}"))?;
    Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

pub fn parse_status_report(buf: &[u8]) -> Result<WheelStatus> {
    let payload = match buf.first().copied() {
        Some(REPORT_JOYSTICK_INPUT) | Some(REPORT_GENERIC_INPUT_OUTPUT) => &buf[1..],
        Some(other) => bail!("Unexpected status report ID: 0x{other:02X}"),
        None => bail!("Empty status report"),
    };

    if payload.len() < STATUS_MIN_LEN {
        bail!(
            "Status report too short: expected at least {STATUS_MIN_LEN} bytes, got {}",
            payload.len()
        );
    }

    let firmware = FirmwareVersion {
        release_type: payload[0],
        major: payload[1],
        minor: payload[2],
        patch: payload[3],
    };
    let is_registered = payload[4] != 0;
    let position = read_i16(payload, 5)?;
    let torque = if payload.len() >= 9 {
        read_i16(payload, 7)?
    } else {
        0
    };
    let buttons = if payload.len() >= 13 {
        read_u32(payload, 9)?
    } else {
        0
    };

    let mut adc = [0u16; 6];
    let adc_payload = payload.get(13..).unwrap_or(&[]);
    for (slot, chunk) in adc.iter_mut().zip(adc_payload.chunks_exact(2)) {
        *slot = u16::from_le_bytes([chunk[0], chunk[1]]);
    }

    Ok(WheelStatus {
        position,
        torque,
        buttons,
        adc,
        is_connected: true,
        firmware,
        is_registered,
        device_id: None,
        serial_key: None,
    })
}

pub fn parse_effect_settings_report(buf: &[u8]) -> Result<EffectSettings> {
    let payload = payload_for_report(buf, REPORT_EFFECT_SETTINGS_FEATURE, EFFECT_SETTINGS_LEN)?;
    Ok(EffectSettings {
        motion_range: read_u16(payload, 0)?,
        static_dampening_strength: read_u16(payload, 2)?,
        soft_stop_dampening_strength: read_u16(payload, 4)?,
        total_effect_strength: payload[6],
        integrated_spring_strength: payload[7],
        soft_stop_range: payload[8],
        soft_stop_strength: payload[9],
        direct_x_constant_direction: payload[10] as i8,
        direct_x_spring_strength: payload[11],
        direct_x_constant_strength: payload[12],
        direct_x_periodic_strength: payload[13],
        dynamic_dampening_strength: read_u16(payload, 14)?,
        ..EffectSettings::default()
    })
}

pub fn parse_hardware_settings_report(buf: &[u8]) -> Result<HardwareSettings> {
    let payload = payload_for_report(buf, REPORT_HARDWARE_SETTINGS_FEATURE, HARDWARE_SETTINGS_LEN)?;
    Ok(HardwareSettings {
        encoder_cpr: read_u16(payload, 0)?,
        integral_gain: read_u16(payload, 2)?,
        proportional_gain: payload[4],
        force_enabled: payload[5],
        debug_torque: payload[6],
        amplifier_gain: payload[7],
        calibration_magnitude: payload[8],
        calibration_speed: payload[9],
        power_limit: payload[10],
        braking_limit: payload[11],
        position_smoothing: payload[12],
        speed_buffer_size: payload[13],
        encoder_direction: payload[14] as i8,
        force_direction: payload.get(15).copied().unwrap_or(0) as i8,
        pole_pairs: payload.get(16).copied().unwrap_or(0),
        ..HardwareSettings::default()
    })
}

pub fn parse_gpio_settings_report(buf: &[u8]) -> Result<GpioSettings> {
    let payload = payload_for_report(buf, REPORT_GPIO_SETTINGS_FEATURE, GPIO_SETTINGS_LEN)?;
    let mut pin_mode = [0u8; 10];
    pin_mode.copy_from_slice(&payload[1..11]);

    let mut button_mode = [0u8; 32];
    button_mode.copy_from_slice(&payload[11..43]);

    Ok(GpioSettings {
        extension_mode: payload[0],
        pin_mode,
        button_mode,
        spi_mode: payload[43],
        spi_latch_mode: payload[44],
        spi_latch_delay: payload[45],
        spi_clk_pulse_length: payload[46],
        ..GpioSettings::default()
    })
}

pub fn parse_adc_settings_report(buf: &[u8]) -> Result<AdcSettings> {
    let payload = payload_for_report(buf, REPORT_ADC_SETTINGS_FEATURE, ADC_SETTINGS_LEN)?;
    let mut raxis_min = [0u16; 3];
    let mut raxis_max = [0u16; 3];
    let mut raxis_smoothing = [0u8; 3];
    let mut raxis_to_button_low = [0u8; 3];
    let mut raxis_to_button_high = [0u8; 3];
    let mut raxis_invert = [0u8; 3];

    for (idx, slot) in raxis_min.iter_mut().enumerate() {
        *slot = read_u16(payload, idx * 2)?;
    }
    for (idx, slot) in raxis_max.iter_mut().enumerate() {
        *slot = read_u16(payload, 6 + idx * 2)?;
    }
    raxis_smoothing.copy_from_slice(&payload[12..15]);
    raxis_to_button_low.copy_from_slice(&payload[15..18]);
    raxis_to_button_high.copy_from_slice(&payload[18..21]);
    raxis_invert.copy_from_slice(&payload[21..24]);

    Ok(AdcSettings {
        raxis_min,
        raxis_max,
        raxis_smoothing,
        raxis_to_button_low,
        raxis_to_button_high,
        raxis_invert,
        ..AdcSettings::default()
    })
}

pub fn parse_license_report(buf: &[u8]) -> Result<LicenseInfo> {
    let payload = payload_for_report(buf, REPORT_FIRMWARE_LICENSE_FEATURE, LICENSE_LEN)?;
    let mut serial = [0u32; 3];
    let mut id = [0u32; 3];

    for (idx, slot) in serial.iter_mut().enumerate() {
        *slot = read_u32(payload, 4 + idx * 4)?;
    }
    for (idx, slot) in id.iter_mut().enumerate() {
        *slot = read_u32(payload, 16 + idx * 4)?;
    }

    Ok((id, serial))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_status_report_fixture() {
        let report = [
            REPORT_GENERIC_INPUT_OUTPUT,
            2,
            1,
            4,
            9,
            1,
            0x34,
            0x12,
            0x78,
            0x56,
            0xEF,
            0xBE,
            0xAD,
            0xDE,
            0x00,
            0x01,
            0xFF,
            0x0F,
        ];

        let status = parse_status_report(&report).unwrap();
        assert_eq!(status.firmware.major, 1);
        assert_eq!(status.position, 0x1234);
        assert_eq!(status.torque, 0x5678);
        assert_eq!(status.buttons, 0xDEADBEEF);
        assert_eq!(status.adc[0], 256);
        assert_eq!(status.adc[1], 4095);
        assert!(status.is_registered);
    }

    #[test]
    fn rejects_short_effect_report() {
        let report = [REPORT_EFFECT_SETTINGS_FEATURE, 0x00, 0x01];
        assert!(parse_effect_settings_report(&report).is_err());
    }

    #[test]
    fn parses_effect_settings_fixture() {
        let report = [
            REPORT_EFFECT_SETTINGS_FEATURE,
            0x68,
            0x01,
            0x10,
            0x00,
            0x20,
            0x00,
            80,
            9,
            10,
            11,
            0xFF,
            12,
            13,
            14,
            0x34,
            0x12,
        ];

        let settings = parse_effect_settings_report(&report).unwrap();
        let motion_range = settings.motion_range;
        let dynamic_dampening_strength = settings.dynamic_dampening_strength;
        assert_eq!(motion_range, 360);
        assert_eq!(dynamic_dampening_strength, 0x1234);
    }

    #[test]
    fn parses_hardware_settings_fixture() {
        let report = [
            REPORT_HARDWARE_SETTINGS_FEATURE,
            0x00,
            0x10,
            0x20,
            0x00,
            1,
            1,
            2,
            3,
            4,
            5,
            6,
            7,
            8,
            9,
            0xFF,
            0xFE,
            12,
        ];

        let settings = parse_hardware_settings_report(&report).unwrap();
        let encoder_cpr = settings.encoder_cpr;
        let force_enabled = settings.force_enabled;
        let force_direction = settings.force_direction;
        let pole_pairs = settings.pole_pairs;
        let encoder_direction = settings.encoder_direction;
        assert_eq!(encoder_cpr, 4096);
        assert_eq!(force_enabled, 1);
        assert_eq!(force_direction, -2);
        assert_eq!(pole_pairs, 12);
        assert_eq!(encoder_direction, -1);
    }

    #[test]
    fn parses_gpio_settings_fixture() {
        let mut report = vec![REPORT_GPIO_SETTINGS_FEATURE, 7];
        report.extend(0u8..10);
        report.extend(10u8..42);
        report.extend([1, 2, 3, 4]);

        let settings = parse_gpio_settings_report(&report).unwrap();
        assert_eq!(settings.extension_mode, 7);
        assert_eq!(settings.pin_mode[0], 0);
        assert_eq!(settings.button_mode[0], 10);
        assert_eq!(settings.spi_clk_pulse_length, 4);
    }

    #[test]
    fn parses_adc_settings_fixture() {
        let report = [
            REPORT_ADC_SETTINGS_FEATURE,
            1,
            0,
            2,
            0,
            3,
            0,
            4,
            0,
            5,
            0,
            6,
            0,
            7,
            8,
            9,
            10,
            11,
            12,
            13,
            14,
            15,
            16,
            17,
            18,
        ];

        let settings = parse_adc_settings_report(&report).unwrap();
        let raxis_min = settings.raxis_min;
        let raxis_max = settings.raxis_max;
        let raxis_invert = settings.raxis_invert;
        assert_eq!(raxis_min, [1, 2, 3]);
        assert_eq!(raxis_max, [4, 5, 6]);
        assert_eq!(raxis_invert, [16, 17, 18]);
    }

    #[test]
    fn parses_license_fixture() {
        let report = [
            REPORT_FIRMWARE_LICENSE_FEATURE,
            2,
            1,
            5,
            9,
            1,
            0,
            0,
            0,
            2,
            0,
            0,
            0,
            3,
            0,
            0,
            0,
            4,
            0,
            0,
            0,
            5,
            0,
            0,
            0,
            6,
            0,
            0,
            0,
            1,
        ];

        let (id, serial) = parse_license_report(&report).unwrap();
        assert_eq!(serial, [1, 2, 3]);
        assert_eq!(id, [4, 5, 6]);
    }
}

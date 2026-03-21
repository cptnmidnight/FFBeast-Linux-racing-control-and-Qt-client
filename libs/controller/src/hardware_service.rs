use crate::gamepad_reader::{GamepadReader, create_gamepad_reader};
use crate::models::{
    AdcSettings, EffectSettings, GpioSettings, HardwareSettingId, HardwareSettings, WheelStatus,
};
use crate::protocol::{
    CMD_DFU_MODE, CMD_FIRMWARE_ACTIVATION_DATA, CMD_OVERRIDE_DATA, CMD_REBOOT, CMD_RESET_CENTER,
    CMD_SAVE_SETTINGS, CMD_SETTINGS_FIELD_DATA, LicenseInfo, REPORT_ADC_SETTINGS_FEATURE,
    REPORT_EFFECT_SETTINGS_FEATURE, REPORT_FIRMWARE_LICENSE_FEATURE, REPORT_GENERIC_INPUT_OUTPUT,
    REPORT_GPIO_SETTINGS_FEATURE, REPORT_HARDWARE_SETTINGS_FEATURE, USB_VID, WHEEL_PID,
    parse_adc_settings_report, parse_effect_settings_report, parse_gpio_settings_report,
    parse_hardware_settings_report, parse_license_report, parse_status_report,
};
use crate::wheel_interface::WheelInterface;
use anyhow::{Result, anyhow};
use hidapi::{HidApi, HidDevice};
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{Arc, Mutex};
use tracing::instrument;

pub struct HardwareService {
    device: Arc<Mutex<Option<HidDevice>>>,
    license_info: Arc<Mutex<Option<LicenseInfo>>>,
    gamepad_reader: Mutex<Option<Box<dyn GamepadReader>>>,
}

impl HardwareService {
    pub fn new() -> Self {
        Self {
            device: Arc::new(Mutex::new(None)),
            license_info: Arc::new(Mutex::new(None)),
            gamepad_reader: Mutex::new(None),
        }
    }

    #[instrument]
    pub fn is_connected_static() -> bool {
        match HidApi::new() {
            Ok(api) => api
                .device_list()
                .any(|d| d.vendor_id() == USB_VID && d.product_id() == WHEEL_PID),
            Err(_) => false,
        }
    }

    #[instrument(skip(self, data), fields(data_len = data.len()), level = "debug", err)]
    fn send_field(&self, field_id: u8, index: u8, data: Vec<u8>) -> Result<()> {
        let dev_lock = self.device.lock().unwrap();
        let dev = dev_lock
            .as_ref()
            .ok_or_else(|| anyhow!("Device not connected"))?;

        let mut buf = [0u8; 65];
        buf[0] = REPORT_GENERIC_INPUT_OUTPUT;
        buf[1] = CMD_SETTINGS_FIELD_DATA;
        buf[2] = field_id;
        buf[3] = index;

        for (i, &byte) in data.iter().enumerate() {
            if i + 4 < 65 {
                buf[i + 4] = byte;
            }
        }

        dev.write(&buf)
            .map_err(|e| anyhow!("Failed to send field {}: {}", field_id, e))?;
        Ok(())
    }

    pub fn activate_license(&self, key: [u32; 3]) -> Result<()> {
        let mut dev_lock = self.device.lock().unwrap();
        let dev = dev_lock
            .as_mut()
            .ok_or_else(|| anyhow!("Device not connected"))?;

        let mut buf = [0u8; 65];
        buf[0] = REPORT_GENERIC_INPUT_OUTPUT;
        buf[1] = CMD_FIRMWARE_ACTIVATION_DATA;

        // Key: 3x u32 (12 bytes)
        let mut offset = 2;
        for i in 0..3 {
            let bytes = key[i].to_le_bytes();
            buf[offset] = bytes[0];
            buf[offset + 1] = bytes[1];
            buf[offset + 2] = bytes[2];
            buf[offset + 3] = bytes[3];
            offset += 4;
        }

        dev.write(&buf)
            .map_err(|e| anyhow!("Failed to activate license: {}", e))?;

        // Clear license cache so it re-reads automatically
        if let Ok(mut lic) = self.license_info.lock() {
            *lic = None;
        }

        Ok(())
    }

    fn read_feature_report(&self, report_id: u8, buffer_size: usize) -> Result<Vec<u8>> {
        let dev_lock = self.device.lock().unwrap();
        let dev = dev_lock
            .as_ref()
            .ok_or_else(|| anyhow!("Device not connected"))?;

        // On Windows, the buffer MUST be size + 1 if report IDs are used.
        // On Linux (hidraw), it is usually the same.
        // We use the provided buffer_size but ensure it's at least enough for typical 64-byte reports.
        let mut buf = vec![0u8; buffer_size];
        buf[0] = report_id;

        match dev.get_feature_report(&mut buf) {
            Ok(res) if res > 0 => Ok(buf[..res].to_vec()),
            Ok(_) => Err(anyhow!("Empty feature report 0x{:02X}", report_id)),
            Err(e) => Err(anyhow!("Feature report 0x{:02X} failed: {}", report_id, e)),
        }
    }

    fn read_license_internal(&self) -> Result<LicenseInfo> {
        let buf = self.read_feature_report(REPORT_FIRMWARE_LICENSE_FEATURE, 65)?;
        let info = parse_license_report(&buf)?;
        tracing::info!(
            "License info fetched: ID={:?}, Serial={:?}",
            info.0,
            info.1
        );
        Ok(info)
    }

    #[cfg(target_os = "linux")]
    fn linux_access_hint() -> &'static str {
        " Check hidraw permissions and udev rules for the FFBeast VID/PID."
    }

    #[cfg(not(target_os = "linux"))]
    fn linux_access_hint() -> &'static str {
        ""
    }
}

impl WheelInterface for HardwareService {
    #[instrument(skip(self), level = "debug")]
    fn is_connected(&self) -> bool {
        let dev = self.device.lock().unwrap();
        dev.is_some()
    }

    #[instrument(skip(self), err, level = "debug")]
    fn connect(&self) -> Result<()> {
        // Avoid re-connecting if already active
        {
            let dev = self.device.lock().unwrap();
            if dev.is_some() {
                return Ok(());
            }
        }

        static PROCESS_COUNTER: AtomicU32 = AtomicU32::new(0);
        let count = PROCESS_COUNTER.fetch_add(1, Ordering::Relaxed);
        if count % 120 == 0 {
            tracing::info!(
                "Searching for FFBeast Controller (VID: {}, PID: {})...",
                USB_VID,
                WHEEL_PID
            );
        }
        let api = HidApi::new().map_err(|e| anyhow!("Failed to init HID: {}", e))?;

        let devices = api.device_list();
        let mut found_path = None;

        for d in devices {
            tracing::debug!(
                "Checking device: VID={:04x}, PID={:04x}, IF={}, Path={:?}",
                d.vendor_id(),
                d.product_id(),
                d.interface_number(),
                d.path()
            );

            if d.vendor_id() == USB_VID && d.product_id() == WHEEL_PID {
                // If we find multiple interfaces, we usually want interface 0 or the one with a path
                // On Windows, sometimes interface_number is -1.
                if found_path.is_none() || d.interface_number() == 0 {
                    found_path = Some(d.path().to_owned());
                }
            }
        }

        let path = found_path.ok_or_else(|| {
            let all_devs: Vec<String> = api
                .device_list()
                .map(|d| {
                    format!(
                        "[{:04x}:{:04x} if={}]",
                        d.vendor_id(),
                        d.product_id(),
                        d.interface_number()
                    )
                })
                .collect();
            anyhow!(
                "FFBeast Controller not found. Available devices: {:?}.{}",
                all_devs,
                Self::linux_access_hint()
            )
        })?;

        let hid_dev = api
            .open_path(&path)
            .map_err(|e| anyhow!("Failed to open device path {:?}: {}.{}", path, e, Self::linux_access_hint()))?;

        // Timeout for non-blocking reading
        hid_dev.set_blocking_mode(false).ok();

        let mut dev = self.device.lock().unwrap();
        *dev = Some(hid_dev);
        drop(dev); // Unlock before calling internal methods

        // Fetch license info on connect
        if let Ok((id, key)) = self.read_license_internal() {
            let mut lic = self.license_info.lock().unwrap();
            *lic = Some((id, key));
            tracing::info!("License info loaded.");
        }

        // Initialize gamepad reader for cross-platform button/axis reading
        match create_gamepad_reader() {
            Ok(reader) => {
                tracing::info!("Gamepad reader initialized: {}", reader.name());
                let mut gp = self.gamepad_reader.lock().unwrap();
                *gp = Some(reader);
            }
            Err(e) => {
                tracing::warn!(
                    "Failed to initialize gamepad reader: {}. Buttons/axes will only work via HID Report.",
                    e
                );
            }
        }

        tracing::info!("FFBeast Controller connected successfully!");
        Ok(())
    }

    fn read_status(&self) -> Result<WheelStatus> {
        let mut dev_lock = self.device.lock().unwrap();
        let dev = dev_lock
            .as_ref()
            .ok_or_else(|| anyhow!("Device not connected"))?;

        let mut buf = [0u8; 64];
        match dev.read_timeout(&mut buf, 10) {
            Ok(res) if res >= 8 => {
                static LOG_COUNT: AtomicU32 = AtomicU32::new(0);
                if LOG_COUNT.fetch_add(1, Ordering::Relaxed) < 5 {
                    tracing::debug!("HID Report [{}bytes]: {:02X?}", res, &buf[..res]);
                }

                let mut status = parse_status_report(&buf[..res])?;

                // ALWAYS log first 10 packets AND whenever buttons/ADC are non-zero
                tracing::trace!(
                    "HID Status [{}bytes]: pos={} torque={} buttons=0x{:08X} adc={:?}",
                    res,
                    status.position,
                    status.torque,
                    status.buttons,
                    status.adc
                );

                // Merge gamepad data (cross-platform button/axis reading)
                // Use gamepad as fallback when HID Report has no data
                if let Ok(mut gp_lock) = self.gamepad_reader.lock() {
                    if let Some(ref mut gamepad) = *gp_lock {
                        if let Ok(gamepad_state) = gamepad.read_state() {
                            // Merge buttons: Use HID if available, otherwise use gamepad
                            if status.buttons == 0 && gamepad_state.buttons != 0 {
                                status.buttons = gamepad_state.buttons;
                                tracing::debug!("Using gamepad buttons: 0x{:08X}", status.buttons);
                            }

                            // Merge ADC: Use HID if available, otherwise use gamepad
                            for i in 0..6 {
                                if status.adc[i] == 0 && gamepad_state.axes[i] > 0 {
                                    status.adc[i] = gamepad_state.axes[i];
                                }
                            }

                            // Log if gamepad provided data
                            if gamepad_state.buttons != 0
                                || gamepad_state.axes.iter().any(|&v| v > 0)
                            {
                                static PROCESS_COUNTER: AtomicU32 = AtomicU32::new(0);
                                let count = PROCESS_COUNTER.fetch_add(1, Ordering::Relaxed);
                                if count % 120 == 0 {
                                    tracing::trace!(
                                        "Gamepad data: buttons=0x{:08X} axes={:?}",
                                        gamepad_state.buttons,
                                        gamepad_state.axes
                                    );
                                }
                            }
                        }
                    }
                }

                // Lazy load license info if missing
                let (final_id, final_key) = {
                    let mut lic_cache = self.license_info.lock().unwrap();
                    if lic_cache.is_none() {
                        let mut feature_buf = [0u8; 65];
                        feature_buf[0] = REPORT_FIRMWARE_LICENSE_FEATURE;
                        if let Ok(count) = dev.get_feature_report(&mut feature_buf) {
                            match parse_license_report(&feature_buf[..count]) {
                                Ok(info) => {
                                    tracing::info!(
                                        "License info auto-loaded: ID={:?}, Serial={:?}",
                                        info.0,
                                        info.1
                                    );
                                    *lic_cache = Some(info);
                                }
                                Err(e) => tracing::warn!("Invalid license feature report: {e}"),
                            }
                        } else {
                            static LOGGED_FAIL: AtomicBool = AtomicBool::new(false);
                            if !LOGGED_FAIL.swap(true, Ordering::Relaxed) {
                                tracing::warn!(
                                    "License feature report (0x25) failed or not supported by device."
                                );
                            }
                        }
                    }

                    match *lic_cache {
                        Some((id, key)) => (Some(id), Some(key)),
                        None => (None, None),
                    }
                };

                status.device_id = final_id;
                status.serial_key = final_key;
                Ok(status)
            }
            Ok(0) => Err(anyhow!("Read Timeout")),
            Ok(res) => Err(anyhow!("Report too short: {} bytes", res)),
            Err(e) => {
                *dev_lock = None;
                tracing::warn!("HID Read Error: {}. Connection reset.", e);
                Err(anyhow!("HID Read Error: {}. Connection reset.", e))
            }
        }
    }

    #[instrument(skip(self), err)]
    fn read_effect_settings(&self) -> Result<EffectSettings> {
        let buf = self.read_feature_report(REPORT_EFFECT_SETTINGS_FEATURE, 65)?;
        parse_effect_settings_report(&buf)
    }

    #[instrument(skip(self), err)]
    fn read_hardware_settings(&self) -> Result<HardwareSettings> {
        let buf = self.read_feature_report(REPORT_HARDWARE_SETTINGS_FEATURE, 65)?;
        parse_hardware_settings_report(&buf)
    }

    #[instrument(skip(self), err)]
    fn read_gpio_settings(&self) -> Result<GpioSettings> {
        let buf = self.read_feature_report(REPORT_GPIO_SETTINGS_FEATURE, 65)?;
        parse_gpio_settings_report(&buf)
    }

    #[instrument(skip(self), err)]
    fn read_adc_settings(&self) -> Result<AdcSettings> {
        let buf = self.read_feature_report(REPORT_ADC_SETTINGS_FEATURE, 65)?;
        parse_adc_settings_report(&buf)
    }

    #[instrument(skip(self), err)]
    fn send_effect_settings(&self, settings: EffectSettings) -> Result<()> {
        // Basic range and global strength
        self.send_field(5, 0, (settings.motion_range as u16).to_le_bytes().to_vec())?;
        self.send_field(4, 0, vec![settings.total_effect_strength])?;

        // Dampening
        self.send_field(
            8,
            0,
            (settings.static_dampening_strength as u16)
                .to_le_bytes()
                .to_vec(),
        )?;
        self.send_field(
            10,
            0,
            (settings.dynamic_dampening_strength as u16)
                .to_le_bytes()
                .to_vec(),
        )?;

        // Soft Stop (Protocol IDs 6, 7, 9)
        self.send_field(6, 0, vec![settings.soft_stop_strength])?;
        self.send_field(7, 0, vec![settings.soft_stop_range])?;
        self.send_field(
            9,
            0,
            (settings.soft_stop_dampening_strength as u16)
                .to_le_bytes()
                .to_vec(),
        )?;

        // Internal Features
        self.send_field(43, 0, vec![settings.integrated_spring_strength])?;

        // DirectX Fields
        self.send_field(0, 0, vec![settings.direct_x_constant_direction as u8])?;
        self.send_field(1, 0, vec![settings.direct_x_spring_strength])?;
        self.send_field(2, 0, vec![settings.direct_x_constant_strength])?;
        self.send_field(3, 0, vec![settings.direct_x_periodic_strength])?;

        Ok(())
    }

    fn update_hardware_setting(
        &self,
        field_id: HardwareSettingId,
        index: u8,
        data: Vec<u8>,
    ) -> Result<()> {
        let id_u8: u8 = field_id.into();
        self.send_field(id_u8, index, data)
    }

    #[instrument(skip(self), err)]
    fn send_gpio_settings(&self, settings: GpioSettings) -> Result<()> {
        // Extension Mode (Field 27)
        self.send_field(27, 0, vec![settings.extension_mode])?;

        // Pin Modes (Field 28, 10 pins)
        for i in 0..10 {
            self.send_field(28, i as u8, vec![settings.pin_mode[i]])?;
        }

        // Button Modes (Field 29, 32 buttons)
        for i in 0..32 {
            self.send_field(29, i as u8, vec![settings.button_mode[i]])?;
        }

        // SPI Settings (Protocol IDs 30, 31, 32, 33)
        self.send_field(30, 0, vec![settings.spi_mode])?;
        self.send_field(31, 0, vec![settings.spi_latch_mode])?;
        self.send_field(32, 0, vec![settings.spi_latch_delay])?;
        self.send_field(33, 0, vec![settings.spi_clk_pulse_length])?;

        Ok(())
    }

    #[instrument(skip(self), err)]
    fn send_adc_settings(&self, settings: AdcSettings) -> Result<()> {
        for i in 0..3 {
            let idx = i as u8;
            self.send_field(
                34,
                idx,
                (settings.raxis_min[i] as u16).to_le_bytes().to_vec(),
            )?;
            self.send_field(
                35,
                idx,
                (settings.raxis_max[i] as u16).to_le_bytes().to_vec(),
            )?;
            self.send_field(36, idx, vec![settings.raxis_to_button_low[i]])?;
            self.send_field(37, idx, vec![settings.raxis_to_button_high[i]])?;
            self.send_field(38, idx, vec![settings.raxis_smoothing[i]])?;
            self.send_field(39, idx, vec![settings.raxis_invert[i]])?;
        }
        Ok(())
    }

    #[instrument(skip(self), err)]
    fn send_reset_center(&self) -> Result<()> {
        let dev_lock = self.device.lock().unwrap();
        let dev = dev_lock
            .as_ref()
            .ok_or_else(|| anyhow!("Device not connected"))?;

        let mut buf = [0u8; 65];
        buf[0] = REPORT_GENERIC_INPUT_OUTPUT;
        buf[1] = CMD_RESET_CENTER;

        dev.write(&buf)?;
        Ok(())
    }

    #[instrument(skip(self), err)]
    fn save_settings(&self) -> Result<()> {
        let dev_lock = self.device.lock().unwrap();
        let dev = dev_lock
            .as_ref()
            .ok_or_else(|| anyhow!("Device not connected"))?;

        let mut buf = [0u8; 65];
        buf[0] = REPORT_GENERIC_INPUT_OUTPUT;
        buf[1] = CMD_SAVE_SETTINGS;

        dev.write(&buf)
            .map_err(|e| anyhow!("Failed to save settings: {}", e))?;
        Ok(())
    }

    #[instrument(skip(self), err)]
    fn reboot_device(&self) -> Result<()> {
        let dev_lock = self.device.lock().unwrap();
        let dev = dev_lock
            .as_ref()
            .ok_or_else(|| anyhow!("Device not connected"))?;

        let mut buf = [0u8; 65];
        buf[0] = REPORT_GENERIC_INPUT_OUTPUT;
        buf[1] = CMD_REBOOT;

        dev.write(&buf)
            .map_err(|e| anyhow!("Failed to reboot device: {}", e))?;
        Ok(())
    }

    #[instrument(skip(self), err)]
    fn switch_to_dfu(&self) -> Result<()> {
        let dev_lock = self.device.lock().unwrap();
        let dev = dev_lock
            .as_ref()
            .ok_or_else(|| anyhow!("Device not connected"))?;

        let mut buf = [0u8; 65];
        buf[0] = REPORT_GENERIC_INPUT_OUTPUT;
        buf[1] = CMD_DFU_MODE;

        dev.write(&buf)
            .map_err(|e| anyhow!("Failed to switch to DFU mode: {}", e))?;
        Ok(())
    }

    #[instrument(skip(self), err)]
    fn send_direct_control(&self, force_type: u8, value: i16) -> Result<()> {
        let dev_lock = self.device.lock().unwrap();
        let dev = dev_lock
            .as_ref()
            .ok_or_else(|| anyhow!("Device not connected"))?;

        let mut buf = [0u8; 65];
        buf[0] = REPORT_GENERIC_INPUT_OUTPUT;
        buf[1] = CMD_OVERRIDE_DATA;

        // DirectControlTypeDef: Spring(i16), Constant(i16), Periodic(i16), ForceDrop(u8)
        let bytes = value.to_le_bytes();
        match force_type {
            1 => {
                // Constant
                buf[4] = bytes[0];
                buf[5] = bytes[1];
            }
            2 => {
                // Sine/Periodic
                buf[6] = bytes[0];
                buf[7] = bytes[1];
            }
            3 => {
                // Spring
                buf[2] = bytes[0];
                buf[3] = bytes[1];
            }
            _ => {}
        }

        dev.write(&buf)
            .map_err(|e| anyhow!("Failed to send direct control: {}", e))?;
        Ok(())
    }
}

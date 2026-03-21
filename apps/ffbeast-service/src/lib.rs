use anyhow::{Result, anyhow};
use ffbeast_backend_api::{
    AppVersions, BackendRequest, BackendResponse, HandshakeResponse, ServiceRequest,
    ServiceResponse,
};
use ffbeast_controller::{
    AdcSettings, EffectSettings, GpioSettings, HardwareService, HardwareSettingId,
    HardwareSettings, WheelInterface, WheelStatus,
};
use sodevs_input_manager::{InputManager, KeyMapping, ManagerConfig};

pub struct FFBeastService {
    hardware: HardwareService,
    input_manager: InputManager,
}

impl FFBeastService {
    pub fn new() -> Self {
        Self {
            hardware: HardwareService::new(),
            input_manager: InputManager::new(),
        }
    }

    pub fn check_hardware(&self) -> bool {
        self.hardware.is_connected()
    }

    pub fn connect_hardware(&self) -> Result<()> {
        self.hardware.connect()
    }

    pub fn reboot_device(&self) -> Result<()> {
        self.hardware.reboot_device()
    }

    pub fn reset_center(&self) -> Result<()> {
        self.hardware.send_reset_center()
    }

    pub fn save_settings(&self) -> Result<()> {
        self.hardware.save_settings()
    }

    pub fn get_effect_settings(&self) -> Result<EffectSettings> {
        self.hardware.read_effect_settings()
    }

    pub fn get_hardware_settings(&self) -> Result<HardwareSettings> {
        self.hardware.read_hardware_settings()
    }

    pub fn get_gpio_settings(&self) -> Result<GpioSettings> {
        self.hardware.read_gpio_settings()
    }

    pub fn get_adc_settings(&self) -> Result<AdcSettings> {
        self.hardware.read_adc_settings()
    }

    pub fn update_effect_settings(&self, settings: EffectSettings) -> Result<()> {
        self.hardware.send_effect_settings(settings)
    }

    pub fn update_hardware_field(&self, field_id: u8, index: u8, data: Vec<u8>) -> Result<()> {
        self.hardware
            .update_hardware_setting(HardwareSettingId::from(field_id), index, data)
    }

    pub fn update_gpio_settings(&self, settings: GpioSettings) -> Result<()> {
        self.hardware.send_gpio_settings(settings)
    }

    pub fn update_adc_settings(&self, settings: AdcSettings) -> Result<()> {
        self.hardware.send_adc_settings(settings)
    }

    pub fn get_handshake(&self) -> Result<HandshakeResponse> {
        if !self.hardware.is_connected() {
            self.hardware.connect()?;
        }

        Ok(HandshakeResponse {
            status: self.hardware.read_status()?,
            fx: self.hardware.read_effect_settings()?,
            hw: self.hardware.read_hardware_settings()?,
            gpio: self.hardware.read_gpio_settings()?,
            adc: self.hardware.read_adc_settings()?,
        })
    }

    pub fn get_status(&self) -> Result<WheelStatus> {
        self.hardware.read_status()
    }

    pub fn poll_telemetry(&self) -> Result<WheelStatus> {
        let status = self.hardware.read_status()?;
        self.input_manager.process(&status);
        Ok(status)
    }

    pub fn activate_license(&self, key_str: &str) -> Result<()> {
        let clean = key_str.trim().replace('-', "").replace(' ', "");
        if clean.len() != 24 {
            return Err(anyhow!(
                "Invalid key length. Expected 24 hex characters."
            ));
        }

        let mut key = [0u32; 3];
        for i in 0..3 {
            let chunk = &clean[i * 8..(i + 1) * 8];
            key[i] = u32::from_str_radix(chunk, 16)
                .map_err(|e| anyhow!("Invalid hex: {e}"))?;
        }

        self.hardware.activate_license(key)
    }

    pub fn switch_to_dfu(&self) -> Result<()> {
        self.hardware.switch_to_dfu()
    }

    pub fn send_direct_control(&self, force_type: u8, value: i16) -> Result<()> {
        self.hardware.send_direct_control(force_type, value)
    }

    pub fn get_keyboard_service_active(&self) -> bool {
        self.input_manager.is_active()
    }

    pub fn set_keyboard_service_active(&self, enabled: bool) -> Result<()> {
        if enabled && !self.hardware.is_connected() {
            return Err(anyhow!(
                "Cannot start mapping service: Hardware not connected"
            ));
        }
        self.input_manager.set_active(enabled);
        Ok(())
    }

    pub fn set_keyboard_mapping(&self, mappings: Vec<KeyMapping>) -> Result<()> {
        self.input_manager.set_mappings(mappings);
        Ok(())
    }

    pub fn get_keyboard_config(&self) -> ManagerConfig {
        self.input_manager.get_config()
    }

    pub fn set_keyboard_config(&self, config: ManagerConfig) -> Result<()> {
        self.input_manager.set_config(config);
        Ok(())
    }

    pub fn get_versions(&self, app_version: &str) -> AppVersions {
        AppVersions {
            app: app_version.to_string(),
            controller: ffbeast_controller::VERSION.to_string(),
        }
    }

    pub fn handle_request(&self, app_version: &str, request: BackendRequest) -> Result<BackendResponse> {
        match request {
            BackendRequest::CheckHardware => Ok(BackendResponse::Bool(self.check_hardware())),
            BackendRequest::ConnectHardware => {
                self.connect_hardware()?;
                Ok(BackendResponse::Ok)
            }
            BackendRequest::GetHandshake => Ok(BackendResponse::Handshake(self.get_handshake()?)),
            BackendRequest::GetStatus => Ok(BackendResponse::Status(self.get_status()?)),
            BackendRequest::GetEffectSettings => {
                Ok(BackendResponse::EffectSettings(self.get_effect_settings()?))
            }
            BackendRequest::GetHardwareSettings => {
                Ok(BackendResponse::HardwareSettings(self.get_hardware_settings()?))
            }
            BackendRequest::GetGpioSettings => {
                Ok(BackendResponse::GpioSettings(self.get_gpio_settings()?))
            }
            BackendRequest::GetAdcSettings => {
                Ok(BackendResponse::AdcSettings(self.get_adc_settings()?))
            }
            BackendRequest::UpdateEffectSettings(settings) => {
                self.update_effect_settings(settings)?;
                Ok(BackendResponse::Ok)
            }
            BackendRequest::UpdateHardwareField(update) => {
                self.update_hardware_field(update.field_id, update.index, update.data)?;
                Ok(BackendResponse::Ok)
            }
            BackendRequest::UpdateGpioSettings(settings) => {
                self.update_gpio_settings(settings)?;
                Ok(BackendResponse::Ok)
            }
            BackendRequest::UpdateAdcSettings(settings) => {
                self.update_adc_settings(settings)?;
                Ok(BackendResponse::Ok)
            }
            BackendRequest::RebootDevice => {
                self.reboot_device()?;
                Ok(BackendResponse::Ok)
            }
            BackendRequest::ResetCenter => {
                self.reset_center()?;
                Ok(BackendResponse::Ok)
            }
            BackendRequest::SaveSettings => {
                self.save_settings()?;
                Ok(BackendResponse::Ok)
            }
            BackendRequest::SwitchToDfu => {
                self.switch_to_dfu()?;
                Ok(BackendResponse::Ok)
            }
            BackendRequest::ActivateLicense(request) => {
                self.activate_license(&request.key)?;
                Ok(BackendResponse::Ok)
            }
            BackendRequest::SendDirectControl(request) => {
                self.send_direct_control(request.force_type, request.value)?;
                Ok(BackendResponse::Ok)
            }
            BackendRequest::SetKeyboardServiceActive(enabled) => {
                self.set_keyboard_service_active(enabled)?;
                Ok(BackendResponse::Ok)
            }
            BackendRequest::GetKeyboardServiceActive => {
                Ok(BackendResponse::Bool(self.get_keyboard_service_active()))
            }
            BackendRequest::SetKeyboardMapping(mappings) => {
                self.set_keyboard_mapping(mappings)?;
                Ok(BackendResponse::Ok)
            }
            BackendRequest::GetKeyboardConfig => {
                Ok(BackendResponse::KeyboardConfig(self.get_keyboard_config()))
            }
            BackendRequest::SetKeyboardConfig(config) => {
                self.set_keyboard_config(config)?;
                Ok(BackendResponse::Ok)
            }
            BackendRequest::GetVersions => {
                Ok(BackendResponse::Versions(self.get_versions(app_version)))
            }
            BackendRequest::StartTelemetry => Ok(BackendResponse::Ok),
            BackendRequest::StopTelemetry => Ok(BackendResponse::Ok),
            BackendRequest::GetTelemetryState => Ok(BackendResponse::Bool(false)),
        }
    }

    pub fn handle_service_request(
        &self,
        app_version: &str,
        request: ServiceRequest,
    ) -> ServiceResponse {
        match self.handle_request(app_version, request.request) {
            Ok(result) => ServiceResponse {
                id: request.id,
                result: Some(result),
                error: None,
            },
            Err(error) => ServiceResponse {
                id: request.id,
                result: None,
                error: Some(error.to_string()),
            },
        }
    }
}

impl Default for FFBeastService {
    fn default() -> Self {
        Self::new()
    }
}

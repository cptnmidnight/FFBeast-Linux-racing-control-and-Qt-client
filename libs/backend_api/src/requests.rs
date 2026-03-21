use ffbeast_controller::{
    AdcSettings, EffectSettings, GpioSettings, HardwareSettings, WheelStatus,
};
use serde::{Deserialize, Serialize};
use sodevs_input_manager::{KeyMapping, ManagerConfig};

use crate::{AppVersions, HandshakeResponse, ServiceEvent};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareFieldUpdate {
    pub field_id: u8,
    pub index: u8,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivateLicenseRequest {
    pub key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectControlRequest {
    pub force_type: u8,
    pub value: i16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRequest {
    pub id: Option<u64>,
    pub request: BackendRequest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceResponse {
    pub id: Option<u64>,
    pub result: Option<BackendResponse>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceMessage {
    Response(ServiceResponse),
    Event(ServiceEvent),
}

impl From<ServiceResponse> for ServiceMessage {
    fn from(value: ServiceResponse) -> Self {
        Self::Response(value)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BackendRequest {
    CheckHardware,
    ConnectHardware,
    GetHandshake,
    GetStatus,
    GetEffectSettings,
    GetHardwareSettings,
    GetGpioSettings,
    GetAdcSettings,
    UpdateEffectSettings(EffectSettings),
    UpdateHardwareField(HardwareFieldUpdate),
    UpdateGpioSettings(GpioSettings),
    UpdateAdcSettings(AdcSettings),
    RebootDevice,
    ResetCenter,
    SaveSettings,
    SwitchToDfu,
    ActivateLicense(ActivateLicenseRequest),
    SendDirectControl(DirectControlRequest),
    SetKeyboardServiceActive(bool),
    GetKeyboardServiceActive,
    SetKeyboardMapping(Vec<KeyMapping>),
    GetKeyboardConfig,
    SetKeyboardConfig(ManagerConfig),
    GetVersions,
    StartTelemetry,
    StopTelemetry,
    GetTelemetryState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BackendResponse {
    Ok,
    Bool(bool),
    Status(WheelStatus),
    Handshake(HandshakeResponse),
    EffectSettings(EffectSettings),
    HardwareSettings(HardwareSettings),
    GpioSettings(GpioSettings),
    AdcSettings(AdcSettings),
    KeyboardConfig(ManagerConfig),
    Versions(AppVersions),
}

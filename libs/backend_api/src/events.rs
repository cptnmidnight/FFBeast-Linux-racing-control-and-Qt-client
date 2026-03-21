use ffbeast_controller::WheelStatus;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetrySnapshot {
    pub status: WheelStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TelemetryEvent {
    Status(TelemetrySnapshot),
    Disconnected { reason: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceEvent {
    Telemetry(TelemetryEvent),
}

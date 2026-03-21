use std::io::{self, BufRead, BufWriter, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use ffbeast_backend_api::{
    BackendRequest, BackendResponse, ServiceEvent, ServiceMessage, ServiceRequest, ServiceResponse,
    TelemetryEvent, TelemetrySnapshot,
};
use ffbeast_service::FFBeastService;

fn write_message(stdout: &mut impl Write, message: &ServiceMessage) -> io::Result<()> {
    serde_json::to_writer(&mut *stdout, message)
        .map_err(io::Error::other)?;
    stdout.write_all(b"\n")?;
    stdout.flush()
}

fn spawn_telemetry_thread(
    service: Arc<FFBeastService>,
    stdout: Arc<Mutex<BufWriter<io::Stdout>>>,
    telemetry_enabled: Arc<AtomicBool>,
) {
    thread::spawn(move || {
        let mut last_read_failed = false;

        loop {
            if telemetry_enabled.load(Ordering::Relaxed) {
                let message = if service.check_hardware() {
                    match service.poll_telemetry() {
                        Ok(status) => {
                            last_read_failed = false;
                            Some(ServiceMessage::Event(ServiceEvent::Telemetry(
                                TelemetryEvent::Status(TelemetrySnapshot { status }),
                            )))
                        }
                        Err(error) => {
                            if !last_read_failed {
                                last_read_failed = true;
                                Some(ServiceMessage::Event(ServiceEvent::Telemetry(
                                    TelemetryEvent::Disconnected {
                                        reason: error.to_string(),
                                    },
                                )))
                            } else {
                                None
                            }
                        }
                    }
                } else {
                    last_read_failed = false;
                    None
                };

                if let Some(message) = message {
                    if let Ok(mut out) = stdout.lock() {
                        if write_message(&mut *out, &message).is_err() {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }

            thread::sleep(Duration::from_millis(8));
        }
    });
}

fn main() -> io::Result<()> {
    let service = Arc::new(FFBeastService::new());
    let stdin = io::stdin();
    let stdout = Arc::new(Mutex::new(BufWriter::new(io::stdout())));
    let telemetry_enabled = Arc::new(AtomicBool::new(false));

    spawn_telemetry_thread(
        service.clone(),
        stdout.clone(),
        telemetry_enabled.clone(),
    );

    for line in stdin.lock().lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }

        let message = match serde_json::from_str::<ServiceRequest>(&line) {
            Ok(request) => {
                let response = match request.request {
                    BackendRequest::StartTelemetry => {
                        telemetry_enabled.store(true, Ordering::Relaxed);
                        ServiceResponse {
                            id: request.id,
                            result: Some(BackendResponse::Ok),
                            error: None,
                        }
                    }
                    BackendRequest::StopTelemetry => {
                        telemetry_enabled.store(false, Ordering::Relaxed);
                        ServiceResponse {
                            id: request.id,
                            result: Some(BackendResponse::Ok),
                            error: None,
                        }
                    }
                    BackendRequest::GetTelemetryState => ServiceResponse {
                        id: request.id,
                        result: Some(BackendResponse::Bool(
                            telemetry_enabled.load(Ordering::Relaxed),
                        )),
                        error: None,
                    },
                    other => service.handle_service_request(
                        env!("CARGO_PKG_VERSION"),
                        ServiceRequest {
                            id: request.id,
                            request: other,
                        },
                    ),
                };

                ServiceMessage::Response(response)
            }
            Err(error) => ServiceResponse {
                id: None,
                result: None,
                error: Some(format!("Invalid request: {error}")),
            }
            .into(),
        };

        if let Ok(mut out) = stdout.lock() {
            write_message(&mut *out, &message)?;
        }
    }

    Ok(())
}

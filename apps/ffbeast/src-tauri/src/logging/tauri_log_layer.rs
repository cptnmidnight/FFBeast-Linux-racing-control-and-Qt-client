use tauri::Emitter;
use crate::state::APP_HANDLE;
use super::log_visitor::LogVisitor;

pub struct TauriLogLayer;

impl<S> tracing_subscriber::layer::Layer<S> for TauriLogLayer
where
    S: tracing::Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
{
    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        // Only emit if APP_HANDLE is available (prevents deadlock and buffering issues)
        if let Some(handle) = APP_HANDLE.get() {
            let level_str = event.metadata().level().to_string().to_lowercase();
            let mut visitor = LogVisitor::new();
            event.record(&mut visitor);

            if !visitor.message.is_empty() {
                let _ = handle.emit(
                    "rust-log",
                    serde_json::json!({
                        "level": level_str,
                        "message": visitor.message,
                    }),
                );
            }
        }
        // If APP_HANDLE is not available yet, logs just go to console (not frontend)
    }
}

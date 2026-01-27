use std::sync::atomic::Ordering;
use crate::state::MIN_LOG_LEVEL;

pub struct GlobalLevelFilter;

impl<S> tracing_subscriber::layer::Filter<S> for GlobalLevelFilter {
    fn enabled(
        &self,
        metadata: &tracing::Metadata<'_>,
        _ctx: &tracing_subscriber::layer::Context<'_, S>,
    ) -> bool {
        let level_num = match *metadata.level() {
            tracing::Level::ERROR => 1,
            tracing::Level::WARN => 2,
            tracing::Level::INFO => 3,
            tracing::Level::DEBUG => 4,
            tracing::Level::TRACE => 5,
        };

        let min_level = MIN_LOG_LEVEL.load(Ordering::Relaxed);
        level_num <= min_level
    }
}

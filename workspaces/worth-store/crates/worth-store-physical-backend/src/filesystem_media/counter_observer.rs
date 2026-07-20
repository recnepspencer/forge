#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaCounterOverflowPolicy {
    Saturate,
}

/// Read-only retained access to causal media counters.
///
/// The observer carries no filesystem handle, lease, or operation authority.
#[derive(Debug, Clone)]
pub struct MediaCounterObserver {
    counters: std::sync::Arc<super::operation_counters::MediaCounterCells>,
}

impl MediaCounterObserver {
    pub(super) fn new(
        counters: std::sync::Arc<super::operation_counters::MediaCounterCells>,
    ) -> Self {
        Self { counters }
    }

    pub fn snapshot(&self) -> super::MediaCounterSnapshot {
        self.counters.snapshot()
    }
}

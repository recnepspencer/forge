//! RAII span guard and cross-thread handle.
//!
//! DOMAIN: Lifecycle management for active spans. The guard restores
//! the previous span on drop; the handle allows worker threads to
//! contribute decisions to the parent span.

use std::sync::{Arc, Mutex};

use super::collector::{SpanCollector, CURRENT_SPAN};
use super::super::data::output::SpanOutput;

/// A handle that can be cloned and sent to worker threads (e.g., inside `par_iter()`).
#[derive(Debug, Clone)]
pub struct KernelSpanHandle {
    collector: Arc<Mutex<SpanCollector>>,
}

impl KernelSpanHandle {
    pub(super) fn new(collector: Arc<Mutex<SpanCollector>>) -> Self {
        Self { collector }
    }

    pub(super) fn into_inner(self) -> Arc<Mutex<SpanCollector>> {
        self.collector
    }
}

/// RAII guard — maintains span context and collects output when dropped.
pub struct KernelSpanGuard {
    collector: Arc<Mutex<SpanCollector>>,
    previous: Option<Arc<Mutex<SpanCollector>>>,
    is_attached: bool,
}

impl KernelSpanGuard {
    pub(super) fn new(
        collector: Arc<Mutex<SpanCollector>>,
        previous: Option<Arc<Mutex<SpanCollector>>>,
        is_attached: bool,
    ) -> Self {
        Self {
            collector,
            previous,
            is_attached,
        }
    }

    /// Extract the accumulated DecisionLog + metrics + warnings.
    /// Should only be called on the guard that was created by `enter`.
    pub fn finish(self) -> SpanOutput {
        debug_assert!(
            !self.is_attached,
            "finish() must not be called on an attached worker guard"
        );

        let mut inner = self.collector.lock().unwrap_or_else(|e| e.into_inner());

        SpanOutput {
            decision_log: std::mem::take(&mut inner.decision_log),
            warnings: std::mem::take(&mut inner.warnings),
            metrics: std::mem::take(&mut inner.metrics),
            lineage_delta: std::mem::take(&mut inner.lineage_delta),
            config_snapshot: inner.config_snapshot.take(),
        }
    }
}

impl Drop for KernelSpanGuard {
    fn drop(&mut self) {
        CURRENT_SPAN.with(|cs| {
            *cs.borrow_mut() = self.previous.take();
        });
    }
}

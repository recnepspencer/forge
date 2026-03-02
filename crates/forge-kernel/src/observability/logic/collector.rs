//! Internal span accumulator and thread-local storage.
//!
//! DOMAIN: The mutable state that lives inside a span while it's active.
//! Not part of the public API — only `logic/span.rs` and `logic/guard.rs`
//! interact with this directly.

use std::cell::RefCell;
use std::sync::{Arc, Mutex};

use crate::configuration::facade::ResolvedConfig;
use forge_core::envelope::{KernelWarning, LineageDelta};
use forge_core::{DecisionLog, OperationMetrics};

/// Internal accumulator for an active span.
#[derive(Debug, Default)]
pub(super) struct SpanCollector {
    pub(super) decision_log: DecisionLog,
    pub(super) warnings: Vec<KernelWarning>,
    pub(super) metrics: OperationMetrics,
    pub(super) lineage_delta: LineageDelta,
    pub(super) config_snapshot: Option<ResolvedConfig>,
}

thread_local! {
    pub(super) static CURRENT_SPAN: RefCell<Option<Arc<Mutex<SpanCollector>>>> = RefCell::new(None);
}

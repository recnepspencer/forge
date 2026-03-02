//! Thread-local, scope-based decision collector.
//!
//! DOMAIN: The public API for recording decisions, warnings, metrics,
//! and lineage deltas into the currently active span.

use std::sync::{Arc, Mutex};

use crate::configuration::facade::ResolvedConfig;
use forge_core::envelope::{KernelWarning, LineageDelta};
use forge_core::tracing::SpanId;
use forge_core::{DecisionLog, OperationMetrics, TracedDecision};

use super::collector::{SpanCollector, CURRENT_SPAN};
use super::guard::{KernelSpanGuard, KernelSpanHandle};

/// Thread-local, scope-based decision collector.
pub struct KernelSpan;

impl KernelSpan {
    /// Enter a new span. Installs the collector in thread-local storage and returns an RAII guard.
    pub fn enter(name: &str) -> KernelSpanGuard {
        let _ = name; // Could be used in tracing or nested span context building.
        let collector = Arc::new(Mutex::new(SpanCollector::default()));

        // Stash the old collector to support nested spans on the same thread
        let previous = CURRENT_SPAN.with(|cs| {
            let mut current = cs.borrow_mut();
            let old = current.take();
            *current = Some(Arc::clone(&collector));
            old
        });

        KernelSpanGuard::new(collector, previous, false)
    }

    /// Record a decision in the active span.
    ///
    /// In debug builds, panics if no span is active.
    pub fn record_decision(decision: TracedDecision) {
        CURRENT_SPAN.with(|cs| {
            if let Some(collector) = cs.borrow().as_ref() {
                if let Ok(mut lock) = collector.lock() {
                    lock.decision_log.record(decision);
                }
            } else {
                debug_assert!(
                    false,
                    "KernelSpan::record_decision called outside of an active span. \
                     Did you forget KernelSpan::enter()?"
                );
            }
        });
    }

    /// Record a warning in the active span.
    pub fn record_warning(warning: KernelWarning) {
        CURRENT_SPAN.with(|cs| {
            if let Some(collector) = cs.borrow().as_ref() {
                if let Ok(mut lock) = collector.lock() {
                    lock.warnings.push(warning);
                }
            }
        });
    }

    /// Update lineage delta for the active span.
    pub fn record_lineage_delta(delta: LineageDelta) {
        CURRENT_SPAN.with(|cs| {
            if let Some(collector) = cs.borrow().as_ref() {
                if let Ok(mut lock) = collector.lock() {
                    lock.lineage_delta.accumulate(&delta);
                }
            }
        });
    }

    /// Merge an entire DecisionLog into the active span.
    pub fn merge_decision_log(log: DecisionLog) {
        CURRENT_SPAN.with(|cs| {
            if let Some(collector) = cs.borrow().as_ref() {
                if let Ok(mut lock) = collector.lock() {
                    lock.decision_log.merge(log);
                }
            }
        });
    }

    /// Extend the active span warnings with a batch.
    pub fn extend_warnings<I: IntoIterator<Item = KernelWarning>>(warnings: I) {
        CURRENT_SPAN.with(|cs| {
            if let Some(collector) = cs.borrow().as_ref() {
                if let Ok(mut lock) = collector.lock() {
                    lock.warnings.extend(warnings);
                }
            }
        });
    }

    /// Add metrics to the active span.
    pub fn add_metrics(metrics: OperationMetrics) {
        CURRENT_SPAN.with(|cs| {
            if let Some(collector) = cs.borrow().as_ref() {
                if let Ok(mut lock) = collector.lock() {
                    lock.metrics.accumulate(&metrics);
                }
            }
        });
    }

    /// Set the config snapshot for the active span.
    pub fn set_config_snapshot(config: ResolvedConfig) {
        CURRENT_SPAN.with(|cs| {
            if let Some(collector) = cs.borrow().as_ref() {
                if let Ok(mut lock) = collector.lock() {
                    lock.config_snapshot = Some(config);
                }
            }
        });
    }

    /// Get the config snapshot for the active span, if any.
    pub fn get_config_snapshot() -> Option<ResolvedConfig> {
        CURRENT_SPAN.with(|cs| {
            if let Some(collector) = cs.borrow().as_ref() {
                if let Ok(lock) = collector.lock() {
                    return lock.config_snapshot.clone();
                }
            }
            None
        })
    }

    /// Record a span start in the active trace.
    pub fn start_span(name: &'static str) -> SpanId {
        CURRENT_SPAN.with(|cs| {
            if let Some(collector) = cs.borrow().as_ref() {
                if let Ok(mut lock) = collector.lock() {
                    return lock.decision_log.start_span(name);
                }
            }
            SpanId(0)
        })
    }

    /// Record a span end in the active trace.
    pub fn end_span(span_id: SpanId, duration_micros: u64) {
        CURRENT_SPAN.with(|cs| {
            if let Some(collector) = cs.borrow().as_ref() {
                if let Ok(mut lock) = collector.lock() {
                    lock.decision_log.end_span(span_id, duration_micros);
                }
            }
        });
    }

    /// Whether a span is currently active on this thread.
    pub fn is_active() -> bool {
        CURRENT_SPAN.with(|cs| cs.borrow().is_some())
    }

    /// Current decision count in the active span, if any.
    pub fn current_decision_count() -> Option<usize> {
        CURRENT_SPAN.with(|cs| {
            cs.borrow().as_ref().and_then(|collector| {
                collector
                    .lock()
                    .ok()
                    .map(|lock| lock.decision_log.len())
            })
        })
    }

    /// Get a handle to the currently active span, suitable for sending to worker threads.
    pub fn current_handle() -> Option<KernelSpanHandle> {
        CURRENT_SPAN.with(|cs| {
            cs.borrow().as_ref().map(|c| KernelSpanHandle::new(Arc::clone(c)))
        })
    }

    /// Attach an existing span handle on a worker thread.
    /// Decisions recorded on this thread go to the parent span.
    pub fn attach(handle: KernelSpanHandle) -> KernelSpanGuard {
        let collector = handle.into_inner();
        let previous = CURRENT_SPAN.with(|cs| {
            let mut current = cs.borrow_mut();
            let old = current.take();
            *current = Some(Arc::clone(&collector));
            old
        });

        KernelSpanGuard::new(collector, previous, true)
    }
}

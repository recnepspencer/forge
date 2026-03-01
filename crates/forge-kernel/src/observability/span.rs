//! Scope-based decision and replay logging.
//!
//! DOMAIN: Thread-local and cross-thread operation tracking.

use std::cell::RefCell;
use std::sync::{Arc, Mutex};

use crate::configuration::facade::ResolvedConfig;
use forge_core::envelope::{KernelWarning, LineageDelta};
use forge_core::tracing::SpanId;
use forge_core::{DecisionLog, OperationMetrics, TracedDecision};

/// Output collected from a completed `KernelSpan`.
#[derive(Debug, Clone)]
pub struct SpanOutput {
    pub decision_log: DecisionLog,
    pub warnings: Vec<KernelWarning>,
    pub metrics: OperationMetrics,
    pub lineage_delta: LineageDelta,
    pub config_snapshot: Option<ResolvedConfig>,
}

#[derive(Debug, Default)]
struct SpanCollector {
    decision_log: DecisionLog,
    warnings: Vec<KernelWarning>,
    metrics: OperationMetrics,
    lineage_delta: LineageDelta,
    config_snapshot: Option<ResolvedConfig>,
}

/// A handle that can be cloned and sent to worker threads (e.g., inside `par_iter()`).
#[derive(Debug, Clone)]
pub struct KernelSpanHandle {
    collector: Arc<Mutex<SpanCollector>>,
}

thread_local! {
    static CURRENT_SPAN: RefCell<Option<Arc<Mutex<SpanCollector>>>> = RefCell::new(None);
}

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

        KernelSpanGuard {
            collector,
            previous,
            is_attached: false,
        }
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
                    lock.lineage_delta.faces_created += delta.faces_created;
                    lock.lineage_delta.faces_deleted += delta.faces_deleted;
                    lock.lineage_delta.vertices_created += delta.vertices_created;
                    lock.lineage_delta.vertices_deleted += delta.vertices_deleted;
                    lock.lineage_delta.edges_created += delta.edges_created;
                    lock.lineage_delta.edges_deleted += delta.edges_deleted;
                    lock.lineage_delta.half_edges_created += delta.half_edges_created;
                    lock.lineage_delta.half_edges_deleted += delta.half_edges_deleted;
                    lock.lineage_delta.loops_created += delta.loops_created;
                    lock.lineage_delta.loops_deleted += delta.loops_deleted;
                    lock.lineage_delta.shells_created += delta.shells_created;
                    lock.lineage_delta.shells_deleted += delta.shells_deleted;
                    lock.lineage_delta.solids_created += delta.solids_created;
                    lock.lineage_delta.solids_deleted += delta.solids_deleted;
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
                    lock.metrics.duration += metrics.duration;
                    lock.metrics.entities_created += metrics.entities_created;
                    lock.metrics.entities_deleted += metrics.entities_deleted;
                    lock.metrics.entities_modified += metrics.entities_modified;
                    lock.metrics.exact_predicate_calls += metrics.exact_predicate_calls;
                    lock.metrics.policy_decisions_made += metrics.policy_decisions_made;
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

    /// Get a handle to the currently active span, suitable for sending to worker threads.
    pub fn current_handle() -> Option<KernelSpanHandle> {
        CURRENT_SPAN.with(|cs| {
            cs.borrow().as_ref().map(|c| KernelSpanHandle {
                collector: Arc::clone(c),
            })
        })
    }

    /// Attach an existing span handle on a worker thread.
    /// Decisions recorded on this thread go to the parent span.
    pub fn attach(handle: KernelSpanHandle) -> KernelSpanGuard {
        let previous = CURRENT_SPAN.with(|cs| {
            let mut current = cs.borrow_mut();
            let old = current.take();
            *current = Some(Arc::clone(&handle.collector));
            old
        });

        KernelSpanGuard {
            collector: handle.collector,
            previous,
            is_attached: true,
        }
    }
}

/// RAII guard — maintains span context and collects output when dropped.
pub struct KernelSpanGuard {
    collector: Arc<Mutex<SpanCollector>>,
    previous: Option<Arc<Mutex<SpanCollector>>>,
    is_attached: bool,
}

impl KernelSpanGuard {
    /// Extract the accumulated DecisionLog + metrics + warnings.
    /// Should only be called on the guard that was created by `enter`.
    pub fn finish(mut self) -> SpanOutput {
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

//! Deterministic trace fingerprint helpers.
//!
//! Default behavior fingerprints semantic trace content while ignoring
//! wall-clock span durations so repeated runs can be compared reliably.

use serde::{Deserialize, Serialize};

use super::decision_log::DecisionLog;
use crate::tracing::decision::TraceEvent;

/// Compact fingerprint summary for a `DecisionLog`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceFingerprint {
    /// Deterministic hash of semantic trace content (timing excluded by default).
    pub trace_hash: u64,
    /// Ordered decision IDs as recorded in the log.
    pub decision_ids: Vec<u64>,
}

/// Compute a deterministic fingerprint for a trace log.
///
/// This ignores span timing (`EndSpan.duration_micros`) to avoid false diffs
/// between otherwise identical executions.
pub fn compute_trace_fingerprint(log: &DecisionLog) -> TraceFingerprint {
    let normalized_events: Vec<TraceEvent> = log
        .get_events()
        .iter()
        .cloned()
        .map(|event| match event {
            TraceEvent::EndSpan { id, .. } => TraceEvent::EndSpan {
                id,
                duration_micros: 0,
            },
            other => other,
        })
        .collect();

    let bytes = serde_json::to_vec(&normalized_events)
        .expect("TraceEvent serialization must succeed for fingerprinting");
    let trace_hash = fnv1a64(&bytes);
    let decision_ids = log.decisions().map(|d| d.get_id().0).collect();

    TraceFingerprint {
        trace_hash,
        decision_ids,
    }
}

fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in bytes {
        h ^= *b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tracing::{DecisionContext, DecisionId, DecisionKind, DecisionTier, TracedDecision};

    fn sample_log(duration_micros: u64) -> DecisionLog {
        let mut log = DecisionLog::new();
        let span = log.start_span("phase");
        log.record(TracedDecision::new(
            DecisionId(1),
            DecisionKind::NearBoundary { threshold: 1e-6 },
            DecisionTier::NearBoundary,
            1e-8,
            DecisionContext::Tolerance {
                measured: 1e-8,
                threshold: 1e-6,
            },
        ));
        log.end_span(span, duration_micros);
        log
    }

    #[test]
    fn trace_fingerprint_is_stable_for_identical_decision_logs() {
        let a = sample_log(10);
        let b = sample_log(10);

        assert_eq!(compute_trace_fingerprint(&a), compute_trace_fingerprint(&b));
    }

    #[test]
    fn trace_fingerprint_changes_when_decision_semantics_change() {
        let mut a = DecisionLog::new();
        a.record(TracedDecision::new(
            DecisionId(1),
            DecisionKind::Exact,
            DecisionTier::Deterministic,
            0.0,
            DecisionContext::Degeneracy {
                description: "a".into(),
            },
        ));

        let mut b = DecisionLog::new();
        b.record(TracedDecision::new(
            DecisionId(1),
            DecisionKind::Forced {
                reason: "manifold".into(),
            },
            DecisionTier::Escalated,
            0.0,
            DecisionContext::Degeneracy {
                description: "a".into(),
            },
        ));

        assert_ne!(
            compute_trace_fingerprint(&a).trace_hash,
            compute_trace_fingerprint(&b).trace_hash
        );
    }

    #[test]
    fn trace_fingerprint_ignores_span_timing_when_configured_default() {
        let a = sample_log(10);
        let b = sample_log(999_999);

        assert_eq!(
            compute_trace_fingerprint(&a).trace_hash,
            compute_trace_fingerprint(&b).trace_hash
        );
    }
}

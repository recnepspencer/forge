//! Decision recording interface and cross-thread handle.
//!
//! DOMAIN: `DecisionSink` is the typed, declarative recording interface for
//! kernel decisions. Call sites describe WHAT happened; implementations handle
//! ID assignment, tier derivation, and storage.
//!
//! Designed for explicit dependency injection — no ambient/global state.
//!
//! Hot-path functions should use `<S: DecisionSink>` (monomorphized, zero vtable
//! cost). Pipeline-level code can use `&mut dyn DecisionSink` where virtual
//! dispatch is acceptable.
//!
//! DEPENDENCIES: `forge-core::tracing` (TracedDecision, DecisionTier, SpanId),
//!               `forge-core::policy` (PolicyKind),
//!               `forge-math::arithmetic::precision` (PrecisionEscalation)

use std::sync::{Arc, Mutex};

use super::decision::{
    DecisionContext, DecisionId, DecisionKind, DecisionTier, SpanId, TracedDecision,
};
use crate::policy::PolicyKind;

// ── Core trait ──────────────────────────────────────────────────────────────

/// Typed decision recording interface.
///
/// Each method represents a specific class of geometric decision.
/// Implementations handle ID assignment, tier derivation, context construction,
/// and storage internally. Call sites describe what happened — nothing more.
///
/// # Design rationale
///
/// - **Typed methods** over a single `record()`: grep-able, self-documenting,
///   and compile-time checked when new decision kinds are added.
/// - **Generic on hot paths**: use `fn foo<S: DecisionSink>(sink: &mut S)` for
///   inner loops (monomorphized). Use `&mut dyn DecisionSink` at pipeline level.
/// - **No global state**: pure dependency injection. Parallelizable by
///   construction — each scope can hold its own sink.
pub trait DecisionSink {
    // ── Tolerance decisions ──────────────────────────────────────────

    /// A vertex was snapped into an existing tolerance sphere.
    fn record_tolerance_snap(
        &mut self,
        entity_index: u32,
        gap: f64,
        threshold: f64,
        tier: DecisionTier,
    );

    /// A value was measured near a tolerance boundary but resolved with
    /// confidence. Logged for transparency even though no policy was needed.
    fn record_near_boundary(&mut self, entity_index: u32, margin: f64, threshold: f64);

    // ── Classification decisions ─────────────────────────────────────

    /// A face or point was classified relative to another solid.
    fn record_classification(&mut self, entity_index: u32, result_label: &str, tier: DecisionTier);

    // ── Precision decisions ──────────────────────────────────────────

    /// Precision was escalated beyond f64 to resolve a predicate.
    fn record_escalation(
        &mut self,
        entity_index: u32,
        escalation: &forge_math::arithmetic::precision::PrecisionEscalation,
    );

    // ── Policy decisions ─────────────────────────────────────────────

    /// A configured policy was applied to resolve an ambiguity.
    ///
    /// `description` is an optional human-readable reason for audit logs.
    fn record_policy_applied(
        &mut self,
        policy: PolicyKind,
        margin: f64,
        default_used: bool,
        description: Option<&str>,
    );

    /// An ambiguity could not be resolved — safe fallback applied.
    fn record_ambiguous(&mut self, fallback_description: &str, margin: f64);

    /// A hard constraint forced a specific outcome.
    fn record_forced(&mut self, reason: &str, entity_index: u32, margin: f64);

    // ── Span management ──────────────────────────────────────────────

    /// Open a named timing span. Returns an ID to close it with `end_span`.
    fn start_span(&mut self, name: &'static str) -> SpanId;

    /// Close a timing span opened by `start_span`.
    fn end_span(&mut self, id: SpanId, duration_micros: u64);

    // ── Raw escape hatch ─────────────────────────────────────────────

    /// Record a pre-built `TracedDecision`. For complex callsites (e.g.,
    /// the policy resolver) that need full control over the decision payload.
    ///
    /// Prefer typed methods for new code.
    fn record_raw(&mut self, decision: TracedDecision);
}

// ── Blanket impl for &mut T ─────────────────────────────────────────────────

/// Enables passing `&mut sink` to nested calls without fighting the borrow
/// checker. Mirrors the standard library's `impl Write for &mut W`.
impl<T: DecisionSink + ?Sized> DecisionSink for &mut T {
    fn record_tolerance_snap(
        &mut self,
        entity_index: u32,
        gap: f64,
        threshold: f64,
        tier: DecisionTier,
    ) {
        (**self).record_tolerance_snap(entity_index, gap, threshold, tier)
    }
    fn record_near_boundary(&mut self, entity_index: u32, margin: f64, threshold: f64) {
        (**self).record_near_boundary(entity_index, margin, threshold)
    }
    fn record_classification(&mut self, entity_index: u32, result_label: &str, tier: DecisionTier) {
        (**self).record_classification(entity_index, result_label, tier)
    }
    fn record_escalation(
        &mut self,
        entity_index: u32,
        escalation: &forge_math::arithmetic::precision::PrecisionEscalation,
    ) {
        (**self).record_escalation(entity_index, escalation)
    }
    fn record_policy_applied(
        &mut self,
        policy: PolicyKind,
        margin: f64,
        default_used: bool,
        description: Option<&str>,
    ) {
        (**self).record_policy_applied(policy, margin, default_used, description)
    }
    fn record_ambiguous(&mut self, fallback_description: &str, margin: f64) {
        (**self).record_ambiguous(fallback_description, margin)
    }
    fn record_forced(&mut self, reason: &str, entity_index: u32, margin: f64) {
        (**self).record_forced(reason, entity_index, margin)
    }
    fn start_span(&mut self, name: &'static str) -> SpanId {
        (**self).start_span(name)
    }
    fn end_span(&mut self, id: SpanId, duration_micros: u64) {
        (**self).end_span(id, duration_micros)
    }
    fn record_raw(&mut self, decision: TracedDecision) {
        (**self).record_raw(decision)
    }
}

// ── NullSink ─────────────────────────────────────────────────────────────────

/// A no-op sink that discards all decisions. Use in tests where tracing
/// is irrelevant, or as a default when no recording is needed.
pub struct NullSink;

impl DecisionSink for NullSink {
    fn record_tolerance_snap(&mut self, _: u32, _: f64, _: f64, _: DecisionTier) {}
    fn record_near_boundary(&mut self, _: u32, _: f64, _: f64) {}
    fn record_classification(&mut self, _: u32, _: &str, _: DecisionTier) {}
    fn record_escalation(
        &mut self,
        _: u32,
        _: &forge_math::arithmetic::precision::PrecisionEscalation,
    ) {
    }
    fn record_policy_applied(&mut self, _: PolicyKind, _: f64, _: bool, _: Option<&str>) {}
    fn record_ambiguous(&mut self, _: &str, _: f64) {}
    fn record_forced(&mut self, _: &str, _: u32, _: f64) {}
    fn start_span(&mut self, _: &'static str) -> SpanId {
        SpanId(0)
    }
    fn end_span(&mut self, _: SpanId, _: u64) {}
    fn record_raw(&mut self, _: TracedDecision) {}
}

// ── DecisionSinkHandle ──────────────────────────────────────────────────────

/// A thread-safe, cloneable handle to a `DecisionSink`.
///
/// Used when decisions need to be recorded from worker threads that cannot
/// hold a `&mut` borrow across a spawn boundary. The inner sink is protected
/// by a `Mutex`, so contention is low as long as decisions are infrequent
/// relative to the work being done (which they are — decisions happen at
/// geometric boundaries, not at every arithmetic op).
///
/// Create via `DecisionSinkHandle::new()`, clone to each worker, and
/// decisions will merge into the original sink.
#[derive(Clone)]
pub struct DecisionSinkHandle {
    inner: Arc<Mutex<Box<dyn DecisionSink + Send>>>,
}

impl DecisionSinkHandle {
    /// Wrap a sink for cross-thread sharing.
    pub fn new(sink: Box<dyn DecisionSink + Send>) -> Self {
        Self {
            inner: Arc::new(Mutex::new(sink)),
        }
    }
}

impl DecisionSink for DecisionSinkHandle {
    fn record_tolerance_snap(
        &mut self,
        entity_index: u32,
        gap: f64,
        threshold: f64,
        tier: DecisionTier,
    ) {
        if let Ok(mut lock) = self.inner.lock() {
            lock.record_tolerance_snap(entity_index, gap, threshold, tier);
        }
    }
    fn record_near_boundary(&mut self, entity_index: u32, margin: f64, threshold: f64) {
        if let Ok(mut lock) = self.inner.lock() {
            lock.record_near_boundary(entity_index, margin, threshold);
        }
    }
    fn record_classification(&mut self, entity_index: u32, result_label: &str, tier: DecisionTier) {
        if let Ok(mut lock) = self.inner.lock() {
            lock.record_classification(entity_index, result_label, tier);
        }
    }
    fn record_escalation(
        &mut self,
        entity_index: u32,
        escalation: &forge_math::arithmetic::precision::PrecisionEscalation,
    ) {
        if let Ok(mut lock) = self.inner.lock() {
            lock.record_escalation(entity_index, escalation);
        }
    }
    fn record_policy_applied(
        &mut self,
        policy: PolicyKind,
        margin: f64,
        default_used: bool,
        description: Option<&str>,
    ) {
        if let Ok(mut lock) = self.inner.lock() {
            lock.record_policy_applied(policy, margin, default_used, description);
        }
    }
    fn record_ambiguous(&mut self, fallback_description: &str, margin: f64) {
        if let Ok(mut lock) = self.inner.lock() {
            lock.record_ambiguous(fallback_description, margin);
        }
    }
    fn record_forced(&mut self, reason: &str, entity_index: u32, margin: f64) {
        if let Ok(mut lock) = self.inner.lock() {
            lock.record_forced(reason, entity_index, margin);
        }
    }
    fn start_span(&mut self, name: &'static str) -> SpanId {
        self.inner
            .lock()
            .map(|mut l| l.start_span(name))
            .unwrap_or(SpanId(0))
    }
    fn end_span(&mut self, id: SpanId, duration_micros: u64) {
        if let Ok(mut lock) = self.inner.lock() {
            lock.end_span(id, duration_micros);
        }
    }
    fn record_raw(&mut self, decision: TracedDecision) {
        if let Ok(mut lock) = self.inner.lock() {
            lock.record_raw(decision);
        }
    }
}

// ── TestSink ────────────────────────────────────────────────────────────────

/// A collecting sink for unit tests. Stores all decisions in a `Vec` for
/// assertion. No ID assignment, no tier derivation — just raw capture.
///
/// # Example
/// ```ignore
/// let mut sink = TestSink::new();
/// snap_or_coalesce_vertex(..., &mut sink);
/// assert_eq!(sink.decisions().len(), 1);
/// ```
pub struct TestSink {
    decisions: Vec<TracedDecision>,
}

impl TestSink {
    /// Create an empty test sink.
    pub fn new() -> Self {
        Self {
            decisions: Vec::new(),
        }
    }

    /// All decisions recorded so far.
    pub fn decisions(&self) -> &[TracedDecision] {
        &self.decisions
    }

    /// Number of decisions recorded.
    pub fn len(&self) -> usize {
        self.decisions.len()
    }

    /// Whether any decisions have been recorded.
    pub fn is_empty(&self) -> bool {
        self.decisions.is_empty()
    }

    /// Drain all decisions, resetting the sink.
    pub fn take(&mut self) -> Vec<TracedDecision> {
        std::mem::take(&mut self.decisions)
    }
}

impl Default for TestSink {
    fn default() -> Self {
        Self::new()
    }
}

impl DecisionSink for TestSink {
    fn record_tolerance_snap(
        &mut self,
        entity_index: u32,
        gap: f64,
        threshold: f64,
        tier: DecisionTier,
    ) {
        let id = DecisionId(self.decisions.len() as u64 + 1);
        self.decisions.push(TracedDecision::new(
            id,
            DecisionKind::PolicyApplied {
                policy: PolicyKind::CoincidentGeometry,
                default_used: true,
            },
            tier,
            gap,
            DecisionContext::Tolerance {
                measured: gap,
                threshold,
            },
        ));
        let _ = entity_index; // captured in future entity_scope extension
    }

    fn record_near_boundary(&mut self, entity_index: u32, margin: f64, threshold: f64) {
        let id = DecisionId(self.decisions.len() as u64 + 1);
        self.decisions.push(TracedDecision::new(
            id,
            DecisionKind::NearBoundary { threshold },
            DecisionTier::NearBoundary,
            margin,
            DecisionContext::Tolerance {
                measured: margin,
                threshold,
            },
        ));
        let _ = entity_index;
    }

    fn record_classification(&mut self, entity_index: u32, result_label: &str, tier: DecisionTier) {
        let id = DecisionId(self.decisions.len() as u64 + 1);
        self.decisions.push(TracedDecision::new(
            id,
            DecisionKind::Exact,
            tier,
            1.0,
            DecisionContext::Classification {
                point: [0.0; 3],
                result: result_label.to_string(),
            },
        ));
        let _ = entity_index;
    }

    fn record_escalation(
        &mut self,
        entity_index: u32,
        escalation: &forge_math::arithmetic::precision::PrecisionEscalation,
    ) {
        let id = DecisionId(self.decisions.len() as u64 + 1);
        self.decisions.push(TracedDecision::new(
            id,
            DecisionKind::Exact,
            DecisionTier::Escalated,
            escalation.disagreement_magnitude.unwrap_or(0.0),
            DecisionContext::PrecisionEscalation {
                escalation: escalation.clone(),
            },
        ));
        let _ = entity_index;
    }

    fn record_policy_applied(
        &mut self,
        policy: PolicyKind,
        margin: f64,
        default_used: bool,
        _description: Option<&str>,
    ) {
        let id = DecisionId(self.decisions.len() as u64 + 1);
        self.decisions.push(TracedDecision::new(
            id,
            DecisionKind::PolicyApplied {
                policy,
                default_used,
            },
            DecisionTier::PolicyApplied,
            margin,
            DecisionContext::Degeneracy {
                description: "policy applied".to_string(),
            },
        ));
    }

    fn record_ambiguous(&mut self, fallback_description: &str, margin: f64) {
        let id = DecisionId(self.decisions.len() as u64 + 1);
        self.decisions.push(TracedDecision::new(
            id,
            DecisionKind::Ambiguous {
                fallback_applied: fallback_description.to_string(),
            },
            DecisionTier::Escalated,
            margin,
            DecisionContext::Degeneracy {
                description: fallback_description.to_string(),
            },
        ));
    }

    fn record_forced(&mut self, reason: &str, entity_index: u32, margin: f64) {
        let id = DecisionId(self.decisions.len() as u64 + 1);
        self.decisions.push(TracedDecision::new(
            id,
            DecisionKind::Forced {
                reason: reason.to_string(),
            },
            DecisionTier::Escalated,
            margin,
            DecisionContext::Degeneracy {
                description: format!("forced: {}", reason),
            },
        ));
        let _ = entity_index;
    }

    fn start_span(&mut self, _name: &'static str) -> SpanId {
        SpanId(0)
    }

    fn end_span(&mut self, _id: SpanId, _duration_micros: u64) {}

    fn record_raw(&mut self, decision: TracedDecision) {
        self.decisions.push(decision);
    }
}

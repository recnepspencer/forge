//! Operation result envelope and decision types (Phase 2B).
//!
//! DOMAIN: Universal return type for every kernel operation. Every Euler
//! operator, Boolean operation, and feature evaluation flows through
//! `OperationResult<T>`, carrying the full decision trace an AI agent
//! needs to reconstruct the state transition.
//!
//! INVARIANTS:
//! - `OperationResult` wraps every kernel function return value
//! - `TracedDecision` records are immutable once created
//! - `DecisionKind` captures *how* a decision was resolved, not *what* it was about
//! - `DecisionLog` is queryable, serializable, and diffable
//!
//! DEPENDENCIES: serde (serialization), std::time (metrics)

use std::fmt;
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::PolicyKind;

// =========================================================================
// ENTITY REFERENCE (crate-neutral topology reference)
// =========================================================================

/// Crate-neutral reference to a topological entity.
///
/// Used in `TracedDecision` to scope a decision to a specific entity
/// without importing typed handles from `forge-topo`. The kernel layer
/// constructs these from `FaceId`, `VertexId`, etc.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntityRef {
    /// Entity kind name: "Face", "HalfEdge", "Vertex", "Loop".
    kind: String,
    /// Arena index of the entity.
    index: u32,
}

impl EntityRef {
    /// Create a new entity reference.
    pub fn new(kind: &str, index: u32) -> Self {
        Self { kind: kind.to_string(), index }
    }

    /// The entity kind name.
    pub fn get_kind(&self) -> &str {
        &self.kind
    }

    /// The arena index.
    pub fn get_index(&self) -> u32 {
        self.index
    }
}

impl fmt::Display for EntityRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}#{}", self.kind, self.index)
    }
}

// =========================================================================
// FEATURE SCOPE SENTINELS
// =========================================================================

/// Sentinel `feature_scope` value for low-level Euler operator decisions.
///
/// Decisions tagged with this scope are filtered out by `DecisionLog::display_compact()`
/// and only shown in verbose/full display mode.
pub const EULER_OP_FEATURE_SCOPE: u64 = u64::MAX;

// =========================================================================
// DECISION KIND (how a decision was resolved)
// =========================================================================

/// How a kernel decision was resolved.
///
/// This captures the *resolution method*, not the *subject*. The subject
/// is captured in `DecisionContext`. Together they answer:
/// "What was decided?" (context) + "How was it resolved?" (kind).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DecisionKind {
    /// Predicate resolved exactly — zero ambiguity.
    Exact,
    /// Ambiguity detected, resolved by a configured `ModelingContext` policy.
    PolicyApplied {
        /// Which policy category was applied.
        policy: PolicyKind,
        /// Whether the system default was used (no user override).
        default_used: bool,
    },
    /// Near a threshold but resolved with confidence.
    /// Logged for transparency even though no policy was needed.
    NearBoundary {
        /// The threshold that was approached.
        threshold: f64,
    },
    /// Could not be resolved by policy — safe default applied, flagged for review.
    Ambiguous {
        /// Description of the fallback that was applied.
        fallback_applied: String,
    },
    /// Hard constraint forced a specific outcome (e.g., manifoldness requirement).
    Forced {
        /// Why this outcome was forced.
        reason: String,
    },
}

impl fmt::Display for DecisionKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecisionKind::Exact => write!(f, "Exact"),
            DecisionKind::PolicyApplied { policy, default_used } => {
                write!(f, "PolicyApplied({:?}, default={})", policy, default_used)
            }
            DecisionKind::NearBoundary { threshold } => {
                write!(f, "NearBoundary(threshold={:.2e})", threshold)
            }
            DecisionKind::Ambiguous { fallback_applied } => {
                write!(f, "Ambiguous(fallback={})", fallback_applied)
            }
            DecisionKind::Forced { reason } => {
                write!(f, "Forced({})", reason)
            }
        }
    }
}

// =========================================================================
// DECISION CONTEXT (what the decision was about)
// =========================================================================

/// What a kernel decision was about.
///
/// Provides structured, machine-readable context for *what* prompted a
/// decision. Paired with `DecisionKind` (how it was resolved).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DecisionContext {
    /// Point classification (in/out/on boundary).
    Classification {
        /// The 3D point being classified.
        point: [f64; 3],
        /// The classification result (e.g., "Inside", "Outside").
        result: String,
    },
    /// Two entities are coincident or nearly so.
    Coincidence {
        /// First entity in the coincidence pair.
        entity_a: EntityRef,
        /// Second entity in the coincidence pair.
        entity_b: EntityRef,
    },
    /// A measured value was compared against a tolerance threshold.
    Tolerance {
        /// The measured value.
        measured: f64,
        /// The threshold it was compared against.
        threshold: f64,
    },
    /// A degenerate geometric configuration was detected.
    Degeneracy {
        /// Human-readable description of the degeneracy.
        description: String,
    },
}

impl fmt::Display for DecisionContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecisionContext::Classification { point, result } => {
                write!(f, "Classification [{:.4}, {:.4}, {:.4}] → {}",
                    point[0], point[1], point[2], result)
            }
            DecisionContext::Coincidence { entity_a, entity_b } => {
                write!(f, "Coincidence {} ↔ {}", entity_a, entity_b)
            }
            DecisionContext::Tolerance { measured, threshold } => {
                write!(f, "Tolerance measured={:.2e} threshold={:.2e}", measured, threshold)
            }
            DecisionContext::Degeneracy { description } => {
                write!(f, "Degeneracy: {}", description)
            }
        }
    }
}

// =========================================================================
// DECISION ID
// =========================================================================

/// Unique identifier for a traced decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DecisionId(pub u64);

impl fmt::Display for DecisionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "decision-{}", self.0)
    }
}

// =========================================================================
// TRACED DECISION (replaces ToleranceDecision)
// =========================================================================

/// A recorded kernel decision with full machine-actionable classification.
///
/// Every time the kernel makes a judgment call — whether exact, policy-driven,
/// or forced — it creates one of these. The AI agent can query all decisions
/// from a completed operation and override any that are marked `overridable`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TracedDecision {
    /// Unique identifier for this decision.
    id: DecisionId,
    /// How the decision was resolved.
    kind: DecisionKind,
    /// How close to the threshold (lower = more marginal). Non-negative.
    margin: f64,
    /// Which feature this decision belongs to (if scoped).
    feature_scope: Option<u64>,
    /// Which topological entity this decision applies to (if scoped).
    entity_scope: Option<EntityRef>,
    /// Whether an agent/user can override this decision without a full rebuild.
    overridable: bool,
    /// Structured context describing what prompted this decision.
    context: DecisionContext,
}

impl TracedDecision {
    /// Create a new traced decision.
    pub fn new(
        id: DecisionId,
        kind: DecisionKind,
        margin: f64,
        context: DecisionContext,
    ) -> Self {
        Self {
            id,
            kind,
            margin,
            feature_scope: None,
            entity_scope: None,
            overridable: true,
            context,
        }
    }

    /// The unique decision identifier.
    pub fn get_id(&self) -> DecisionId {
        self.id
    }

    /// How the decision was resolved.
    pub fn get_kind(&self) -> &DecisionKind {
        &self.kind
    }

    /// How close to the threshold (lower = more marginal).
    pub fn get_margin(&self) -> f64 {
        self.margin
    }

    /// The feature scope, if any.
    pub fn get_feature_scope(&self) -> Option<u64> {
        self.feature_scope
    }

    /// Set the feature scope.
    pub fn set_feature_scope(&mut self, feature_id: u64) {
        self.feature_scope = Some(feature_id);
    }

    /// The entity scope, if any.
    pub fn get_entity_scope(&self) -> Option<&EntityRef> {
        self.entity_scope.as_ref()
    }

    /// Set the entity scope.
    pub fn set_entity_scope(&mut self, entity: EntityRef) {
        self.entity_scope = Some(entity);
    }

    /// Whether this decision can be overridden.
    pub fn is_overridable(&self) -> bool {
        self.overridable
    }

    /// Set whether this decision can be overridden.
    pub fn set_overridable(&mut self, overridable: bool) {
        self.overridable = overridable;
    }

    /// The structured context of this decision.
    pub fn get_context(&self) -> &DecisionContext {
        &self.context
    }
}

impl fmt::Display for TracedDecision {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {} margin={:.2e}", self.id, self.kind, self.margin)?;
        if let Some(ref entity) = self.entity_scope {
            write!(f, " entity={}", entity)?;
        }
        if let Some(feature_id) = self.feature_scope {
            write!(f, " feature={}", feature_id)?;
        }
        write!(f, " | {}", self.context)
    }
}

// =========================================================================
// DECISION LOG (queryable, serializable, diffable)
// =========================================================================

/// A queryable, serializable collection of traced decisions.
///
/// Every kernel operation populates a `DecisionLog`. After completion,
/// the caller can query it to understand what happened, what was ambiguous,
/// and what can be overridden.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DecisionLog {
    /// All decisions recorded during the operation.
    decisions: Vec<TracedDecision>,
}

impl DecisionLog {
    /// Create an empty decision log.
    pub fn new() -> Self {
        Self { decisions: Vec::new() }
    }

    /// Record a decision.
    pub fn record(&mut self, decision: TracedDecision) {
        self.decisions.push(decision);
    }

    /// All recorded decisions.
    pub fn get_all(&self) -> &[TracedDecision] {
        &self.decisions
    }

    /// Look up a decision by ID.
    pub fn get_by_id(&self, id: DecisionId) -> Option<&TracedDecision> {
        self.decisions.iter().find(|d| d.get_id() == id)
    }

    /// Filter decisions that match a given kind discriminant.
    pub fn by_kind_discriminant(&self, discriminant: &str) -> Vec<&TracedDecision> {
        self.decisions.iter().filter(|d| {
            let kind_str = format!("{}", d.get_kind());
            kind_str.starts_with(discriminant)
        }).collect()
    }

    /// Decisions sorted by margin ascending (most marginal first).
    pub fn by_margin_ascending(&self) -> Vec<&TracedDecision> {
        let mut refs: Vec<&TracedDecision> = self.decisions.iter().collect();
        refs.sort_by(|a, b| {
            a.get_margin().partial_cmp(&b.get_margin())
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        refs
    }

    /// Only decisions with `DecisionKind::Ambiguous`.
    pub fn ambiguous_only(&self) -> Vec<&TracedDecision> {
        self.decisions.iter().filter(|d| {
            matches!(d.get_kind(), DecisionKind::Ambiguous { .. })
        }).collect()
    }

    /// Only decisions that are overridable.
    pub fn overridable_only(&self) -> Vec<&TracedDecision> {
        self.decisions.iter().filter(|d| d.is_overridable()).collect()
    }

    /// Returns `true` if there are zero `Ambiguous` decisions.
    pub fn is_clean(&self) -> bool {
        !self.decisions.iter().any(|d| {
            matches!(d.get_kind(), DecisionKind::Ambiguous { .. })
        })
    }

    /// Produce a summary of the decision log.
    pub fn summary(&self) -> DecisionSummary {
        let mut summary = DecisionSummary {
            total: self.decisions.len(),
            exact: 0,
            policy_applied: 0,
            near_boundary: 0,
            ambiguous: 0,
            forced: 0,
            min_margin: f64::INFINITY,
        };

        for d in &self.decisions {
            match d.get_kind() {
                DecisionKind::Exact => summary.exact += 1,
                DecisionKind::PolicyApplied { .. } => summary.policy_applied += 1,
                DecisionKind::NearBoundary { .. } => summary.near_boundary += 1,
                DecisionKind::Ambiguous { .. } => summary.ambiguous += 1,
                DecisionKind::Forced { .. } => summary.forced += 1,
            }
            if d.get_margin() < summary.min_margin {
                summary.min_margin = d.get_margin();
            }
        }

        if summary.total == 0 {
            summary.min_margin = 0.0;
        }

        summary
    }

    /// Merge another log into this one (for aggregation across sub-operations).
    pub fn merge(&mut self, other: DecisionLog) {
        self.decisions.extend(other.decisions);
    }

    /// Number of decisions recorded.
    pub fn len(&self) -> usize {
        self.decisions.len()
    }

    /// Whether the log is empty.
    pub fn is_empty(&self) -> bool {
        self.decisions.is_empty()
    }

    /// Format the log without low-level Euler operator decisions.
    ///
    /// Returns a string showing only high-level decisions (classify, select,
    /// split summary, disjoint containment). Euler ops tagged with
    /// `feature_scope == EULER_OP_FEATURE_SCOPE` are excluded.
    pub fn display_compact(&self) -> String {
        use std::fmt::Write;
        let high_level: Vec<&TracedDecision> = self.decisions.iter()
            .filter(|d| d.get_feature_scope() != Some(EULER_OP_FEATURE_SCOPE))
            .collect();
        let mut out = String::new();
        let _ = writeln!(out, "{} decisions ({} high-level, {} euler ops)",
            self.decisions.len(), high_level.len(),
            self.decisions.len() - high_level.len());
        for d in &high_level {
            let _ = writeln!(out, "  {}", d);
        }
        out
    }
}

impl fmt::Display for DecisionLog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let summary = self.summary();
        writeln!(f, "{}", summary)?;
        for d in &self.decisions {
            writeln!(f, "  {}", d)?;
        }
        Ok(())
    }
}

// =========================================================================
// DECISION SUMMARY
// =========================================================================

/// Aggregate statistics over a `DecisionLog`.
///
/// The quick check: if `ambiguous == 0`, the operation made no assumptions
/// that need review.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DecisionSummary {
    /// Total number of decisions.
    pub total: usize,
    /// Decisions resolved exactly (zero ambiguity).
    pub exact: usize,
    /// Decisions resolved by a configured policy.
    pub policy_applied: usize,
    /// Decisions near a threshold boundary (logged for transparency).
    pub near_boundary: usize,
    /// Decisions where a safe fallback was applied (needs review).
    pub ambiguous: usize,
    /// Decisions forced by hard constraints.
    pub forced: usize,
    /// Smallest margin across all decisions (most marginal).
    pub min_margin: f64,
}

impl fmt::Display for DecisionSummary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} decisions ({} exact, {} policy, {} near-boundary, {} ambiguous, {} forced, min_margin={:.2e})",
            self.total, self.exact, self.policy_applied, self.near_boundary,
            self.ambiguous, self.forced, self.min_margin)
    }
}

// =========================================================================
// KERNEL WARNING
// =========================================================================

/// Non-fatal warning emitted during an operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KernelWarning {
    /// A sliver face was created (area below threshold).
    SliverFaceCreated {
        /// Index of the sliver face.
        face_index: u32,
        /// Computed area of the face.
        area: f64,
        /// Threshold below which a face is a sliver.
        threshold: f64,
    },
    /// A near-degenerate edge was created (length below threshold).
    ShortEdgeCreated {
        /// Index of the short edge.
        halfedge_index: u32,
        /// Computed length.
        length: f64,
        /// Minimum acceptable length threshold.
        threshold: f64,
    },
    /// A tolerance decision was made automatically.
    AutoDecision {
        /// The decision that was auto-applied.
        decision_id: DecisionId,
    },
}

impl fmt::Display for KernelWarning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KernelWarning::SliverFaceCreated { face_index, area, threshold } => {
                write!(f, "Sliver face {} (area {:.2e}, threshold {:.2e})", face_index, area, threshold)
            }
            KernelWarning::ShortEdgeCreated { halfedge_index, length, threshold } => {
                write!(f, "Short edge {} (length {:.2e}, threshold {:.2e})", halfedge_index, length, threshold)
            }
            KernelWarning::AutoDecision { decision_id } => {
                write!(f, "Automatic tolerance decision: {}", decision_id)
            }
        }
    }
}

// =========================================================================
// OPERATION METRICS
// =========================================================================

/// Performance and accounting metrics for an operation.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OperationMetrics {
    /// Wall-clock duration of the operation.
    pub duration: Duration,
    /// Number of entities created during the operation.
    pub entities_created: u32,
    /// Number of entities deleted during the operation.
    pub entities_deleted: u32,
    /// Number of entities modified during the operation.
    pub entities_modified: u32,
    /// Number of exact predicate evaluations.
    pub exact_predicate_calls: u64,
    /// Number of policy-driven decisions made.
    pub policy_decisions_made: u32,
}

// =========================================================================
// LINEAGE DELTA
// =========================================================================

/// Summary of lineage changes from an operation.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LineageDelta {
    /// Number of faces created.
    pub faces_created: u32,
    /// Number of faces deleted.
    pub faces_deleted: u32,
    /// Number of half-edges created.
    pub half_edges_created: u32,
    /// Number of half-edges deleted.
    pub half_edges_deleted: u32,
    /// Number of vertices created.
    pub vertices_created: u32,
    /// Number of vertices deleted.
    pub vertices_deleted: u32,
}

// =========================================================================
// OPERATION RESULT (Universal Envelope)
// =========================================================================

/// Universal envelope wrapping every kernel operation's return value.
///
/// Carries the primary result alongside a queryable decision log,
/// warnings, performance metrics, lineage changes, and topology hashes.
/// An AI agent can reconstruct the full state transition from this
/// envelope alone.
///
/// # Example
/// ```
/// use forge_core::result::{OperationResult, OperationMetrics, LineageDelta};
///
/// let result: OperationResult<i32> = OperationResult::new(42);
/// assert_eq!(*result.get_value(), 42);
/// assert!(result.get_decision_log().is_empty());
/// assert!(result.get_decision_log().is_clean());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationResult<T> {
    /// The primary return value.
    value: T,
    /// Non-fatal warnings emitted during the operation.
    warnings: Vec<KernelWarning>,
    /// Full decision trace for this operation.
    decision_log: DecisionLog,
    /// Performance and accounting metrics.
    metrics: OperationMetrics,
    /// Summary of lineage changes.
    lineage_delta: LineageDelta,
    /// Topology hash before the operation.
    state_hash_before: u128,
    /// Topology hash after the operation.
    state_hash_after: u128,
}

impl<T> OperationResult<T> {
    /// Create a new operation result with empty metadata.
    pub fn new(value: T) -> Self {
        Self {
            value,
            warnings: Vec::new(),
            decision_log: DecisionLog::new(),
            metrics: OperationMetrics::default(),
            lineage_delta: LineageDelta::default(),
            state_hash_before: 0,
            state_hash_after: 0,
        }
    }

    /// Create an operation result with full metadata.
    pub fn with_metadata(
        value: T,
        warnings: Vec<KernelWarning>,
        decision_log: DecisionLog,
        metrics: OperationMetrics,
        lineage_delta: LineageDelta,
        state_hash_before: u128,
        state_hash_after: u128,
    ) -> Self {
        Self { value, warnings, decision_log, metrics, lineage_delta, state_hash_before, state_hash_after }
    }

    /// The primary return value of the operation.
    pub fn get_value(&self) -> &T {
        &self.value
    }

    /// Consume the result and return the inner value.
    pub fn into_value(self) -> T {
        self.value
    }

    /// Non-fatal warnings emitted during the operation.
    pub fn get_warnings(&self) -> &[KernelWarning] {
        &self.warnings
    }

    /// The full decision log.
    pub fn get_decision_log(&self) -> &DecisionLog {
        &self.decision_log
    }

    /// Mutable access to the decision log (for populating during execution).
    pub fn get_decision_log_mut(&mut self) -> &mut DecisionLog {
        &mut self.decision_log
    }

    /// Performance metrics for the operation.
    pub fn get_metrics(&self) -> &OperationMetrics {
        &self.metrics
    }

    /// Summary of lineage changes from the operation.
    pub fn get_lineage_delta(&self) -> &LineageDelta {
        &self.lineage_delta
    }

    /// Topology hash before the operation.
    pub fn get_state_hash_before(&self) -> u128 {
        self.state_hash_before
    }

    /// Topology hash after the operation.
    pub fn get_state_hash_after(&self) -> u128 {
        self.state_hash_after
    }

    /// Whether any warnings were emitted.
    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }

    /// Whether any decisions were recorded.
    pub fn has_decisions(&self) -> bool {
        !self.decision_log.is_empty()
    }

    /// Set the operation metrics.
    pub fn set_metrics(&mut self, metrics: OperationMetrics) {
        self.metrics = metrics;
    }

    /// Set the lineage delta.
    pub fn set_lineage_delta(&mut self, delta: LineageDelta) {
        self.lineage_delta = delta;
    }

    /// Set the topology hash before the operation.
    pub fn set_state_hash_before(&mut self, hash: u128) {
        self.state_hash_before = hash;
    }

    /// Set the topology hash after the operation.
    pub fn set_state_hash_after(&mut self, hash: u128) {
        self.state_hash_after = hash;
    }

    /// Set the decision log.
    pub fn set_decision_log(&mut self, log: DecisionLog) {
        self.decision_log = log;
    }

    /// Add a warning.
    pub fn add_warning(&mut self, warning: KernelWarning) {
        self.warnings.push(warning);
    }

    /// Transform the inner value while preserving all metadata.
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> OperationResult<U> {
        OperationResult {
            value: f(self.value),
            warnings: self.warnings,
            decision_log: self.decision_log,
            metrics: self.metrics,
            lineage_delta: self.lineage_delta,
            state_hash_before: self.state_hash_before,
            state_hash_after: self.state_hash_after,
        }
    }
}

// =========================================================================
// TEST LOGGING (Universal verbosity-controlled output)
// =========================================================================

/// Verbosity level for test decision log output.
///
/// Controlled by the `FORGE_LOG` environment variable:
/// - `off`     → `Off` (silent, CI-friendly)
/// - `compact` → `Compact` (default; summary + high-level decisions)
/// - `full`    → `Full` (everything including Euler ops)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    /// No output.
    Off,
    /// Summary + high-level decisions (no Euler ops).
    Compact,
    /// Full decision log including Euler operator decisions.
    Full,
}

/// Read the current log level from the `FORGE_LOG` env var.
///
/// Returns `Compact` if the variable is unset or has an unrecognized value.
pub fn log_level() -> LogLevel {
    match std::env::var("FORGE_LOG").as_deref() {
        Ok("full") | Ok("FULL") => LogLevel::Full,
        Ok("off") | Ok("OFF") => LogLevel::Off,
        _ => LogLevel::Compact,
    }
}

/// Log the decision log from an `OperationResult` at the current verbosity.
///
/// Writes to stderr so output appears with `--nocapture` but doesn't
/// pollute stdout assertions.
pub fn log_result<T>(label: &str, result: &OperationResult<T>) {
    log_decision_log(label, result.get_decision_log());
}

/// Log a raw `DecisionLog` at the current verbosity.
///
/// Use this when you have a `DecisionLog` without an `OperationResult` envelope.
pub fn log_decision_log(label: &str, log: &DecisionLog) {
    match log_level() {
        LogLevel::Off => {}
        LogLevel::Compact => eprint!("[{}] {}", label, log.display_compact()),
        LogLevel::Full => eprint!("[{}] {}", label, log),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operation_result_new_has_empty_metadata() {
        let result: OperationResult<i32> = OperationResult::new(42);
        assert_eq!(*result.get_value(), 42);
        assert!(result.get_warnings().is_empty());
        assert!(result.get_decision_log().is_empty());
        assert_eq!(result.get_metrics().entities_created, 0);
        assert!(!result.has_warnings());
        assert!(!result.has_decisions());
        assert_eq!(result.get_state_hash_before(), 0);
        assert_eq!(result.get_state_hash_after(), 0);
    }

    #[test]
    fn operation_result_into_value() {
        let result = OperationResult::new(String::from("hello"));
        let value = result.into_value();
        assert_eq!(value, "hello");
    }

    #[test]
    fn operation_result_add_warning() {
        let mut result = OperationResult::new(0);
        result.add_warning(KernelWarning::SliverFaceCreated {
            face_index: 3,
            area: 1e-12,
            threshold: 1e-10,
        });
        assert!(result.has_warnings());
        assert_eq!(result.get_warnings().len(), 1);
    }

    #[test]
    fn traced_decision_creation() {
        let decision = TracedDecision::new(
            DecisionId(1),
            DecisionKind::Exact,
            0.5,
            DecisionContext::Tolerance { measured: 1e-8, threshold: 1e-6 },
        );
        assert_eq!(decision.get_id(), DecisionId(1));
        assert_eq!(*decision.get_kind(), DecisionKind::Exact);
        assert!(decision.is_overridable());
        assert_eq!(decision.get_margin(), 0.5);
    }

    #[test]
    fn decision_id_display() {
        let id = DecisionId(42);
        assert_eq!(format!("{}", id), "decision-42");
    }

    #[test]
    fn decision_log_query_api() {
        let mut log = DecisionLog::new();

        log.record(TracedDecision::new(
            DecisionId(1),
            DecisionKind::Exact,
            1.0,
            DecisionContext::Tolerance { measured: 0.0, threshold: 1e-6 },
        ));
        log.record(TracedDecision::new(
            DecisionId(2),
            DecisionKind::Ambiguous { fallback_applied: "snap_to_edge".to_string() },
            0.001,
            DecisionContext::Tolerance { measured: 9e-7, threshold: 1e-6 },
        ));
        log.record(TracedDecision::new(
            DecisionId(3),
            DecisionKind::NearBoundary { threshold: 1e-6 },
            0.1,
            DecisionContext::Tolerance { measured: 8e-7, threshold: 1e-6 },
        ));
        log.record(TracedDecision::new(
            DecisionId(4),
            DecisionKind::PolicyApplied {
                policy: PolicyKind::CoincidentGeometry,
                default_used: true,
            },
            0.05,
            DecisionContext::Coincidence {
                entity_a: EntityRef::new("Vertex", 0),
                entity_b: EntityRef::new("Vertex", 1),
            },
        ));

        assert_eq!(log.len(), 4);
        assert!(!log.is_clean());
        assert_eq!(log.ambiguous_only().len(), 1);
        assert_eq!(log.ambiguous_only()[0].get_id(), DecisionId(2));

        let by_margin = log.by_margin_ascending();
        assert_eq!(by_margin[0].get_id(), DecisionId(2));
        assert_eq!(by_margin[1].get_id(), DecisionId(4));

        let summary = log.summary();
        assert_eq!(summary.total, 4);
        assert_eq!(summary.exact, 1);
        assert_eq!(summary.ambiguous, 1);
        assert_eq!(summary.near_boundary, 1);
        assert_eq!(summary.policy_applied, 1);
        assert_eq!(summary.forced, 0);
        assert!((summary.min_margin - 0.001).abs() < 1e-10);
    }

    #[test]
    fn decision_log_is_clean_when_no_ambiguous() {
        let mut log = DecisionLog::new();
        log.record(TracedDecision::new(
            DecisionId(1),
            DecisionKind::Exact,
            1.0,
            DecisionContext::Tolerance { measured: 0.0, threshold: 1e-6 },
        ));
        assert!(log.is_clean());
    }

    #[test]
    fn decision_log_merge() {
        let mut log_a = DecisionLog::new();
        log_a.record(TracedDecision::new(
            DecisionId(1), DecisionKind::Exact, 1.0,
            DecisionContext::Tolerance { measured: 0.0, threshold: 1e-6 },
        ));

        let mut log_b = DecisionLog::new();
        log_b.record(TracedDecision::new(
            DecisionId(2), DecisionKind::Exact, 0.5,
            DecisionContext::Tolerance { measured: 0.0, threshold: 1e-6 },
        ));

        log_a.merge(log_b);
        assert_eq!(log_a.len(), 2);
    }

    #[test]
    fn with_metadata_constructor() {
        let log = DecisionLog::new();
        let result = OperationResult::with_metadata(
            99,
            vec![KernelWarning::AutoDecision { decision_id: DecisionId(1) }],
            log,
            OperationMetrics { duration: Duration::from_millis(5), entities_created: 3, entities_deleted: 1, entities_modified: 0, exact_predicate_calls: 10, policy_decisions_made: 2 },
            LineageDelta { faces_created: 1, ..LineageDelta::default() },
            0xAABB,
            0xCCDD,
        );
        assert_eq!(*result.get_value(), 99);
        assert!(result.has_warnings());
        assert_eq!(result.get_metrics().entities_created, 3);
        assert_eq!(result.get_metrics().exact_predicate_calls, 10);
        assert_eq!(result.get_lineage_delta().faces_created, 1);
        assert_eq!(result.get_state_hash_before(), 0xAABB);
        assert_eq!(result.get_state_hash_after(), 0xCCDD);
    }

    #[test]
    fn serde_roundtrip_operation_result() {
        let mut log = DecisionLog::new();
        log.record(TracedDecision::new(
            DecisionId(1),
            DecisionKind::PolicyApplied {
                policy: PolicyKind::CoincidentGeometry,
                default_used: false,
            },
            0.42,
            DecisionContext::Coincidence {
                entity_a: EntityRef::new("Vertex", 10),
                entity_b: EntityRef::new("Vertex", 20),
            },
        ));
        log.record(TracedDecision::new(
            DecisionId(2),
            DecisionKind::Ambiguous { fallback_applied: "merge".to_string() },
            0.001,
            DecisionContext::Tolerance { measured: 9.5e-7, threshold: 1e-6 },
        ));

        let result = OperationResult::with_metadata(
            42_i32,
            vec![KernelWarning::SliverFaceCreated { face_index: 5, area: 1e-12, threshold: 1e-10 }],
            log,
            OperationMetrics {
                duration: Duration::from_micros(1234),
                entities_created: 6,
                entities_deleted: 2,
                entities_modified: 1,
                exact_predicate_calls: 100,
                policy_decisions_made: 3,
            },
            LineageDelta { faces_created: 4, vertices_created: 8, ..LineageDelta::default() },
            0x1234_5678_9ABC_DEF0,
            0xFEDC_BA98_7654_3210,
        );

        let json = serde_json::to_string(&result).expect("serialize");
        let restored: OperationResult<i32> = serde_json::from_str(&json).expect("deserialize");

        assert_eq!(*restored.get_value(), 42);
        assert_eq!(restored.get_decision_log().len(), 2);
        assert_eq!(restored.get_decision_log().get_all()[0].get_id(), DecisionId(1));
        assert_eq!(restored.get_decision_log().get_all()[1].get_id(), DecisionId(2));
        assert!(!restored.get_decision_log().is_clean());
        assert_eq!(restored.get_warnings().len(), 1);
        assert_eq!(restored.get_metrics().exact_predicate_calls, 100);
        assert_eq!(restored.get_lineage_delta().faces_created, 4);
        assert_eq!(restored.get_state_hash_before(), 0x1234_5678_9ABC_DEF0);
        assert_eq!(restored.get_state_hash_after(), 0xFEDC_BA98_7654_3210);

        let summary = restored.get_decision_log().summary();
        assert_eq!(summary.total, 2);
        assert_eq!(summary.policy_applied, 1);
        assert_eq!(summary.ambiguous, 1);
    }

    #[test]
    fn operation_result_map_preserves_metadata() {
        let mut result = OperationResult::new(10);
        result.set_state_hash_before(0xAA);
        result.set_state_hash_after(0xBB);
        result.add_warning(KernelWarning::AutoDecision { decision_id: DecisionId(1) });

        let mapped = result.map(|v| v * 2);
        assert_eq!(*mapped.get_value(), 20);
        assert_eq!(mapped.get_state_hash_before(), 0xAA);
        assert_eq!(mapped.get_state_hash_after(), 0xBB);
        assert!(mapped.has_warnings());
    }
}

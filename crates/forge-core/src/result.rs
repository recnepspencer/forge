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
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
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
/// Decisions tagged with this scope are filtered out in compact display
/// and only shown in verbose/full display mode.
pub const EULER_OP_FEATURE_SCOPE: u64 = u64::MAX;

// =========================================================================
// SPAN ID
// =========================================================================

/// Unique identifier for a trace span within a `DecisionLog`.
///
/// Monotonically increasing within a single log instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SpanId(pub u64);

impl fmt::Display for SpanId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "span-{}", self.0)
    }
}

// =========================================================================
// DECISION TIER (significance classification)
// =========================================================================

/// Significance tier for a kernel decision.
///
/// Set at record-time by the caller, not inferred by the view layer.
/// `Ord` is derived so `tier_at_least()` uses simple comparison.
/// Deterministic < Resolved < NearBoundary < PolicyApplied < Escalated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum DecisionTier {
    /// Tier 0: Predicate resolved exactly. Zero agent value.
    Deterministic,
    /// Tier 1: Unambiguous but involved a tolerance comparison. Auditable.
    Resolved,
    /// Tier 2: Result correct but margin is small. Brittle.
    NearBoundary,
    /// Tier 3: Kernel applied a fallback policy. Agent can override.
    PolicyApplied,
    /// Tier 4: Kernel could not proceed. Agent must act.
    Escalated,
}

impl fmt::Display for DecisionTier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecisionTier::Deterministic => write!(f, "deterministic"),
            DecisionTier::Resolved => write!(f, "resolved"),
            DecisionTier::NearBoundary => write!(f, "near-boundary"),
            DecisionTier::PolicyApplied => write!(f, "policy-applied"),
            DecisionTier::Escalated => write!(f, "escalated"),
        }
    }
}

// =========================================================================
// TRACE EVENT (protocol event for span-based tracing)
// =========================================================================

/// A single event in the trace protocol.
///
/// The `DecisionLog` stores a flat `Vec<TraceEvent>`. Tree structure is
/// reconstructed on read by matching `StartSpan`/`EndSpan` pairs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TraceEvent {
    /// An atomic kernel decision.
    Decision(TracedDecision),
    /// Start of a named scope (logical phase).
    StartSpan {
        /// Unique span identifier.
        id: SpanId,
        /// Parent span, if nested.
        parent_id: Option<SpanId>,
        /// Human-readable phase name.
        name: String,
    },
    /// End of a named scope, with computed duration.
    EndSpan {
        /// Must match a previous `StartSpan.id`.
        id: SpanId,
        /// Wall-clock duration in microseconds.
        duration_micros: u64,
    },
}

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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
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
    /// Significance tier (set at record-time).
    tier: DecisionTier,
    /// How close to the threshold (lower = more marginal).
    margin: f64,
    /// Feature that produced this decision (if any).
    feature_scope: Option<u64>,
    /// Entity this decision applies to (if any).
    entity_scope: Option<EntityRef>,
    /// Whether the caller can override this decision.
    overridable: bool,
    /// Structured context for what triggered this decision.
    context: DecisionContext,
    /// The span this decision was recorded in (stamped automatically).
    #[serde(default)]
    span_id: Option<SpanId>,
}

impl TracedDecision {
    /// Create a new traced decision with explicit tier.
    pub fn new(
        id: DecisionId,
        kind: DecisionKind,
        tier: DecisionTier,
        margin: f64,
        context: DecisionContext,
    ) -> Self {
        Self {
            id,
            kind,
            tier,
            margin,
            feature_scope: None,
            entity_scope: None,
            overridable: true,
            context,
            span_id: None,
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

    /// The significance tier.
    pub fn get_tier(&self) -> DecisionTier {
        self.tier
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

    /// The span this decision was recorded in.
    pub fn get_span_id(&self) -> Option<SpanId> {
        self.span_id
    }

    /// Set the span this decision belongs to (called by DecisionLog::record).
    pub fn set_span_id(&mut self, span_id: SpanId) {
        self.span_id = Some(span_id);
    }
}

impl fmt::Display for TracedDecision {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] [{}] {} margin={:.2e}", self.id, self.tier, self.kind, self.margin)?;
        if let Some(span) = self.span_id {
            write!(f, " {}", span)?;
        }
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

/// Span-aware, queryable collection of trace events.
///
/// Stores a flat `Vec<TraceEvent>` for performance. Tree structure is
/// reconstructed on read by matching `StartSpan`/`EndSpan` pairs.
/// An ephemeral span stack tracks the current active span during recording.
#[derive(Debug, Clone, Serialize)]
pub struct DecisionLog {
    /// All events: decisions + span start/end markers.
    events: Vec<TraceEvent>,
    /// Ephemeral span stack (not serialized).
    #[serde(skip)]
    span_stack: Vec<SpanId>,
    /// Monotonic span counter (not serialized).
    #[serde(skip)]
    span_counter: u64,
    /// Cached count of decisions (excludes span events).
    #[serde(skip)]
    decision_count: usize,
    /// O(1) span ID → name lookup.
    #[serde(skip)]
    span_names: std::collections::HashMap<SpanId, String>,
    /// O(1) decision ID → event index lookup.
    #[serde(skip)]
    decision_index: std::collections::HashMap<DecisionId, usize>,
    /// Running summary, updated incrementally on each `record()` call.
    #[serde(skip)]
    running_summary: DecisionSummary,
}

impl<'de> Deserialize<'de> for DecisionLog {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        /// Minimal struct for deserialization (only the serialized field).
        #[derive(Deserialize)]
        struct RawLog {
            events: Vec<TraceEvent>,
        }

        let raw = RawLog::deserialize(deserializer)?;
        let mut log = DecisionLog {
            events: raw.events,
            span_stack: Vec::new(),
            span_counter: 0,
            decision_count: 0,
            span_names: std::collections::HashMap::new(),
            decision_index: std::collections::HashMap::new(),
            running_summary: DecisionSummary::empty(),
        };
        log.rebuild_indexes();
        Ok(log)
    }
}

impl Default for DecisionLog {
    fn default() -> Self {
        Self::new()
    }
}

impl DecisionLog {
    /// Create an empty decision log.
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            span_stack: Vec::new(),
            span_counter: 0,
            decision_count: 0,
            span_names: std::collections::HashMap::new(),
            decision_index: std::collections::HashMap::new(),
            running_summary: DecisionSummary::empty(),
        }
    }

    /// Start a named span. Returns the `SpanId` for the caller to close.
    pub fn start_span(&mut self, name: &str) -> SpanId {
        self.span_counter += 1;
        let id = SpanId(self.span_counter);
        let parent_id = self.span_stack.last().copied();
        self.span_names.insert(id, name.to_string());
        self.events.push(TraceEvent::StartSpan {
            id,
            parent_id,
            name: name.to_string(),
        });
        self.span_stack.push(id);
        id
    }

    /// End a span, recording its wall-clock duration.
    ///
    /// Handles mismatched closes: if `id` is not the top of the stack,
    /// truncates to (and removes) the matching span entry.
    pub fn end_span(&mut self, id: SpanId, duration_micros: u64) {
        self.events.push(TraceEvent::EndSpan { id, duration_micros });
        if let Some(pos) = self.span_stack.iter().rposition(|s| *s == id) {
            self.span_stack.truncate(pos);
        }
    }

    /// The currently active span, if any.
    pub fn active_span(&self) -> Option<SpanId> {
        self.span_stack.last().copied()
    }

    /// Record a decision, stamping it with the active span.
    pub fn record(&mut self, mut decision: TracedDecision) {
        if let Some(span_id) = self.active_span() {
            decision.set_span_id(span_id);
        }
        let event_idx = self.events.len();
        self.decision_index.insert(decision.get_id(), event_idx);
        self.running_summary.incorporate(&decision);
        self.decision_count += 1;
        self.events.push(TraceEvent::Decision(decision));
    }

    /// All trace events (decisions + spans).
    pub fn get_events(&self) -> &[TraceEvent] {
        &self.events
    }

    /// Iterator over only the decisions (skipping span events).
    pub fn decisions(&self) -> impl Iterator<Item = &TracedDecision> {
        self.events.iter().filter_map(|e| match e {
            TraceEvent::Decision(d) => Some(d),
            _ => None,
        })
    }

    /// Decisions at or above a given tier.
    pub fn tier_at_least(&self, min_tier: DecisionTier) -> Vec<&TracedDecision> {
        self.decisions().filter(|d| d.get_tier() >= min_tier).collect()
    }

    /// Decisions at Tier 2 (NearBoundary) or above.
    pub fn interesting_only(&self) -> Vec<&TracedDecision> {
        self.tier_at_least(DecisionTier::NearBoundary)
    }

    /// Look up a decision by ID (O(1) via index).
    pub fn get_by_id(&self, id: DecisionId) -> Option<&TracedDecision> {
        let &idx = self.decision_index.get(&id)?;
        match self.events.get(idx) {
            Some(TraceEvent::Decision(d)) => Some(d),
            _ => None,
        }
    }

    /// Decisions sorted by margin ascending (most marginal first).
    pub fn by_margin_ascending(&self) -> Vec<&TracedDecision> {
        let mut refs: Vec<&TracedDecision> = self.decisions().collect();
        refs.sort_by(|a, b| {
            a.get_margin().partial_cmp(&b.get_margin())
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        refs
    }

    /// Only decisions with `DecisionKind::Ambiguous`.
    pub fn ambiguous_only(&self) -> Vec<&TracedDecision> {
        self.decisions().filter(|d| {
            matches!(d.get_kind(), DecisionKind::Ambiguous { .. })
        }).collect()
    }

    /// Only decisions that are overridable.
    pub fn overridable_only(&self) -> Vec<&TracedDecision> {
        self.decisions().filter(|d| d.is_overridable()).collect()
    }

    /// Returns `true` if there are zero `Ambiguous` decisions.
    pub fn is_clean(&self) -> bool {
        !self.decisions().any(|d| {
            matches!(d.get_kind(), DecisionKind::Ambiguous { .. })
        })
    }

    /// Produce a summary counting decisions by kind (O(1) via cached running summary).
    pub fn summary(&self) -> DecisionSummary {
        self.running_summary.clone()
    }

    /// Merge another log into this one (for aggregation across sub-operations).
    pub fn merge(&mut self, other: DecisionLog) {
        self.events.extend(other.events);
        self.rebuild_indexes();
    }

    /// Number of decisions recorded (O(1) via cached count).
    pub fn len(&self) -> usize {
        self.decision_count
    }

    /// Whether the log contains zero decisions (O(1)).
    pub fn is_empty(&self) -> bool {
        self.decision_count == 0
    }

    /// Extract a `TraceSummary` for diffing across evaluations.
    pub fn to_summary(&self, state_hash: u128) -> TraceSummary {
        let interesting: Vec<TracedDecision> = self.decisions()
            .filter(|d| d.get_tier() >= DecisionTier::NearBoundary)
            .cloned()
            .collect();

        let span_summaries = self.compute_span_summaries();

        TraceSummary {
            state_hash,
            interesting,
            span_summaries,
        }
    }

    /// Display using the Inverted Noise Rule: show only spans that contain
    /// Tier 2+ decisions. Boring spans are collapsed to a one-liner.
    pub fn display_interesting(&self) -> String {
        use std::fmt::Write;
        let mut out = String::new();
        let total = self.len();
        let interesting = self.interesting_only();
        let _ = writeln!(out, "{} decisions ({} interesting)", total, interesting.len());

        let span_summaries = self.compute_span_summaries();
        for ss in &span_summaries {
            if ss.max_tier >= DecisionTier::NearBoundary {
                let _ = writeln!(out, "  ▸ {} ({} decisions, max={}, {}µs)",
                    ss.name, ss.total_decisions, ss.max_tier, ss.duration_micros);
                for d in self.decisions() {
                    if d.get_span_id().map(|s| self.span_name(s).as_deref() == Some(ss.name.as_str())).unwrap_or(false)
                        && d.get_tier() >= DecisionTier::NearBoundary
                    {
                        let _ = writeln!(out, "    {}", d);
                    }
                }
            } else {
                let _ = writeln!(out, "  ▹ {} ({} decisions, clean)", ss.name, ss.total_decisions);
            }
        }

        for d in &interesting {
            if d.get_span_id().is_none() {
                let _ = writeln!(out, "  {}", d);
            }
        }

        out
    }

    /// Resolve a span ID to its name (O(1) via cached index).
    fn span_name(&self, id: SpanId) -> Option<String> {
        self.span_names.get(&id).cloned()
    }

    /// Compute per-span aggregate statistics (uses cached span_names for O(1) name lookup).
    fn compute_span_summaries(&self) -> Vec<SpanSummaryEntry> {
        let mut spans: Vec<SpanSummaryEntry> = Vec::new();
        let mut span_idx: std::collections::HashMap<SpanId, usize> = std::collections::HashMap::new();

        for event in &self.events {
            if let TraceEvent::StartSpan { id, name, .. } = event {
                span_idx.insert(*id, spans.len());
                spans.push(SpanSummaryEntry {
                    span_id: *id,
                    name: name.clone(),
                    total_decisions: 0,
                    max_tier: DecisionTier::Deterministic,
                    duration_micros: 0,
                });
            }
        }

        for d in self.decisions() {
            if let Some(sid) = d.get_span_id() {
                if let Some(&idx) = span_idx.get(&sid) {
                    spans[idx].total_decisions += 1;
                    if d.get_tier() > spans[idx].max_tier {
                        spans[idx].max_tier = d.get_tier();
                    }
                }
            }
        }

        for event in &self.events {
            if let TraceEvent::EndSpan { id, duration_micros } = event {
                if let Some(&idx) = span_idx.get(id) {
                    spans[idx].duration_micros = *duration_micros;
                }
            }
        }

        spans
    }

    /// Rebuild all skip-serialization cached indexes from the events vec.
    ///
    /// Called after deserialization and after `merge()`.
    pub fn rebuild_indexes(&mut self) {
        self.decision_count = 0;
        self.span_names.clear();
        self.decision_index.clear();
        self.running_summary = DecisionSummary::empty();

        for (idx, event) in self.events.iter().enumerate() {
            match event {
                TraceEvent::Decision(d) => {
                    self.decision_count += 1;
                    self.decision_index.insert(d.get_id(), idx);
                    self.running_summary.incorporate(d);
                }
                TraceEvent::StartSpan { id, name, .. } => {
                    self.span_names.insert(*id, name.clone());
                    if id.0 >= self.span_counter {
                        self.span_counter = id.0 + 1;
                    }
                }
                TraceEvent::EndSpan { .. } => {}
            }
        }
    }
}

impl fmt::Display for DecisionLog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let summary = self.summary();
        writeln!(f, "{}", summary)?;
        for d in self.decisions() {
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
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
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

impl DecisionSummary {
    /// Create a zero-valued summary.
    pub fn empty() -> Self {
        Self {
            total: 0,
            exact: 0,
            policy_applied: 0,
            near_boundary: 0,
            ambiguous: 0,
            forced: 0,
            min_margin: 0.0,
        }
    }

    /// Incrementally update this summary with a new decision.
    pub fn incorporate(&mut self, d: &TracedDecision) {
        self.total += 1;
        match d.get_kind() {
            DecisionKind::Exact => self.exact += 1,
            DecisionKind::PolicyApplied { .. } => self.policy_applied += 1,
            DecisionKind::NearBoundary { .. } => self.near_boundary += 1,
            DecisionKind::Ambiguous { .. } => self.ambiguous += 1,
            DecisionKind::Forced { .. } => self.forced += 1,
        }
        if self.total == 1 || d.get_margin() < self.min_margin {
            self.min_margin = d.get_margin();
        }
    }
}

impl fmt::Display for DecisionSummary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} decisions ({} exact, {} policy, {} near-boundary, {} ambiguous, {} forced, min_margin={:.2e})",
            self.total, self.exact, self.policy_applied, self.near_boundary,
            self.ambiguous, self.forced, self.min_margin)
    }
}

// =========================================================================
// SPAN SUMMARY ENTRY
// =========================================================================

/// Per-span aggregate statistics for trace summaries.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpanSummaryEntry {
    /// The span this entry describes.
    pub span_id: SpanId,
    /// Human-readable span name.
    pub name: String,
    /// Number of decisions in this span.
    pub total_decisions: usize,
    /// Highest tier decision in this span.
    pub max_tier: DecisionTier,
    /// Wall-clock duration in microseconds.
    pub duration_micros: u64,
}

// =========================================================================
// TRACE SUMMARY (diffable snapshot)
// =========================================================================

/// Lightweight snapshot for diffing across evaluations.
///
/// Contains only Tier 2+ decisions and per-span stats.
/// Produced by `DecisionLog::to_summary()`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceSummary {
    /// Topology hash of the result (for O(1) equality check).
    state_hash: u128,
    /// Only Tier 2+ decisions (the "interesting" subset).
    interesting: Vec<TracedDecision>,
    /// Per-span aggregate stats.
    span_summaries: Vec<SpanSummaryEntry>,
}

impl TraceSummary {
    /// The topology state hash.
    pub fn get_state_hash(&self) -> u128 {
        self.state_hash
    }

    /// The interesting (Tier 2+) decisions.
    pub fn get_interesting(&self) -> &[TracedDecision] {
        &self.interesting
    }

    /// Per-span summaries.
    pub fn get_span_summaries(&self) -> &[SpanSummaryEntry] {
        &self.span_summaries
    }

    /// Diff this summary against another (typically the previous evaluation).
    pub fn diff(&self, other: &TraceSummary) -> TraceDiff {
        use std::collections::BTreeMap;
        let state_hash_changed = self.state_hash != other.state_hash;

        let old_by_id: BTreeMap<DecisionId, &TracedDecision> =
            other.interesting.iter().map(|d| (d.get_id(), d)).collect();
        let new_by_id: BTreeMap<DecisionId, &TracedDecision> =
            self.interesting.iter().map(|d| (d.get_id(), d)).collect();

        let mut added = Vec::new();
        let mut removed = Vec::new();
        let mut changed = Vec::new();

        for (id, new_d) in &new_by_id {
            match old_by_id.get(id) {
                None => added.push((*new_d).clone()),
                Some(old_d) => {
                    if old_d.get_tier() != new_d.get_tier()
                        || std::mem::discriminant(old_d.get_kind()) != std::mem::discriminant(new_d.get_kind())
                    {
                        changed.push(((*old_d).clone(), (*new_d).clone()));
                    }
                }
            }
        }

        for (id, old_d) in &old_by_id {
            if !new_by_id.contains_key(id) {
                removed.push((*old_d).clone());
            }
        }

        TraceDiff { added, removed, changed, state_hash_changed }
    }
}

// =========================================================================
// TRACE DIFF
// =========================================================================

/// Diff between two trace summaries.
///
/// Produced by `TraceSummary::diff()`. Shows what changed between
/// two evaluations of the same operation.
#[derive(Debug, Clone)]
pub struct TraceDiff {
    /// Decisions present in new but not old.
    pub added: Vec<TracedDecision>,
    /// Decisions present in old but not new.
    pub removed: Vec<TracedDecision>,
    /// Decisions where the tier or kind changed (old, new).
    pub changed: Vec<(TracedDecision, TracedDecision)>,
    /// Whether the state hash changed.
    pub state_hash_changed: bool,
}

impl TraceDiff {
    /// Whether the diff is empty (no changes in interesting decisions).
    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty() && self.changed.is_empty()
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
    ///
    /// If `FORGE_TRACE_DIR` is set, automatically persists the decision
    /// log as a JSON trace file before returning. This is the universal
    /// hook — every kernel operation that produces an `OperationResult`
    /// gets traced with zero wiring.
    pub fn into_value(self) -> T {
        self.maybe_persist_trace();
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

    /// Take ownership of the decision log, replacing it with an empty one.
    pub fn take_decision_log(&mut self) -> DecisionLog {
        std::mem::take(&mut self.decision_log)
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

    /// Persist the trace to disk if `FORGE_TRACE_DIR` is set.
    ///
    /// Uses `OnceLock` to check the env var exactly once per process.
    /// When the dir is set, writes a JSON file compatible with
    /// `forge_view::trace_store::TraceFile`.
    ///
    /// In debug/test builds, falls back to `workspace_root/traces/`
    /// (relative to this crate's compile-time `CARGO_MANIFEST_DIR`).
    fn maybe_persist_trace(&self) {
        let dir = match resolve_trace_dir() {
            Some(d) => d,
            None => return,
        };

        if self.decision_log.is_empty() {
            return;
        }

        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            write_trace_file(&dir, &self.decision_log, self.state_hash_after, "ok");
        }));
    }
}

/// Resolve the trace output directory (cached, checked once per process).
///
/// Priority:
/// 1. `FORGE_TRACE_DIR` env var (explicit override)
/// 2. In debug builds: `{workspace_root}/traces` (auto-detected from crate location)
/// 3. `None` in release builds without the env var
pub fn resolve_trace_dir() -> Option<PathBuf> {
    static TRACE_DIR: OnceLock<Option<PathBuf>> = OnceLock::new();

    TRACE_DIR.get_or_init(|| {
        if let Ok(dir) = std::env::var("FORGE_TRACE_DIR") {
            return Some(PathBuf::from(dir));
        }

        #[cfg(debug_assertions)]
        {
            let workspace_traces = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../traces");
            if workspace_traces.exists() || std::fs::create_dir_all(&workspace_traces).is_ok() {
                return workspace_traces.canonicalize().ok();
            }
        }

        None
    }).clone()
}

/// Write a trace JSON file to disk (infallible — silently drops errors).
pub fn write_trace_file(dir: &Path, log: &DecisionLog, state_hash: u128, status: &str) {
    if std::fs::create_dir_all(dir).is_err() {
        return;
    }

    let test_name = std::thread::current()
        .name()
        .unwrap_or("unknown")
        .replace("::", "_");

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);

    let filename = format!("{}_{}.json", test_name, timestamp);
    let path = dir.join(&filename);

    #[derive(Serialize)]
    struct TraceFilePayload<'a> {
        name: &'a str,
        timestamp: String,
        state_hash: u128,
        status: &'a str,
        log: &'a DecisionLog,
    }

    let payload = TraceFilePayload {
        name: &test_name,
        timestamp: format!("{}", timestamp),
        state_hash,
        status,
        log,
    };

    if let Ok(json) = serde_json::to_string_pretty(&payload) {
        let _ = std::fs::write(&path, json);
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
        LogLevel::Compact => eprint!("[{}] {}", label, log.display_interesting()),
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
            DecisionTier::Deterministic,
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
            DecisionTier::Deterministic,
            1.0,
            DecisionContext::Tolerance { measured: 0.0, threshold: 1e-6 },
        ));
        log.record(TracedDecision::new(
            DecisionId(2),
            DecisionKind::Ambiguous { fallback_applied: "snap_to_edge".to_string() },
            DecisionTier::Escalated,
            0.001,
            DecisionContext::Tolerance { measured: 9e-7, threshold: 1e-6 },
        ));
        log.record(TracedDecision::new(
            DecisionId(3),
            DecisionKind::NearBoundary { threshold: 1e-6 },
            DecisionTier::NearBoundary,
            0.1,
            DecisionContext::Tolerance { measured: 8e-7, threshold: 1e-6 },
        ));
        log.record(TracedDecision::new(
            DecisionId(4),
            DecisionKind::PolicyApplied {
                policy: PolicyKind::CoincidentGeometry,
                default_used: true,
            },
            DecisionTier::PolicyApplied,
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
            DecisionTier::Deterministic,
            1.0,
            DecisionContext::Tolerance { measured: 0.0, threshold: 1e-6 },
        ));
        assert!(log.is_clean());
    }

    #[test]
    fn decision_log_merge() {
        let mut log_a = DecisionLog::new();
        log_a.record(TracedDecision::new(
            DecisionId(1), DecisionKind::Exact, DecisionTier::Deterministic, 1.0,
            DecisionContext::Tolerance { measured: 0.0, threshold: 1e-6 },
        ));

        let mut log_b = DecisionLog::new();
        log_b.record(TracedDecision::new(
            DecisionId(2), DecisionKind::Exact, DecisionTier::Deterministic, 0.5,
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
            DecisionTier::PolicyApplied,
            0.42,
            DecisionContext::Coincidence {
                entity_a: EntityRef::new("Vertex", 10),
                entity_b: EntityRef::new("Vertex", 20),
            },
        ));
        log.record(TracedDecision::new(
            DecisionId(2),
            DecisionKind::Ambiguous { fallback_applied: "merge".to_string() },
            DecisionTier::Escalated,
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
        let decisions: Vec<_> = restored.get_decision_log().decisions().collect();
        assert_eq!(decisions[0].get_id(), DecisionId(1));
        assert_eq!(decisions[1].get_id(), DecisionId(2));
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

    // =====================================================================
    // Phase C: Span-Based Tracing Verification Tests
    // =====================================================================

    fn make_decision(id: u64, tier: DecisionTier, kind: DecisionKind) -> TracedDecision {
        TracedDecision::new(
            DecisionId(id),
            kind,
            tier,
            0.5,
            DecisionContext::Tolerance { measured: 1e-8, threshold: 1e-6 },
        )
    }

    #[test]
    fn mismatched_span_close_truncates_stack() {
        let mut log = DecisionLog::new();
        let outer = log.start_span("outer");
        let inner = log.start_span("inner");

        assert_eq!(log.active_span(), Some(inner));

        log.end_span(outer, 100);

        assert_eq!(log.active_span(), None, "Closing outer should truncate inner too");
    }

    #[test]
    fn closing_unknown_span_is_harmless() {
        let mut log = DecisionLog::new();
        let real = log.start_span("real");

        log.end_span(SpanId(999), 50);

        assert_eq!(log.active_span(), Some(real), "Unknown close should not affect stack");

        log.end_span(real, 100);
        assert_eq!(log.active_span(), None);
    }

    #[test]
    fn nested_spans_record_parent_ids() {
        let mut log = DecisionLog::new();
        let outer = log.start_span("outer");
        let inner = log.start_span("inner");
        let deepest = log.start_span("deepest");

        log.end_span(deepest, 10);
        log.end_span(inner, 20);
        log.end_span(outer, 30);

        let starts: Vec<_> = log.get_events().iter().filter_map(|e| match e {
            TraceEvent::StartSpan { id, parent_id, .. } => Some((*id, *parent_id)),
            _ => None,
        }).collect();

        assert_eq!(starts.len(), 3);
        assert_eq!(starts[0], (outer, None));
        assert_eq!(starts[1], (inner, Some(outer)));
        assert_eq!(starts[2], (deepest, Some(inner)));
    }

    #[test]
    fn decisions_stamped_with_active_span() {
        let mut log = DecisionLog::new();
        let span_a = log.start_span("phase_a");

        log.record(make_decision(1, DecisionTier::Deterministic, DecisionKind::Exact));
        log.end_span(span_a, 100);

        log.record(make_decision(2, DecisionTier::Deterministic, DecisionKind::Exact));

        let decisions: Vec<_> = log.decisions().collect();
        assert_eq!(decisions[0].get_span_id(), Some(span_a));
        assert_eq!(decisions[1].get_span_id(), None);
    }

    #[test]
    fn serde_roundtrip_resets_ephemeral_span_counter() {
        let mut log = DecisionLog::new();
        let _span = log.start_span("test");
        log.record(make_decision(1, DecisionTier::Deterministic, DecisionKind::Exact));
        log.end_span(_span, 100);

        let json = serde_json::to_string(&log).expect("serialize");
        let restored: DecisionLog = serde_json::from_str(&json).expect("deserialize");

        assert_eq!(restored.active_span(), None, "span_stack is ephemeral, should be empty");

        assert_eq!(
            restored.decisions().count(),
            log.decisions().count(),
            "Decisions should survive serde roundtrip",
        );

        let new_span = restored.clone();
        assert_eq!(new_span.get_events().len(), log.get_events().len());
    }

    #[test]
    fn tier_filtering_returns_only_tier_2_plus() {
        let mut log = DecisionLog::new();

        log.record(make_decision(1, DecisionTier::Deterministic, DecisionKind::Exact));
        log.record(make_decision(2, DecisionTier::NearBoundary,
            DecisionKind::NearBoundary { threshold: 1e-6 }));
        log.record(make_decision(3, DecisionTier::Escalated,
            DecisionKind::Ambiguous { fallback_applied: "snap".into() }));
        log.record(make_decision(4, DecisionTier::Deterministic, DecisionKind::Exact));

        let interesting = log.interesting_only();
        assert_eq!(interesting.len(), 2);
        assert_eq!(interesting[0].get_id(), DecisionId(2));
        assert_eq!(interesting[1].get_id(), DecisionId(3));
    }

    #[test]
    fn display_interesting_empty_for_boring_spans() {
        let mut log = DecisionLog::new();

        for i in 0..10 {
            let span = log.start_span(&format!("boring_{}", i));
            log.record(make_decision(i, DecisionTier::Deterministic, DecisionKind::Exact));
            log.end_span(span, 10);
        }

        let output = log.display_interesting();
        assert!(
            !output.contains("NearBoundary"),
            "All-boring log should have no interesting content in display",
        );
    }

    #[test]
    fn trace_summary_diff_detects_added_decisions() {
        let mut log_old = DecisionLog::new();
        log_old.record(make_decision(1, DecisionTier::NearBoundary,
            DecisionKind::NearBoundary { threshold: 1e-6 }));

        let mut log_new = DecisionLog::new();
        log_new.record(make_decision(1, DecisionTier::NearBoundary,
            DecisionKind::NearBoundary { threshold: 1e-6 }));
        log_new.record(make_decision(2, DecisionTier::Escalated,
            DecisionKind::Ambiguous { fallback_applied: "snap".into() }));

        let summary_old = log_old.to_summary(0xAAAA);
        let summary_new = log_new.to_summary(0xBBBB);

        let diff = summary_new.diff(&summary_old);

        assert!(diff.state_hash_changed);
        assert_eq!(diff.added.len(), 1);
        assert_eq!(diff.added[0].get_id(), DecisionId(2));
        assert!(diff.removed.is_empty());
        assert!(diff.changed.is_empty());
    }

    #[test]
    fn trace_summary_diff_detects_removed_decisions() {
        let mut log_old = DecisionLog::new();
        log_old.record(make_decision(1, DecisionTier::NearBoundary,
            DecisionKind::NearBoundary { threshold: 1e-6 }));
        log_old.record(make_decision(2, DecisionTier::Escalated,
            DecisionKind::Ambiguous { fallback_applied: "snap".into() }));

        let mut log_new = DecisionLog::new();
        log_new.record(make_decision(1, DecisionTier::NearBoundary,
            DecisionKind::NearBoundary { threshold: 1e-6 }));

        let summary_old = log_old.to_summary(0xAAAA);
        let summary_new = log_new.to_summary(0xAAAA);

        let diff = summary_new.diff(&summary_old);

        assert!(!diff.state_hash_changed);
        assert!(diff.added.is_empty());
        assert_eq!(diff.removed.len(), 1);
        assert_eq!(diff.removed[0].get_id(), DecisionId(2));
    }

    #[test]
    fn trace_summary_diff_detects_changed_tier() {
        let mut log_old = DecisionLog::new();
        log_old.record(make_decision(1, DecisionTier::NearBoundary,
            DecisionKind::NearBoundary { threshold: 1e-6 }));

        let mut log_new = DecisionLog::new();
        log_new.record(make_decision(1, DecisionTier::Escalated,
            DecisionKind::NearBoundary { threshold: 1e-6 }));

        let summary_old = log_old.to_summary(0xAAAA);
        let summary_new = log_new.to_summary(0xAAAA);

        let diff = summary_new.diff(&summary_old);

        assert!(diff.added.is_empty());
        assert!(diff.removed.is_empty());
        assert_eq!(diff.changed.len(), 1);
        assert_eq!(diff.changed[0].0.get_tier(), DecisionTier::NearBoundary);
        assert_eq!(diff.changed[0].1.get_tier(), DecisionTier::Escalated);
    }

    #[test]
    fn trace_summary_diff_identical_is_empty() {
        let mut log = DecisionLog::new();
        log.record(make_decision(1, DecisionTier::NearBoundary,
            DecisionKind::NearBoundary { threshold: 1e-6 }));
        log.record(make_decision(2, DecisionTier::Escalated,
            DecisionKind::Ambiguous { fallback_applied: "snap".into() }));

        let summary = log.to_summary(0xAAAA);

        let diff = summary.diff(&summary);

        assert!(!diff.state_hash_changed);
        assert!(diff.is_empty(), "Diffing a summary against itself should be empty");
    }

    #[test]
    fn empty_log_to_summary_is_empty() {
        let log = DecisionLog::new();
        let summary = log.to_summary(0);

        assert!(summary.get_interesting().is_empty());
        assert!(summary.get_span_summaries().is_empty());
    }
}

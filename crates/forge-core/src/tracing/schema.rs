//! Tracing data shapes — decision taxonomy and span protocol.

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::policy::PolicyKind;

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
// TRACED DECISION
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

//! Core traced decision record.
//!
//! DOMAIN: `TracedDecision` is the atomic unit of the tracing protocol.
//! Every kernel judgment call produces one. `DecisionId` uniquely identifies
//! it, and `TopologyDelta` captures any topology mutations it caused.

use std::fmt;

use serde::{Deserialize, Serialize};

use super::decision_kind::{DecisionContext, DecisionKind};
use super::decision_tier::DecisionTier;
use super::entity_ref::EntityRef;
use super::span::SpanId;

/// Unique identifier for a traced decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct DecisionId(pub u64);

impl fmt::Display for DecisionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "decision-{}", self.0)
    }
}

/// Topology entities created or deleted as a result of a decision.
///
/// Attached to `TracedDecision` when a decision directly mutates the
/// topology (e.g., splitting a face, removing an edge). Enables
/// answering "which entities did this decision create?"
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopologyDelta {
    /// Faces created as a result of this decision.
    pub created_faces: Vec<u32>,
    /// HalfEdges created as a result of this decision.
    pub created_halfedges: Vec<u32>,
    /// Vertices created as a result of this decision.
    pub created_vertices: Vec<u32>,
    /// Faces deleted as a result of this decision.
    pub deleted_faces: Vec<u32>,
    /// HalfEdges deleted as a result of this decision.
    pub deleted_halfedges: Vec<u32>,
    /// Vertices deleted as a result of this decision.
    pub deleted_vertices: Vec<u32>,
}

impl TopologyDelta {
    /// Create an empty topology delta.
    pub fn new() -> Self {
        Self {
            created_faces: Vec::new(),
            created_halfedges: Vec::new(),
            created_vertices: Vec::new(),
            deleted_faces: Vec::new(),
            deleted_halfedges: Vec::new(),
            deleted_vertices: Vec::new(),
        }
    }

    /// Whether this delta is empty (no topology changes).
    pub fn is_empty(&self) -> bool {
        self.created_faces.is_empty()
            && self.created_halfedges.is_empty()
            && self.created_vertices.is_empty()
            && self.deleted_faces.is_empty()
            && self.deleted_halfedges.is_empty()
            && self.deleted_vertices.is_empty()
    }
}

impl Default for TopologyDelta {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for TopologyDelta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_empty() {
            return write!(f, "Δ 0");
        }

        let mut parts = Vec::new();
        let f_diff = self.created_faces.len() as isize - self.deleted_faces.len() as isize;
        let he_diff = self.created_halfedges.len() as isize - self.deleted_halfedges.len() as isize;
        let v_diff = self.created_vertices.len() as isize - self.deleted_vertices.len() as isize;

        if f_diff > 0 {
            parts.push(format!("+{}F", f_diff));
        } else if f_diff < 0 {
            parts.push(format!("{}F", f_diff));
        }
        if he_diff > 0 {
            parts.push(format!("+{}HE", he_diff));
        } else if he_diff < 0 {
            parts.push(format!("{}HE", he_diff));
        }
        if v_diff > 0 {
            parts.push(format!("+{}V", v_diff));
        } else if v_diff < 0 {
            parts.push(format!("{}V", v_diff));
        }

        if parts.is_empty() {
            write!(f, "Δ changed")
        } else {
            write!(f, "Δ {}", parts.join(" "))
        }
    }
}

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
    /// Topology entities created or deleted by this decision.
    #[serde(default)]
    topology_delta: Option<TopologyDelta>,
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
            topology_delta: None,
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

    /// Topology entities created or deleted by this decision.
    pub fn get_topology_delta(&self) -> Option<&TopologyDelta> {
        self.topology_delta.as_ref()
    }

    /// Set the topology delta for this decision.
    pub fn set_topology_delta(&mut self, delta: TopologyDelta) {
        self.topology_delta = Some(delta);
    }
}

impl fmt::Display for TracedDecision {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] [{}] {} margin={:.2e}",
            self.id, self.tier, self.kind, self.margin
        )?;
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

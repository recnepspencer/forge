//! Counterfactual replay data shapes.
//!
//! DOMAIN: Represents the result of replaying a decision with mutated
//! inputs to test "what would have happened if this decision went
//! the other way?"
//!
//! DEPENDENCIES: `forge-core` (DecisionId, DecisionKind, TracedDecision,
//! DecisionLog, KernelError)

use serde::{Deserialize, Serialize};

use forge_core::{DecisionId, DecisionKind, DecisionTier, TracedDecision};

/// Override specification for a counterfactual replay.
///
/// Tells `replay_decision` how to mutate the target decision:
/// the forced kind (e.g., flip Exact to PolicyApplied) and
/// the forced margin.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionOverride {
    /// Which decision to override.
    target_id: DecisionId,
    /// The forced decision kind.
    forced_kind: DecisionKind,
    /// The forced decision tier.
    forced_tier: DecisionTier,
    /// The forced margin value.
    forced_margin: f64,
}

impl DecisionOverride {
    /// Create a new override specification.
    pub fn new(
        target_id: DecisionId,
        forced_kind: DecisionKind,
        forced_tier: DecisionTier,
        forced_margin: f64,
    ) -> Self {
        Self {
            target_id,
            forced_kind,
            forced_tier,
            forced_margin,
        }
    }

    /// The target decision ID.
    pub fn get_target_id(&self) -> DecisionId {
        self.target_id
    }

    /// The forced decision kind.
    pub fn get_forced_kind(&self) -> &DecisionKind {
        &self.forced_kind
    }

    /// The forced decision tier.
    pub fn get_forced_tier(&self) -> DecisionTier {
        self.forced_tier
    }

    /// The forced margin.
    pub fn get_forced_margin(&self) -> f64 {
        self.forced_margin
    }
}

/// Validation status of a counterfactual topology.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CounterfactualValidation {
    /// The counterfactual topology passes all invariant checks.
    Valid,
    /// The counterfactual topology breaks topological invariants.
    TopologyBroken {
        /// Human-readable description of the validation failure.
        errors: Vec<String>,
    },
    /// The topology diverged (different hash) but still passes validation.
    DivergentButValid,
}

impl CounterfactualValidation {
    /// Whether the counterfactual topology is valid.
    pub fn is_valid(&self) -> bool {
        matches!(self, Self::Valid | Self::DivergentButValid)
    }

    /// Whether the counterfactual topology is broken.
    pub fn is_broken(&self) -> bool {
        matches!(self, Self::TopologyBroken { .. })
    }
}

/// Result of a counterfactual replay.
///
/// Captures the original vs. counterfactual state, the entity-level
/// delta between them, and whether the counterfactual result passes
/// topological validation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CounterfactualResult {
    /// The decision that was overridden.
    overridden_decision: TracedDecision,
    /// The override that was applied.
    applied_override: DecisionOverride,
    /// Topology hash from the original execution.
    original_hash: u128,
    /// Topology hash after the counterfactual replay.
    counterfactual_hash: u128,
    /// Entity-level differences between original and counterfactual.
    entity_delta: EntityDelta,
    /// Validation status of the counterfactual topology.
    validation: CounterfactualValidation,
}

impl CounterfactualResult {
    /// Create a new counterfactual result.
    pub fn new(
        overridden_decision: TracedDecision,
        applied_override: DecisionOverride,
        original_hash: u128,
        counterfactual_hash: u128,
        entity_delta: EntityDelta,
        validation: CounterfactualValidation,
    ) -> Self {
        Self {
            overridden_decision,
            applied_override,
            original_hash,
            counterfactual_hash,
            entity_delta,
            validation,
        }
    }

    /// The original decision before override.
    pub fn get_overridden_decision(&self) -> &TracedDecision {
        &self.overridden_decision
    }

    /// The override that was applied.
    pub fn get_applied_override(&self) -> &DecisionOverride {
        &self.applied_override
    }

    /// Original topology hash.
    pub fn get_original_hash(&self) -> u128 {
        self.original_hash
    }

    /// Counterfactual topology hash.
    pub fn get_counterfactual_hash(&self) -> u128 {
        self.counterfactual_hash
    }

    /// Whether the hashes diverged.
    pub fn has_diverged(&self) -> bool {
        self.original_hash != self.counterfactual_hash
    }

    /// Entity-level differences.
    pub fn get_entity_delta(&self) -> &EntityDelta {
        &self.entity_delta
    }

    /// Validation status.
    pub fn get_validation(&self) -> &CounterfactualValidation {
        &self.validation
    }
}

/// Entity-level differences between original and counterfactual topologies.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityDelta {
    /// Number of faces that differ.
    faces_changed: usize,
    /// Number of edges that differ.
    edges_changed: usize,
    /// Number of vertices that differ.
    vertices_changed: usize,
    /// Description of the most significant change.
    summary: String,
}

impl EntityDelta {
    /// Create a new entity delta.
    pub fn new(
        faces_changed: usize,
        edges_changed: usize,
        vertices_changed: usize,
        summary: String,
    ) -> Self {
        Self {
            faces_changed,
            edges_changed,
            vertices_changed,
            summary,
        }
    }

    /// Zero-delta (no changes).
    pub fn empty() -> Self {
        Self {
            faces_changed: 0,
            edges_changed: 0,
            vertices_changed: 0,
            summary: "No entity changes".to_string(),
        }
    }

    /// Total number of changed entities.
    pub fn total_changes(&self) -> usize {
        self.faces_changed + self.edges_changed + self.vertices_changed
    }

    /// Whether there are any entity changes.
    pub fn is_empty(&self) -> bool {
        self.total_changes() == 0
    }

    /// Number of modified faces.
    pub fn get_faces_changed(&self) -> usize {
        self.faces_changed
    }

    /// Number of modified edges.
    pub fn get_edges_changed(&self) -> usize {
        self.edges_changed
    }

    /// Number of modified vertices.
    pub fn get_vertices_changed(&self) -> usize {
        self.vertices_changed
    }

    /// The summary description.
    pub fn get_summary(&self) -> &str {
        &self.summary
    }
}

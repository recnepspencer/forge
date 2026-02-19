//! Data shapes for causal decision chain reconstruction.
//!
//! DOMAIN: Tracing the complete chain of decisions that led to a
//! topological entity's creation, from origin feature to present state.
//!
//! DEPENDENCIES: `forge-core` (TracedDecision, EntityRef),
//! `forge-topo` (OpSignature)

use serde::{Deserialize, Serialize};

use forge_core::{EntityRef, TracedDecision};
use forge_topo::lineage::OpSignature;

/// Complete causal chain for a single topological entity.
///
/// Traces the entity from its origin feature through every operation
/// that created or modified it, including the decisions made at each step.
/// The `summary` field provides a token-budgeted narrative suitable for
/// agent consumption (< 200 tokens).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalChain {
    /// The entity whose history we're tracing.
    target: EntityRef,
    /// Ordered list of causal steps, from origin to present.
    steps: Vec<CausalStep>,
    /// Semantic summary of the chain (agent-consumable, < 200 tokens).
    summary: ChainSummary,
}

impl CausalChain {
    /// Construct a new causal chain.
    pub fn new(target: EntityRef, steps: Vec<CausalStep>, summary: ChainSummary) -> Self {
        Self { target, steps, summary }
    }

    /// The entity whose history we're tracing.
    pub fn get_target(&self) -> &EntityRef {
        &self.target
    }

    /// Ordered list of causal steps, from origin to present.
    pub fn get_steps(&self) -> &[CausalStep] {
        &self.steps
    }

    /// Semantic summary of the chain.
    pub fn get_summary(&self) -> &ChainSummary {
        &self.summary
    }
}

/// A single step in the causal chain of an entity.
///
/// Records which operation occurred, the entity's state at that point,
/// the decisions made during the operation, topology hashes, and a
/// human-readable summary.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalStep {
    /// The operation that occurred.
    operation: OpSignature,
    /// The entity at this stage of its life.
    entity_state: EntityRef,
    /// Decisions made during this operation that affected this entity.
    decisions: Vec<TracedDecision>,
    /// Pre/post topology hash for this operation.
    topology_hashes: (u128, u128),
    /// Human/agent-readable one-line summary of what this step did to this entity.
    semantic_summary: String,
}

impl CausalStep {
    /// Construct a new causal step.
    pub fn new(
        operation: OpSignature,
        entity_state: EntityRef,
        decisions: Vec<TracedDecision>,
        topology_hashes: (u128, u128),
        semantic_summary: String,
    ) -> Self {
        Self {
            operation,
            entity_state,
            decisions,
            topology_hashes,
            semantic_summary,
        }
    }

    /// The operation that occurred.
    pub fn get_operation(&self) -> &OpSignature {
        &self.operation
    }

    /// The entity at this stage of its life.
    pub fn get_entity_state(&self) -> &EntityRef {
        &self.entity_state
    }

    /// Decisions made during this operation that affected this entity.
    pub fn get_decisions(&self) -> &[TracedDecision] {
        &self.decisions
    }

    /// Pre/post topology hash for this operation.
    pub fn get_topology_hashes(&self) -> (u128, u128) {
        self.topology_hashes
    }

    /// Human/agent-readable one-line summary.
    pub fn get_semantic_summary(&self) -> &str {
        &self.semantic_summary
    }
}

/// Token-budgeted summary of a causal chain.
///
/// Designed to fit within < 200 tokens for agent consumption.
/// Contains the most salient facts: step counts, tightest margin,
/// and a one-line narrative of the entity's lifecycle.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainSummary {
    /// Total steps in the chain.
    total_steps: usize,
    /// Steps containing NearBoundary or Ambiguous decisions (the interesting ones).
    decision_steps: usize,
    /// The tightest margin across all decisions in the chain.
    min_margin: f64,
    /// One-line narrative: "Face created by Extrude-1, split by Boolean-3, classified Inside"
    narrative: String,
}

impl ChainSummary {
    /// Construct a new chain summary.
    pub fn new(
        total_steps: usize,
        decision_steps: usize,
        min_margin: f64,
        narrative: String,
    ) -> Self {
        Self { total_steps, decision_steps, min_margin, narrative }
    }

    /// Total steps in the chain.
    pub fn get_total_steps(&self) -> usize {
        self.total_steps
    }

    /// Steps containing NearBoundary or Ambiguous decisions.
    pub fn get_decision_steps(&self) -> usize {
        self.decision_steps
    }

    /// The tightest margin across all decisions in the chain.
    pub fn get_min_margin(&self) -> f64 {
        self.min_margin
    }

    /// One-line narrative of the entity's lifecycle.
    pub fn get_narrative(&self) -> &str {
        &self.narrative
    }

    /// Approximate token count of the narrative (word-based estimate).
    pub fn narrative_token_count(&self) -> usize {
        self.narrative.split_whitespace().count()
    }
}

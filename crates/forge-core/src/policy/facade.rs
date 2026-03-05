//! Public API surface for the policy domain.
//!
//! External components depend ONLY on this facade.

pub use super::data::{
    // Invariant validation contract types
    InvariantGroup, InvariantTier, ValidatorCost,
    InvariantId, InvariantRelation, InvariantContract,
    APPLICABLE_BY_KIND, CLOSED_SHEET_EXTRA, DEFER_SEMANTIC_TIER, DEFER_UNCERTIFIED,
    applicable_mask_for, deferred_mask_for,
    // Policy types
    PolicyKind, PolicyQuery, PolicyResult,
    // Topology classification
    CertificationStage, Closure, Manifoldness, TopologyContext, TopologyKind,
    // Validation checkpoints
    ValidationCheckpoint,
};

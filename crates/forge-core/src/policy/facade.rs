//! Public API surface for the policy domain.
//!
//! External components depend ONLY on this facade.

pub use super::data::{
    applicable_mask_for,
    deferred_mask_for,
    // Topology classification
    CertificationStage,
    Closure,
    InvariantContract,
    // Invariant validation contract types
    InvariantGroup,
    InvariantId,
    InvariantRelation,
    InvariantTier,
    Manifoldness,
    // Policy types
    PolicyKind,
    PolicyQuery,
    PolicyResult,
    TopologyContext,
    TopologyKind,
    // Validation checkpoints
    ValidationCheckpoint,
    ValidatorCost,
    APPLICABLE_BY_KIND,
    CLOSED_SHEET_EXTRA,
    DEFER_SEMANTIC_TIER,
    DEFER_UNCERTIFIED,
};

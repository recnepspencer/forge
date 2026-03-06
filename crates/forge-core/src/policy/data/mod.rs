//! Policy data shapes for the Forge geometry kernel.

pub mod invariant_group;
pub mod invariant_id;
pub mod policy_kind;
pub mod policy_query;
pub mod policy_result;
pub mod topology_kind;
pub mod validation_checkpoint;

pub use invariant_group::{
    applicable_mask_for, deferred_mask_for, InvariantGroup, InvariantTier, ValidatorCost,
    APPLICABLE_BY_KIND, CLOSED_SHEET_EXTRA, DEFER_SEMANTIC_TIER, DEFER_UNCERTIFIED,
};
pub use invariant_id::{InvariantContract, InvariantId, InvariantRelation};
pub use policy_kind::PolicyKind;
pub use policy_query::PolicyQuery;
pub use policy_result::PolicyResult;
pub use topology_kind::{CertificationStage, Closure, Manifoldness, TopologyContext, TopologyKind};
pub use validation_checkpoint::ValidationCheckpoint;

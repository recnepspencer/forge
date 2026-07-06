mod chunk_placeholder;
mod chunk_plan;
mod counters;
mod evidence;
mod movable_reference;
mod non_claim;
mod physical_execution;
mod tier_plan;

pub use chunk_placeholder::{FutureChunkStabilityBasis, PhysicalChunkStabilityPlaceholder};
pub use chunk_plan::ChunkMigrationReadInterlockPlan;
pub use counters::TierMovementStabilityCounterSnapshot;
pub use evidence::{
    tier_movement_stability_capability, FoundationalTierMovementNonClaimEvidence,
    FutureChunkStabilityRecipe, TierMovementStabilityCapability,
};
pub use movable_reference::{
    MovablePhysicalRef, MovablePhysicalRefKind, TierMovementAdmissionLabel,
};
pub use non_claim::{
    FutureBlobMigrationNonClaim, FutureBlobMigrationNonClaimReport, TierMovementStabilityDenial,
};
#[cfg(any(test, feature = "certification-authority"))]
pub use physical_execution::physical_placement_movement_execution_for_certification_test;
pub use physical_execution::PhysicalPlacementMovementExecutionReceipt;
pub use tier_plan::{
    TierMovementReadInterlockPlan, TierMovementStabilityVerdict, UnsupportedTierMovementClaim,
    UnsupportedTierMovementRequest,
};

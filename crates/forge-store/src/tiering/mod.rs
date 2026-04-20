mod plans;
mod policy;
mod proofs;
mod read_path;
mod work_units;

pub const TIERING_FAMILY_VERSION: u32 = 1;

pub use plans::{
    AuthoritativePlacementPlanningReport, AuthoritativeTierMovePlan,
    BroadenedRecallPlan, DerivedPlacementPlanningReport, DerivedTierMovePlan,
    FamilyLocalPlacementPlan, PlacementStabilityPlan, ReadPlacementPlanningReport,
    RecallBreadthSummary, RecallPreparationPlan, RetainedRangePlacementPlan,
    TierLocalityFootprint, TierMoveBreadthSummary, TierMoveRejection,
    WorkingSetDebtSummary,
};
pub use policy::{
    AdaptivePlacementDebtMarker, ColdDerivedFamilyPolicy, ConservativePlacementPolicy,
    PlacementObservationScopeClass, PlacementPolicyClass,
};
pub use proofs::{
    AuthoritativeTierResidency, CanonicalResidencyManifest, ColdRecallTierPath,
    DerivedTierResidency, HotnessClassificationVerdict, PlacementArtifactFamily,
    PlacementBudgetClass, PlacementDemandSummary, PlacementExecutionOrigin,
    PlacementNonAuthorityWitness, RecallAmplificationBudget, RecallCoalescingKey,
    RecallCompletionWitness, RecallCostClass, RecallEligibilityWitness, RetiredTierReplica,
    TierCutoverWitness, TierCoolingCandidate, TierPlacementEvidence, TierPromotionCandidate,
    TierResidenceClass, TierTransferIntent, TransferredTierReplica, VerifiedTierReplica,
    WorkingSetObservationWindow,
};
pub use read_path::{
    ColdRecallLease, PlacementBoundArtifactRef, PlacementResolvedReadHandle, ResidentReadLease,
};
pub use work_units::{
    AuthoritativeTierMoveUnit, DeltaRecallUnit, DerivedTierMoveUnit, FamilyLocalRecallUnit,
    LayoutFamilyRecallUnit, PlacementObservationUnit, SchedulerPlacementWorkToken,
    SnapshotRecallUnit,
};

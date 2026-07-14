mod interleaving;
mod plans;
mod policy;
mod proofs;
mod read_path;
mod results;
mod work_units;

pub const TIERING_FAMILY_VERSION: u32 = 1;

pub use interleaving::{
    InterleavedContinuationParityReport, InterleavedReadParityReport, PlacementRaceOutcome,
    TierInterleavingObservation,
};
pub use plans::{
    AuthoritativePlacementPlanningReport, AuthoritativeTierMovePlan, BroadenedRecallPlan,
    ColdRecallPlan, DerivedPlacementPlanningReport, DerivedTierMovePlan, FamilyLocalPlacementPlan,
    PlacementStabilityPlan, ReadPlacementPlanningReport, RecallBreadthSummary, RecallDebtSummary,
    RecallPreparationPlan, RetainedRangePlacementPlan, TierLocalityFootprint,
    TierMoveBreadthSummary, TierMoveRejection, WorkingSetDebtSummary,
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
    RecallCompletionWitness, RecallCostClass, RecallEligibilityWitness, RetainedReadPlacementPath,
    RetiredTierReplica, TierCoolingCandidate, TierCutoverWitness, TierMissOutcome,
    TierPlacementEvidence, TierPromotionCandidate, TierResidenceClass, TierTransferIntent,
    TransferredTierReplica, VerifiedTierReplica, WorkingSetObservationWindow,
};
pub use read_path::{
    ColdRecallLease, PlacementBoundArtifactRef, PlacementResolvedReadHandle, ResidentReadLease,
};
pub use results::{CoalescedRecallReport, RecallExecutionDisposition};
pub use work_units::{
    AuthoritativeTierMoveUnit, DeltaRecallUnit, DerivedTierMoveUnit, FamilyLocalRecallUnit,
    LayoutFamilyRecallUnit, PlacementObservationUnit, SchedulerPlacementWorkToken,
    SnapshotRecallUnit,
};

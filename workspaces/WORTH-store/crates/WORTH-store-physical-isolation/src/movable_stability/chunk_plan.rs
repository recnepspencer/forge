use super::evidence::resolve_future_chunk_stability_recipe;
use super::{
    FoundationalTierMovementNonClaimEvidence, FutureBlobMigrationNonClaimReport,
    FutureChunkStabilityRecipe, PhysicalChunkStabilityPlaceholder,
    TierMovementReadInterlockPlan, TierMovementStabilityCounterSnapshot,
    TierMovementStabilityDenial,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChunkMigrationReadInterlockPlan {
    placeholder: PhysicalChunkStabilityPlaceholder,
    tier_plan: TierMovementReadInterlockPlan,
    non_claim: FutureBlobMigrationNonClaimReport,
    counters: TierMovementStabilityCounterSnapshot,
}

impl ChunkMigrationReadInterlockPlan {
    pub fn admit(
        placeholder: PhysicalChunkStabilityPlaceholder,
    ) -> Result<Self, TierMovementStabilityDenial> {
        super::transitions::admit_chunk_migration_interlock(placeholder)
    }

    pub(crate) const fn from_admitted_transition(
        placeholder: PhysicalChunkStabilityPlaceholder,
        tier_plan: TierMovementReadInterlockPlan,
        non_claim: FutureBlobMigrationNonClaimReport,
        counters: TierMovementStabilityCounterSnapshot,
    ) -> Self {
        Self {
            placeholder,
            tier_plan,
            non_claim,
            counters,
        }
    }

    pub const fn placeholder(self) -> PhysicalChunkStabilityPlaceholder {
        self.placeholder
    }

    pub const fn tier_plan(self) -> TierMovementReadInterlockPlan {
        self.tier_plan
    }

    pub const fn non_claim(self) -> FutureBlobMigrationNonClaimReport {
        self.non_claim
    }

    pub const fn counters(self) -> TierMovementStabilityCounterSnapshot {
        self.counters
    }

    pub fn foundational_non_claim_evidence(self) -> FoundationalTierMovementNonClaimEvidence {
        FoundationalTierMovementNonClaimEvidence::from_non_claim_report(self.non_claim)
    }

    pub fn resolved_stability_recipe(self) -> FutureChunkStabilityRecipe {
        resolve_future_chunk_stability_recipe(self)
    }

    pub const fn require_blob_lifecycle_authority(self) -> Result<(), TierMovementStabilityDenial> {
        Err(TierMovementStabilityDenial::BlobLifecycleRemainsS7Scope)
    }

    pub const fn require_blob_retention_authority(self) -> Result<(), TierMovementStabilityDenial> {
        Err(TierMovementStabilityDenial::BlobRetentionRemainsS7Scope)
    }

    pub const fn require_blob_dedupe_authority(self) -> Result<(), TierMovementStabilityDenial> {
        Err(TierMovementStabilityDenial::BlobDedupeRemainsS7Scope)
    }

    pub const fn require_resumable_write_authority(
        self,
    ) -> Result<(), TierMovementStabilityDenial> {
        Err(TierMovementStabilityDenial::ResumableWritesRemainS7Scope)
    }

    pub const fn require_cold_tier_qos(self) -> Result<(), TierMovementStabilityDenial> {
        Err(TierMovementStabilityDenial::ColdTierQosRemainsS6Scope)
    }

    pub const fn deny_proof_assumption_blob_authority_promotion(
        self,
    ) -> Result<(), TierMovementStabilityDenial> {
        Err(TierMovementStabilityDenial::ProofAssumptionCannotPromoteToBlobAuthority)
    }

    pub const fn deny_proof_assumption_cold_tier_qos_promotion(
        self,
    ) -> Result<(), TierMovementStabilityDenial> {
        Err(TierMovementStabilityDenial::ProofAssumptionCannotPromoteToColdTierQos)
    }
}

use crate::PhysicalReferenceGenerationMismatch;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FutureBlobMigrationNonClaim {
    S7OwnsBlobLifecycle,
    S7OwnsBlobRetention,
    S7OwnsBlobDedupe,
    S7OwnsResumableWrites,
    S6OwnsColdTierQos,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FutureBlobMigrationNonClaimReport {
    blob_lifecycle: FutureBlobMigrationNonClaim,
    blob_retention: FutureBlobMigrationNonClaim,
    blob_dedupe: FutureBlobMigrationNonClaim,
    resumable_writes: FutureBlobMigrationNonClaim,
    cold_tier_qos: FutureBlobMigrationNonClaim,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TierMovementStabilityDenial {
    MissingChunkEpoch,
    StaleGeneration(PhysicalReferenceGenerationMismatch),
    WrongMovableReferenceKind,
    CopiedMigrationLabel,
    UnsupportedTierMovement,
    PlaceholderBasisMismatch,
    BlobLifecycleRemainsS7Scope,
    BlobRetentionRemainsS7Scope,
    BlobDedupeRemainsS7Scope,
    ResumableWritesRemainS7Scope,
    ColdTierQosRemainsS6Scope,
    FoundationalSurfaceCannotPromoteToBlobAuthority,
    ProofAssumptionCannotPromoteToBlobAuthority,
    ProofAssumptionCannotPromoteToColdTierQos,
}

impl FutureBlobMigrationNonClaimReport {
    pub const fn s5_stability_only() -> Self {
        Self {
            blob_lifecycle: FutureBlobMigrationNonClaim::S7OwnsBlobLifecycle,
            blob_retention: FutureBlobMigrationNonClaim::S7OwnsBlobRetention,
            blob_dedupe: FutureBlobMigrationNonClaim::S7OwnsBlobDedupe,
            resumable_writes: FutureBlobMigrationNonClaim::S7OwnsResumableWrites,
            cold_tier_qos: FutureBlobMigrationNonClaim::S6OwnsColdTierQos,
        }
    }

    pub const fn blob_lifecycle(self) -> FutureBlobMigrationNonClaim {
        self.blob_lifecycle
    }

    pub const fn blob_retention(self) -> FutureBlobMigrationNonClaim {
        self.blob_retention
    }

    pub const fn blob_dedupe(self) -> FutureBlobMigrationNonClaim {
        self.blob_dedupe
    }

    pub const fn resumable_writes(self) -> FutureBlobMigrationNonClaim {
        self.resumable_writes
    }

    pub const fn cold_tier_qos(self) -> FutureBlobMigrationNonClaim {
        self.cold_tier_qos
    }
}

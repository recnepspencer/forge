use super::{
    FutureBlobMigrationNonClaimReport, MovablePhysicalRef, MovablePhysicalRefKind,
    TierMovementAdmissionLabel, TierMovementStabilityCounterSnapshot, TierMovementStabilityDenial,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TierMovementStabilityVerdict {
    StabilityOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TierMovementReadInterlockPlan {
    reference: MovablePhysicalRef,
    label: TierMovementAdmissionLabel,
    verdict: TierMovementStabilityVerdict,
    counters: TierMovementStabilityCounterSnapshot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnsupportedTierMovementRequest {
    reference_kind: MovablePhysicalRefKind,
    claim: UnsupportedTierMovementClaim,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnsupportedTierMovementClaim {
    ColdTierQos,
    HardwareMediaPlacement,
    BlobLifecycleMigration,
}

impl TierMovementReadInterlockPlan {
    pub fn admit(reference: MovablePhysicalRef) -> Result<Self, TierMovementStabilityDenial> {
        Self::admit_with_label(
            reference,
            TierMovementAdmissionLabel::for_movable_reference(reference),
        )
    }

    pub fn admit_with_label(
        reference: MovablePhysicalRef,
        label: TierMovementAdmissionLabel,
    ) -> Result<Self, TierMovementStabilityDenial> {
        if !label.matches(reference) {
            return Err(TierMovementStabilityDenial::CopiedMigrationLabel);
        }
        Ok(Self {
            reference,
            label,
            verdict: TierMovementStabilityVerdict::StabilityOnly,
            counters: TierMovementStabilityCounterSnapshot::default().with_stability_admission(),
        })
    }

    pub const fn reject_unsupported_tier_movement(
        request: UnsupportedTierMovementRequest,
    ) -> Result<Self, TierMovementStabilityDenial> {
        let _ = request;
        Err(TierMovementStabilityDenial::UnsupportedTierMovement)
    }

    pub const fn reference(self) -> MovablePhysicalRef {
        self.reference
    }

    pub const fn label(self) -> TierMovementAdmissionLabel {
        self.label
    }

    pub const fn verdict(self) -> TierMovementStabilityVerdict {
        self.verdict
    }

    pub const fn counters(self) -> TierMovementStabilityCounterSnapshot {
        self.counters
    }

    pub const fn non_claims(self) -> FutureBlobMigrationNonClaimReport {
        FutureBlobMigrationNonClaimReport::s5_stability_only()
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
}

impl UnsupportedTierMovementRequest {
    pub const fn new(
        reference_kind: MovablePhysicalRefKind,
        claim: UnsupportedTierMovementClaim,
    ) -> Self {
        Self {
            reference_kind,
            claim,
        }
    }

    pub const fn reference_kind(self) -> MovablePhysicalRefKind {
        self.reference_kind
    }

    pub const fn claim(self) -> UnsupportedTierMovementClaim {
        self.claim
    }
}

use std::marker::PhantomData;

use forge_store_physical_backend::{BackendDurabilityProfile, BackendDurabilityProfileId};

use super::{
    CheckpointCoveredLsnRange, CheckpointDurabilityEvidenceSet, CheckpointId,
    CheckpointRecoveryCounterSnapshot, CheckpointValidation, CheckpointValidationDenial,
    CheckpointValidationDenialKind,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckpointPublicationPlan<P: BackendDurabilityProfile> {
    profile: PhantomData<P>,
    validation: CheckpointValidation,
    counters: CheckpointRecoveryCounterSnapshot,
}

impl<P: BackendDurabilityProfile> CheckpointPublicationPlan<P> {
    pub fn plan_cutover(
        validation: CheckpointValidation,
        durability: CheckpointDurabilityEvidenceSet<P>,
    ) -> Result<Self, CheckpointValidationDenial> {
        let counters = validation.counters().with_cutover_decision();
        if durability.profile_id() != P::ID {
            return Err(CheckpointValidationDenial::new(
                CheckpointValidationDenialKind::CutoverDurabilityProfileMismatch,
                counters,
            )
            .with_profile_id(durability.profile_id()));
        }
        if durability.checkpoint_id() != validation.checkpoint_id() {
            return Err(CheckpointValidationDenial::new(
                CheckpointValidationDenialKind::CutoverDurabilityCheckpointMismatch,
                counters,
            ));
        }
        Ok(Self {
            profile: PhantomData,
            validation,
            counters,
        })
    }

    pub fn validation(&self) -> &CheckpointValidation {
        &self.validation
    }

    pub const fn counters(&self) -> CheckpointRecoveryCounterSnapshot {
        self.counters
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckpointCutoverReceipt {
    checkpoint_id: CheckpointId,
    profile_id: BackendDurabilityProfileId,
    covered_lsn_range: CheckpointCoveredLsnRange,
    counters: CheckpointRecoveryCounterSnapshot,
}

impl CheckpointCutoverReceipt {
    pub fn publish<P: BackendDurabilityProfile>(plan: CheckpointPublicationPlan<P>) -> Self {
        Self {
            checkpoint_id: plan.validation().checkpoint_id().clone(),
            profile_id: P::ID,
            covered_lsn_range: plan.validation().manifest().covered_lsn_range(),
            counters: plan.counters(),
        }
    }

    #[cfg(feature = "certification-test-authority")]
    pub fn recovered_corrupt_cutover_report_for_certification(
        checkpoint_id: CheckpointId,
        profile_id: BackendDurabilityProfileId,
        covered_lsn_range: CheckpointCoveredLsnRange,
        counters: CheckpointRecoveryCounterSnapshot,
    ) -> Self {
        Self {
            checkpoint_id,
            profile_id,
            covered_lsn_range,
            counters,
        }
    }

    pub fn checkpoint_id(&self) -> &CheckpointId {
        &self.checkpoint_id
    }

    pub const fn profile_id(&self) -> BackendDurabilityProfileId {
        self.profile_id
    }

    pub const fn covered_lsn_range(&self) -> CheckpointCoveredLsnRange {
        self.covered_lsn_range
    }

    pub const fn counters(&self) -> CheckpointRecoveryCounterSnapshot {
        self.counters
    }
}

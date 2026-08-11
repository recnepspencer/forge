use std::sync::atomic::{AtomicU8, Ordering};

use super::super::{
    PhysicalRecoveryFreshReopenStage, PhysicalRecoveryPublicationCommandStage,
    PhysicalRecoveryStagingCommandStage,
};

pub(super) struct RecoveryCoordinationCertificationFaults {
    staging_signal_failure_stage: AtomicU8,
    publication_signal_failure_stage: AtomicU8,
    reopen_signal_failure_stage: AtomicU8,
    publication_scheduler_failure_stage: AtomicU8,
    reopen_scheduler_failure_stage: AtomicU8,
}

impl RecoveryCoordinationCertificationFaults {
    pub(super) const fn new() -> Self {
        Self {
            staging_signal_failure_stage: AtomicU8::new(0),
            publication_signal_failure_stage: AtomicU8::new(0),
            reopen_signal_failure_stage: AtomicU8::new(0),
            publication_scheduler_failure_stage: AtomicU8::new(0),
            reopen_scheduler_failure_stage: AtomicU8::new(0),
        }
    }

    pub(super) fn fail_signal_settlement_at(&self, stage: PhysicalRecoveryStagingCommandStage) {
        self.staging_signal_failure_stage
            .store(staging_stage(stage), Ordering::Release);
    }

    pub(super) fn fail_publication_signal_settlement_at(
        &self,
        stage: PhysicalRecoveryPublicationCommandStage,
    ) {
        self.publication_signal_failure_stage
            .store(publication_stage(stage), Ordering::Release);
    }

    pub(super) fn fail_reopen_signal_settlement_at(&self, stage: PhysicalRecoveryFreshReopenStage) {
        self.reopen_signal_failure_stage
            .store(reopen_stage(stage), Ordering::Release);
    }

    pub(super) fn fail_reopen_scheduler_settlement_at(
        &self,
        stage: PhysicalRecoveryFreshReopenStage,
    ) {
        self.reopen_scheduler_failure_stage
            .store(reopen_stage(stage), Ordering::Release);
    }

    pub(super) fn fail_publication_scheduler_settlement_at(
        &self,
        stage: PhysicalRecoveryPublicationCommandStage,
    ) {
        self.publication_scheduler_failure_stage
            .store(publication_stage(stage), Ordering::Release);
    }

    pub(super) fn take_signal_failure(
        &self,
        stage: super::super::settlement::PhysicalRecoverySettlementCertificationStage,
    ) -> bool {
        match stage {
            super::super::settlement::PhysicalRecoverySettlementCertificationStage::Staging(
                stage,
            ) => take(&self.staging_signal_failure_stage, staging_stage(stage)),
            super::super::settlement::PhysicalRecoverySettlementCertificationStage::Publication(
                stage,
            ) => take(
                &self.publication_signal_failure_stage,
                publication_stage(stage),
            ),
            super::super::settlement::PhysicalRecoverySettlementCertificationStage::FreshReopen(
                stage,
            ) => take(&self.reopen_signal_failure_stage, reopen_stage(stage)),
        }
    }

    pub(super) fn take_reopen_scheduler_failure(
        &self,
        stage: PhysicalRecoveryFreshReopenStage,
    ) -> bool {
        take(&self.reopen_scheduler_failure_stage, reopen_stage(stage))
    }

    pub(super) fn take_publication_scheduler_failure(
        &self,
        stage: PhysicalRecoveryPublicationCommandStage,
    ) -> bool {
        take(
            &self.publication_scheduler_failure_stage,
            publication_stage(stage),
        )
    }
}

fn take(stage: &AtomicU8, expected: u8) -> bool {
    stage
        .compare_exchange(expected, 0, Ordering::AcqRel, Ordering::Acquire)
        .is_ok()
}

const fn staging_stage(stage: PhysicalRecoveryStagingCommandStage) -> u8 {
    match stage {
        PhysicalRecoveryStagingCommandStage::Materialization => 1,
        PhysicalRecoveryStagingCommandStage::Synchronization => 2,
    }
}

const fn reopen_stage(stage: PhysicalRecoveryFreshReopenStage) -> u8 {
    match stage {
        PhysicalRecoveryFreshReopenStage::CurrentSelector => 3,
        PhysicalRecoveryFreshReopenStage::RootManifest => 4,
        PhysicalRecoveryFreshReopenStage::ExactBinding => 5,
    }
}

const fn publication_stage(stage: PhysicalRecoveryPublicationCommandStage) -> u8 {
    match stage {
        PhysicalRecoveryPublicationCommandStage::CandidateMaterialization => 1,
        PhysicalRecoveryPublicationCommandStage::CandidateSynchronization => 2,
        PhysicalRecoveryPublicationCommandStage::RootProtocolReplacement => 3,
        PhysicalRecoveryPublicationCommandStage::RecordNamespaceSynchronization => 4,
    }
}

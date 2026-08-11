use super::PhysicalRecoveryCoordination;

impl PhysicalRecoveryCoordination {
    pub fn certification_fail_signal_settlement_at(
        &self,
        stage: super::super::PhysicalRecoveryStagingCommandStage,
    ) {
        self.certification_faults.fail_signal_settlement_at(stage);
    }

    pub fn certification_fail_publication_signal_settlement_at(
        &self,
        stage: super::super::PhysicalRecoveryPublicationCommandStage,
    ) {
        self.certification_faults
            .fail_publication_signal_settlement_at(stage);
    }

    pub fn certification_fail_reopen_signal_settlement_at(
        &self,
        stage: super::super::PhysicalRecoveryFreshReopenStage,
    ) {
        self.certification_faults
            .fail_reopen_signal_settlement_at(stage);
    }

    pub fn certification_fail_reopen_scheduler_settlement_at(
        &self,
        stage: super::super::PhysicalRecoveryFreshReopenStage,
    ) {
        self.certification_faults
            .fail_reopen_scheduler_settlement_at(stage);
    }

    pub fn certification_fail_publication_scheduler_settlement_at(
        &self,
        stage: super::super::PhysicalRecoveryPublicationCommandStage,
    ) {
        self.certification_faults
            .fail_publication_scheduler_settlement_at(stage);
    }

    pub(in crate::physical_runtime::recovery_coordination) fn take_certification_signal_failure(
        &self,
        stage: super::super::settlement::PhysicalRecoverySettlementCertificationStage,
    ) -> bool {
        self.certification_faults.take_signal_failure(stage)
    }

    pub(in crate::physical_runtime::recovery_coordination) fn take_certification_reopen_scheduler_failure(
        &self,
        stage: super::super::PhysicalRecoveryFreshReopenStage,
    ) -> bool {
        self.certification_faults
            .take_reopen_scheduler_failure(stage)
    }

    pub(in crate::physical_runtime::recovery_coordination) fn take_certification_publication_scheduler_failure(
        &self,
        stage: super::super::PhysicalRecoveryPublicationCommandStage,
    ) -> bool {
        self.certification_faults
            .take_publication_scheduler_failure(stage)
    }
}

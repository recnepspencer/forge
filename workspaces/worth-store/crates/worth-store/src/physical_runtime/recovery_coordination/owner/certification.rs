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

    pub fn certification_shift_cleanup_generation(&self) {
        self.certification_faults.shift_cleanup_generation();
    }

    pub fn certification_fail_cleanup_eligibility_after_read(&self) {
        self.certification_faults
            .fail_cleanup_eligibility_after_read();
    }

    pub fn certification_fail_cleanup_freshness_signal_settlement(&self) {
        self.certification_faults
            .fail_cleanup_freshness_signal_settlement();
    }

    pub fn certification_fail_cleanup_scheduler_settlement_at(
        &self,
        stage: super::super::PhysicalRecoveryCleanupCommandStage,
    ) {
        self.certification_faults
            .fail_cleanup_scheduler_settlement_at(stage);
    }

    pub fn certification_defer_cleanup_background(&self) {
        self.certification_faults.defer_cleanup_background();
    }

    pub(in crate::physical_runtime) fn take_certification_cleanup_generation_shift(&self) -> bool {
        self.certification_faults.take_cleanup_generation_shift()
    }

    pub(in crate::physical_runtime) fn take_certification_cleanup_eligibility_failure(
        &self,
    ) -> bool {
        self.certification_faults.take_cleanup_eligibility_failure()
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

    pub(in crate::physical_runtime::recovery_coordination) fn take_certification_cleanup_scheduler_failure(
        &self,
        stage: super::super::PhysicalRecoveryCleanupCommandStage,
    ) -> bool {
        self.certification_faults
            .take_cleanup_scheduler_failure(stage)
    }

    pub(in crate::physical_runtime::recovery_coordination) fn take_certification_cleanup_background_deferral(
        &self,
    ) -> bool {
        self.certification_faults.take_cleanup_background_deferral()
    }
}

use crate::entry::{record_coordinator_created, PhysicalRecoveryLimits};
use worth_store::physical_runtime::{
    AdmittedRecoveryFilesystemMedia, PhysicalRecoveryCoordination,
    PhysicalRecoveryCoordinationAdmissionError, PhysicalRecoveryCoordinationCapacity,
    PhysicalRecoveryRegisteredSessionAuthority,
};

pub(crate) struct RecoveryCoordination {
    owner: PhysicalRecoveryCoordination,
}

impl RecoveryCoordination {
    pub(crate) fn fresh(
        media: &AdmittedRecoveryFilesystemMedia,
        session: PhysicalRecoveryRegisteredSessionAuthority,
        limits: PhysicalRecoveryLimits,
    ) -> Result<Self, PhysicalRecoveryCoordinationAdmissionError> {
        let limits = limits.declaration();
        let capacity = PhysicalRecoveryCoordinationCapacity::admit(
            limits.concurrent_commands,
            limits.observation_bytes,
        )
        .expect("admitted recovery limits are nonzero and fit the platform");
        let owner = session.admit_coordination(media, capacity)?;
        record_coordinator_created();
        Ok(Self { owner })
    }

    pub(crate) fn is_ready(&self) -> bool {
        self.owner.is_ready()
    }

    pub(crate) fn quiescence_observation(
        &self,
    ) -> worth_store::physical_runtime::PhysicalRecoveryQuiescenceObservation {
        self.owner.quiescence_observation()
    }

    #[cfg(feature = "certification-test-authority")]
    pub(crate) fn fail_signal_settlement_at_for_certification(
        &self,
        stage: worth_store::physical_runtime::PhysicalRecoveryStagingCommandStage,
    ) {
        self.owner.certification_fail_signal_settlement_at(stage);
    }

    #[cfg(feature = "certification-test-authority")]
    pub(crate) fn fail_reopen_scheduler_settlement_at_for_certification(
        &self,
        stage: worth_store::physical_runtime::PhysicalRecoveryFreshReopenStage,
    ) {
        self.owner
            .certification_fail_reopen_scheduler_settlement_at(stage);
    }

    #[cfg(feature = "certification-test-authority")]
    pub(crate) fn fail_reopen_signal_settlement_at_for_certification(
        &self,
        stage: worth_store::physical_runtime::PhysicalRecoveryFreshReopenStage,
    ) {
        self.owner
            .certification_fail_reopen_signal_settlement_at(stage);
    }

    #[cfg(feature = "certification-test-authority")]
    pub(crate) fn fail_publication_scheduler_settlement_at_for_certification(
        &self,
        stage: worth_store::physical_runtime::PhysicalRecoveryPublicationCommandStage,
    ) {
        self.owner
            .certification_fail_publication_scheduler_settlement_at(stage);
    }

    #[cfg(feature = "certification-test-authority")]
    pub(crate) fn fail_publication_signal_settlement_at_for_certification(
        &self,
        stage: worth_store::physical_runtime::PhysicalRecoveryPublicationCommandStage,
    ) {
        self.owner
            .certification_fail_publication_signal_settlement_at(stage);
    }

    #[cfg(feature = "certification-test-authority")]
    pub(crate) fn shift_cleanup_generation_for_certification(&self) {
        self.owner.certification_shift_cleanup_generation();
    }

    #[cfg(feature = "certification-test-authority")]
    pub(crate) fn fail_cleanup_eligibility_after_read_for_certification(&self) {
        self.owner
            .certification_fail_cleanup_eligibility_after_read();
    }

    #[cfg(feature = "certification-test-authority")]
    pub(crate) fn fail_cleanup_freshness_signal_settlement_for_certification(&self) {
        self.owner
            .certification_fail_cleanup_freshness_signal_settlement();
    }

    #[cfg(feature = "certification-test-authority")]
    pub(crate) fn fail_cleanup_scheduler_settlement_at_for_certification(
        &self,
        stage: worth_store::physical_runtime::PhysicalRecoveryCleanupCommandStage,
    ) {
        self.owner
            .certification_fail_cleanup_scheduler_settlement_at(stage);
    }

    #[cfg(feature = "certification-test-authority")]
    pub(crate) fn defer_cleanup_background_for_certification(&self) {
        self.owner.certification_defer_cleanup_background();
    }

    pub(crate) fn shutdown_is_quiescent(self) -> bool {
        self.owner.shutdown_is_quiescent()
    }

    pub(crate) const fn owner(&self) -> &PhysicalRecoveryCoordination {
        &self.owner
    }

    pub(crate) fn into_owner(self) -> PhysicalRecoveryCoordination {
        self.owner
    }
}

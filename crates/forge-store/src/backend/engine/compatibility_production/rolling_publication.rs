use crate::authority::{PersistedAuthoritativeCommit, VerifiedAuthoritativeAppend};
use crate::compatibility::{
    first_ship_commit_rolling_edge_registry, plan_first_ship_rolling_upgrade,
    CompatibilityFamilyKind, CompatibilityRollingPublicationOutcome,
    CompatibilityRollingPublicationRequest,
};
use crate::failure::StoreError;

use super::super::{
    compatibility_runtime::compatibility_rejection_error, StateBackedStoreBackend, StatePersistence,
};

impl<P: StatePersistence> StateBackedStoreBackend<P> {
    pub(crate) fn execute_rolling_commit_publication(
        &mut self,
        request: CompatibilityRollingPublicationRequest,
        verified: VerifiedAuthoritativeAppend,
    ) -> Result<CompatibilityRollingPublicationOutcome, StoreError> {
        let mut counters = crate::CompatibilityAdmissionCounters::default();
        let plan = plan_first_ship_rolling_upgrade(
            &mut counters,
            &first_ship_commit_rolling_edge_registry(),
            request.rolling_window(),
            request.reader_capabilities(),
            request.writer_capabilities(),
        )
        .map_err(|rejection| {
            compatibility_rejection_error("execute_rolling_commit_publication", rejection)
        })?;

        if request.rolling_window().family_id()
            != &CompatibilityFamilyKind::CommitEnvelope.family_id()
        {
            return Err(StoreError::new(
                crate::StoreErrorKind::CompatibilityRollingUpgradeRejected,
                "rolling commit publication requires the commit-envelope compatibility family",
            ));
        }

        let persisted: PersistedAuthoritativeCommit = self.append(verified)?;
        Ok(CompatibilityRollingPublicationOutcome::new(
            plan.relation(),
            plan.store_posture().clone(),
            plan.replica_posture().clone(),
            persisted,
            crate::Milestone12AdmissionReport::from_admission_counters(&counters),
        ))
    }
}

use std::sync::Arc;

use super::WorthQueryPrimaryGraphProvider;
use crate::domain_computation::{
    WorthQueryProviderExecutionPlanView, WorthQueryProviderSessionFailure,
    WorthQueryProviderSessionLifecycle, WorthQueryProviderSessionProtocolStage,
    WorthQueryProviderSessionToken, WorthQueryProviderSessionTokenAdmission,
    WorthQueryProviderSessionView,
};

impl WorthQueryProviderSessionLifecycle for Arc<WorthQueryPrimaryGraphProvider> {
    fn readmit_provider_plan(
        &self,
        _plan: &WorthQueryProviderExecutionPlanView<'_>,
        admission: WorthQueryProviderSessionTokenAdmission,
    ) -> Result<WorthQueryProviderSessionToken, WorthQueryProviderSessionFailure> {
        admission.admit("primary-application-relational-session")
    }

    fn prepare_provider_session(
        &self,
        _session: &WorthQueryProviderSessionView<'_>,
    ) -> Result<(), WorthQueryProviderSessionFailure> {
        #[cfg(test)]
        if self.take_rejected_session_prepare() {
            return Err(super::session_commit::provider_failure(
                WorthQueryProviderSessionProtocolStage::SessionPreparation,
                "injected primary graph session preparation rejection",
            ));
        }
        Ok(())
    }

    fn prepare_staged_session(
        &self,
        session: &WorthQueryProviderSessionView<'_>,
    ) -> Result<(), WorthQueryProviderSessionFailure> {
        let sessions = self
            .sessions
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if sessions.session_overlays.contains_key(session.identity()) {
            Ok(())
        } else {
            Err(super::session_commit::provider_failure(
                WorthQueryProviderSessionProtocolStage::StagedPreparation,
                "primary graph session has no exact staged overlay",
            ))
        }
    }

    fn commit_prepared_session(
        &self,
        session: &WorthQueryProviderSessionView<'_>,
    ) -> Result<String, WorthQueryProviderSessionFailure> {
        super::session_commit::commit_prepared_session(self, session.identity())
    }

    fn abort_provider_session(
        &self,
        session: &WorthQueryProviderSessionView<'_>,
    ) -> Result<String, WorthQueryProviderSessionFailure> {
        let mut sessions = self
            .sessions
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if let Some(overlay) = sessions.session_overlays.remove(session.identity()) {
            sessions.overlays.remove(&overlay);
        }
        sessions.application_attempts.remove(session.identity());
        sessions.validated_mutations.remove(session.identity());
        sessions.invariant_work.remove(session.identity());
        Ok(format!("primary-application-abort:{}", session.identity()))
    }
}

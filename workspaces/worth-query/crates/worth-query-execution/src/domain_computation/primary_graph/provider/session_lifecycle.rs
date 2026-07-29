use std::sync::Arc;

use super::WorthQueryPrimaryGraphProvider;
use crate::domain_computation::{
    WorthQueryProviderExecutionPlanView, WorthQueryProviderSessionDenialKind,
    WorthQueryProviderSessionFailure, WorthQueryProviderSessionLifecycle,
    WorthQueryProviderSessionProtocolCounters, WorthQueryProviderSessionProtocolStage,
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
            return Err(provider_failure(
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
            Err(provider_failure(
                WorthQueryProviderSessionProtocolStage::StagedPreparation,
                "primary graph session has no exact staged overlay",
            ))
        }
    }

    fn commit_prepared_session(
        &self,
        session: &WorthQueryProviderSessionView<'_>,
    ) -> Result<String, WorthQueryProviderSessionFailure> {
        let attempt = {
            let mut sessions = self
                .sessions
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            let overlay = sessions
                .session_overlays
                .remove(session.identity())
                .ok_or_else(|| {
                    provider_failure(
                        WorthQueryProviderSessionProtocolStage::Commit,
                        "primary graph session lost its staged overlay",
                    )
                })?;
            sessions.overlays.remove(&overlay);
            sessions
                .application_attempts
                .remove(session.identity())
                .ok_or_else(|| {
                    provider_failure(
                        WorthQueryProviderSessionProtocolStage::Commit,
                        "primary graph session lost its application attempt",
                    )
                })?
        };
        #[cfg(test)]
        if self.take_rejected_commit_before_transaction() {
            return Err(provider_failure(
                WorthQueryProviderSessionProtocolStage::Commit,
                "injected rejection before the atomic application transaction",
            ));
        }
        let emissions = attempt.emissions;
        self.graph.with_runtime_mut(|runtime| {
            let before = runtime.snapshots().snapshot();
            let mut transaction = runtime.begin_transaction(
                worth_relational::facade::transactions::TransactionOptions::default(),
            );
            transaction.push_batch(attempt.batch);
            let committed = match transaction.commit() {
                Ok(committed) => committed,
                Err(_) => {
                    let _ = runtime.snapshots().release_snapshot(&before);
                    return Err(provider_failure(
                        WorthQueryProviderSessionProtocolStage::Commit,
                        "Relational rejected the atomic application transaction",
                    ));
                }
            };
            let commit_id = committed.envelope().commit.commit_id;
            let changed = committed.patch().len();
            let after = runtime.snapshots().snapshot();
            self.graph
                .aggregate_projections
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner())
                .refresh_after_commit(runtime, &before, &after, committed.patch());
            let _ = runtime.snapshots().release_snapshot(&before);
            let _ = runtime.snapshots().release_snapshot(&after);
            let emitted = self
                .publish_application_commit_causality(commit_id, emissions)
                .map_err(|detail| {
                    provider_failure(WorthQueryProviderSessionProtocolStage::Commit, detail)
                })?;
            #[cfg(test)]
            if self.take_failed_index_publication() {
                return Err(provider_failure(
                    WorthQueryProviderSessionProtocolStage::Commit,
                    "injected primary index publication failure after authoritative commit",
                ));
            }
            let indexes = runtime.index_authority().build_for_commit(
                worth_relational::facade::indexes::DerivedIndexBuildRequest {
                    source_commit_id: commit_id,
                    branch_id: worth_relational::facade::history::BranchId("main".to_owned()),
                    index_ids: self.graph.primary_index_ids.to_vec(),
                },
            );
            if !indexes.failed_indexes.is_empty() {
                return Err(provider_failure(
                    WorthQueryProviderSessionProtocolStage::Commit,
                    "application commit succeeded but primary indexes did not refresh",
                ));
            }
            #[cfg(test)]
            if self.take_lost_commit_response() {
                return Err(provider_failure(
                    WorthQueryProviderSessionProtocolStage::Commit,
                    "application commit response was lost after authoritative publication",
                ));
            }
            Ok(format!(
                "primary-application-commit:{}:{changed}:{emitted}",
                commit_id.0,
            ))
        })
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
        Ok(format!("primary-application-abort:{}", session.identity()))
    }
}

fn provider_failure(
    stage: WorthQueryProviderSessionProtocolStage,
    detail: &'static str,
) -> WorthQueryProviderSessionFailure {
    WorthQueryProviderSessionFailure::new(
        WorthQueryProviderSessionDenialKind::ProviderRejected,
        stage,
        detail,
        WorthQueryProviderSessionProtocolCounters::default(),
    )
}

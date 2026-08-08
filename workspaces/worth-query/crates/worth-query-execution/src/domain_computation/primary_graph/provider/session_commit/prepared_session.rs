//! Atomic extraction and validation of one prepared session's commit inputs.

use crate::domain_computation::application_aftermath::WorthQueryRetainedPreImage;
use crate::domain_computation::{
    WorthQueryProviderSessionFailure, WorthQueryProviderSessionProtocolStage,
};

use super::super::{
    mutation_work::WorthQueryPrimaryMutationWorkCounters, WorthQueryPrimaryGraphApplicationAttempt,
    WorthQueryPrimaryGraphProvider,
};

pub(super) struct WorthQueryPreparedApplicationCommit {
    pub(super) attempt: WorthQueryPrimaryGraphApplicationAttempt,
    pub(super) candidate: worth_relational::facade::transactions::ValidatedRelationalMutation,
    pub(super) work: WorthQueryPrimaryMutationWorkCounters,
    pub(super) branch: worth_relational::facade::history::BranchId,
    pub(super) retained_preimage: Option<WorthQueryRetainedPreImage>,
}

impl WorthQueryPreparedApplicationCommit {
    pub(super) fn validate_decision_work(&self) -> Result<(), WorthQueryProviderSessionFailure> {
        let complete = self.attempt.decision_fact_count == self.attempt.facts.len()
            && self.attempt.graph_work_session.as_u64() != 0
            && self.work.decision_fact_count() == self.attempt.decision_fact_count
            && self.work.proposed_fact_count() == self.attempt.expected_steps.len();
        if complete {
            Ok(())
        } else {
            Err(super::provider_failure(
                WorthQueryProviderSessionProtocolStage::Commit,
                "application attempt lost its complete session decision facts",
            ))
        }
    }
}

pub(super) fn take_prepared_session(
    provider: &WorthQueryPrimaryGraphProvider,
    identity: &str,
) -> Result<WorthQueryPreparedApplicationCommit, WorthQueryProviderSessionFailure> {
    let mut sessions = provider
        .sessions
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let overlay = remove_staged_overlay(&mut sessions, identity)?;
    sessions.overlays.remove(&overlay);
    let attempt = sessions
        .application_attempts
        .remove(identity)
        .ok_or_else(|| commit_failure("primary graph session lost its application attempt"))?;
    let candidate = sessions
        .validated_mutations
        .remove(identity)
        .ok_or_else(|| {
            commit_failure("primary graph session has no owner-validated mutation candidate")
        })?;
    let work = sessions
        .invariant_work
        .remove(identity)
        .ok_or_else(|| commit_failure("primary graph session has no invariant work evidence"))?;
    let branch = attempt.branch.clone();
    let retained_preimage = super::preimage_retention::retain_attempt_preimage(
        &attempt,
        &candidate.mutation_footprint(),
    )?;
    Ok(WorthQueryPreparedApplicationCommit {
        attempt,
        candidate,
        work,
        branch,
        retained_preimage,
    })
}

fn remove_staged_overlay(
    sessions: &mut super::super::WorthQueryPrimaryGraphProviderSessions,
    identity: &str,
) -> Result<String, WorthQueryProviderSessionFailure> {
    sessions
        .session_overlays
        .remove(identity)
        .ok_or_else(|| commit_failure("primary graph session lost its staged overlay"))
}

fn commit_failure(detail: &'static str) -> WorthQueryProviderSessionFailure {
    super::provider_failure(WorthQueryProviderSessionProtocolStage::Commit, detail)
}

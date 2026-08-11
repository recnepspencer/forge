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
    attempt: WorthQueryPrimaryGraphApplicationAttempt,
    candidate: worth_relational::facade::transactions::ValidatedRelationalMutation,
    work: WorthQueryPrimaryMutationWorkCounters,
    branch: worth_relational::facade::history::BranchId,
    retained_preimage: Option<WorthQueryRetainedPreImage>,
    preimage_retention_work: super::preimage_retention::WorthQueryPreImageRetentionWork,
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

    pub(super) fn into_commit_parts(
        self,
        _mint: super::relational_commit::WorthQueryCommitProgressionMint,
    ) -> (
        WorthQueryPrimaryGraphApplicationAttempt,
        worth_relational::facade::transactions::ValidatedRelationalMutation,
        WorthQueryPrimaryMutationWorkCounters,
        worth_relational::facade::history::BranchId,
        Option<WorthQueryRetainedPreImage>,
        super::preimage_retention::WorthQueryPreImageRetentionWork,
    ) {
        (
            self.attempt,
            self.candidate,
            self.work,
            self.branch,
            self.retained_preimage,
            self.preimage_retention_work,
        )
    }
}

pub(super) fn take_prepared_session(
    provider: &WorthQueryPrimaryGraphProvider,
    affinity: crate::domain_computation::WorthQueryProviderSessionAffinityIdentity,
) -> Result<WorthQueryPreparedApplicationCommit, WorthQueryProviderSessionFailure> {
    let prepared = provider
        .attempts
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .take_commit_prepared(affinity)
        .ok_or_else(|| {
            commit_failure("primary graph session has no exact commit-prepared application attempt")
        })?;
    let (attempt, candidate, work) = prepared.into_parts();
    let branch = attempt.branch.clone();
    let (retained_preimage, preimage_retention_work) =
        super::preimage_retention::retain_attempt_preimage(&attempt, &candidate)?.into_parts();
    Ok(WorthQueryPreparedApplicationCommit {
        attempt,
        candidate,
        work,
        branch,
        retained_preimage,
        preimage_retention_work,
    })
}

fn commit_failure(detail: &'static str) -> WorthQueryProviderSessionFailure {
    super::provider_failure(WorthQueryProviderSessionProtocolStage::Commit, detail)
}

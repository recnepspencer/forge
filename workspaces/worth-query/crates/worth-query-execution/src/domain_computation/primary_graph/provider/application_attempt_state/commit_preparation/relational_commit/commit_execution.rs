//! The single authoritative Relational commit transition.

mod publication;
pub(in crate::domain_computation::primary_graph) use publication::WorthQueryPrimaryGraphCommittedApplication;

use super::super::WorthQueryPreparedApplicationCommit;
use crate::domain_computation::primary_graph::provider::{
    mutation_work::WorthQueryPrimaryMutationWorkCounters,
    session_commit::{provider_failure, WorthQueryPreImageRetentionWork},
    WorthQueryPrimaryGraphApplicationAttempt, WorthQueryPrimaryGraphProvider,
};
use crate::domain_computation::{
    WorthQueryProviderSessionFailure, WorthQueryProviderSessionProtocolStage,
};

pub(super) struct WorthQueryCommittedApplicationSession {
    attempt: WorthQueryPrimaryGraphApplicationAttempt,
    work: WorthQueryPrimaryMutationWorkCounters,
    retained_preimage:
        Option<crate::domain_computation::application_aftermath::WorthQueryRetainedPreImage>,
    preimage_retention_work: WorthQueryPreImageRetentionWork,
    branch: worth_relational::facade::history::BranchId,
    before: worth_relational::facade::snapshots::SnapshotHandle,
    committed: worth_relational::facade::transactions::CommitResult,
}

pub(super) fn commit(
    runtime: &mut worth_relational::facade::runtime::RelationalRuntime,
    prepared: WorthQueryPreparedApplicationCommit,
    mint: super::WorthQueryCommitProgressionMint,
) -> Result<WorthQueryCommittedApplicationSession, WorthQueryProviderSessionFailure> {
    let WorthQueryPreparedApplicationCommit {
        attempt,
        candidate,
        work,
        branch,
        retained_preimage,
        preimage_retention_work,
    } = prepared;
    let _ = mint;
    let before = runtime
        .snapshots()
        .historical_snapshot_for_branch(&branch)
        .ok_or_else(|| failure("application branch has no current pre-commit snapshot"))?;
    let committed = runtime.commit_validated_mutation(candidate).map_err(|_| {
        let _ = runtime.snapshots().release_snapshot(&before);
        failure("Relational rejected the atomic application transaction")
    })?;
    Ok(WorthQueryCommittedApplicationSession {
        attempt,
        work,
        retained_preimage,
        preimage_retention_work,
        branch,
        before,
        committed,
    })
}

impl WorthQueryCommittedApplicationSession {
    pub(super) const fn attempt(&self) -> &WorthQueryPrimaryGraphApplicationAttempt {
        &self.attempt
    }

    pub(super) const fn work(&self) -> WorthQueryPrimaryMutationWorkCounters {
        self.work
    }

    pub(super) const fn retained_preimage(
        &self,
    ) -> Option<&crate::domain_computation::application_aftermath::WorthQueryRetainedPreImage> {
        self.retained_preimage.as_ref()
    }

    pub(super) const fn preimage_retention_work(&self) -> WorthQueryPreImageRetentionWork {
        self.preimage_retention_work
    }

    pub(super) const fn committed(&self) -> &worth_relational::facade::transactions::CommitResult {
        &self.committed
    }

    pub(super) fn publish_and_encode(
        self,
        provider: &WorthQueryPrimaryGraphProvider,
        runtime: &mut worth_relational::facade::runtime::RelationalRuntime,
        evidence: super::WorthQueryPrimaryGraphCommitEvidence,
    ) -> Result<
        crate::domain_computation::WorthQueryProviderTerminalDescription,
        WorthQueryProviderSessionFailure,
    > {
        let published = publication::publish(provider, runtime, self, evidence)?;
        publication::encode(provider, published)
    }
}

fn failure(detail: &'static str) -> WorthQueryProviderSessionFailure {
    provider_failure(WorthQueryProviderSessionProtocolStage::Commit, detail)
}

//! Required publication after the authoritative commit has succeeded.

mod provider_result;

use super::super::super::super::WorthQueryPrimaryGraphProvider;
use super::WorthQueryCommittedApplicationSession;
use crate::domain_computation::{
    WorthQueryProviderSessionFailure, WorthQueryProviderSessionProtocolStage,
};

pub(in crate::domain_computation::primary_graph::provider) struct WorthQueryCommittedApplicationPublicationSeal {
    runtime_instance_id: u64,
    changed_record_count: usize,
    emitted_effect_count: usize,
    outcome_identity:
        crate::domain_computation::primary_graph::application_attempt::WorthQueryApplicationCommitOutcomeIdentity,
    evidence: super::super::WorthQueryPrimaryGraphCommitEvidence,
}

pub(in crate::domain_computation::primary_graph::provider::session_commit::relational_commit) struct WorthQueryPublishedApplicationCommit {
    application: crate::domain_computation::primary_graph::provider::WorthQueryPrimaryGraphCommittedApplication,
}

pub(super) fn publish(
    provider: &WorthQueryPrimaryGraphProvider,
    runtime: &mut worth_relational::facade::runtime::RelationalRuntime,
    committed: WorthQueryCommittedApplicationSession,
    evidence: super::super::WorthQueryPrimaryGraphCommitEvidence,
) -> Result<WorthQueryPublishedApplicationCommit, WorthQueryProviderSessionFailure> {
    let WorthQueryCommittedApplicationSession {
        attempt,
        branch,
        before,
        committed,
        ..
    } = committed;
    let commit_id = committed.envelope().commit.commit_id;
    let changed_record_count = committed.patch().len();
    let after = runtime
        .snapshots()
        .snapshot_for_branch(&branch)
        .ok_or_else(|| {
            let _ = runtime.snapshots().release_snapshot(&before);
            failure("application branch has no current post-commit snapshot")
        })?;
    let runtime_instance_id = after.runtime_instance_id;
    provider
        .graph
        .aggregate_projections
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .refresh_after_commit(runtime, &before, &after, committed.patch());
    let _ = runtime.snapshots().release_snapshot(&before);
    let _ = runtime.snapshots().release_snapshot(&after);
    let emitted_effect_count = provider
        .publish_application_commit_causality(commit_id, attempt.emissions)
        .map_err(failure)?;
    let seal = WorthQueryCommittedApplicationPublicationSeal {
        runtime_instance_id,
        changed_record_count,
        emitted_effect_count,
        outcome_identity: attempt.outcome_identity,
        evidence,
    };
    let application = crate::domain_computation::primary_graph::provider::WorthQueryPrimaryGraphCommittedApplication::from_publication(seal);
    provider
        .completed_commit_evidence
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .record(application.clone());
    publish_indexes(provider, runtime, branch, commit_id)?;
    Ok(WorthQueryPublishedApplicationCommit { application })
}

impl WorthQueryCommittedApplicationPublicationSeal {
    pub(in crate::domain_computation::primary_graph::provider) fn into_parts(
        self,
    ) -> (
        crate::domain_computation::primary_graph::application_attempt::WorthQueryApplicationCommitOutcomeIdentity,
        u64,
        usize,
        usize,
        super::super::WorthQueryPrimaryGraphCommitEvidence,
    ){
        (
            self.outcome_identity,
            self.runtime_instance_id,
            self.changed_record_count,
            self.emitted_effect_count,
            self.evidence,
        )
    }
}

impl WorthQueryPublishedApplicationCommit {
    pub(super) const fn application(
        &self,
    ) -> &crate::domain_computation::primary_graph::provider::WorthQueryPrimaryGraphCommittedApplication
    {
        &self.application
    }
}

pub(super) fn encode(
    provider: &WorthQueryPrimaryGraphProvider,
    published: WorthQueryPublishedApplicationCommit,
) -> Result<String, WorthQueryProviderSessionFailure> {
    provider_result::encode(provider, published)
}

fn publish_indexes(
    provider: &WorthQueryPrimaryGraphProvider,
    runtime: &mut worth_relational::facade::runtime::RelationalRuntime,
    branch: worth_relational::facade::history::BranchId,
    commit_id: worth_relational::facade::history::CommitId,
) -> Result<(), WorthQueryProviderSessionFailure> {
    #[cfg(test)]
    if provider.take_failed_index_publication() {
        return Err(failure(
            "injected primary index publication failure after authoritative commit",
        ));
    }
    let indexes = runtime.index_authority().build_for_commit(
        worth_relational::facade::indexes::DerivedIndexBuildRequest {
            source_commit_id: commit_id,
            branch_id: branch,
            index_ids: provider.graph.primary_index_ids.to_vec(),
        },
    );
    if indexes.failed_indexes.is_empty() {
        Ok(())
    } else {
        Err(failure(
            "application commit succeeded but primary indexes did not refresh",
        ))
    }
}

fn failure(detail: &'static str) -> WorthQueryProviderSessionFailure {
    super::super::super::provider_failure(WorthQueryProviderSessionProtocolStage::Commit, detail)
}

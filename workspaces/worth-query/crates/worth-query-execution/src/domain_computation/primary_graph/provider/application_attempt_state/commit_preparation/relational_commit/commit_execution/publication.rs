//! Required publication after the authoritative commit has succeeded.

mod committed_application;
mod provider_result;

pub(in crate::domain_computation::primary_graph) use committed_application::WorthQueryPrimaryGraphCommittedApplication;

use super::WorthQueryCommittedApplicationSession;
use crate::domain_computation::primary_graph::provider::{
    session_commit::provider_failure, WorthQueryPrimaryGraphProvider,
};
use crate::domain_computation::{
    WorthQueryProviderSessionFailure, WorthQueryProviderSessionProtocolStage,
};

pub(super) struct WorthQueryCommittedApplicationPublicationSeal {
    runtime_instance_id: u64,
    changed_record_count: usize,
    emitted_effect_count: usize,
    outcome_identity:
        crate::domain_computation::primary_graph::application_attempt::WorthQueryApplicationCommitOutcomeIdentity,
    basis_descriptor: worth_relational::facade::branch::RelationalBranchBasisDescriptor,
    evidence: super::super::WorthQueryPrimaryGraphCommitEvidence,
}

pub(super) struct WorthQueryPublishedApplicationCommit {
    _private: (),
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
    let after =
        crate::domain_computation::primary_graph::exact_basis_access::open_current_branch_snapshot(
            runtime, &branch,
        )
        .ok_or_else(|| {
            let _ = runtime.snapshots().release_snapshot(&before);
            failure("application branch has no current post-commit snapshot")
        })?;
    let runtime_instance_id = after.runtime_instance_id();
    let branch_identity = runtime
        .branch_identity(&branch)
        .map_err(|_| failure("application branch identity became unavailable after commit"))?;
    let (basis_descriptor, basis) = runtime
        .observe_branch(&branch_identity)
        .map_err(|_| failure("application commit basis could not be observed"))?;
    let observed_commit = runtime
        .history()
        .branch_head_for_observation(&basis.observation())
        .map_err(|_| failure("application commit basis was rejected by its owner"))?
        .map(|receipt| receipt.commit_id);
    if observed_commit != Some(commit_id) {
        return Err(failure(
            "application commit basis does not select the published commit",
        ));
    }
    let basis_retention_lease = runtime
        .retain_component_basis(&basis)
        .map_err(|_| failure("application commit basis could not be retained"))?;
    provider
        .graph
        .aggregate_projections
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .refresh_after_commit(runtime, &before, &after, committed.patch());
    let _ = runtime.snapshots().release_snapshot(&before);
    let _ = runtime.snapshots().release_snapshot(&after);
    let causality = attempt
        .publish_causality(provider, commit_id)
        .map_err(failure)?;
    let seal = WorthQueryCommittedApplicationPublicationSeal {
        runtime_instance_id,
        changed_record_count,
        emitted_effect_count: causality.emitted_effect_count(),
        outcome_identity: causality.outcome_identity(),
        basis_descriptor,
        evidence,
    };
    let application = WorthQueryPrimaryGraphCommittedApplication::from_publication(seal);
    provider
        .receipt_basis_retention
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .retain(commit_id, basis_retention_lease);
    provider
        .completed_commit_evidence
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .record(application);
    publish_indexes(provider, runtime, branch, commit_id)?;
    provider
        .graph
        .bind_truth_head_basis_in_runtime(runtime, &basis)
        .map_err(failure)?;
    Ok(WorthQueryPublishedApplicationCommit { _private: () })
}

pub(super) fn encode(
    provider: &WorthQueryPrimaryGraphProvider,
    published: WorthQueryPublishedApplicationCommit,
) -> Result<
    crate::domain_computation::WorthQueryProviderTerminalDescription,
    WorthQueryProviderSessionFailure,
> {
    provider_result::encode(provider, published)
}

fn publish_indexes(
    provider: &WorthQueryPrimaryGraphProvider,
    runtime: &mut worth_relational::facade::runtime::RelationalRuntime,
    branch: worth_relational::facade::history::BranchId,
    commit_id: worth_relational::facade::history::CommitId,
) -> Result<(), WorthQueryProviderSessionFailure> {
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
    provider_failure(WorthQueryProviderSessionProtocolStage::Commit, detail)
}

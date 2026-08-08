//! Relational commit, exact evidence handoff, and post-commit publication.

use crate::domain_computation::{
    WorthQueryProviderSessionFailure, WorthQueryProviderSessionProtocolStage,
};

use super::super::WorthQueryPrimaryGraphProvider;
use super::prepared_session::WorthQueryPreparedApplicationCommit;

struct WorthQueryCommitSnapshotBasis {
    branch: worth_relational::facade::history::BranchId,
    before: worth_relational::facade::snapshots::SnapshotHandle,
}

pub(super) fn commit_owner_validated(
    provider: &WorthQueryPrimaryGraphProvider,
    prepared: WorthQueryPreparedApplicationCommit,
) -> Result<String, WorthQueryProviderSessionFailure> {
    provider
        .graph
        .with_runtime_mut(|runtime| commit_in_runtime(provider, runtime, prepared))
}

fn commit_in_runtime(
    provider: &WorthQueryPrimaryGraphProvider,
    runtime: &mut worth_relational::facade::runtime::RelationalRuntime,
    prepared: WorthQueryPreparedApplicationCommit,
) -> Result<String, WorthQueryProviderSessionFailure> {
    let WorthQueryPreparedApplicationCommit {
        attempt,
        candidate,
        work,
        branch,
        retained_preimage,
    } = prepared;
    let before = runtime
        .snapshots()
        .snapshot_for_branch(&branch)
        .ok_or_else(|| commit_failure("application branch has no current pre-commit snapshot"))?;
    let committed = commit_candidate(runtime, candidate, &before)?;
    let commit_id = committed.envelope().commit.commit_id;
    let changed = committed.patch().len();
    record_exact_commit_evidence(provider, work, &committed, retained_preimage);
    let snapshots = WorthQueryCommitSnapshotBasis { branch, before };
    let runtime_instance_id = refresh_snapshots(provider, runtime, &snapshots, &committed)?;
    let emitted = provider
        .publish_application_commit_causality(commit_id, attempt.emissions)
        .map_err(commit_failure)?;
    publish_indexes(provider, runtime, snapshots.branch, commit_id)?;
    reject_lost_response(provider)?;
    Ok(format!(
        "primary-application-commit:{runtime_instance_id}:{}:{changed}:{emitted}:{}",
        commit_id.0,
        attempt.outcome_identity.get(),
    ))
}

fn commit_candidate(
    runtime: &mut worth_relational::facade::runtime::RelationalRuntime,
    candidate: worth_relational::facade::transactions::ValidatedRelationalMutation,
    before: &worth_relational::facade::snapshots::SnapshotHandle,
) -> Result<worth_relational::facade::transactions::CommitResult, WorthQueryProviderSessionFailure>
{
    runtime.commit_validated_mutation(candidate).map_err(|_| {
        let _ = runtime.snapshots().release_snapshot(before);
        commit_failure("Relational rejected the atomic application transaction")
    })
}

fn record_exact_commit_evidence(
    provider: &WorthQueryPrimaryGraphProvider,
    work: super::super::mutation_work::WorthQueryPrimaryMutationWorkCounters,
    committed: &worth_relational::facade::transactions::CommitResult,
    retained_preimage: Option<
        crate::domain_computation::application_aftermath::WorthQueryRetainedPreImage,
    >,
) {
    let commit_id = committed.envelope().commit.commit_id;
    let evidence = super::super::mutation_work::WorthQueryPrimaryMutationWorkEvidence::from_commit(
        work,
        &committed.changed_records,
    );
    let mut sessions = provider
        .sessions
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    #[cfg(test)]
    {
        sessions.observed_completed_mutation_work = Some(evidence.clone());
    }
    sessions.completed_commit_evidence.record(
        commit_id,
        super::WorthQueryPrimaryGraphCommitEvidence::new(evidence, retained_preimage),
    );
}

fn refresh_snapshots(
    provider: &WorthQueryPrimaryGraphProvider,
    runtime: &mut worth_relational::facade::runtime::RelationalRuntime,
    snapshots: &WorthQueryCommitSnapshotBasis,
    committed: &worth_relational::facade::transactions::CommitResult,
) -> Result<u64, WorthQueryProviderSessionFailure> {
    let after = runtime
        .snapshots()
        .snapshot_for_branch(&snapshots.branch)
        .ok_or_else(|| {
            let _ = runtime.snapshots().release_snapshot(&snapshots.before);
            commit_failure("application branch has no current post-commit snapshot")
        })?;
    let runtime_instance_id = after.runtime_instance_id;
    provider
        .graph
        .aggregate_projections
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .refresh_after_commit(runtime, &snapshots.before, &after, committed.patch());
    let _ = runtime.snapshots().release_snapshot(&snapshots.before);
    let _ = runtime.snapshots().release_snapshot(&after);
    Ok(runtime_instance_id)
}

fn publish_indexes(
    provider: &WorthQueryPrimaryGraphProvider,
    runtime: &mut worth_relational::facade::runtime::RelationalRuntime,
    branch: worth_relational::facade::history::BranchId,
    commit_id: worth_relational::facade::history::CommitId,
) -> Result<(), WorthQueryProviderSessionFailure> {
    #[cfg(test)]
    if provider.take_failed_index_publication() {
        return Err(commit_failure(
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
        Err(commit_failure(
            "application commit succeeded but primary indexes did not refresh",
        ))
    }
}

fn reject_lost_response(
    _provider: &WorthQueryPrimaryGraphProvider,
) -> Result<(), WorthQueryProviderSessionFailure> {
    #[cfg(test)]
    if _provider.take_lost_commit_response() {
        return Err(commit_failure(
            "application commit response was lost after authoritative publication",
        ));
    }
    Ok(())
}

fn commit_failure(detail: &'static str) -> WorthQueryProviderSessionFailure {
    super::provider_failure(WorthQueryProviderSessionProtocolStage::Commit, detail)
}

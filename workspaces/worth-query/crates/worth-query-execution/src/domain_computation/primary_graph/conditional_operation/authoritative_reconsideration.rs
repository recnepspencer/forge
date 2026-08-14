use std::collections::BTreeSet;

use worth_proof::TransitionOutcome;
use worth_runtime_bridge::facade::{
    BridgeInstalledConditionalLowering, BridgeOwnedSignalRuntime,
    RelationalBridgeRecordIdentityParts, RelationalCommittedPatchRequest, TruthCommitIdentity,
};

use super::signal_decision_reentry::{
    reconsider_retained_wake, WorthQueryConditionalTruthBasis, WorthQueryRetainedConditionalWake,
};

pub(super) struct WorthQueryRelevantAuthoritativeCommits {
    commits: Vec<(u64, worth_relational::facade::history::CommitId)>,
    next_cursor: u64,
    work_remaining: bool,
}

impl WorthQueryRelevantAuthoritativeCommits {
    pub(super) fn commit_count(&self) -> usize {
        self.commits.len()
    }

    pub(super) fn work_remaining(&self) -> bool {
        self.work_remaining
    }
}

pub(super) fn relevant_authoritative_commits<Schema>(
    runtime: &crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime<
        Schema,
    >,
    cursor: Option<u64>,
    maximum_commits: usize,
    watched_records: impl IntoIterator<Item = worth_relational::facade::transactions::RecordRef>,
    include_whole_graph: bool,
) -> Result<WorthQueryRelevantAuthoritativeCommits, String> {
    let current = cursor
        .ok_or_else(|| "conditional authoritative-change cursor was not initialized".to_string())?;
    let batch = runtime
        .primary_provider
        .conditional_commits_after_records(
            current,
            maximum_commits,
            watched_records,
            include_whole_graph,
        )
        .map_err(str::to_string)?;
    Ok(WorthQueryRelevantAuthoritativeCommits {
        commits: batch.commits,
        next_cursor: batch.cursor,
        work_remaining: batch.work_remaining,
    })
}

#[allow(clippy::too_many_arguments)]
pub(super) fn deliver_authoritative_commits(
    bridge: &mut BridgeOwnedSignalRuntime,
    lowering: &std::sync::Arc<BridgeInstalledConditionalLowering>,
    cursor: &mut Option<u64>,
    commits: WorthQueryRelevantAuthoritativeCommits,
    wakes: &mut [WorthQueryRetainedConditionalWake],
    query_binding_identity: &str,
    query_capability_identity: u64,
    truth: &WorthQueryConditionalTruthBasis,
) -> Result<bool, String> {
    for (sequence, commit) in &commits.commits {
        let changed_records = deliver_commit_dependencies(bridge, lowering, *commit)?;
        for wake in wakes
            .iter_mut()
            .filter(|wake| changed_records.contains(&wake.due.source_record_identity()))
        {
            reconsider_retained_wake(
                bridge,
                wake,
                lowering,
                query_binding_identity,
                query_capability_identity,
                truth,
            );
        }
        *cursor = Some(*sequence);
    }
    *cursor = Some(commits.next_cursor);
    Ok(commits.work_remaining())
}

fn deliver_commit_dependencies(
    bridge: &mut BridgeOwnedSignalRuntime,
    lowering: &BridgeInstalledConditionalLowering,
    commit: worth_relational::facade::history::CommitId,
) -> Result<BTreeSet<RelationalBridgeRecordIdentityParts>, String> {
    let mut changed_records = BTreeSet::new();
    for dependency_ordinal in 0..lowering.contract().dependency_count() {
        let outcome = bridge
            .deliver_authoritative_change(
                lowering,
                dependency_ordinal,
                RelationalCommittedPatchRequest::new(
                    TruthCommitIdentity::from_relational_commit_id(commit.0),
                ),
            )
            .map_err(|denial| denial.detail().to_string())?;
        let receipt = match outcome {
            TransitionOutcome::Success(receipt) => receipt,
            TransitionOutcome::Denied(_) => {
                return Err("Bridge denied conditional authoritative change".to_string())
            }
            TransitionOutcome::Failed(_) => {
                return Err("Bridge failed conditional authoritative change".to_string())
            }
            TransitionOutcome::Deferred(_) => {
                return Err("Bridge deferred conditional authoritative change".to_string())
            }
            TransitionOutcome::Stale(_) => {
                return Err("Bridge found stale conditional authoritative change".to_string())
            }
            TransitionOutcome::RebindRequired(_) => {
                return Err("Bridge requires conditional correspondence rebinding".to_string())
            }
        };
        changed_records.extend(
            receipt
                .change_set()
                .changes()
                .iter()
                .filter_map(|change| change.relational_record_identity()),
        );
    }
    Ok(changed_records)
}

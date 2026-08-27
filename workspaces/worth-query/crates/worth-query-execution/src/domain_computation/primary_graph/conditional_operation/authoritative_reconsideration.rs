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

pub(super) struct WorthQueryDeliveredAuthoritativeCommits {
    pub(super) work_remaining: bool,
    pub(super) granular_invalidations:
        Vec<worth_runtime_bridge::facade::BridgeGranularInvalidationDelivery>,
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
) -> Result<WorthQueryDeliveredAuthoritativeCommits, String> {
    let mut granular_invalidations = Vec::new();
    for (sequence, commit) in &commits.commits {
        let delivered = deliver_commit_dependencies(bridge, lowering, *commit, truth)?;
        for wake in wakes.iter_mut().filter(|wake| {
            delivered
                .changed_records
                .contains(&wake.due.source_record_identity())
        }) {
            if let Some(triggering_correspondence) = delivered
                .granular_invalidations
                .iter()
                .map(|delivery| delivery.correspondence_receipt())
                .find(|receipt| {
                    receipt.change_set().changes().iter().any(|change| {
                        change.relational_record_identity()
                            == Some(wake.due.source_record_identity())
                    })
                })
            {
                reconsider_retained_wake(
                    bridge,
                    wake,
                    lowering,
                    query_binding_identity,
                    query_capability_identity,
                    truth,
                    triggering_correspondence,
                );
            }
        }
        granular_invalidations.extend(promote_performed_signal_deliveries(
            delivered.granular_invalidations,
            wakes,
        ));
        *cursor = Some(*sequence);
    }
    *cursor = Some(commits.next_cursor);
    Ok(WorthQueryDeliveredAuthoritativeCommits {
        work_remaining: commits.work_remaining(),
        granular_invalidations,
    })
}

pub(super) fn promote_performed_signal_deliveries(
    deliveries: Vec<worth_runtime_bridge::facade::BridgeGranularInvalidationDelivery>,
    wakes: &mut [WorthQueryRetainedConditionalWake],
) -> Vec<worth_runtime_bridge::facade::BridgeGranularInvalidationDelivery> {
    deliveries
        .into_iter()
        .map(|delivery| {
            if delivery.performed_signal().is_some() {
                return delivery;
            }
            for wake in wakes.iter_mut() {
                let Some(evidence) = retained_decision_evidence_mut(&mut wake.decision) else {
                    continue;
                };
                match worth_runtime_bridge::facade::assemble_granular_invalidation_delivery(
                    delivery.correspondence_receipt(),
                    Some(evidence),
                ) {
                    Ok(performed) => return performed,
                    Err(_) => continue,
                }
            }
            delivery
        })
        .collect()
}

fn retained_decision_evidence_mut(
    decision: &mut super::signal_decision_reentry::WorthQueryRetainedConditionalDecision,
) -> Option<&mut worth_runtime_bridge::facade::BridgeConditionalDecisionEvidence> {
    use super::signal_decision_reentry::WorthQueryRetainedConditionalDecision as Decision;
    match decision {
        Decision::Eligible(evidence)
        | Decision::Suppressed(evidence)
        | Decision::Deferred(evidence)
        | Decision::OperationRetryable(evidence, _)
        | Decision::OperationBackpressured(evidence, _)
        | Decision::OperationControlStopped(evidence, _)
        | Decision::OperationTerminalFailure(evidence, _)
        | Decision::OperationSettlementDeferred(evidence, _)
        | Decision::OperationIndeterminate(evidence, _)
        | Decision::OperationCommitted(evidence)
        | Decision::OperationAlreadyCommitted(evidence) => Some(evidence),
        Decision::Failed(_) => None,
    }
}

struct WorthQueryDeliveredCommitDependencies {
    changed_records: BTreeSet<RelationalBridgeRecordIdentityParts>,
    granular_invalidations: Vec<worth_runtime_bridge::facade::BridgeGranularInvalidationDelivery>,
}

fn deliver_commit_dependencies(
    bridge: &mut BridgeOwnedSignalRuntime,
    lowering: &BridgeInstalledConditionalLowering,
    commit: worth_relational::facade::history::CommitId,
    truth: &WorthQueryConditionalTruthBasis,
) -> Result<WorthQueryDeliveredCommitDependencies, String> {
    let mut changed_records = BTreeSet::new();
    let mut granular_invalidations = Vec::new();
    for dependency_ordinal in 0..lowering.contract().dependency_count() {
        let outcome = bridge
            .deliver_authoritative_change(
                lowering,
                dependency_ordinal,
                RelationalCommittedPatchRequest::at_snapshot(
                    TruthCommitIdentity::from_relational_commit_id(commit.0),
                    truth.snapshot().clone(),
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
        if !receipt.change_set().changes().is_empty() {
            granular_invalidations.push(
                worth_runtime_bridge::facade::BridgeGranularInvalidationDelivery::direct(&receipt),
            );
        }
    }
    Ok(WorthQueryDeliveredCommitDependencies {
        changed_records,
        granular_invalidations,
    })
}

use std::sync::Arc;

use crate::durability::data::{DurabilityError, RecoveryFailureClass};
use crate::history::data::CanonicalCommitEnvelope;
use crate::history::data::{HistoryDriftClass, VersionNode};
use crate::runtime::RelationalRuntime;

use super::super::super::derived_index_artifacts::apply_envelope_derived_index_artifacts;

pub(super) fn apply_authoritative_commit_artifacts(
    runtime: &mut RelationalRuntime,
    envelope: &CanonicalCommitEnvelope,
    allow_reconstructed_replacement: bool,
    advance_branch_currentness: bool,
) -> Result<(), DurabilityError> {
    let history_before = runtime.history.clone();
    if runtime
        .history
        .commit_envelopes
        .get(&envelope.commit.commit_id)
        .is_some_and(|existing| existing.as_ref() != envelope)
        && !allow_reconstructed_replacement
    {
        return Err(DurabilityError::new(
            RecoveryFailureClass::CorruptCheckpoint,
            format!(
                "recovery commit envelope conflicts for commit {}",
                envelope.commit.commit_id.0
            ),
        ));
    }
    if runtime
        .history
        .commit_catalog
        .get(envelope.commit.commit_id)
        .is_some_and(|existing| existing.envelope().as_ref() != envelope)
        && !allow_reconstructed_replacement
    {
        return Err(DurabilityError::new(
            RecoveryFailureClass::CorruptCheckpoint,
            format!(
                "recovery commit artifact conflicts for commit {}",
                envelope.commit.commit_id.0
            ),
        ));
    }
    runtime.history.commit_graph.insert(
        envelope.commit.commit_id,
        VersionNode {
            commit: envelope.commit.clone(),
        },
    );
    runtime
        .history
        .commit_envelopes
        .insert(envelope.commit.commit_id, Arc::new(envelope.clone()));
    runtime
        .history
        .patch_stream_index
        .insert(envelope.patch.position, envelope.commit.commit_id);
    let result = runtime
        .history
        .record_recovered_commit(
            envelope,
            allow_reconstructed_replacement,
            advance_branch_currentness,
        )
        .map_err(|detail| DurabilityError::new(RecoveryFailureClass::CorruptCheckpoint, detail));
    if let Err(error) = result {
        runtime.history = history_before;
        return Err(error);
    }

    if !envelope.lineage_events().is_empty() {
        for event in envelope.lineage_events() {
            if let Some(existing) = runtime
                .lineage
                .events
                .iter_mut()
                .find(|candidate| candidate.event_id == event.event_id)
            {
                *existing = event.clone();
            } else {
                runtime.lineage.events.push(event.clone());
            }
        }
        runtime.lineage.events.sort_by_key(|event| event.event_id);
    }

    apply_envelope_derived_index_artifacts(runtime, envelope);
    Ok(())
}

pub(super) fn validate_recovered_history_parity(
    runtime: &RelationalRuntime,
    durable_envelope: &CanonicalCommitEnvelope,
) -> Result<(), DurabilityError> {
    let replay_access = runtime.replay();
    let Some(recovered_envelope) =
        replay_access.canonical_commit_envelope(durable_envelope.commit.commit_id)
    else {
        return Err(DurabilityError::new(
            RecoveryFailureClass::ReplayFailure,
            format!(
                "recovered commit envelope missing for durable commit {}",
                durable_envelope.commit.commit_id.0
            ),
        )
        .with_history_drift_class(HistoryDriftClass::DurabilityParityDrift));
    };
    runtime
        .performance_access()
        .count_merge_history_parent_comparisons(
            durable_envelope
                .commit
                .ordered_parents()
                .len()
                .max(recovered_envelope.commit.ordered_parents().len()),
        );
    if durable_envelope.commit.ordered_parents() != recovered_envelope.commit.ordered_parents()
        || durable_envelope.merge_parent_branches != recovered_envelope.merge_parent_branches
        || durable_envelope.merge_base_commits != recovered_envelope.merge_base_commits
    {
        return Err(DurabilityError::new(
            RecoveryFailureClass::ReplayFailure,
            format!(
                "recovered durable history parity drifted for commit {}",
                durable_envelope.commit.commit_id.0
            ),
        )
        .with_history_drift_class(HistoryDriftClass::DurabilityParityDrift));
    }
    Ok(())
}

pub(super) fn validate_expected_recovery_parent_shape(
    runtime: &RelationalRuntime,
    envelope: &CanonicalCommitEnvelope,
) -> Result<(), DurabilityError> {
    let ordered_parents = envelope.commit.ordered_parents();
    let observed_parents = ordered_parents.as_slice();
    let current_branch_head = runtime
        .history
        .branch_cell(&envelope.branch_context)
        .and_then(|cell| match cell.observation().target() {
            worth_foundational::FoundationalBranchTarget::Basis(target) => {
                Some(crate::history::data::CommitId(target.commit_id()))
            }
            worth_foundational::FoundationalBranchTarget::Empty => None,
        });
    // The branch-reference cell is the only currentness authority.  An empty
    // cell is an explicit empty target, not permission to infer a head from
    // the envelope's parent list.
    let admitted_target_head = current_branch_head;

    if envelope.merge_parent_branches.is_empty() {
        let expected = admitted_target_head.into_iter().collect::<Vec<_>>();
        if observed_parents != expected.as_slice() {
            return Err(DurabilityError::new(
                RecoveryFailureClass::ReplayFailure,
                format!(
                    "recovered durable history parity drifted for commit {}",
                    envelope.commit.commit_id.0
                ),
            )
            .with_history_drift_class(HistoryDriftClass::DurabilityParityDrift));
        }
        return Ok(());
    }

    let mut expected = Vec::with_capacity(envelope.merge_parent_branches.len() + 1);
    if let Some(target_head) = admitted_target_head {
        expected.push(target_head);
    }
    for branch in &envelope.merge_parent_branches {
        let branch_head = runtime
            .history
            .branch_cell(branch)
            .and_then(|cell| match cell.observation().target() {
                worth_foundational::FoundationalBranchTarget::Basis(target) => {
                    Some(crate::history::data::CommitId(target.commit_id()))
                }
                worth_foundational::FoundationalBranchTarget::Empty => None,
            })
            .ok_or_else(|| {
                DurabilityError::new(
                    RecoveryFailureClass::ReplayFailure,
                    format!(
                        "recovered durable history parity drifted for commit {}",
                        envelope.commit.commit_id.0
                    ),
                )
                .with_history_drift_class(HistoryDriftClass::DurabilityParityDrift)
            })?;
        expected.push(branch_head);
    }
    if observed_parents != expected.as_slice() {
        return Err(DurabilityError::new(
            RecoveryFailureClass::ReplayFailure,
            format!(
                "recovered durable history parity drifted for commit {}",
                envelope.commit.commit_id.0
            ),
        )
        .with_history_drift_class(HistoryDriftClass::DurabilityParityDrift));
    }
    Ok(())
}

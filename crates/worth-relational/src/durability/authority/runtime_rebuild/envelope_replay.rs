use std::collections::BTreeSet;
use std::sync::Arc;

use crate::authority::commit::pipeline::{
    execute_authoritative_commit, AuthoritativeCommitContext,
};
use crate::durability::data::{DurabilityError, RecoveryFailureClass, RecoveryPlan};
use crate::history::data::HistoryDriftClass;
use crate::logic::runtime::RelationalRuntime;
use crate::replay::data::{CanonicalCommitAuthorityKind, CanonicalCommitEnvelope};
use crate::transactions::data::{TransactionOptions, WorkerIntentBatch};

use super::history_parity::{
    apply_authoritative_commit_artifacts, validate_expected_recovery_parent_shape,
    validate_recovered_history_parity,
};
use super::merge_plan::merge_commit_mutation_plan_from_envelope;

pub(super) fn replay_durable_envelope(
    restored: &mut RelationalRuntime,
    envelope: &CanonicalCommitEnvelope,
    available_commit_ids: &BTreeSet<crate::history::data::CommitId>,
    plan: &RecoveryPlan,
) -> Result<(), DurabilityError> {
    let authoritative_parent_list = envelope.commit.ordered_parents();
    restored
        .performance_access()
        .count_merge_history_durability_validation(1, authoritative_parent_list.len());
    if authoritative_parent_list
        .as_slice()
        .iter()
        .any(|parent| !available_commit_ids.contains(parent))
    {
        return Err(DurabilityError::new(
            RecoveryFailureClass::MissingAuthoritativeParentClosure,
            format!(
                "missing authoritative ordered parent closure for commit {}",
                envelope.commit.commit_id.0
            ),
        )
        .with_history_drift_class(HistoryDriftClass::CanonicalHistoryDrift));
    }
    if authoritative_parent_list
        .as_slice()
        .iter()
        .any(|parent| !restored.history.commit_envelopes.contains_key(parent))
    {
        return Err(DurabilityError::new(
            RecoveryFailureClass::MissingAuthoritativeParentClosure,
            format!(
                "authoritative parent commit not recoverable before child {}",
                envelope.commit.commit_id.0
            ),
        )
        .with_history_drift_class(HistoryDriftClass::CanonicalHistoryDrift));
    }
    if !restored
        .history
        .branch_heads
        .contains_key(&envelope.branch_context)
    {
        let parent_head = envelope
            .commit
            .ordered_parents()
            .as_slice()
            .first()
            .and_then(|parent| restored.history.commit_envelopes.get(parent))
            .map(|parent| parent.commit.clone());
        restored
            .history
            .branch_heads
            .insert(envelope.branch_context.clone(), parent_head);
    }
    validate_expected_recovery_parent_shape(restored, envelope)?;
    if is_metadata_only_lineage_commit(envelope) || is_metadata_only_merge_commit(envelope) {
        restored.history_authority().publish_metadata_only_commit(
            envelope.commit.commit_id,
            envelope.commit.clone(),
            envelope.branch_context.clone(),
            envelope.patch.position,
            Arc::new(envelope.clone()),
        );
        if plan.should_restore_authoritative_envelope(envelope.commit.commit_id) {
            apply_authoritative_commit_artifacts(restored, envelope);
            validate_recovered_history_parity(restored, envelope)?;
        }
        return Ok(());
    }
    if envelope.merge_parent_branches.is_empty() {
        let mut txn = restored.begin_transaction(TransactionOptions {
            target_branch: Some(envelope.branch_context.clone()),
            merge_parent_branches: envelope.merge_parent_branches.clone(),
            ..schema_transition_options_for_replay(envelope)
        });
        txn.push_batch(WorkerIntentBatch {
            name: format!("recovery-commit-{}", envelope.commit.commit_id.0),
            partition_key: None,
            worker_local_only: true,
            intents: envelope
                .merged_plan
                .merged_intents
                .clone()
                .into_iter()
                .map(crate::transactions::data::MutationIntent::from)
                .collect(),
        });
        txn.commit().map_err(|error| {
            DurabilityError::new(
                RecoveryFailureClass::ReplayFailure,
                format!(
                    "failed to replay durable commit {}: {error:?}",
                    envelope.commit.commit_id.0
                ),
            )
        })?;
    } else {
        let merge_plan = merge_commit_mutation_plan_from_envelope(envelope).ok_or_else(|| {
            DurabilityError::new(
                RecoveryFailureClass::ReplayFailure,
                format!(
                    "failed to reconstruct merge execution summary for durable merge commit {}",
                    envelope.commit.commit_id.0
                ),
            )
        })?;
        let context = AuthoritativeCommitContext::from_merge(
            TransactionOptions {
                target_branch: Some(envelope.branch_context.clone()),
                merge_parent_branches: envelope.merge_parent_branches.clone(),
                ..TransactionOptions::default()
            },
            merge_plan,
        )
        .map_err(|_| {
            DurabilityError::new(
                RecoveryFailureClass::ReplayFailure,
                format!(
                    "failed to reconstruct merge authority context for durable merge commit {}",
                    envelope.commit.commit_id.0
                ),
            )
        })?;
        execute_authoritative_commit(restored, context).map_err(|_| {
            DurabilityError::new(
                RecoveryFailureClass::ReplayFailure,
                format!(
                    "failed to replay durable merge commit {}",
                    envelope.commit.commit_id.0
                ),
            )
        })?;
    }
    if plan.should_restore_authoritative_envelope(envelope.commit.commit_id) {
        apply_authoritative_commit_artifacts(restored, envelope);
        validate_recovered_history_parity(restored, envelope)?;
    }
    Ok(())
}

fn is_metadata_only_lineage_commit(envelope: &CanonicalCommitEnvelope) -> bool {
    envelope.authority_kind() == CanonicalCommitAuthorityKind::MetadataOnlyLineage
}

fn is_metadata_only_merge_commit(envelope: &CanonicalCommitEnvelope) -> bool {
    envelope.authority_kind() == CanonicalCommitAuthorityKind::VersionedTransaction
        && !envelope.merge_parent_branches.is_empty()
        && envelope.merged_plan.merged_intents.is_empty()
}

fn schema_transition_options_for_replay(envelope: &CanonicalCommitEnvelope) -> TransactionOptions {
    let options = TransactionOptions::default();
    let Some(transition) = envelope.schema_transition.as_ref() else {
        return options;
    };
    options.with_schema_transition(
        crate::schema::data::ProposedSchemaTransition {
            source_schema_id: transition.source_schema_id.clone(),
            source_schema_version_id: transition.source_schema_version_id,
            target_schema_id: transition.target_schema_id.clone(),
            target_schema_version_id: transition.target_schema_version_id,
            diff_atoms: transition.diff_atoms.clone(),
        },
        Some(transition.reconciliation_descriptor.policy),
    )
}

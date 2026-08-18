use std::collections::BTreeSet;
use std::sync::Arc;

use crate::authority::commit::pipeline::{
    execute_authoritative_commit, AuthoritativeCommitContext,
};
use crate::durability::data::{DurabilityError, RecoveryFailureClass, RecoveryPlan};
use crate::history::data::HistoryDriftClass;
use crate::history::data::{CanonicalCommitAuthorityKind, CanonicalCommitEnvelope};
use crate::runtime::RelationalRuntime;
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
    let had_checkpoint_artifact = restored
        .history
        .commit_envelopes
        .contains_key(&envelope.commit.commit_id);
    validate_parent_closure(restored, envelope, available_commit_ids)?;
    match envelope.branch_cell_checkpoint.clone() {
        Some(checkpoint) => restored
            .history
            .admit_recovered_branch_cell(checkpoint, &envelope.branch_context)
            .map_err(|detail| {
                DurabilityError::new(RecoveryFailureClass::CorruptCheckpoint, detail)
            })?,
        None if !restored.history.has_branch(&envelope.branch_context) => {
            return Err(DurabilityError::new(
                RecoveryFailureClass::CorruptCheckpoint,
                format!(
                    "recovery checkpoint omitted branch cell `{}` and its commit envelope carried no exact admission",
                    envelope.branch_context.0
                ),
            ));
        }
        None => {}
    }
    restored
        .history
        .require_recovered_branch(&envelope.branch_context)
        .map_err(|detail| DurabilityError::new(RecoveryFailureClass::CorruptCheckpoint, detail))?;
    for branch in &envelope.merge_parent_branches {
        restored
            .history
            .require_recovered_branch(branch)
            .map_err(|detail| {
                DurabilityError::new(RecoveryFailureClass::CorruptCheckpoint, detail)
            })?;
    }
    validate_expected_recovery_parent_shape(restored, envelope)?;
    restored
        .history
        .prepare_recovery_sequence(envelope.commit.commit_id, envelope.commit.version_id);
    replay_envelope_effect(restored, envelope)?;
    restore_authoritative_artifacts_when_required(
        restored,
        envelope,
        plan,
        !had_checkpoint_artifact,
    )
}

fn validate_parent_closure(
    restored: &RelationalRuntime,
    envelope: &CanonicalCommitEnvelope,
    available_commit_ids: &BTreeSet<crate::history::data::CommitId>,
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
    Ok(())
}

fn replay_envelope_effect(
    restored: &mut RelationalRuntime,
    envelope: &CanonicalCommitEnvelope,
) -> Result<(), DurabilityError> {
    if is_metadata_only_lineage_commit(envelope) || is_metadata_only_merge_commit(envelope) {
        if is_metadata_only_merge_commit(envelope) {
            apply_authoritative_commit_artifacts(restored, envelope, false)?;
            restored
                .history_authority()
                .publish_commit(
                    envelope.commit.commit_id,
                    envelope.commit.clone(),
                    envelope.branch_context.clone(),
                    envelope.patch.position,
                    Arc::new(envelope.clone()),
                )
                .map_err(|detail| {
                    DurabilityError::new(RecoveryFailureClass::ReplayFailure, detail)
                })?;
            return Ok(());
        }
        restored
            .history_authority()
            .publish_metadata_artifact(
                envelope.commit.commit_id,
                envelope.commit.clone(),
                envelope.branch_context.clone(),
                envelope.patch.position,
                Arc::new(envelope.clone()),
            )
            .map_err(|detail| DurabilityError::new(RecoveryFailureClass::ReplayFailure, detail))?;
        return Ok(());
    }
    if envelope.merge_parent_branches.is_empty() {
        replay_ordinary_commit(restored, envelope)?;
    } else {
        replay_merge_commit(restored, envelope)?;
    }
    Ok(())
}

fn restore_authoritative_artifacts_when_required(
    restored: &mut RelationalRuntime,
    envelope: &CanonicalCommitEnvelope,
    plan: &RecoveryPlan,
    allow_reconstructed_replacement: bool,
) -> Result<(), DurabilityError> {
    if plan.should_restore_authoritative_envelope(envelope.commit.commit_id) {
        apply_authoritative_commit_artifacts(restored, envelope, allow_reconstructed_replacement)?;
        validate_recovered_history_parity(restored, envelope)?;
    }
    Ok(())
}

fn replay_ordinary_commit(
    restored: &mut RelationalRuntime,
    envelope: &CanonicalCommitEnvelope,
) -> Result<(), DurabilityError> {
    let mut options = owner_options_for_branch(restored, &envelope.branch_context)?;
    options = options.with_merge_parent_bindings(owner_merge_parent_bindings(
        restored,
        &envelope.merge_parent_branches,
    )?);
    let options = schema_transition_options_for_replay(options, envelope);
    let mut txn = restored.begin_transaction(options);
    txn.push_batch(WorkerIntentBatch {
        name: format!("recovery-commit-{}", envelope.commit.commit_id.0),
        partition_key: None,
        worker_local_only: true,
        intents: envelope.merged_plan.merged_intents.clone().to_vec(),
    });
    txn.commit().map(|_| ()).map_err(|error| {
        DurabilityError::new(
            RecoveryFailureClass::ReplayFailure,
            format!(
                "failed to replay durable commit {}: {error:?}",
                envelope.commit.commit_id.0
            ),
        )
    })
}

fn replay_merge_commit(
    restored: &mut RelationalRuntime,
    envelope: &CanonicalCommitEnvelope,
) -> Result<(), DurabilityError> {
    let merge_plan = merge_commit_mutation_plan_from_envelope(envelope).ok_or_else(|| {
        DurabilityError::new(
            RecoveryFailureClass::ReplayFailure,
            format!(
                "failed to reconstruct merge execution summary for durable merge commit {}",
                envelope.commit.commit_id.0
            ),
        )
    })?;
    let mut options = owner_options_for_branch(restored, &envelope.branch_context)?;
    options = options.with_merge_parent_bindings(owner_merge_parent_bindings(
        restored,
        &envelope.merge_parent_branches,
    )?);
    let context = AuthoritativeCommitContext::from_merge(options, merge_plan).map_err(|_| {
        DurabilityError::new(
            RecoveryFailureClass::ReplayFailure,
            format!(
                "failed to reconstruct merge authority context for durable merge commit {}",
                envelope.commit.commit_id.0
            ),
        )
    })?;
    execute_authoritative_commit(restored, context)
        .map(|_| ())
        .map_err(|_| {
            DurabilityError::new(
                RecoveryFailureClass::ReplayFailure,
                format!(
                    "failed to replay durable merge commit {}",
                    envelope.commit.commit_id.0
                ),
            )
        })
}

fn is_metadata_only_lineage_commit(envelope: &CanonicalCommitEnvelope) -> bool {
    envelope.authority_kind() == CanonicalCommitAuthorityKind::MetadataOnlyLineage
}

fn is_metadata_only_merge_commit(envelope: &CanonicalCommitEnvelope) -> bool {
    envelope.authority_kind() == CanonicalCommitAuthorityKind::VersionedTransaction
        && !envelope.merge_parent_branches.is_empty()
        && envelope.merged_plan.merged_intents.is_empty()
}

fn schema_transition_options_for_replay(
    options: TransactionOptions,
    envelope: &CanonicalCommitEnvelope,
) -> TransactionOptions {
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

fn owner_options_for_branch(
    restored: &RelationalRuntime,
    branch: &crate::history::data::BranchId,
) -> Result<TransactionOptions, DurabilityError> {
    let identity = restored.branch_identity(branch).map_err(|denial| {
        DurabilityError::new(
            RecoveryFailureClass::ReplayFailure,
            format!("recovered branch cannot issue transaction binding: {denial:?}"),
        )
    })?;
    restored
        .transaction_options_for(&identity)
        .map_err(|denial| {
            DurabilityError::new(
                RecoveryFailureClass::ReplayFailure,
                format!("recovered branch binding was denied: {denial:?}"),
            )
        })
}

fn owner_merge_parent_bindings(
    restored: &RelationalRuntime,
    branches: &[crate::history::data::BranchId],
) -> Result<Vec<crate::branch::RelationalLegacyBranchBinding>, DurabilityError> {
    branches
        .iter()
        .map(|branch| {
            let identity = restored.branch_identity(branch).map_err(|denial| {
                DurabilityError::new(
                    RecoveryFailureClass::ReplayFailure,
                    format!("recovered merge parent identity was denied: {denial:?}"),
                )
            })?;
            restored
                .transaction_options_for(&identity)
                .map(|options| options.branch_binding().clone())
                .map_err(|denial| {
                    DurabilityError::new(
                        RecoveryFailureClass::ReplayFailure,
                        format!("recovered merge parent binding was denied: {denial:?}"),
                    )
                })
        })
        .collect()
}

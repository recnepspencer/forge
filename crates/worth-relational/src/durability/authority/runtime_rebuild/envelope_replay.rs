use std::collections::BTreeSet;
use std::sync::Arc;

use crate::authority::commit::pipeline::{
    execute_authoritative_commit, AuthoritativeCommitContext,
};
use crate::durability::data::{DurabilityError, RecoveryFailureClass, RecoveryPlan};
use crate::history::data::HistoryDriftClass;
use crate::history::data::{CanonicalCommitAuthorityKind, CanonicalCommitEnvelope};
use crate::runtime::RelationalRuntime;
use crate::transactions::data::WorkerIntentBatch;

use super::history_parity::{
    apply_authoritative_commit_artifacts, validate_expected_recovery_parent_shape,
    validate_recovered_history_parity,
};
use super::merge_plan::merge_commit_mutation_plan_from_envelope;
use super::recovered_schema_basis::RecoveredSchemaBasis;
use super::root_inventory::RecoveredRootInventory;

mod owner_bindings;

use owner_bindings::{owner_merge_parent_bindings, owner_options_for_branch};

pub(super) fn replay_durable_envelope(
    restored: &mut RelationalRuntime,
    envelope: &CanonicalCommitEnvelope,
    available_commit_ids: &BTreeSet<crate::history::data::CommitId>,
    plan: &RecoveryPlan,
    recovered_roots: &mut RecoveredRootInventory,
) -> Result<(), DurabilityError> {
    let had_checkpoint_artifact = restored
        .history
        .commit_envelopes
        .contains_key(&envelope.commit.commit_id);
    validate_parent_closure(restored, envelope, available_commit_ids)?;
    match envelope.branch_cell_checkpoint.clone() {
        Some(checkpoint) => {
            let recovered_root = match checkpoint.observation.target() {
                worth_foundational::FoundationalBranchTarget::Empty => None,
                worth_foundational::FoundationalBranchTarget::Basis(target) => {
                    recovered_roots.resolve(crate::history::data::CommitId(target.commit_id()))
                }
            };
            let recovered_provenance_root =
                checkpoint.fork_provenance.as_ref().and_then(|provenance| {
                    match provenance.target() {
                        worth_foundational::FoundationalBranchTarget::Empty => None,
                        worth_foundational::FoundationalBranchTarget::Basis(target) => {
                            recovered_roots
                                .resolve(crate::history::data::CommitId(target.commit_id()))
                        }
                    }
                });
            restored
                .history
                .admit_recovered_branch_cell(
                    checkpoint,
                    &envelope.branch_context,
                    recovered_root,
                    recovered_provenance_root,
                    &restored.services.symbols,
                )
                .map_err(|detail| {
                    DurabilityError::new(RecoveryFailureClass::CorruptCheckpoint, detail)
                })?
        }
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
    let replays_mutation_pipeline = envelope.authority_kind()
        == CanonicalCommitAuthorityKind::VersionedTransaction
        && !is_metadata_only_merge_commit(envelope);
    if !replays_mutation_pipeline && !envelope.record_allocations().is_empty() {
        return Err(DurabilityError::new(
            RecoveryFailureClass::ReplayFailure,
            format!(
                "durable commit {} carries record allocation evidence without a mutation replay path",
                envelope.commit.commit_id.0
            ),
        ));
    }
    if replays_mutation_pipeline {
        restored
            .record_identity
            .stage_replay_allocations(envelope.record_allocations().to_vec())
            .map_err(|detail| DurabilityError::new(RecoveryFailureClass::ReplayFailure, detail))?;
    }
    let replay_result = replay_envelope_effect(restored, envelope);
    let staged_was_not_consumed = restored.record_identity.clear_staged_replay_allocations();
    replay_result?;
    if staged_was_not_consumed && replays_mutation_pipeline {
        return Err(DurabilityError::new(
            RecoveryFailureClass::ReplayFailure,
            format!(
                "durable commit {} did not consume its canonical record allocation evidence",
                envelope.commit.commit_id.0
            ),
        ));
    }
    restore_authoritative_artifacts_when_required(
        restored,
        envelope,
        plan,
        !had_checkpoint_artifact,
    )?;
    recovered_roots.retain_current(restored, &envelope.branch_context)
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
            if governed_merge_authority_was_published(envelope) {
                require_merge_execution_authority(envelope)?;
            }
            // Preflight the owner door before recovery writes any sidecar or
            // advances the branch cell. Artifact recovery below restores the
            // immutable catalog/index projections only; its explicit
            // currentness flag is false so publication owns the sole advance.
            recovered_branch_binding(restored, &envelope.branch_context)?;
            apply_authoritative_commit_artifacts(restored, envelope, false, false)?;
            // Re-issue the owner binding after sidecar admission so
            // publication validates the exact pre-publication cell and
            // performs the one currentness transition for this commit.
            let branch_binding = recovered_branch_binding(restored, &envelope.branch_context)?;
            let published_partition_delta =
                restored.storage_authority().affirm_no_partition_changes();
            restored
                .mvcc_publication_authority()
                .publish_commit(
                    envelope.commit.commit_id,
                    envelope.commit.clone(),
                    &branch_binding,
                    published_partition_delta,
                    envelope.patch.position,
                    Arc::new(envelope.clone()),
                )
                .map_err(|detail| {
                    DurabilityError::new(RecoveryFailureClass::ReplayFailure, detail)
                })?;
            return Ok(());
        }
        let branch_binding = recovered_branch_binding(restored, &envelope.branch_context)?;
        restored
            .mvcc_publication_authority()
            .publish_metadata_artifact(
                envelope.commit.commit_id,
                envelope.commit.clone(),
                &branch_binding,
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

fn recovered_branch_binding(
    restored: &RelationalRuntime,
    branch_id: &crate::history::data::BranchId,
) -> Result<crate::branch::RelationalLegacyBranchBinding, DurabilityError> {
    let identity = restored.branch_identity(branch_id).map_err(|denial| {
        DurabilityError::new(
            RecoveryFailureClass::CorruptCheckpoint,
            format!("recovery branch identity admission denied: {denial:?}"),
        )
    })?;
    restored
        .legacy_branch_binding_for_identity(&identity)
        .map_err(|denial| {
            DurabilityError::new(
                RecoveryFailureClass::CorruptCheckpoint,
                format!("recovery branch binding admission denied: {denial:?}"),
            )
        })
}

fn restore_authoritative_artifacts_when_required(
    restored: &mut RelationalRuntime,
    envelope: &CanonicalCommitEnvelope,
    plan: &RecoveryPlan,
    allow_reconstructed_replacement: bool,
) -> Result<(), DurabilityError> {
    if plan.should_restore_authoritative_envelope(envelope.commit.commit_id) {
        apply_authoritative_commit_artifacts(
            restored,
            envelope,
            allow_reconstructed_replacement,
            true,
        )?;
        validate_recovered_history_parity(restored, envelope)?;
    }
    Ok(())
}

fn replay_ordinary_commit(
    restored: &mut RelationalRuntime,
    envelope: &CanonicalCommitEnvelope,
) -> Result<(), DurabilityError> {
    let schema_basis = RecoveredSchemaBasis::admit(restored, envelope)?;
    let mut options = owner_options_for_branch(restored, &envelope.branch_context)?;
    options = options.with_merge_parent_bindings(owner_merge_parent_bindings(
        restored,
        &envelope.merge_parent_branches,
    )?);
    let options = schema_basis.apply(options);
    let mut txn = restored.begin_transaction(options);
    txn.push_batch(WorkerIntentBatch {
        name: format!("recovery-commit-{}", envelope.commit.commit_id.0),
        partition_key: None,
        worker_local_only: true,
        intents: envelope.merged_plan.merged_intents.clone().to_vec(),
    });
    let outcome = txn.commit().map_err(|error| {
        DurabilityError::new(
            RecoveryFailureClass::ReplayFailure,
            format!(
                "failed to replay durable commit {}: {error:?}",
                envelope.commit.commit_id.0
            ),
        )
    })?;
    schema_basis.validate_replayed(outcome.envelope())
}

fn replay_merge_commit(
    restored: &mut RelationalRuntime,
    envelope: &CanonicalCommitEnvelope,
) -> Result<(), DurabilityError> {
    let merge_plan = require_merge_execution_authority(envelope)?;
    let schema_basis = RecoveredSchemaBasis::admit(restored, envelope)?;
    let mut options = owner_options_for_branch(restored, &envelope.branch_context)?;
    options = options.with_merge_parent_bindings(owner_merge_parent_bindings(
        restored,
        &envelope.merge_parent_branches,
    )?);
    let context = AuthoritativeCommitContext::from_merge(schema_basis.apply(options), merge_plan)
        .map_err(|_| {
        DurabilityError::new(
            RecoveryFailureClass::ReplayFailure,
            format!(
                "failed to reconstruct merge authority context for durable merge commit {}",
                envelope.commit.commit_id.0
            ),
        )
    })?;
    let outcome = execute_authoritative_commit(restored, context).map_err(|_| {
        DurabilityError::new(
            RecoveryFailureClass::ReplayFailure,
            format!(
                "failed to replay durable merge commit {}",
                envelope.commit.commit_id.0
            ),
        )
    })?;
    schema_basis.validate_replayed(outcome.envelope())
}

fn require_merge_execution_authority(
    envelope: &CanonicalCommitEnvelope,
) -> Result<crate::transactions::data::MergeCommitMutationPlan, DurabilityError> {
    merge_commit_mutation_plan_from_envelope(envelope).ok_or_else(|| {
        DurabilityError::new(
            RecoveryFailureClass::ReplayFailure,
            format!(
                "failed to reconstruct merge execution summary for durable merge commit {}",
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

fn governed_merge_authority_was_published(envelope: &CanonicalCommitEnvelope) -> bool {
    envelope.merge_execution_authority.is_some()
        || envelope.diagnostics_summary.entries.iter().any(|entry| {
            entry.code == crate::diagnostics::data::DiagnosticCode::MergeExecutionPublished
        })
}

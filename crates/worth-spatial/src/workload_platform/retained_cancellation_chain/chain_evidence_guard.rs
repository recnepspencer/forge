use std::collections::BTreeSet;

use super::{
    chain_checkpoint::RetainedCancellationCheckpoint, chain_policy::RetainedCancellationChainError,
};
use crate::workload_platform::evidence_ledger::{
    CompleteWorkloadEvidenceLedger, WorkloadEvidenceStage,
};

pub(super) fn require_distinct_checkpoint_evidence(
    checkpoints: &[RetainedCancellationCheckpoint],
) -> Result<(), RetainedCancellationChainError> {
    let mut checkpoint_identities = BTreeSet::new();
    let mut transform_identities = BTreeSet::new();
    let mut capture_identities = BTreeSet::new();
    let mut replay_identities = BTreeSet::new();
    let mut projection_identities = BTreeSet::new();
    for checkpoint in checkpoints {
        let accepted = checkpoint_identities.insert(checkpoint.checkpoint_identity())
            && transform_identities.insert(checkpoint.transform_stage_identity())
            && capture_identities.insert(checkpoint.retained_artifact_capture_identity())
            && replay_identities.insert(checkpoint.replay_checkpoint_identity())
            && projection_identities.insert(checkpoint.projection_consumed_identity());
        if !accepted {
            return Err(
                RetainedCancellationChainError::DuplicateCheckpointEvidence {
                    step_index: checkpoint.step_index(),
                },
            );
        }
    }
    Ok(())
}

pub(super) fn require_checkpoint_stages_match_ledger(
    checkpoints: &[RetainedCancellationCheckpoint],
    evidence_ledger: &CompleteWorkloadEvidenceLedger,
) -> Result<(), RetainedCancellationChainError> {
    let transform_identity =
        required_stage_identity(evidence_ledger, WorkloadEvidenceStage::Transform)?;
    let retained_replay_identity =
        required_stage_identity(evidence_ledger, WorkloadEvidenceStage::RetainedReplay)?;

    for checkpoint in checkpoints {
        if checkpoint.transform_stage_receipt_identity() != transform_identity {
            return Err(
                RetainedCancellationChainError::CheckpointNotFromPlatformStage {
                    step_index: checkpoint.step_index(),
                    stage: WorkloadEvidenceStage::Transform,
                },
            );
        }
        if checkpoint.retained_replay_stage_identity() != retained_replay_identity {
            return Err(
                RetainedCancellationChainError::CheckpointNotFromPlatformStage {
                    step_index: checkpoint.step_index(),
                    stage: WorkloadEvidenceStage::RetainedReplay,
                },
            );
        }
    }
    Ok(())
}

pub(super) fn require_projection_consumed_checkpoint_match(
    checkpoints: &[RetainedCancellationCheckpoint],
    retained_basis_identity: &str,
) -> Result<(), RetainedCancellationChainError> {
    if let Some(checkpoint) = checkpoints.iter().find(|checkpoint| {
        checkpoint.retained_basis_identity() != retained_basis_identity
            || !checkpoint.projection_matches_retained_checkpoint()
    }) {
        return Err(
            RetainedCancellationChainError::ProjectionConsumedFactMismatch {
                step_index: checkpoint.step_index(),
            },
        );
    }
    Ok(())
}

fn required_stage_identity(
    evidence_ledger: &CompleteWorkloadEvidenceLedger,
    stage: WorkloadEvidenceStage,
) -> Result<&str, RetainedCancellationChainError> {
    evidence_ledger
        .row_for_stage(stage)
        .filter(|row| row.is_receipt_backed() && row.is_admitted())
        .map(|row| row.evidence_identity())
        .ok_or(RetainedCancellationChainError::MissingReceiptBackedStage(
            stage,
        ))
}

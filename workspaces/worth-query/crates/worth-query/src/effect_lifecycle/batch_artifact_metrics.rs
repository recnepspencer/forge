use crate::WorthQueryEvidenceIdentity;

use super::batch::LoweredEffectBatchExecutionArtifact;

pub(super) fn lowered_batch_artifact_identity(
    artifact: &LoweredEffectBatchExecutionArtifact,
) -> &WorthQueryEvidenceIdentity {
    match artifact {
        LoweredEffectBatchExecutionArtifact::RelationalMutation(batch) => {
            batch.batch_mutation_identity()
        }
    }
}

pub(super) fn lowered_batch_artifact_width(
    artifact: &LoweredEffectBatchExecutionArtifact,
) -> usize {
    match artifact {
        LoweredEffectBatchExecutionArtifact::RelationalMutation(batch) => batch
            .declarations()
            .iter()
            .map(|declaration| declaration.counters().workflow_lowering_width())
            .sum(),
    }
}

pub(super) fn lowered_batch_artifact_executor_rediscovery_count(
    artifact: &LoweredEffectBatchExecutionArtifact,
) -> usize {
    match artifact {
        LoweredEffectBatchExecutionArtifact::RelationalMutation(batch) => batch
            .declarations()
            .iter()
            .map(|declaration| declaration.counters().workflow_executor_rediscovery_count())
            .sum(),
    }
}

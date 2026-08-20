use worth_foundational::FoundationalBranchTarget;

use super::{
    RelationalBranchCellCheckpoint, RelationalBranchCellDenial, RelationalBranchIdentity,
    RelationalBranchObservationConstructionDenial, RelationalBranchReferenceCell,
};

impl RelationalBranchReferenceCell {
    pub(crate) fn from_checkpoint(
        expected_runtime_instance_id: u64,
        checkpoint: RelationalBranchCellCheckpoint,
    ) -> Result<Self, RelationalBranchCellDenial> {
        if checkpoint.observation.branch_id().as_str()
            != format!(
                "relational/{}/{}",
                checkpoint.runtime_instance_id, checkpoint.branch_id.0
            )
        {
            return Err(RelationalBranchCellDenial::CheckpointObservationMismatch);
        }
        if let FoundationalBranchTarget::Basis(target) = checkpoint.observation.target() {
            if target.runtime_instance_id() != checkpoint.runtime_instance_id {
                return Err(RelationalBranchCellDenial::CheckpointObservationMismatch);
            }
        }
        match (
            checkpoint.fork_provenance.as_ref(),
            checkpoint.fork_source_branch_id.as_ref(),
        ) {
            (None, None) => {}
            (Some(source), Some(source_branch_id)) => {
                let expected_branch_id = format!(
                    "relational/{}/{}",
                    checkpoint.runtime_instance_id, source_branch_id.0
                );
                if source.branch_id().as_str() != expected_branch_id {
                    return Err(RelationalBranchCellDenial::CheckpointForkProvenanceMismatch);
                }
                if let FoundationalBranchTarget::Basis(target) = source.target() {
                    if target.runtime_instance_id() != checkpoint.runtime_instance_id {
                        return Err(RelationalBranchCellDenial::CheckpointObservationMismatch);
                    }
                }
            }
            _ => return Err(RelationalBranchCellDenial::CheckpointForkProvenanceMismatch),
        }
        let cell = Self {
            identity: RelationalBranchIdentity::new(
                checkpoint.runtime_instance_id,
                checkpoint.branch_id,
            ),
            observation: checkpoint.observation,
            truth_version: checkpoint.truth_version,
            head_retention_obligations: checkpoint.head_retention_obligations,
            fork_provenance: checkpoint.fork_provenance,
            fork_source_branch_id: checkpoint.fork_source_branch_id,
        };
        cell.rebind_runtime(expected_runtime_instance_id)
            .map_err(|denial| match denial {
                RelationalBranchObservationConstructionDenial::ForkProvenanceMismatch => {
                    RelationalBranchCellDenial::CheckpointForkProvenanceMismatch
                }
                _ => RelationalBranchCellDenial::CheckpointObservationMismatch,
            })
    }
}

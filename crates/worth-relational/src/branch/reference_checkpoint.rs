use worth_foundational::FoundationalBranchTarget;

use super::{
    RelationalBranchCellCheckpoint, RelationalBranchCellDenial, RelationalBranchIdentity,
    RelationalBranchObservationConstructionDenial, RelationalBranchReferenceCell,
};

impl RelationalBranchReferenceCell {
    /// Replace the descriptive target prepared during preflight with the
    /// storage owner's content-backed target. Generation/truth progression
    /// was already checked and advanced by `advance_truth`.
    pub(crate) fn replace_truth_target(
        &mut self,
        target: FoundationalBranchTarget<super::RelationalBranchTarget>,
    ) {
        if let FoundationalBranchTarget::Basis(target) = &target {
            assert_eq!(
                target.runtime_instance_id(),
                self.identity.runtime_instance_id(),
                "owner-produced content target remains runtime-affine"
            );
        }
        let mut state = self.state();
        state.observation = super::RelationalBranchReferenceObservation::new(
            state.observation.branch_id().clone(),
            target,
            state.observation.generation(),
        );
    }

    pub(crate) fn from_checkpoint_with_root(
        expected_runtime_instance_id: u64,
        checkpoint: RelationalBranchCellCheckpoint,
        root: Option<std::sync::Arc<super::super::RelationalBranchRoot>>,
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
                checkpoint.branch_id.clone(),
            ),
            state: std::sync::Arc::new(std::sync::Mutex::new(
                super::RelationalBranchReferenceMutableState {
                    observation: checkpoint.observation,
                    truth_version: checkpoint.truth_version,
                    head_retention_obligations: checkpoint.head_retention_obligations,
                    fork_provenance: checkpoint.fork_provenance,
                    fork_source_branch_id: checkpoint.fork_source_branch_id,
                    root,
                },
            )),
            basis_registry: crate::branch::RelationalBranchBasisRegistry::default(),
            coordination: crate::branch::coordination::RelationalBranchCoordinationCell::fresh(
                checkpoint.runtime_instance_id,
                &checkpoint.branch_id,
            ),
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

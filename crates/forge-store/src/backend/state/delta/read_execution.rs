use crate::{
    delta::{
        BranchDeltaReadRequest, BranchDeltaReadResult, BranchDeltaReadStrategy,
        Milestone7IndependentReference, SameBranchDescendantWitness,
    },
    failure::{StoreError, StoreErrorKind},
};

use crate::backend::records::StoreState;

impl StoreState {
    pub fn read_branch_delta(
        &self,
        witness: SameBranchDescendantWitness,
    ) -> Result<BranchDeltaReadResult, StoreError> {
        let plan = self.plan_branch_delta_read_from_witness(&witness)?;
        match plan.strategy {
            BranchDeltaReadStrategy::DirectLayerRead => {
                let export = self.materialize_branch_delta_export(&plan)?;
                let parity = self
                    .read_branch_delta_control(witness.clone())?
                    .authoritative_export()
                    .clone()
                    .into_canonicalized();
                let direct = export.clone().into_canonicalized();
                if direct.canonical_json() != parity.canonical_json() {
                    return Err(StoreError::new(
                        StoreErrorKind::BranchDeltaReplayParityViolation,
                        format!(
                            "branch delta direct-layer read for branch `{}` target {} diverged from authoritative replay parity",
                            plan.locality.branch_id.0, plan.locality.target_commit_id.0
                        ),
                    ));
                }
                Ok(BranchDeltaReadResult::new(plan, export))
            }
            BranchDeltaReadStrategy::AuthorityReplayControl => Err(StoreError::new(
                StoreErrorKind::BranchDeltaBasisUnsupported,
                format!(
                    "branch delta read for branch `{}` target {} requires the authority replay control lane and is not admitted on the direct-layer path",
                    plan.locality.branch_id.0, plan.locality.target_commit_id.0
                ),
            )),
            BranchDeltaReadStrategy::EmptyBranchReuse => {
                let export = self.materialize_branch_delta_export(&plan)?;
                Ok(BranchDeltaReadResult::new(plan, export))
            }
        }
    }

    pub fn read_branch_delta_control(
        &self,
        witness: SameBranchDescendantWitness,
    ) -> Result<BranchDeltaReadResult, StoreError> {
        let plan = self.plan_branch_delta_control_from_witness(&witness);
        let export = self.materialize_authority_replay_control_export(&witness)?;
        Ok(BranchDeltaReadResult::new(plan, export))
    }

    pub fn read_branch_delta_control_from_milestone_7_reference(
        &self,
        reference: Milestone7IndependentReference,
    ) -> Result<BranchDeltaReadResult, StoreError> {
        let witness = self.admit_same_branch_descendant(BranchDeltaReadRequest::new(
            reference.branch_id().clone(),
            reference.target_commit_id(),
        ))?;
        self.read_branch_delta_control(witness)
    }
}

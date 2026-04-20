use crate::failure::StoreError;

use super::{StateBackedStoreBackend, StatePersistence};
use super::layout_support::{
    merge_milestone_6_structural_block_record, milestone_6_chunk_membership_record,
    milestone_6_commit_coupled_layout_seed_rebuild_records,
    milestone_6_scope_slice_membership_record, milestone_6_structural_block_record,
};

impl<P: StatePersistence> StateBackedStoreBackend<P> {
    pub fn rebuild_milestone_6_derived_artifacts_from_materializations(
        &mut self,
    ) -> Result<crate::Milestone6DerivedArtifactRebuildReport, StoreError> {
        let commit_coupled_seeds =
            milestone_6_commit_coupled_layout_seed_rebuild_records(&self.state)?;
        let mut next = self.state.clone();
        next.milestone_6_scope_slice_membership_records.clear();
        next.milestone_6_chunk_membership_records.clear();
        next.milestone_6_structural_block_records.clear();

        for commit_coupled_seed in &commit_coupled_seeds {
            let plan = match self
                .state
                .plan_aspect_layout_read(commit_coupled_seed.request.clone())?
            {
                crate::AspectLayoutReadPlanDecision::Admitted(plan) => plan,
                crate::AspectLayoutReadPlanDecision::Fallback(plan) => {
                    return Err(StoreError::backend_integrity(format!(
                        "commit-coupled milestone 6 layout seed `{}` no longer admits during rebuild: {}",
                        commit_coupled_seed.artifact_id,
                        plan.reason()
                    )))
                }
                crate::AspectLayoutReadPlanDecision::Rejected(plan) => {
                    return Err(StoreError::backend_integrity(format!(
                        "commit-coupled milestone 6 layout seed `{}` was rejected during rebuild: {}",
                        commit_coupled_seed.artifact_id,
                        plan.reason()
                    )))
                }
            };
            let expected_materialization_artifact_id =
                crate::layout::layout_materialization_artifact_id(&plan);
            if commit_coupled_seed.layout_materialization_artifact_id
                != expected_materialization_artifact_id
            {
                return Err(StoreError::backend_integrity(format!(
                    "commit-coupled milestone 6 layout seed `{}` drifted from expected materialization `{expected_materialization_artifact_id}`",
                    commit_coupled_seed.artifact_id
                )));
            }
            let materialization = self.fetch_existing_milestone_6_layout_support(
                &commit_coupled_seed.layout_materialization_artifact_id,
            )?;
            if materialization.admitted_plan() != &plan {
                return Err(StoreError::backend_integrity(format!(
                    "persisted milestone 6 materialization `{}` drifted from rebuild admission plan",
                    materialization.artifact_id()
                )));
            }
            let scope_membership_record =
                milestone_6_scope_slice_membership_record(&materialization)?;
            let chunk_membership_record = milestone_6_chunk_membership_record(&materialization);
            let structural_block_record = milestone_6_structural_block_record(&materialization);
            next.milestone_6_scope_slice_membership_records.insert(
                scope_membership_record.artifact_id.clone(),
                scope_membership_record,
            );
            next.milestone_6_chunk_membership_records.insert(
                chunk_membership_record.artifact_id.clone(),
                chunk_membership_record,
            );
            merge_milestone_6_structural_block_record(&mut next, structural_block_record);
        }

        self.commit_replacement_state(next)?;
        Ok(crate::Milestone6DerivedArtifactRebuildReport::new(
            self.state.milestone_6_layout_materialization_records.len(),
            self.state.milestone_6_scope_slice_membership_records.len(),
            self.state.milestone_6_structural_block_records.len(),
            self.state.milestone_6_chunk_membership_records.len(),
        ))
    }

    pub fn rebuild_milestone_6_derived_artifacts_from_authority(
        &mut self,
    ) -> Result<crate::Milestone6DerivedArtifactRebuildReport, StoreError> {
        let commit_coupled_seeds =
            milestone_6_commit_coupled_layout_seed_rebuild_records(&self.state)?;
        let mut next = self.state.clone();
        next.milestone_6_layout_materialization_records.clear();
        next.milestone_6_scope_slice_membership_records.clear();
        next.milestone_6_chunk_membership_records.clear();
        next.milestone_6_structural_block_records.clear();

        for commit_coupled_seed in &commit_coupled_seeds {
            let plan = match self
                .state
                .plan_aspect_layout_read(commit_coupled_seed.request.clone())?
            {
                crate::AspectLayoutReadPlanDecision::Admitted(plan) => plan,
                crate::AspectLayoutReadPlanDecision::Fallback(plan) => {
                    return Err(StoreError::backend_integrity(format!(
                        "commit-coupled milestone 6 layout seed `{}` no longer admits during authority rebuild: {}",
                        commit_coupled_seed.artifact_id,
                        plan.reason()
                    )))
                }
                crate::AspectLayoutReadPlanDecision::Rejected(plan) => {
                    return Err(StoreError::backend_integrity(format!(
                        "commit-coupled milestone 6 layout seed `{}` was rejected during authority rebuild: {}",
                        commit_coupled_seed.artifact_id,
                        plan.reason()
                    )))
                }
            };
            let artifact_id = crate::layout::layout_materialization_artifact_id(&plan);
            if commit_coupled_seed.layout_materialization_artifact_id != artifact_id {
                return Err(StoreError::backend_integrity(format!(
                    "commit-coupled milestone 6 layout seed `{}` drifted from expected authority-rebuilt materialization `{artifact_id}`",
                    commit_coupled_seed.artifact_id
                )));
            }
            let block_reuse = self.state.admit_structural_block_reuse(plan.clone())?;
            let frozen_layout = self.state.freeze_chunk_model(plan.clone())?;
            let milestone_7_reference = self
                .state
                .admit_milestone_7_independent_layout_reference(plan.clone())?;
            let milestone_9_reference = self
                .state
                .admit_milestone_9_physical_chunk_reference(frozen_layout.clone())?;
            let control = self
                .state
                .read_branch_delta_control_from_milestone_7_reference(
                    crate::Milestone7IndependentReference::new(
                        milestone_7_reference.branch_id().clone(),
                        milestone_7_reference.frontier_commit_id(),
                    ),
                )?;
            let materialization = crate::layout::Milestone6LayoutMaterialization::new(
                artifact_id.clone(),
                plan,
                block_reuse,
                frozen_layout,
                milestone_7_reference,
                milestone_9_reference,
                crate::layout::stable_layout_truth_digest(control.authoritative_export()),
                control.authoritative_export().commit_envelopes.len(),
            );
            let scope_membership_record =
                milestone_6_scope_slice_membership_record(&materialization)?;
            let chunk_membership_record = milestone_6_chunk_membership_record(&materialization);
            let structural_block_record = milestone_6_structural_block_record(&materialization);
            next.milestone_6_layout_materialization_records.insert(
                artifact_id.clone(),
                crate::backend::records::Milestone6LayoutMaterializationRecord {
                    artifact_id,
                    materialization,
                },
            );
            next.milestone_6_scope_slice_membership_records.insert(
                scope_membership_record.artifact_id.clone(),
                scope_membership_record,
            );
            next.milestone_6_chunk_membership_records.insert(
                chunk_membership_record.artifact_id.clone(),
                chunk_membership_record,
            );
            merge_milestone_6_structural_block_record(&mut next, structural_block_record);
        }

        self.commit_replacement_state(next)?;
        Ok(crate::Milestone6DerivedArtifactRebuildReport::new(
            self.state.milestone_6_layout_materialization_records.len(),
            self.state.milestone_6_scope_slice_membership_records.len(),
            self.state.milestone_6_structural_block_records.len(),
            self.state.milestone_6_chunk_membership_records.len(),
        ))
    }
}

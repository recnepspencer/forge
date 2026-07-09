use crate::failure::StoreError;
use crate::layout::{AspectLayoutReadRequest, Milestone6LayoutMaterialization};
use worth_relational::facade::history::BranchId;

use super::layout_support::{
    attach_milestone_6_commit_coupled_layout_seed_to_commit_support_summary,
    merge_milestone_6_structural_block_record, milestone_6_chunk_membership_record,
    milestone_6_commit_coupled_layout_seed_record, milestone_6_scope_slice_membership_record,
    milestone_6_structural_block_record,
};
use super::{StateBackedStoreBackend, StatePersistence};

impl<P: StatePersistence> StateBackedStoreBackend<P> {
    pub(crate) fn note_milestone_6_scope_prepare(
        &mut self,
        request: &AspectLayoutReadRequest,
    ) -> Result<u64, StoreError> {
        let artifact_id = crate::layout::published_layout_request_artifact_id(request)?;
        let entry = self
            .milestone_6_scope_prepare_counts
            .entry(artifact_id)
            .or_insert(0);
        *entry += 1;
        Ok(*entry)
    }

    pub(crate) fn milestone_6_branch_has_materialized_support(&self, branch_id: &BranchId) -> bool {
        self.state
            .milestone_6_layout_materialization_records
            .values()
            .any(|record| {
                record
                    .materialization
                    .admitted_plan()
                    .request()
                    .target()
                    .branch_id()
                    == branch_id
            })
    }

    pub(crate) fn record_milestone_6_proof_only_prepare(&self) {
        self.counters.record_milestone_6_proof_only_prepare();
    }

    pub(crate) fn record_milestone_6_on_demand_materialize(&self) {
        self.counters.record_milestone_6_on_demand_materialize();
    }

    pub(crate) fn record_milestone_6_policy_eager_resolution(&self) {
        self.counters.record_milestone_6_policy_eager_resolution();
    }

    pub(crate) fn record_milestone_6_policy_eager_publish(&self) {
        self.counters.record_milestone_6_policy_eager_publish();
    }

    pub(crate) fn record_milestone_6_policy_eager_reuse_existing(&self) {
        self.counters
            .record_milestone_6_policy_eager_reuse_existing();
    }

    pub fn materialize_milestone_6_layout_support(
        &mut self,
        request: AspectLayoutReadRequest,
    ) -> Result<Milestone6LayoutMaterialization, StoreError> {
        let plan = self.require_admitted_aspect_layout_plan(request, "layout materialization")?;
        let block_reuse = self.admit_structural_block_reuse(plan.clone())?;
        let frozen_layout = self.freeze_chunk_model(plan.clone())?;
        let milestone_7_reference =
            self.admit_milestone_7_independent_layout_reference(plan.clone())?;
        let milestone_9_reference =
            self.admit_milestone_9_physical_chunk_reference(frozen_layout.clone())?;
        let control = self
            .state
            .read_branch_delta_control_from_milestone_7_reference(
                crate::Milestone7IndependentReference::new(
                    milestone_7_reference.branch_id().clone(),
                    milestone_7_reference.frontier_commit_id(),
                ),
            )?;
        let artifact_id = crate::layout::layout_materialization_artifact_id(&plan);
        let materialization = Milestone6LayoutMaterialization::new(
            artifact_id.clone(),
            plan,
            block_reuse,
            frozen_layout,
            milestone_7_reference,
            milestone_9_reference,
            crate::layout::stable_layout_truth_digest(control.authoritative_export()),
            control.authoritative_export().commit_envelopes.len(),
        );
        let authority_basis_commit = self
            .state
            .commit_record(materialization.admitted_plan().request().target().frontier_commit_id())
            .ok_or_else(|| {
                StoreError::backend_integrity(format!(
                    "milestone 6 layout materialization `{}` targeted missing authority frontier commit `{}`",
                    materialization.artifact_id(),
                    materialization
                        .admitted_plan()
                        .request()
                        .target()
                        .frontier_commit_id()
                        .0
                ))
            })?;
        let commit_coupled_seed_record = milestone_6_commit_coupled_layout_seed_record(
            &materialization,
            authority_basis_commit,
        )?;
        let scope_membership_record = milestone_6_scope_slice_membership_record(&materialization)?;
        let chunk_membership_record = milestone_6_chunk_membership_record(&materialization);
        let structural_block_record = milestone_6_structural_block_record(&materialization);

        let mut next = self.state.clone();
        next.milestone_6_layout_materialization_records.insert(
            artifact_id.clone(),
            crate::backend::records::Milestone6LayoutMaterializationRecord {
                artifact_id,
                materialization: materialization.clone(),
            },
        );
        next.milestone_6_commit_coupled_layout_seed_records.insert(
            commit_coupled_seed_record.artifact_id.clone(),
            commit_coupled_seed_record,
        );
        attach_milestone_6_commit_coupled_layout_seed_to_commit_support_summary(
            &mut next,
            materialization
                .admitted_plan()
                .request()
                .target()
                .frontier_commit_id(),
            &materialization,
        )?;
        next.milestone_6_scope_slice_membership_records.insert(
            scope_membership_record.artifact_id.clone(),
            scope_membership_record,
        );
        next.milestone_6_chunk_membership_records.insert(
            chunk_membership_record.artifact_id.clone(),
            chunk_membership_record,
        );
        merge_milestone_6_structural_block_record(&mut next, structural_block_record);
        self.commit_replacement_state(next)?;
        Ok(materialization)
    }

    pub fn fetch_milestone_6_layout_support(
        &self,
        request: AspectLayoutReadRequest,
    ) -> Result<Milestone6LayoutMaterialization, StoreError> {
        let plan =
            self.require_admitted_aspect_layout_plan(request, "layout materialization fetch")?;
        let artifact_id = crate::layout::layout_materialization_artifact_id(&plan);
        self.fetch_existing_milestone_6_layout_support(&artifact_id)
    }

    pub(crate) fn fetch_existing_milestone_6_layout_support(
        &self,
        artifact_id: &str,
    ) -> Result<Milestone6LayoutMaterialization, StoreError> {
        self.state
            .milestone_6_layout_materialization_records
            .get(artifact_id)
            .map(|record| record.materialization.clone())
            .ok_or_else(|| {
                StoreError::new(
                    crate::failure::StoreErrorKind::AspectLayoutArtifactMissing,
                    format!("milestone 6 layout materialization `{artifact_id}` not found"),
                )
            })
    }
}

use crate::{
    delta::BranchDeltaReadPlan,
    evidence::{Milestone5DeltaStorageReport, Milestone5ReadPathReport},
    failure::{StoreError, StoreErrorKind},
};
use worth_relational::facade::history::{BranchId, CommitId};

use crate::backend::{integrity::branch_key, records::StoreState};

impl StoreState {
    pub(crate) fn milestone_5_delta_storage_report(
        &self,
        branch_id: BranchId,
        target_commit_id: CommitId,
        direct_plan: &BranchDeltaReadPlan,
        control_plan: &BranchDeltaReadPlan,
    ) -> Result<Milestone5DeltaStorageReport, StoreError> {
        let basis = self
            .branch_shared_base_records
            .get(&branch_key(&branch_id))
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::BranchDeltaBasisUnsupported,
                    format!(
                        "branch `{}` does not publish a shared-base branch delta basis yet",
                        branch_id.0
                    ),
                )
            })?;
        let live_layers = self
            .branch_delta_layer_records
            .values()
            .filter(|record| record.branch_id == branch_id)
            .collect::<Vec<_>>();
        Ok(Milestone5DeltaStorageReport {
            branch_id,
            target_commit_id,
            shared_base_source_branch_id: basis.source_branch_id.clone(),
            shared_base_source_frontier_commit_id: basis.source_frontier_commit_id,
            live_layer_count: live_layers.len(),
            live_layer_commit_count: live_layers
                .iter()
                .map(|record| record.commit_ids.len())
                .sum(),
            replacement_layer_count: live_layers
                .iter()
                .filter(|record| !record.replacement_of_layer_ids.is_empty())
                .count(),
            direct_path: Milestone5ReadPathReport::from(direct_plan),
            control_path: Milestone5ReadPathReport::from(control_plan),
            control_reference_surface: "Milestone7IndependentReference".to_string(),
        })
    }
}

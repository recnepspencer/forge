use crate::{
    backend::records::{Milestone6CommitCoupledLayoutSeedRecord, StoreState},
    failure::StoreError,
    layout::published_layout_request_artifact_id,
};

impl StoreState {
    pub(super) fn verify_commit_coupled_layout_seed_record(
        &self,
        stored_key: &str,
        record: &Milestone6CommitCoupledLayoutSeedRecord,
    ) -> Result<(), StoreError> {
        let expected_artifact_id = published_layout_request_artifact_id(&record.request)?;
        if stored_key != expected_artifact_id {
            return Err(StoreError::backend_integrity(format!(
                "milestone 6 commit-coupled layout seed map key `{stored_key}` did not match expected artifact id `{expected_artifact_id}`"
            )));
        }
        if record.artifact_id != expected_artifact_id {
            return Err(StoreError::backend_integrity(format!(
                "milestone 6 commit-coupled layout seed payload `{}` drifted from expected artifact id `{expected_artifact_id}`",
                record.artifact_id
            )));
        }
        let expected_plan = match crate::layout::classify_layout_request(record.request.clone())? {
            crate::AspectLayoutReadPlanDecision::Admitted(plan) => plan,
            crate::AspectLayoutReadPlanDecision::Fallback(plan) => {
                return Err(StoreError::backend_integrity(format!(
                    "milestone 6 commit-coupled layout seed `{expected_artifact_id}` no longer admits during integrity verification: {}",
                    plan.reason()
                )))
            }
            crate::AspectLayoutReadPlanDecision::Rejected(plan) => {
                return Err(StoreError::backend_integrity(format!(
                    "milestone 6 commit-coupled layout seed `{expected_artifact_id}` was rejected during integrity verification: {}",
                    plan.reason()
                )))
            }
        };
        let expected_layout_materialization_artifact_id =
            crate::layout::layout_materialization_artifact_id(&expected_plan);
        if record.layout_materialization_artifact_id != expected_layout_materialization_artifact_id
        {
            return Err(StoreError::backend_integrity(format!(
                "milestone 6 commit-coupled layout seed `{expected_artifact_id}` drifted from expected layout materialization `{expected_layout_materialization_artifact_id}`"
            )));
        }
        if record.authority_basis_commit_id != record.request.target().frontier_commit_id() {
            return Err(StoreError::backend_integrity(format!(
                "milestone 6 commit-coupled layout seed `{expected_artifact_id}` drifted from its request frontier commit basis"
            )));
        }
        let authority_basis_commit = self.commit_record(record.authority_basis_commit_id).ok_or_else(|| {
            StoreError::backend_integrity(format!(
                "milestone 6 commit-coupled layout seed `{expected_artifact_id}` referenced missing authority basis commit `{}`",
                record.authority_basis_commit_id.0
            ))
        })?;
        if authority_basis_commit.envelope.branch_context != *record.request.target().branch_id() {
            return Err(StoreError::backend_integrity(format!(
                "milestone 6 commit-coupled layout seed `{expected_artifact_id}` drifted from authority basis branch `{}`",
                authority_basis_commit.envelope.branch_context.0
            )));
        }
        if authority_basis_commit.envelope_digest != record.authority_basis_commit_digest {
            return Err(StoreError::backend_integrity(format!(
                "milestone 6 commit-coupled layout seed `{expected_artifact_id}` drifted from authority basis digest for commit `{}`",
                record.authority_basis_commit_id.0
            )));
        }
        if authority_basis_commit.commit_sequence != record.authority_basis_commit_sequence {
            return Err(StoreError::backend_integrity(format!(
                "milestone 6 commit-coupled layout seed `{expected_artifact_id}` drifted from authority basis sequence for commit `{}`",
                record.authority_basis_commit_id.0
            )));
        }
        if let Some(materialization) = self
            .milestone_6_layout_materialization_records
            .get(&record.layout_materialization_artifact_id)
        {
            if record.request != *materialization.materialization.admitted_plan().request() {
                return Err(StoreError::backend_integrity(format!(
                    "milestone 6 commit-coupled layout seed `{expected_artifact_id}` drifted from the persisted admitted request"
                )));
            }
        }
        Ok(())
    }
}

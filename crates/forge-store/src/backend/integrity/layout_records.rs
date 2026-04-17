use crate::{
    backend::records::{
        Milestone6ChunkMembershipRecord, Milestone6CommitCoupledLayoutSeedRecord,
        Milestone6LayoutMaterializationRecord, Milestone6ScopeSliceMembershipRecord,
        Milestone6StructuralBlockRecord, StoreState,
    },
    failure::StoreError,
    layout::{
        chunk_membership_artifact_id, layout_materialization_artifact_id,
        layout_scope_membership_artifact_id, published_layout_request_artifact_id,
    },
};

impl StoreState {
    pub fn verify_layout_record_family(&self) -> Result<(), StoreError> {
        for (stored_key, record) in &self.milestone_6_layout_materialization_records {
            self.verify_layout_materialization_record(stored_key, record)?;
        }
        for (stored_key, record) in &self.milestone_6_commit_coupled_layout_seed_records {
            self.verify_commit_coupled_layout_seed_record(stored_key, record)?;
        }
        for (stored_key, record) in &self.milestone_6_scope_slice_membership_records {
            self.verify_scope_slice_membership_record(stored_key, record)?;
        }
        for (stored_key, record) in &self.milestone_6_chunk_membership_records {
            self.verify_chunk_membership_record(stored_key, record)?;
        }
        for (stored_key, record) in &self.milestone_6_structural_block_records {
            self.verify_structural_block_record(stored_key, record)?;
        }
        Ok(())
    }

    fn verify_commit_coupled_layout_seed_record(
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
            layout_materialization_artifact_id(&expected_plan);
        if record.layout_materialization_artifact_id != expected_layout_materialization_artifact_id {
            return Err(StoreError::backend_integrity(format!(
                "milestone 6 commit-coupled layout seed `{expected_artifact_id}` drifted from expected layout materialization `{expected_layout_materialization_artifact_id}`"
            )));
        }
        if record.authority_basis_commit_id != record.request.target().frontier_commit_id() {
            return Err(StoreError::backend_integrity(format!(
                "milestone 6 commit-coupled layout seed `{expected_artifact_id}` drifted from its request frontier commit basis"
            )));
        }
        let authority_basis_commit = self
            .commit_record(record.authority_basis_commit_id)
            .ok_or_else(|| {
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

    fn verify_layout_materialization_record(
        &self,
        stored_key: &str,
        record: &Milestone6LayoutMaterializationRecord,
    ) -> Result<(), StoreError> {
        let expected_artifact_id =
            layout_materialization_artifact_id(record.materialization.admitted_plan());
        if stored_key != expected_artifact_id {
            return Err(StoreError::backend_integrity(format!(
                "milestone 6 layout materialization map key `{stored_key}` did not match expected artifact id `{expected_artifact_id}`"
            )));
        }
        if record.artifact_id != expected_artifact_id {
            return Err(StoreError::backend_integrity(format!(
                "milestone 6 layout materialization payload `{}` drifted from expected artifact id `{expected_artifact_id}`",
                record.artifact_id
            )));
        }
        if record.materialization.artifact_id() != expected_artifact_id {
            return Err(StoreError::backend_integrity(format!(
                "milestone 6 layout materialization `{expected_artifact_id}` drifted from its internal artifact id"
            )));
        }
        if record.materialization.block_reuse().structural_block_id()
            != record.materialization.admitted_plan().structural_block_id()
        {
            return Err(StoreError::backend_integrity(format!(
                "milestone 6 layout materialization `{expected_artifact_id}` drifted between admitted plan and structural block reuse witness"
            )));
        }
        if record.materialization.frozen_layout().witness().physical_chunk_id()
            != record.materialization.milestone_9_reference().physical_chunk_id()
        {
            return Err(StoreError::backend_integrity(format!(
                "milestone 6 layout materialization `{expected_artifact_id}` drifted between frozen layout witness and milestone 9 physical chunk reference"
            )));
        }
        if record.materialization.frozen_layout().witness().determinism_digest()
            != record.materialization.milestone_9_reference().determinism_digest()
        {
            return Err(StoreError::backend_integrity(format!(
                "milestone 6 layout materialization `{expected_artifact_id}` drifted between frozen layout determinism and milestone 9 physical chunk determinism"
            )));
        }
        if record.materialization.milestone_7_reference().branch_id()
            != record.materialization.admitted_plan().request().target().branch_id()
        {
            return Err(StoreError::backend_integrity(format!(
                "milestone 6 layout materialization `{expected_artifact_id}` drifted between admitted plan target branch and milestone 7 reference"
            )));
        }
        if record.materialization.milestone_7_reference().frontier_commit_id()
            != record
                .materialization
                .admitted_plan()
                .request()
                .target()
                .frontier_commit_id()
        {
            return Err(StoreError::backend_integrity(format!(
                "milestone 6 layout materialization `{expected_artifact_id}` drifted between admitted plan target frontier and milestone 7 reference"
            )));
        }
        let control = self.read_branch_delta_control_from_milestone_7_reference(
            crate::Milestone7IndependentReference::new(
                record.materialization.milestone_7_reference().branch_id().clone(),
                record.materialization.milestone_7_reference().frontier_commit_id(),
            ),
        )?;
        let expected_semantic_truth_digest =
            crate::layout::stable_layout_truth_digest(control.authoritative_export());
        if record.materialization.semantic_truth_digest() != expected_semantic_truth_digest {
            return Err(StoreError::backend_integrity(format!(
                "milestone 6 layout materialization `{expected_artifact_id}` drifted from canonical semantic truth digest"
            )));
        }
        let expected_authoritative_commit_count = control.authoritative_export().commit_envelopes.len();
        if record.materialization.authoritative_commit_count() != expected_authoritative_commit_count {
            return Err(StoreError::backend_integrity(format!(
                "milestone 6 layout materialization `{expected_artifact_id}` drifted from canonical authoritative commit count"
            )));
        }
        Ok(())
    }

    fn verify_scope_slice_membership_record(
        &self,
        stored_key: &str,
        record: &Milestone6ScopeSliceMembershipRecord,
    ) -> Result<(), StoreError> {
        let materialization = self
            .milestone_6_layout_materialization_records
            .get(&record.layout_materialization_artifact_id)
            .ok_or_else(|| {
                StoreError::backend_integrity(format!(
                    "milestone 6 scope membership `{}` referenced missing layout materialization `{}`",
                    record.artifact_id, record.layout_materialization_artifact_id
                ))
            })?;
        let expected_artifact_id =
            layout_scope_membership_artifact_id(materialization.materialization.admitted_plan().request())?;
        if stored_key != expected_artifact_id {
            return Err(StoreError::backend_integrity(format!(
                "milestone 6 scope membership map key `{stored_key}` did not match expected artifact id `{expected_artifact_id}`"
            )));
        }
        if record.artifact_id != expected_artifact_id {
            return Err(StoreError::backend_integrity(format!(
                "milestone 6 scope membership payload `{}` drifted from expected artifact id `{expected_artifact_id}`",
                record.artifact_id
            )));
        }
        if record.branch_id != *materialization.materialization.admitted_plan().request().target().branch_id()
        {
            return Err(StoreError::backend_integrity(format!(
                "milestone 6 scope membership `{expected_artifact_id}` drifted from admitted plan branch"
            )));
        }
        if record.frontier_commit_id
            != materialization
                .materialization
                .admitted_plan()
                .request()
                .target()
                .frontier_commit_id()
        {
            return Err(StoreError::backend_integrity(format!(
                "milestone 6 scope membership `{expected_artifact_id}` drifted from admitted plan frontier"
            )));
        }
        if record.scope_class
            != materialization
                .materialization
                .admitted_plan()
                .request()
                .scope_class()
                .label()
        {
            return Err(StoreError::backend_integrity(format!(
                "milestone 6 scope membership `{expected_artifact_id}` drifted from admitted plan scope class"
            )));
        }
        if record.projection_digest
            != materialization
                .materialization
                .milestone_7_reference()
                .projection_digest()
        {
            return Err(StoreError::backend_integrity(format!(
                "milestone 6 scope membership `{expected_artifact_id}` drifted from milestone 7 projection digest"
            )));
        }
        if record.slice_ids != materialization.materialization.admitted_plan().slice_ids() {
            return Err(StoreError::backend_integrity(format!(
                "milestone 6 scope membership `{expected_artifact_id}` drifted from admitted plan slice ids"
            )));
        }
        Ok(())
    }

    fn verify_chunk_membership_record(
        &self,
        stored_key: &str,
        record: &Milestone6ChunkMembershipRecord,
    ) -> Result<(), StoreError> {
        let materialization = self
            .milestone_6_layout_materialization_records
            .get(&record.layout_materialization_artifact_id)
            .ok_or_else(|| {
                StoreError::backend_integrity(format!(
                    "milestone 6 chunk membership `{}` referenced missing layout materialization `{}`",
                    record.artifact_id, record.layout_materialization_artifact_id
                ))
            })?;
        let expected_artifact_id =
            chunk_membership_artifact_id(materialization.materialization.frozen_layout());
        if stored_key != expected_artifact_id {
            return Err(StoreError::backend_integrity(format!(
                "milestone 6 chunk membership map key `{stored_key}` did not match expected artifact id `{expected_artifact_id}`"
            )));
        }
        if record.artifact_id != expected_artifact_id {
            return Err(StoreError::backend_integrity(format!(
                "milestone 6 chunk membership payload `{}` drifted from expected artifact id `{expected_artifact_id}`",
                record.artifact_id
            )));
        }
        if record.physical_chunk_id
            != *materialization
                .materialization
                .frozen_layout()
                .witness()
                .physical_chunk_id()
        {
            return Err(StoreError::backend_integrity(format!(
                "milestone 6 chunk membership `{expected_artifact_id}` drifted from frozen witness physical chunk id"
            )));
        }
        if record.chunk_shape_version
            != materialization
                .materialization
                .frozen_layout()
                .witness()
                .chunk_shape_version()
        {
            return Err(StoreError::backend_integrity(format!(
                "milestone 6 chunk membership `{expected_artifact_id}` drifted from frozen witness chunk shape version"
            )));
        }
        if record.determinism_digest
            != materialization
                .materialization
                .frozen_layout()
                .witness()
                .determinism_digest()
        {
            return Err(StoreError::backend_integrity(format!(
                "milestone 6 chunk membership `{expected_artifact_id}` drifted from frozen witness determinism digest"
            )));
        }
        if record.slice_ids
            != materialization
                .materialization
                .frozen_layout()
                .witness()
                .ordered_slice_ids()
        {
            return Err(StoreError::backend_integrity(format!(
                "milestone 6 chunk membership `{expected_artifact_id}` drifted from frozen witness slice ids"
            )));
        }
        Ok(())
    }

    fn verify_structural_block_record(
        &self,
        stored_key: &str,
        record: &Milestone6StructuralBlockRecord,
    ) -> Result<(), StoreError> {
        let expected_artifact_id = format!(
            "layout-structural-block:{}",
            record.structural_block_id.as_str()
        );
        if stored_key != expected_artifact_id {
            return Err(StoreError::backend_integrity(format!(
                "milestone 6 structural block map key `{stored_key}` did not match expected artifact id `{expected_artifact_id}`"
            )));
        }
        if record.artifact_id != expected_artifact_id {
            return Err(StoreError::backend_integrity(format!(
                "milestone 6 structural block payload `{}` drifted from expected artifact id `{expected_artifact_id}`",
                record.artifact_id
            )));
        }
        if record.supporting_layout_materialization_artifact_ids.is_empty() {
            return Err(StoreError::backend_integrity(format!(
                "milestone 6 structural block `{expected_artifact_id}` has no supporting layout materializations"
            )));
        }
        for layout_materialization_artifact_id in &record.supporting_layout_materialization_artifact_ids {
            let materialization = self
                .milestone_6_layout_materialization_records
                .get(layout_materialization_artifact_id)
                .ok_or_else(|| {
                    StoreError::backend_integrity(format!(
                        "milestone 6 structural block `{}` referenced missing layout materialization `{}`",
                        record.artifact_id, layout_materialization_artifact_id
                    ))
                })?;
            if record.scope_class != materialization.materialization.block_reuse().scope_class() {
                return Err(StoreError::backend_integrity(format!(
                    "milestone 6 structural block `{expected_artifact_id}` drifted from structural block reuse scope class"
                )));
            }
            if record.equivalence_contract_version
                != materialization
                    .materialization
                    .block_reuse()
                    .equivalence_contract_version()
            {
                return Err(StoreError::backend_integrity(format!(
                    "milestone 6 structural block `{expected_artifact_id}` drifted from structural block reuse equivalence contract version"
                )));
            }
            if record.slice_ids != materialization.materialization.block_reuse().slice_ids() {
                return Err(StoreError::backend_integrity(format!(
                    "milestone 6 structural block `{expected_artifact_id}` drifted from structural block reuse slice ids"
                )));
            }
            if record.structural_block_id
                != *materialization
                    .materialization
                    .block_reuse()
                    .structural_block_id()
            {
                return Err(StoreError::backend_integrity(format!(
                    "milestone 6 structural block `{expected_artifact_id}` drifted from structural block reuse id"
                )));
            }
        }
        Ok(())
    }
}

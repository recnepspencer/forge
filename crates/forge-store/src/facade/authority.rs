use crate::{
    authority::{AdvanceCursorWitness, AuthoritativeBranchHeadRecord, DurableCursorAcknowledgeRequest, DurableCursorResumePlan, DurableCursorResumeRequest, FetchedAuthoritativeCommit, FetchedDurableCursorIdentity, PersistedAuthoritativeCommit, PersistedSubscriberCheckpoint, ResumeAdmittedCursor},
    delta::{BranchDeltaReadPlan, BranchDeltaReadRequest, BranchDeltaReadResult, SameBranchDescendantWitness, SharedBaseBranchCreationReceipt, SharedBaseBranchCreationRequest, SharedBaseBranchCreationWitness},
    failure::StoreError,
};
use forge_relational::facade::{history::{BranchId, CommitId}, replay::CanonicalCommitEnvelope};

use super::ForgeStore;

impl ForgeStore {
    pub fn create_branch(
        &mut self,
        new_branch: BranchId,
        from_branch: Option<&BranchId>,
    ) -> Result<AuthoritativeBranchHeadRecord, StoreError> {
        self.backend.create_branch(new_branch, from_branch)
    }

    pub fn create_shared_base_branch(
        &mut self,
        request: SharedBaseBranchCreationRequest,
    ) -> Result<SharedBaseBranchCreationReceipt, StoreError> {
        self.backend.create_shared_base_branch(request)
    }

    pub fn admit_shared_base_branch_creation(
        &self,
        request: SharedBaseBranchCreationRequest,
    ) -> Result<SharedBaseBranchCreationWitness, StoreError> {
        self.backend.admit_shared_base_branch_creation(request)
    }

    pub fn append_canonical_commit(
        &mut self,
        envelope: CanonicalCommitEnvelope,
    ) -> Result<PersistedAuthoritativeCommit, StoreError> {
        self.append_runtime_envelope(envelope)
    }

    pub fn fetch_canonical_commit(
        &self,
        commit_id: CommitId,
    ) -> Result<FetchedAuthoritativeCommit, StoreError> {
        self.backend.fetch_commit(commit_id)
    }

    pub fn fetch_branch_head(
        &self,
        branch_id: &BranchId,
    ) -> Result<AuthoritativeBranchHeadRecord, StoreError> {
        self.backend.fetch_branch_head(branch_id)
    }

    pub fn plan_branch_delta_read(
        &self,
        request: BranchDeltaReadRequest,
    ) -> Result<BranchDeltaReadPlan, StoreError> {
        self.backend.plan_branch_delta_read(request)
    }

    pub fn admit_same_branch_descendant(
        &self,
        request: BranchDeltaReadRequest,
    ) -> Result<SameBranchDescendantWitness, StoreError> {
        self.backend.admit_same_branch_descendant(request)
    }

    pub fn admit_milestone_7_independent_reference(
        &self,
        request: BranchDeltaReadRequest,
    ) -> Result<crate::Milestone7IndependentReference, StoreError> {
        self.backend
            .admit_milestone_7_independent_reference(request)
    }

    pub fn read_branch_delta(
        &self,
        witness: SameBranchDescendantWitness,
    ) -> Result<BranchDeltaReadResult, StoreError> {
        self.backend.read_branch_delta(witness)
    }

    pub fn read_branch_delta_control(
        &self,
        witness: SameBranchDescendantWitness,
    ) -> Result<BranchDeltaReadResult, StoreError> {
        self.backend.read_branch_delta_control(witness)
    }

    pub fn read_branch_delta_control_from_milestone_7_reference(
        &self,
        reference: crate::Milestone7IndependentReference,
    ) -> Result<BranchDeltaReadResult, StoreError> {
        self.backend
            .read_branch_delta_control_from_milestone_7_reference(reference)
    }

    pub fn acknowledge_cursor(
        &mut self,
        request: DurableCursorAcknowledgeRequest,
    ) -> Result<PersistedSubscriberCheckpoint, StoreError> {
        let witness = self.admit_cursor_advance(request)?;
        self.acknowledge_cursor_progress(witness)
    }

    pub fn fetch_durable_cursor_identity(
        &self,
        cursor_id: &str,
    ) -> Result<FetchedDurableCursorIdentity, StoreError> {
        self.backend.fetch_durable_cursor_identity(cursor_id)
    }

    pub fn plan_cursor_resume(
        &self,
        request: DurableCursorResumeRequest,
    ) -> Result<DurableCursorResumePlan, StoreError> {
        self.backend.plan_cursor_resume(request)
    }


    pub fn admit_cursor_resume(
        &self,
        request: DurableCursorResumeRequest,
    ) -> Result<ResumeAdmittedCursor, StoreError> {
        Ok(ResumeAdmittedCursor::new(
            self.backend.plan_cursor_resume(request)?,
        ))
    }

    pub fn admit_cursor_advance(
        &self,
        request: DurableCursorAcknowledgeRequest,
    ) -> Result<AdvanceCursorWitness, StoreError> {
        Ok(AdvanceCursorWitness::new(request))
    }

    pub fn admit_resumed_cursor_advance(
        &self,
        resumed: &ResumeAdmittedCursor,
        request: DurableCursorAcknowledgeRequest,
    ) -> Result<AdvanceCursorWitness, StoreError> {
        let identity = resumed.identity();
        if identity.cursor_id != request.cursor_id()
            || identity.subscriber_id != request.subscriber_id()
            || identity.branch_id != *request.branch_id()
            || identity.feed_shape_id != request.feed_shape_id()
            || identity.schema_interpretation_id != request.schema_interpretation_id()
            || identity.cursor_semantics_version != request.cursor_semantics_version()
        {
            return Err(StoreError::new(
                crate::StoreErrorKind::CursorEquivalenceViolation,
                "cursor advance witness does not match the admitted resume identity basis",
            ));
        }
        Ok(AdvanceCursorWitness::new(request))
    }

    pub fn acknowledge_cursor_progress(
        &mut self,
        witness: AdvanceCursorWitness,
    ) -> Result<PersistedSubscriberCheckpoint, StoreError> {
        self.backend.acknowledge_cursor(witness.into_request())
    }

    pub fn acknowledge_resumed_cursor_progress(
        &mut self,
        resumed: &ResumeAdmittedCursor,
        witness: AdvanceCursorWitness,
    ) -> Result<PersistedSubscriberCheckpoint, StoreError> {
        if resumed.identity().cursor_id != witness.cursor_id() {
            return Err(StoreError::new(
                crate::StoreErrorKind::CursorEquivalenceViolation,
                "resume-admitted cursor and advance witness must reference the same cursor identity",
            ));
        }
        self.backend.acknowledge_cursor(witness.into_request())
    }
}

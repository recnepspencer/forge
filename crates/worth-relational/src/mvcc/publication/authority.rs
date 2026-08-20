use std::sync::Arc;

use crate::branch::RelationalLegacyBranchBinding;
use crate::history::data::{CanonicalCommitEnvelope, CommitId, RelationalCommitReceipt};
use crate::publication::patch::data::PatchStreamPosition;
use crate::runtime::RelationalRuntime;

use super::validation::{
    validate_publication, PublicationRequest, PublicationSequence, ValidatedPublication,
};

pub(crate) struct RelationalPublicationAuthority<'runtime> {
    runtime: &'runtime mut RelationalRuntime,
}

impl RelationalRuntime {
    pub(crate) fn mvcc_publication_authority(&mut self) -> RelationalPublicationAuthority<'_> {
        RelationalPublicationAuthority { runtime: self }
    }
}

impl<'runtime> RelationalPublicationAuthority<'runtime> {
    pub(crate) fn validate_versioned_publication(
        &self,
        commit_id: CommitId,
        commit_reference: &RelationalCommitReceipt,
        binding: &RelationalLegacyBranchBinding,
        envelope: &CanonicalCommitEnvelope,
    ) -> Result<(), String> {
        self.validate(PublicationRequest {
            commit_id,
            commit_reference,
            binding,
            envelope,
            sequence: PublicationSequence::Truth,
        })
        .map(|_| ())
    }

    pub(crate) fn validate_metadata_publication(
        &self,
        commit_id: CommitId,
        commit_reference: &RelationalCommitReceipt,
        binding: &RelationalLegacyBranchBinding,
        envelope: &CanonicalCommitEnvelope,
    ) -> Result<(), String> {
        self.validate(PublicationRequest {
            commit_id,
            commit_reference,
            binding,
            envelope,
            sequence: PublicationSequence::Metadata,
        })
        .map(|_| ())
    }

    pub(crate) fn publish_commit(
        &mut self,
        commit_id: CommitId,
        commit_reference: RelationalCommitReceipt,
        binding: &RelationalLegacyBranchBinding,
        patch_position: PatchStreamPosition,
        envelope: Arc<CanonicalCommitEnvelope>,
    ) -> Result<(), String> {
        self.publish(
            commit_id,
            commit_reference,
            binding,
            patch_position,
            envelope,
            PublicationSequence::Truth,
        )
    }

    pub(crate) fn publish_metadata_artifact(
        &mut self,
        commit_id: CommitId,
        commit_reference: RelationalCommitReceipt,
        binding: &RelationalLegacyBranchBinding,
        patch_position: PatchStreamPosition,
        envelope: Arc<CanonicalCommitEnvelope>,
    ) -> Result<(), String> {
        self.validate_metadata_publication(
            commit_id,
            &commit_reference,
            binding,
            envelope.as_ref(),
        )?;
        self.publish(
            commit_id,
            commit_reference,
            binding,
            patch_position,
            envelope,
            PublicationSequence::Metadata,
        )
    }

    fn validate(&self, request: PublicationRequest<'_>) -> Result<ValidatedPublication, String> {
        validate_publication(self.runtime, request)
    }

    fn publish(
        &mut self,
        commit_id: CommitId,
        commit_reference: RelationalCommitReceipt,
        binding: &RelationalLegacyBranchBinding,
        patch_position: PatchStreamPosition,
        envelope: Arc<CanonicalCommitEnvelope>,
        sequence: PublicationSequence,
    ) -> Result<(), String> {
        let validated = self.validate(PublicationRequest {
            commit_id,
            commit_reference: &commit_reference,
            binding,
            envelope: envelope.as_ref(),
            sequence,
        })?;
        match sequence {
            PublicationSequence::Truth => self.runtime.history.publish_versioned_artifact(
                commit_id,
                commit_reference,
                validated.branch_id,
                validated.next_cell,
                patch_position,
                envelope,
            ),
            PublicationSequence::Metadata => self.runtime.history.publish_metadata_artifact(
                commit_id,
                commit_reference,
                validated.branch_id,
                validated.next_cell,
                patch_position,
                envelope,
            ),
        }
    }
}

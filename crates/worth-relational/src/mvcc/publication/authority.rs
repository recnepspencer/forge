use std::sync::Arc;

use crate::branch::{AdmittedRelationalBranchBasis, SelectedRelationalBranchState};
use crate::history::data::{CanonicalCommitEnvelope, CommitId, RelationalCommitReceipt};
use crate::history::RelationalCommitArtifact;
use crate::publication::patch::data::PatchStreamPosition;
use crate::runtime::RelationalRuntime;

use super::validation::{
    validate_publication, PublicationRequest, PublicationSequence, ValidatedPublication,
};

pub(crate) struct RelationalPublicationAuthority<'runtime> {
    runtime: &'runtime mut RelationalRuntime,
}

pub(crate) struct PreparedRelationalPublication {
    publication: crate::runtime::PreparedVersionedArtifactPublication,
}

#[derive(Clone)]
pub(crate) struct PreparedIndexRefreshBasis {
    branch_id: crate::history::data::BranchId,
    version_id: crate::identity::data::VersionId,
    root: Arc<crate::branch::RelationalBranchRoot>,
}

impl PreparedRelationalPublication {
    pub(crate) fn index_refresh_basis(&self) -> PreparedIndexRefreshBasis {
        PreparedIndexRefreshBasis {
            branch_id: self.publication.branch_id.clone(),
            version_id: self.publication.commit_reference.version_id,
            root: Arc::clone(&self.publication.root),
        }
    }

    pub(crate) fn install(self, runtime: &mut RelationalRuntime) {
        runtime
            .history
            .install_prepared_versioned_artifact(self.publication);
    }

    #[cfg(test)]
    pub(crate) fn materialization_counts(&self) -> (u64, u64) {
        let cost = self.publication.root.publication_cost();
        (cost.touched_regions, cost.reused_regions)
    }
}

impl PreparedIndexRefreshBasis {
    pub(crate) fn branch_id(&self) -> &crate::history::data::BranchId {
        &self.branch_id
    }

    pub(crate) const fn version_id(&self) -> crate::identity::data::VersionId {
        self.version_id
    }

    pub(crate) fn root(&self) -> &crate::branch::RelationalBranchRoot {
        &self.root
    }
}

impl RelationalRuntime {
    pub(crate) fn mvcc_publication_authority(&mut self) -> RelationalPublicationAuthority<'_> {
        RelationalPublicationAuthority { runtime: self }
    }
}

impl<'runtime> RelationalPublicationAuthority<'runtime> {
    #[cfg(test)]
    pub(crate) fn validate_versioned_publication(
        &self,
        commit_id: CommitId,
        commit_reference: &RelationalCommitReceipt,
        binding: &AdmittedRelationalBranchBasis,
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

    pub(crate) fn publish_commit(
        &mut self,
        commit_id: CommitId,
        commit_reference: RelationalCommitReceipt,
        binding: &AdmittedRelationalBranchBasis,
        published_partition_delta: crate::storage::RelationalPublishedPartitionDelta,
        patch_position: PatchStreamPosition,
        envelope: Arc<CanonicalCommitEnvelope>,
    ) -> Result<(), String> {
        let selected_branch_state = self
            .runtime
            .selected_branch_state(binding)
            .map_err(|error| error.detail())?;
        let prepared = self.prepare_commit_for_sequence(
            commit_id,
            commit_reference,
            binding,
            &selected_branch_state,
            published_partition_delta,
            patch_position,
            envelope,
            PublicationSequence::RecoveryTruth,
        )?;
        prepared.install(self.runtime);
        Ok(())
    }

    pub(crate) fn prepare_commit(
        &mut self,
        commit_id: CommitId,
        commit_reference: RelationalCommitReceipt,
        binding: &AdmittedRelationalBranchBasis,
        selected_branch_state: &SelectedRelationalBranchState,
        published_partition_delta: crate::storage::RelationalPublishedPartitionDelta,
        patch_position: PatchStreamPosition,
        envelope: Arc<CanonicalCommitEnvelope>,
    ) -> Result<PreparedRelationalPublication, String> {
        self.prepare_commit_for_sequence(
            commit_id,
            commit_reference,
            binding,
            selected_branch_state,
            published_partition_delta,
            patch_position,
            envelope,
            PublicationSequence::Truth,
        )
    }

    fn prepare_commit_for_sequence(
        &mut self,
        commit_id: CommitId,
        commit_reference: RelationalCommitReceipt,
        binding: &AdmittedRelationalBranchBasis,
        selected_branch_state: &SelectedRelationalBranchState,
        published_partition_delta: crate::storage::RelationalPublishedPartitionDelta,
        patch_position: PatchStreamPosition,
        envelope: Arc<CanonicalCommitEnvelope>,
        sequence: PublicationSequence,
    ) -> Result<PreparedRelationalPublication, String> {
        let mut validated = self.validate(PublicationRequest {
            commit_id,
            commit_reference: &commit_reference,
            binding,
            envelope: envelope.as_ref(),
            sequence,
        })?;
        let previous_root = selected_branch_state.root().cloned();
        let prepared_root = self
            .runtime
            .history
            .prepare_branch_root_capture(
                selected_branch_state.state(),
                &published_partition_delta,
                previous_root.as_ref(),
                Arc::clone(&envelope),
                &self.runtime.config.schema.registry,
                &self.runtime.services.symbols,
            )
            .map_err(|denial| format!("branch-root capture denied: {denial:?}"))?;
        let target = crate::branch::RelationalBranchTarget::from_commit_receipt(
            self.runtime.history.runtime_instance_id,
            &commit_reference,
            prepared_root
                .root()
                .descriptor()
                .cloned()
                .expect("prepared branch root has a descriptor"),
        );
        validated
            .next_cell
            .replace_truth_target(worth_foundational::FoundationalBranchTarget::basis(target));
        let (root, next_root_issuer) = prepared_root.into_parts();
        let artifact = RelationalCommitArtifact::from_envelope_with_root(
            Arc::clone(&envelope),
            Arc::clone(&root),
        )
        .map_err(|denial| format!("prepared catalog artifact denied: {denial:?}"))?;
        let new_authoritative_bytes = root
            .publication_cost()
            .new_authoritative_bytes
            .saturating_add(
                artifact
                    .authoritative_allocation_observations()
                    .iter()
                    .map(|allocation| allocation.authoritative_bytes)
                    .sum(),
            );
        Ok(PreparedRelationalPublication {
            publication: crate::runtime::PreparedVersionedArtifactPublication {
                commit_id,
                commit_reference,
                branch_id: validated.branch_id,
                next_cell: validated.next_cell,
                patch_position,
                envelope,
                root,
                artifact,
                new_authoritative_bytes,
                next_root_issuer,
                recovery_readmission: matches!(sequence, PublicationSequence::RecoveryTruth),
            },
        })
    }

    fn validate(&self, request: PublicationRequest<'_>) -> Result<ValidatedPublication, String> {
        validate_publication(self.runtime, request)
    }
}

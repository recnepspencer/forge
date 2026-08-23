use crate::branch::{RelationalBranchReferenceObservation, RelationalBranchVersion};
use crate::history::data::CanonicalCommitEnvelope;
use crate::history::data::{BranchId, CommitId, CommittedVersionSummary, RelationalCommitReceipt};
use crate::publication::patch::data::PatchStreamPosition;

use super::HistoryAccess;

impl<'runtime> HistoryAccess<'runtime> {
    /// Read the canonical head carried by one owner-admitted repeatable
    /// observation. No live branch cell or catalog-latest lookup participates.
    pub fn branch_head_for_observation<'observation>(
        &self,
        observation: &'observation crate::mvcc::RelationalBranchObservation,
    ) -> Result<
        Option<&'observation RelationalCommitReceipt>,
        crate::branch::RelationalBranchBasisDenial,
    > {
        if observation.identity().runtime_instance_id() != self.runtime.runtime_instance_id() {
            return Err(crate::branch::RelationalBranchBasisDenial::ForeignRuntime {
                expected_runtime_instance_id: self.runtime.runtime_instance_id(),
                actual_runtime_instance_id: observation.identity().runtime_instance_id(),
            });
        }
        Ok(observation
            .selected_root()
            .canonical_envelope()
            .map(|envelope| &envelope.commit))
    }

    /// Diagnostic immutable-identity lookup. This returns commit identity only
    /// and cannot select a branch head or a visible read root.
    pub fn immutable_commit_identity(
        &self,
        commit_id: CommitId,
    ) -> Option<crate::history::RelationalCommitIdentity> {
        self.runtime
            .history
            .commit_catalog
            .get(commit_id)
            .map(|artifact| artifact.identity().clone())
    }

    /// Return the immutable catalog receipt, including ordered parentage, for
    /// evidence consumers that need to compare the canonical artifact without
    /// receiving a mutable history authority.
    pub fn immutable_commit_receipt(&self, commit_id: CommitId) -> Option<RelationalCommitReceipt> {
        self.runtime
            .history
            .commit_catalog
            .get(commit_id)
            .map(|artifact| artifact.envelope().commit.clone())
    }

    pub fn latest_catalog_commit_identity(
        &self,
    ) -> Option<crate::history::RelationalCommitIdentity> {
        self.runtime
            .history
            .commit_catalog
            .latest_identity()
            .cloned()
    }

    pub fn immutable_commit_count(&self) -> usize {
        self.runtime.history.commit_catalog.len()
    }

    /// Transitional immutable-history read for existing Query consumers.
    /// This returns catalog evidence only; it cannot select a branch cell or
    /// mint currentness authority.
    pub fn historical_latest_commit(&self) -> Option<&RelationalCommitReceipt> {
        self.runtime
            .history
            .commit_catalog
            .latest_artifact()
            .map(|artifact| &artifact.envelope().commit)
    }

    /// Transitional immutable version lookup for existing Query consumers.
    pub fn historical_committed_version(
        &self,
        version_id: crate::identity::data::VersionId,
    ) -> Option<CommittedVersionSummary> {
        self.commit_envelope_for_version(version_id)
            .map(|envelope| {
                CommittedVersionSummary::new(
                    envelope.commit.clone(),
                    envelope.patch.authoritative_record_patches.len(),
                )
            })
    }

    pub(crate) fn commit_envelope(&self, commit_id: CommitId) -> Option<&CanonicalCommitEnvelope> {
        self.runtime
            .history
            .commit_catalog
            .get(commit_id)
            .map(|artifact| artifact.envelope().as_ref())
    }

    pub(crate) fn commit_envelope_for_version(
        &self,
        version_id: crate::identity::data::VersionId,
    ) -> Option<&CanonicalCommitEnvelope> {
        self.runtime
            .history
            .commit_catalog
            .find_by_version(version_id)
            .map(|artifact| artifact.envelope().as_ref())
    }

    pub(crate) fn latest_patch_stream_position(&self) -> Option<PatchStreamPosition> {
        self.runtime
            .history
            .patch_stream_index
            .last_key_value()
            .map(|(position, _)| *position)
    }

    pub(crate) fn contains_patch_stream_position(&self, position: PatchStreamPosition) -> bool {
        self.runtime
            .history
            .patch_stream_index
            .contains_key(&position)
    }

    pub(crate) fn commit_envelopes_snapshot(&self) -> Vec<CanonicalCommitEnvelope> {
        self.runtime
            .history
            .commit_catalog
            .snapshot()
            .into_iter()
            .map(|artifact| artifact.envelope().as_ref().clone())
            .collect()
    }

    pub(crate) fn branch_cells_snapshot(
        &self,
    ) -> Vec<crate::branch::RelationalBranchCellCheckpoint> {
        self.runtime.history.branch_cells_snapshot()
    }

    pub(crate) fn next_commit_id(&self) -> CommitId {
        self.runtime.history.preview_next_commit_id()
    }

    pub(crate) fn preview_next_version_id(&self) -> crate::identity::data::VersionId {
        self.runtime.history.preview_next_version_id()
    }

    pub(crate) fn commit_count(&self) -> usize {
        self.runtime.history.commit_catalog.len()
    }

    pub(crate) fn branch_head(&self, branch_id: &BranchId) -> Option<&RelationalCommitReceipt> {
        let cell = self.runtime.history.branch_cell(branch_id)?;
        let commit_id = match cell.observation().target() {
            worth_foundational::FoundationalBranchTarget::Empty => return None,
            worth_foundational::FoundationalBranchTarget::Basis(target) => {
                CommitId(target.commit_id())
            }
        };
        self.runtime
            .history
            .commit_catalog
            .get(commit_id)
            .map(|artifact| &artifact.envelope().commit)
    }

    pub(crate) fn latest_commit(&self) -> Option<&RelationalCommitReceipt> {
        self.historical_latest_commit()
    }

    pub(crate) fn committed_version(
        &self,
        version_id: crate::identity::data::VersionId,
    ) -> Option<CommittedVersionSummary> {
        self.historical_committed_version(version_id)
    }

    pub(crate) fn branch_reference_state(
        &self,
        branch_id: &BranchId,
    ) -> Option<(
        RelationalBranchReferenceObservation,
        RelationalBranchVersion,
    )> {
        self.runtime
            .history
            .branch_cell(branch_id)
            .map(|cell| (cell.observation().clone(), cell.truth_version()))
    }

    pub(crate) fn recent_commit_ids(
        &self,
        branch_id: Option<&BranchId>,
        limit: usize,
    ) -> Vec<CommitId> {
        match branch_id {
            Some(branch_id) => self
                .branch_commit_envelopes(branch_id)
                .into_iter()
                .rev()
                .take(limit)
                .map(|envelope| envelope.commit.commit_id)
                .collect(),
            None => self
                .runtime
                .history
                .commit_envelopes
                .values()
                .rev()
                .take(limit)
                .map(|envelope| envelope.commit.commit_id)
                .collect(),
        }
    }

    pub(crate) fn branch_head_versions(&self) -> Vec<crate::identity::data::VersionId> {
        self.runtime
            .history
            .branch_ids_snapshot()
            .into_iter()
            .filter_map(|branch_id| self.branch_head(&branch_id).map(|head| head.version_id))
            .collect()
    }

    pub(super) fn branch_commit_envelopes(
        &self,
        branch_id: &BranchId,
    ) -> Vec<&CanonicalCommitEnvelope> {
        let Some(head) = self.branch_head(branch_id) else {
            return Vec::new();
        };
        let branch_commits = self.ancestor_set(head.commit_id);
        let mut envelopes = self
            .runtime
            .history
            .commit_envelopes
            .values()
            .filter(|envelope| {
                branch_commits.contains(&envelope.commit.commit_id)
                    && envelope.commit.branch_id == *branch_id
            })
            .map(|envelope| envelope.as_ref())
            .collect::<Vec<_>>();
        envelopes
            .sort_by_key(|envelope| (envelope.commit.version_id.0, envelope.commit.commit_id.0));
        envelopes
    }
}

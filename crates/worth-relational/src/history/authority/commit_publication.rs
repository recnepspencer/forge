use std::sync::Arc;

use crate::history::data::CanonicalCommitEnvelope;
use crate::history::data::{BranchId, CommitId, RelationalCommitReceipt, VersionNode};
use crate::publication::patch::data::PatchStreamPosition;
use worth_foundational::FoundationalBranchTarget;

use crate::branch::RelationalBranchTarget;

use super::HistoryAuthority;

impl<'runtime> HistoryAuthority<'runtime> {
    pub(crate) fn validate_versioned_publication(
        &self,
        commit_id: CommitId,
        commit_reference: &RelationalCommitReceipt,
        branch_id: &BranchId,
        canonical_commit_envelope: &CanonicalCommitEnvelope,
    ) -> Result<(), String> {
        self.validate_publication(
            commit_id,
            commit_reference,
            branch_id,
            canonical_commit_envelope,
            PublicationSequence::Truth,
        )
    }

    pub(crate) fn publish_commit(
        &mut self,
        commit_id: CommitId,
        commit_reference: RelationalCommitReceipt,
        branch_id: BranchId,
        patch_position: PatchStreamPosition,
        canonical_commit_envelope: Arc<CanonicalCommitEnvelope>,
    ) -> Result<(), String> {
        self.publish_artifact(
            commit_id,
            commit_reference,
            branch_id,
            patch_position,
            canonical_commit_envelope,
            PublicationSequence::Truth,
        )
    }

    pub(crate) fn publish_metadata_artifact(
        &mut self,
        commit_id: CommitId,
        commit_reference: RelationalCommitReceipt,
        branch_id: BranchId,
        patch_position: PatchStreamPosition,
        canonical_commit_envelope: Arc<CanonicalCommitEnvelope>,
    ) -> Result<(), String> {
        self.publish_artifact(
            commit_id,
            commit_reference,
            branch_id,
            patch_position,
            canonical_commit_envelope,
            PublicationSequence::Metadata,
        )
    }

    fn validate_publication(
        &self,
        commit_id: CommitId,
        commit_reference: &RelationalCommitReceipt,
        branch_id: &BranchId,
        canonical_commit_envelope: &CanonicalCommitEnvelope,
        sequence: PublicationSequence,
    ) -> Result<(), String> {
        if commit_reference.commit_id != commit_id {
            return Err(format!(
                "publication commit identity mismatch: expected {}, got {}",
                commit_id.0, commit_reference.commit_id.0
            ));
        }
        if matches!(sequence, PublicationSequence::Truth)
            && self
                .runtime
                .history
                .next_version_id
                .checked_add(1)
                .is_none()
        {
            return Err("version id sequence overflow".to_owned());
        }
        if self.runtime.history.next_commit_id.checked_add(1).is_none() {
            return Err("commit id sequence overflow".to_owned());
        }
        self.runtime
            .history
            .commit_catalog
            .validate_envelope(canonical_commit_envelope)
            .map_err(|denial| format!("publication catalog admission denied: {denial:?}"))?;
        let mut cell = self
            .runtime
            .history
            .branch_cell(branch_id)
            .cloned()
            .ok_or_else(|| {
                format!(
                    "publication cannot mint a missing branch cell `{}`",
                    branch_id.0
                )
            })?;
        match sequence {
            PublicationSequence::Truth => {
                let roots = RelationalBranchTarget::roots_for_commit(commit_reference);
                let target = RelationalBranchTarget::from_commit_receipt(
                    self.runtime.history.runtime_instance_id,
                    commit_reference,
                    roots,
                );
                cell.advance_truth(FoundationalBranchTarget::basis(target))
                    .map_err(|denial| format!("publication branch advance denied: {denial:?}"))?;
            }
            PublicationSequence::Metadata => cell
                .advance_metadata()
                .map_err(|denial| format!("publication metadata advance denied: {denial:?}"))?,
        }
        Ok(())
    }

    fn publish_artifact(
        &mut self,
        commit_id: CommitId,
        commit_reference: RelationalCommitReceipt,
        branch_id: BranchId,
        patch_position: PatchStreamPosition,
        canonical_commit_envelope: Arc<CanonicalCommitEnvelope>,
        sequence: PublicationSequence,
    ) -> Result<(), String> {
        self.validate_publication(
            commit_id,
            &commit_reference,
            &branch_id,
            canonical_commit_envelope.as_ref(),
            sequence,
        )?;
        match sequence {
            PublicationSequence::Truth => self
                .runtime
                .history
                .advance_commit_sequence()
                .map_err(str::to_owned)?,
            PublicationSequence::Metadata => self
                .runtime
                .history
                .advance_metadata_commit_sequence()
                .map_err(str::to_owned)?,
        }
        insert_published_commit(
            &mut self.runtime.history,
            commit_id,
            commit_reference,
            branch_id,
            patch_position,
            canonical_commit_envelope,
            sequence,
        )
    }

    #[cfg(test)]
    pub(crate) fn remove_commit_envelope_for_test(
        &mut self,
        commit_id: crate::history::data::CommitId,
    ) -> bool {
        let Some(envelope) = self.runtime.history.commit_envelopes.remove(&commit_id) else {
            return false;
        };
        self.runtime
            .history
            .patch_stream_index
            .remove(&envelope.patch.position);
        self.runtime.history.replace_catalog_from_legacy_for_test();
        true
    }

    #[cfg(test)]
    pub(crate) fn remove_commit_envelope_preserving_patch_stream_position_for_test(
        &mut self,
        commit_id: crate::history::data::CommitId,
    ) -> bool {
        let removed = self.runtime.history.commit_envelopes.remove(&commit_id);
        self.runtime.history.replace_catalog_from_legacy_for_test();
        removed.is_some()
    }

    #[cfg(test)]
    pub(crate) fn tamper_commit_patch_for_test(
        &mut self,
        commit_id: crate::history::data::CommitId,
        mutate: impl FnOnce(&mut crate::publication::patch::data::PublishedAuthoritativePatchEnvelope),
    ) -> bool {
        let Some(envelope) = self.runtime.history.commit_envelopes.get(&commit_id) else {
            return false;
        };
        let mut replacement = envelope.as_ref().clone();
        mutate(&mut replacement.patch);
        self.runtime
            .history
            .commit_envelopes
            .insert(commit_id, Arc::new(replacement));
        self.runtime.history.replace_catalog_from_legacy_for_test();
        true
    }

    #[cfg(test)]
    pub(crate) fn tamper_commit_envelope_for_test(
        &mut self,
        commit_id: crate::history::data::CommitId,
        mutate: impl FnOnce(&mut CanonicalCommitEnvelope),
    ) -> bool {
        let Some(envelope) = self.runtime.history.commit_envelopes.get(&commit_id) else {
            return false;
        };
        let mut replacement = envelope.as_ref().clone();
        if replacement.derived_index_artifacts().is_empty() {
            let generations = self
                .runtime
                .index_access()
                .generations_snapshot()
                .into_iter()
                .filter(|generation| generation.source_commit_id == commit_id)
                .collect::<Vec<_>>();
            replacement.derived_index_artifacts =
                crate::indexes::data::DerivedIndexArtifacts::new(generations);
        }
        mutate(&mut replacement);
        self.runtime
            .history
            .commit_envelopes
            .insert(commit_id, Arc::new(replacement));
        self.runtime.history.replace_catalog_from_legacy_for_test();
        true
    }
}

#[derive(Debug, Clone, Copy)]
enum PublicationSequence {
    Truth,
    Metadata,
}

fn insert_published_commit(
    history: &mut crate::runtime::HistorySubsystem,
    commit_id: CommitId,
    commit_reference: RelationalCommitReceipt,
    branch_id: BranchId,
    patch_position: PatchStreamPosition,
    canonical_commit_envelope: Arc<CanonicalCommitEnvelope>,
    sequence: PublicationSequence,
) -> Result<(), String> {
    let roots = RelationalBranchTarget::roots_for_commit(&commit_reference);
    if let Some(existing) = history.commit_catalog.get(commit_id) {
        if existing.envelope().as_ref() != canonical_commit_envelope.as_ref() {
            return Err(format!(
                "commit id {} cannot name two immutable catalog artifacts",
                commit_id.0
            ));
        }
    } else {
        history
            .commit_catalog
            .append_envelope(Arc::clone(&canonical_commit_envelope))
            .map_err(|denial| format!("published catalog admission denied: {denial:?}"))?;
    }
    if !history.has_branch(&branch_id) {
        return Err(format!(
            "publication cannot mint a missing branch cell `{}`",
            branch_id.0
        ));
    }
    match sequence {
        PublicationSequence::Truth => {
            let target = RelationalBranchTarget::from_commit_receipt(
                history.runtime_instance_id,
                &commit_reference,
                roots,
            );
            history
                .branch_cell_mut(&branch_id)
                .ok_or_else(|| format!("published branch `{}` is missing", branch_id.0))?
                .advance_truth(FoundationalBranchTarget::basis(target))
                .map_err(|denial| format!("published branch reference denied: {denial:?}"))?;
        }
        PublicationSequence::Metadata => {
            history
                .branch_cell_mut(&branch_id)
                .ok_or_else(|| format!("published branch `{}` is missing", branch_id.0))?
                .advance_metadata()
                .map_err(|denial| format!("metadata publication denied: {denial:?}"))?;
        }
    }
    history.commit_graph.insert(
        commit_id,
        VersionNode {
            commit: commit_reference,
        },
    );
    history
        .commit_envelopes
        .insert(commit_id, canonical_commit_envelope);
    history.patch_stream_index.insert(patch_position, commit_id);
    Ok(())
}

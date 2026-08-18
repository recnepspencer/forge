use std::sync::Arc;

use crate::branch::{RelationalBranchCellCheckpoint, RelationalBranchReferenceCell};
use crate::history::data::CanonicalCommitEnvelope;
use crate::history::data::{BranchId, CommitId};
use crate::history::RelationalCommitCatalog;
use worth_foundational::FoundationalBranchTarget;

use super::HistorySubsystem;

impl HistorySubsystem {
    pub(crate) fn restore_branch_cells(
        &mut self,
        checkpoints: &[RelationalBranchCellCheckpoint],
    ) -> Result<(), String> {
        if checkpoints.is_empty() {
            return Err("durable checkpoint omitted exact branch-cell state".to_owned());
        }
        let mut cells = std::collections::BTreeMap::new();
        for checkpoint in checkpoints.iter().cloned() {
            let cell = RelationalBranchReferenceCell::from_checkpoint(
                self.runtime_instance_id,
                checkpoint,
            )
            .map_err(|denial| format!("invalid durable branch-cell state: {denial:?}"))?;
            let branch_id = cell.identity().branch_id().clone();
            if cells.insert(branch_id.clone(), cell).is_some() {
                return Err(format!(
                    "duplicate durable branch-cell state for `{}`",
                    branch_id.0
                ));
            }
        }
        if !cells.contains_key(&self.main_branch) {
            return Err("durable checkpoint omitted the configured main branch cell".to_owned());
        }
        for (branch_id, cell) in &cells {
            validate_branch_target_artifact(
                &self.commit_catalog,
                branch_id,
                cell.observation().target(),
            )?;
            if let Some(source_branch_id) = cell.fork_source_branch_id() {
                let source_cell = cells.get(source_branch_id).ok_or_else(|| {
                    format!(
                        "branch cell `{}` names missing fork source `{}`",
                        branch_id.0, source_branch_id.0
                    )
                })?;
                let Some(provenance) = cell.fork_provenance() else {
                    return Err(format!(
                        "branch cell `{}` names source `{}` without fork provenance",
                        branch_id.0, source_branch_id.0
                    ));
                };
                // Provenance is the exact source observation captured at fork
                // time.  The source may legitimately advance after the fork;
                // requiring its target to remain current would reject every
                // valid post-fork source publication during recovery.  The
                // source identity and generation bound still prove that the
                // carried observation belongs to this live source lineage.
                if provenance.branch_id() != source_cell.observation().branch_id()
                    || provenance.generation().get() > source_cell.observation().generation().get()
                {
                    return Err(format!(
                        "branch cell `{}` fork provenance disagrees with source `{}`",
                        branch_id.0, source_branch_id.0
                    ));
                }
                validate_branch_target_artifact(
                    &self.commit_catalog,
                    branch_id,
                    provenance.target(),
                )?;
            } else if cell.fork_provenance().is_some() {
                return Err(format!(
                    "branch cell `{}` carries provenance without a fork source",
                    branch_id.0
                ));
            }
        }
        self.branch_cells.restore_all(cells);
        Ok(())
    }

    pub(crate) fn rebuild_phase4_registry(&mut self) {
        self.commit_catalog = RelationalCommitCatalog::default();
        let envelopes = self
            .commit_envelopes
            .values()
            .cloned()
            .collect::<Vec<Arc<CanonicalCommitEnvelope>>>();
        for envelope in envelopes {
            let _ = self
                .commit_catalog
                .append_envelope(envelope)
                .expect("durable commit parentage must be ordered and unique");
        }
    }

    pub(crate) fn record_recovered_commit(
        &mut self,
        envelope: &CanonicalCommitEnvelope,
        allow_reconstructed_replacement: bool,
    ) -> Result<(), String> {
        let catalog_artifact = self.commit_catalog.get(envelope.commit.commit_id);
        if catalog_artifact.is_some_and(|artifact| artifact.envelope().as_ref() != envelope) {
            if !allow_reconstructed_replacement {
                return Err(format!(
                    "recovery commit artifact conflicts for commit {}",
                    envelope.commit.commit_id.0
                ));
            }
            let mut catalog = RelationalCommitCatalog::default();
            for candidate in self.commit_envelopes.values().cloned() {
                catalog
                    .append_envelope(candidate)
                    .map_err(|denial| format!("recovery catalog replacement denied: {denial:?}"))?;
            }
            self.commit_catalog = catalog;
        } else if catalog_artifact.is_none() {
            self.commit_catalog
                .append_envelope(Arc::new(envelope.clone()))
                .map_err(|denial| {
                    format!(
                        "recovery commit artifact could not be admitted for commit {}: {denial:?}",
                        envelope.commit.commit_id.0
                    )
                })?;
        }
        self.require_recovered_branch(&envelope.branch_context)
            .map_err(|detail| detail.to_owned())?;
        if is_metadata_only_envelope(envelope) {
            let metadata_already_applied = envelope
                .branch_cell_checkpoint
                .as_ref()
                .and_then(|checkpoint| {
                    let checkpoint = RelationalBranchReferenceCell::from_checkpoint(
                        self.runtime_instance_id,
                        checkpoint.clone(),
                    )
                    .ok()?
                    .checkpoint();
                    let cell = self.branch_cell(&envelope.branch_context)?;
                    let expected_generation =
                        checkpoint.observation.generation().checked_advance().ok()?;
                    (cell.observation().branch_id() == checkpoint.observation.branch_id()
                        && cell.observation().target() == checkpoint.observation.target()
                        && cell.observation().generation() == expected_generation
                        && cell.truth_version() == checkpoint.truth_version
                        && cell.fork_provenance() == checkpoint.fork_provenance.as_ref()
                        && cell.fork_source_branch_id()
                            == checkpoint.fork_source_branch_id.as_ref())
                    .then_some(())
                })
                .is_some();
            if !metadata_already_applied {
                self.branch_cell_mut(&envelope.branch_context)
                    .ok_or_else(|| {
                        format!(
                            "recovered branch cell missing for `{}`",
                            envelope.branch_context.0
                        )
                    })?
                    .advance_metadata()
                    .map_err(|denial| format!("recovered metadata reference denied: {denial:?}"))?;
            }
            return Ok(());
        }
        let roots = self
            .commit_catalog
            .get(envelope.commit.commit_id)
            .map(|artifact| artifact.roots().clone())
            .ok_or_else(|| "recovered commit must have a catalog artifact".to_owned())?;
        let target = crate::branch::RelationalBranchTarget::from_commit_receipt(
            self.runtime_instance_id,
            &envelope.commit,
            roots,
        );
        let already_points_at_commit = self
            .branch_cell(&envelope.branch_context)
            .and_then(|cell| match cell.observation().target() {
                FoundationalBranchTarget::Basis(current)
                    if current.commit_id() == envelope.commit.commit_id.0
                        && current.version_id() == envelope.commit.version_id.0 =>
                {
                    Some(())
                }
                _ => None,
            })
            .is_some();
        if !already_points_at_commit {
            self.branch_cell_mut(&envelope.branch_context)
                .ok_or_else(|| {
                    format!(
                        "recovered branch cell missing for `{}`",
                        envelope.branch_context.0
                    )
                })?
                .advance_truth(FoundationalBranchTarget::basis(target))
                .map_err(|denial| format!("recovered branch reference denied: {denial:?}"))?;
        }
        Ok(())
    }

    #[cfg(test)]
    pub(crate) fn replace_catalog_from_legacy_for_test(&mut self) {
        let mut catalog = RelationalCommitCatalog::default();
        for envelope in self.commit_envelopes.values().cloned() {
            catalog
                .append_envelope(envelope)
                .expect("test envelopes retain ordered unique parentage");
        }
        self.commit_catalog = catalog;
    }
}

/// Replay may reacquire a retention lease while reconstructing a commit. That
/// lease is not branch currentness; the owner truth axes must still match the
/// carried checkpoint exactly before the envelope is admitted.
pub(super) fn branch_cell_truth_matches(
    existing: &RelationalBranchCellCheckpoint,
    incoming: &RelationalBranchCellCheckpoint,
) -> bool {
    existing.runtime_instance_id == incoming.runtime_instance_id
        && existing.branch_id == incoming.branch_id
        && existing.observation == incoming.observation
        && existing.truth_version == incoming.truth_version
        && existing.fork_provenance == incoming.fork_provenance
        && existing.fork_source_branch_id == incoming.fork_source_branch_id
}

/// Validate a branch-cell checkpoint admitted while replay is extending an
/// already restored checkpoint. Tail admission must use the same artifact and
/// fork-provenance court as the complete checkpoint restore; structural
/// deserialization alone is not an operational admission.
pub(super) fn validate_tail_branch_cell(
    history: &HistorySubsystem,
    cell: &RelationalBranchReferenceCell,
) -> Result<(), String> {
    let branch_id = cell.identity().branch_id();
    validate_branch_target_artifact(
        &history.commit_catalog,
        branch_id,
        cell.observation().target(),
    )?;
    match (cell.fork_source_branch_id(), cell.fork_provenance()) {
        (Some(source_branch_id), Some(provenance)) => {
            let source_cell = history.branch_cell(source_branch_id).ok_or_else(|| {
                format!(
                    "branch cell `{}` names missing fork source `{}`",
                    branch_id.0, source_branch_id.0
                )
            })?;
            // See the complete-checkpoint validator above: source currentness
            // is allowed to move after the fork, so provenance is compared to
            // the source identity/generation only, then its own target is
            // validated against the immutable commit catalog.
            if provenance.branch_id() != source_cell.observation().branch_id()
                || provenance.generation().get() > source_cell.observation().generation().get()
            {
                return Err(format!(
                    "branch cell `{}` fork provenance disagrees with source `{}`",
                    branch_id.0, source_branch_id.0
                ));
            }
            validate_branch_target_artifact(
                &history.commit_catalog,
                branch_id,
                provenance.target(),
            )?;
            Ok(())
        }
        (None, None) => Ok(()),
        (Some(source_branch_id), None) => Err(format!(
            "branch cell `{}` names source `{}` without fork provenance",
            branch_id.0, source_branch_id.0
        )),
        (None, Some(_)) => Err(format!(
            "branch cell `{}` carries provenance without a fork source",
            branch_id.0
        )),
    }
}

fn validate_branch_target_artifact(
    catalog: &RelationalCommitCatalog,
    branch_id: &BranchId,
    target: &FoundationalBranchTarget<crate::branch::RelationalBranchTarget>,
) -> Result<(), String> {
    let FoundationalBranchTarget::Basis(target) = target else {
        return Ok(());
    };
    let commit_id = CommitId(target.commit_id());
    let artifact = catalog.get(commit_id).ok_or_else(|| {
        format!(
            "branch cell `{}` references missing commit artifact `{}`",
            branch_id.0, commit_id.0
        )
    })?;
    if artifact.version_id().0 != target.version_id()
        || artifact
            .parentage()
            .as_slice()
            .iter()
            .map(|parent| parent.0)
            .collect::<Vec<_>>()
            != target.parent_commit_ids()
        || artifact.roots() != target.roots()
    {
        return Err(format!(
            "branch cell `{}` target does not match immutable commit artifact `{}`",
            branch_id.0, commit_id.0
        ));
    }
    Ok(())
}

fn is_metadata_only_envelope(envelope: &CanonicalCommitEnvelope) -> bool {
    envelope.authority_kind
        == crate::history::data::CanonicalCommitAuthorityKind::MetadataOnlyLineage
}

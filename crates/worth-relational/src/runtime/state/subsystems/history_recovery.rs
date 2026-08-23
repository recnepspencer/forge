use std::sync::Arc;

use crate::branch::{RelationalBranchCellCheckpoint, RelationalBranchReferenceCell};
use crate::history::data::CanonicalCommitEnvelope;
use crate::history::data::{BranchId, CommitId};
use crate::history::RelationalCommitCatalog;
use worth_foundational::FoundationalBranchTarget;

use super::history_recovery_lineage::{
    validate_branch_target_lineage, validate_target_authoring_lineage,
};
use super::HistorySubsystem;

impl HistorySubsystem {
    pub(crate) fn restore_branch_cells(
        &mut self,
        checkpoints: &[RelationalBranchCellCheckpoint],
        root_partitions: &std::collections::BTreeMap<
            CommitId,
            std::collections::BTreeMap<
                crate::identity::data::PartitionId,
                crate::storage::overlay::PartitionState,
            >,
        >,
        root_schema_authorities: &std::collections::BTreeMap<
            CommitId,
            Arc<crate::branch::RelationalBranchRootSchemaAuthority>,
        >,
        symbols: &crate::symbols::data::StringInterner,
    ) -> Result<(), String> {
        if checkpoints.is_empty() {
            return Err("durable checkpoint omitted exact branch-cell state".to_owned());
        }
        let mut cells = std::collections::BTreeMap::new();
        let mut readmitted_roots: std::collections::BTreeMap<
            CommitId,
            Arc<crate::branch::RelationalBranchRoot>,
        > = std::collections::BTreeMap::new();
        for checkpoint in checkpoints.iter().cloned() {
            let root = match checkpoint.observation.target() {
                FoundationalBranchTarget::Empty => None,
                FoundationalBranchTarget::Basis(target) => {
                    let commit_id = CommitId(target.commit_id());
                    validate_branch_target_envelope(
                        &self.commit_envelopes,
                        &checkpoint.branch_id,
                        checkpoint.observation.target(),
                    )?;
                    if let Some(root) = readmitted_roots.get(&commit_id) {
                        Some(root.clone())
                    } else {
                        let partitions = root_partitions.get(&commit_id).ok_or_else(|| {
                            format!(
                                "durable branch cell references missing branch-root image `{}`",
                                commit_id.0
                            )
                        })?;
                        let envelope =
                            self.commit_envelopes
                                .get(&commit_id)
                                .cloned()
                                .ok_or_else(|| {
                                    format!(
                                    "durable branch cell references missing commit envelope `{}`",
                                    commit_id.0
                                )
                                })?;
                        let schema_authority = root_schema_authorities
                            .get(&commit_id)
                            .cloned()
                            .ok_or_else(|| {
                                format!(
                                    "durable branch cell references missing root schema authority `{}`",
                                    commit_id.0
                                )
                            })?;
                        let root = self
                            .readmit_branch_root(
                                partitions,
                                envelope,
                                target.roots().clone(),
                                schema_authority,
                                symbols,
                            )
                            .map_err(|denial| {
                                format!("durable branch root readmission denied: {denial:?}")
                            })?;
                        if root.descriptor() != Some(target.roots()) {
                            return Err(format!(
                                "durable branch-root image does not match target `{}`",
                                commit_id.0
                            ));
                        }
                        self.root_identity_issuer.observe_root(&root);
                        readmitted_roots.insert(commit_id, root.clone());
                        Some(root)
                    }
                }
            };
            let cell = RelationalBranchReferenceCell::from_checkpoint_with_root(
                self.runtime_instance_id,
                checkpoint,
                root,
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
        self.branch_cells.restore_all(cells);
        self.rebuild_catalog_with_checkpoint_targets(checkpoints, symbols)?;
        for (branch_id, cell) in self
            .branch_cells
            .values()
            .map(|cell| (cell.identity().branch_id().clone(), cell))
        {
            validate_branch_target_artifact(
                &self.commit_catalog,
                &branch_id,
                cell.observation().target(),
            )?;
            validate_branch_target_lineage(
                self,
                &branch_id,
                cell.observation().target(),
                cell.fork_source_branch_id(),
                cell.fork_provenance(),
            )?;
            if let Some(source_branch_id) = cell.fork_source_branch_id() {
                let source_cell = self.branch_cell(source_branch_id).ok_or_else(|| {
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
                    &branch_id,
                    provenance.target(),
                )?;
                validate_target_authoring_lineage(self, source_branch_id, provenance.target())?;
            } else if cell.fork_provenance().is_some() {
                return Err(format!(
                    "branch cell `{}` carries provenance without a fork source",
                    branch_id.0
                ));
            }
        }
        Ok(())
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

pub(super) fn replayed_branch_cell_accepts_canonical_target(
    replayed: &RelationalBranchCellCheckpoint,
    canonical: &RelationalBranchCellCheckpoint,
) -> bool {
    replayed.runtime_instance_id == canonical.runtime_instance_id
        && replayed.branch_id == canonical.branch_id
        && replayed.observation.branch_id() == canonical.observation.branch_id()
        && replayed.observation.generation() == canonical.observation.generation()
        && target_commit_shape_matches(
            replayed.observation.target(),
            canonical.observation.target(),
        )
        && replayed.truth_version == canonical.truth_version
        && replayed.fork_provenance == canonical.fork_provenance
        && replayed.fork_source_branch_id == canonical.fork_source_branch_id
}

fn target_commit_shape_matches(
    replayed: &FoundationalBranchTarget<crate::branch::RelationalBranchTarget>,
    canonical: &FoundationalBranchTarget<crate::branch::RelationalBranchTarget>,
) -> bool {
    match (replayed, canonical) {
        (FoundationalBranchTarget::Empty, FoundationalBranchTarget::Empty) => true,
        (FoundationalBranchTarget::Basis(replayed), FoundationalBranchTarget::Basis(canonical)) => {
            replayed.commit_id() == canonical.commit_id()
                && replayed.version_id() == canonical.version_id()
                && replayed.parent_commit_ids() == canonical.parent_commit_ids()
        }
        _ => false,
    }
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
    validate_branch_target_lineage(
        history,
        branch_id,
        cell.observation().target(),
        cell.fork_source_branch_id(),
        cell.fork_provenance(),
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
            validate_target_authoring_lineage(history, source_branch_id, provenance.target())?;
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

pub(super) fn validate_branch_target_artifact(
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
        let root_shape = artifact.linked_root().map(|root| {
            root.partition_ids()
                .into_iter()
                .filter_map(|partition_id| {
                    root.partition_state(partition_id).map(|partition| {
                        (
                            partition_id,
                            partition.entity_arena.generations.clone(),
                            partition.entity_arena.lifecycle.clone(),
                            partition.relation_arena.generations.clone(),
                            partition.relation_arena.lifecycle.clone(),
                        )
                    })
                })
                .collect::<Vec<_>>()
        });
        return Err(format!(
            "branch cell `{}` target does not match immutable commit artifact `{}`: target version/parents/roots = {}/{:?}/{:?}, artifact = {}/{:?}/{:?}, root shape = {:?}",
            branch_id.0,
            commit_id.0,
            target.version_id(),
            target.parent_commit_ids(),
            target.roots(),
            artifact.version_id().0,
            artifact.parentage().as_slice(),
            artifact.roots(),
            root_shape,
        ));
    }
    Ok(())
}

pub(super) fn require_branch_target_artifact(
    catalog: &RelationalCommitCatalog,
    branch_id: &BranchId,
    target: &FoundationalBranchTarget<crate::branch::RelationalBranchTarget>,
) -> Result<(), String> {
    let FoundationalBranchTarget::Basis(target) = target else {
        return Ok(());
    };
    let commit_id = CommitId(target.commit_id());
    if catalog.get(commit_id).is_none() {
        return Err(format!(
            "branch cell `{}` references missing commit artifact `{}`",
            branch_id.0, commit_id.0
        ));
    }
    Ok(())
}

fn validate_branch_target_envelope(
    envelopes: &std::collections::BTreeMap<CommitId, Arc<CanonicalCommitEnvelope>>,
    branch_id: &BranchId,
    target: &FoundationalBranchTarget<crate::branch::RelationalBranchTarget>,
) -> Result<(), String> {
    let FoundationalBranchTarget::Basis(target) = target else {
        return Ok(());
    };
    let commit_id = CommitId(target.commit_id());
    let envelope = envelopes.get(&commit_id).ok_or_else(|| {
        format!(
            "branch cell `{}` references missing commit artifact `{}`",
            branch_id.0, commit_id.0
        )
    })?;
    if envelope.commit.version_id.0 != target.version_id()
        || envelope
            .commit
            .parents
            .iter()
            .map(|parent| parent.0)
            .collect::<Vec<_>>()
            != target.parent_commit_ids()
    {
        return Err(format!(
            "branch cell `{}` target does not match commit envelope `{}`",
            branch_id.0, commit_id.0
        ));
    }
    Ok(())
}

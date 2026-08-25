use std::collections::BTreeMap;
use std::sync::atomic::Ordering;
use std::sync::Arc;

use worth_foundational::FoundationalBranchTarget;

use crate::branch::{
    RelationalBranchCellCheckpoint, RelationalBranchCellDenial, RelationalBranchReferenceCell,
    RelationalBranchRoot,
};
use crate::history::data::{BranchId, CommitId};

use super::{history_recovery_validation, HistorySubsystem};

impl HistorySubsystem {
    pub(crate) fn transition_empty_branches_to_initial_schema(
        &mut self,
        registry: &crate::schema::data::RelationalSchemaRegistry,
    ) -> Result<(), RelationalBranchCellDenial> {
        self.record_branch_population_scan();
        let existing = self.branch_cells.take_all();
        let transitioned = existing
            .iter()
            .map(|(branch_id, cell)| {
                let mut candidate = cell.clone();
                if matches!(
                    candidate.observation().target(),
                    FoundationalBranchTarget::Empty
                ) {
                    candidate.advance_metadata()?;
                    candidate.install_root(RelationalBranchRoot::empty_with_schema(
                        registry,
                        crate::schema::data::runtime_descriptor_semantics_policy()
                            .current_write_version(),
                    ));
                }
                Ok((branch_id.clone(), candidate))
            })
            .collect::<Result<BTreeMap<_, _>, RelationalBranchCellDenial>>();
        match transitioned {
            Ok(cells) => {
                self.branch_cells.restore_all(cells);
                Ok(())
            }
            Err(denial) => {
                self.branch_cells.restore_all(existing);
                Err(denial)
            }
        }
    }

    pub(crate) fn admit_recovered_branch_cell(
        &mut self,
        checkpoint: RelationalBranchCellCheckpoint,
        expected_branch_id: &BranchId,
        recovered_root: Option<Arc<crate::branch::RelationalBranchRoot>>,
        recovered_provenance_root: Option<Arc<crate::branch::RelationalBranchRoot>>,
        symbols: &crate::symbols::data::StringInterner,
    ) -> Result<(), String> {
        if let Some(FoundationalBranchTarget::Basis(target)) = checkpoint
            .fork_provenance
            .as_ref()
            .map(|provenance| provenance.target())
        {
            let commit_id = CommitId(target.selected_commit_id());
            self.readmit_replayed_root_descriptor(
                commit_id,
                target.roots().clone(),
                recovered_provenance_root.ok_or_else(|| {
                    format!("recovery provenance root `{}` is unavailable", commit_id.0)
                })?,
                symbols,
            )?;
        }
        let readmission_root = match checkpoint.observation.target() {
            FoundationalBranchTarget::Empty => None,
            FoundationalBranchTarget::Basis(target) => {
                let commit_id = CommitId(target.selected_commit_id());
                Some(
                    self.readmit_replayed_root_descriptor(
                        commit_id,
                        target.roots().clone(),
                        recovered_root.ok_or_else(|| {
                            format!("recovery root `{}` is unavailable", commit_id.0)
                        })?,
                        symbols,
                    )
                    .map_err(|detail| {
                        format!(
                            "tail branch cell references unavailable branch root `{}`: {detail}",
                            commit_id.0
                        )
                    })?,
                )
            }
        };
        history_recovery_validation::require_branch_target_artifact(
            &self.commit_catalog,
            expected_branch_id,
            checkpoint.observation.target(),
        )?;
        super::history_recovery_lineage::validate_branch_target_lineage(
            self,
            expected_branch_id,
            checkpoint.observation.target(),
            checkpoint.fork_source_branch_id.as_ref(),
            checkpoint.fork_provenance.as_ref(),
        )?;
        history_recovery_validation::validate_branch_target_artifact(
            &self.commit_catalog,
            expected_branch_id,
            checkpoint.observation.target(),
        )?;
        let cell = RelationalBranchReferenceCell::from_checkpoint_with_root(
            self.runtime_instance_id,
            checkpoint,
            readmission_root,
        )
        .map_err(|denial| format!("invalid durable branch-cell state: {denial:?}"))?;
        let branch_id = cell.identity().branch_id().clone();
        if &branch_id != expected_branch_id {
            return Err(format!(
                "recovery branch-cell `{}` does not match envelope branch `{}`",
                branch_id.0, expected_branch_id.0
            ));
        }
        history_recovery_validation::validate_recovered_branch_cell(self, &cell)?;
        if let Some(existing) = self.branch_cell(&branch_id) {
            if !history_recovery_validation::branch_cell_truth_matches(
                &existing.checkpoint(),
                &cell.checkpoint(),
            ) {
                return Err(format!(
                    "recovery branch-cell state conflicts for `{}`",
                    branch_id.0
                ));
            }
            return Ok(());
        }
        self.insert_branch_cell(cell);
        Ok(())
    }

    pub(crate) fn branch_cells_snapshot(&self) -> Vec<RelationalBranchCellCheckpoint> {
        self.record_branch_population_scan();
        self.branch_cells.checkpoints()
    }

    pub(crate) fn branch_root_checkpoints(
        &self,
    ) -> Result<Vec<crate::branch::RelationalBranchRootCheckpoint>, String> {
        self.record_branch_population_scan();
        let mut roots = BTreeMap::new();
        for cell in self.branch_cells.values() {
            let Some(root) = cell.root() else {
                continue;
            };
            let commit_id = root.commit_id().ok_or_else(|| {
                format!(
                    "committed branch `{}` has a root without commit identity",
                    cell.identity().branch_id().0
                )
            })?;
            if let Some(existing) = roots.get(&commit_id) {
                if !Arc::ptr_eq(existing, &root) {
                    return Err(format!(
                        "commit {} is represented by competing immutable roots",
                        commit_id.0
                    ));
                }
                continue;
            }
            roots.insert(commit_id, Arc::clone(&root));
        }
        Ok(roots
            .into_iter()
            .map(|(commit_id, root)| {
                let partitions = root
                    .partition_ids()
                    .into_iter()
                    .filter_map(|partition_id| root.partition_state(partition_id).cloned())
                    .collect();
                crate::branch::RelationalBranchRootCheckpoint::new(
                    commit_id,
                    partitions,
                    root.retained_schema_authority(),
                )
            })
            .collect())
    }

    pub(crate) fn branch_ids_snapshot(&self) -> Vec<BranchId> {
        self.record_branch_population_scan();
        let mut branch_ids = self.branch_cells.keys().cloned().collect::<Vec<_>>();
        branch_ids.sort();
        branch_ids
    }

    pub(super) fn record_branch_population_scan(&self) {
        self.branch_population_scans.fetch_add(1, Ordering::Relaxed);
    }
}

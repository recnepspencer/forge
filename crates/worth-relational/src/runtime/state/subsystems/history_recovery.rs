use std::sync::Arc;

use crate::branch::{RelationalBranchCellCheckpoint, RelationalBranchReferenceCell};
use crate::history::data::CommitId;
use worth_foundational::FoundationalBranchTarget;

use super::history_recovery_validation::{
    validate_branch_target_envelope, validate_recovered_branch_cell,
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
                    let commit_id = CommitId(target.selected_commit_id());
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
        for cell in self.branch_cells.values() {
            validate_recovered_branch_cell(self, cell)?;
        }
        Ok(())
    }
}

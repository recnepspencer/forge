use std::sync::Arc;

use worth_foundational::FoundationalBranchTarget;

use crate::history::data::CanonicalCommitEnvelope;
use crate::history::RelationalCommitCatalog;

use super::HistorySubsystem;

impl HistorySubsystem {
    pub(crate) fn install_recovered_canonical_inventory(
        &mut self,
        commits: Vec<Arc<crate::history::data::PositionedCanonicalCommit>>,
    ) -> Result<(), String> {
        self.rebuild_recovered_canonical_routes(commits.iter().cloned())
            .map_err(str::to_owned)?;
        self.commit_envelopes = commits
            .iter()
            .map(|commit| {
                (
                    commit.envelope().commit.commit_id,
                    Arc::clone(commit.canonical_arc()),
                )
            })
            .collect();
        self.patch_stream_index = commits
            .iter()
            .map(|commit| (commit.position(), commit.envelope().commit.commit_id))
            .collect();
        for commit in commits {
            self.commit_catalog
                .synchronize_recovered_envelope(Arc::clone(commit.canonical_arc()))
                .map_err(|denial| {
                    format!("recovered canonical catalog synchronization denied: {denial:?}")
                })?;
        }
        Ok(())
    }

    pub(crate) fn rebuild_catalog_from_durable_envelopes(&mut self) {
        self.commit_catalog = RelationalCommitCatalog::default();
        let envelopes = self
            .commit_envelopes
            .values()
            .cloned()
            .collect::<Vec<Arc<CanonicalCommitEnvelope>>>();
        for envelope in envelopes {
            self.commit_catalog
                .append_envelope(envelope)
                .expect("durable commit parentage must be ordered and unique");
        }
    }

    pub(super) fn rebuild_catalog_with_live_roots(
        &mut self,
        symbols: &crate::symbols::data::StringInterner,
    ) -> Result<(), String> {
        self.rebuild_catalog_with_live_roots_and_descriptors(
            &std::collections::BTreeMap::new(),
            &std::collections::BTreeMap::new(),
            symbols,
        )
    }

    pub(super) fn rebuild_catalog_with_checkpoint_targets(
        &mut self,
        checkpoints: &[crate::branch::RelationalBranchCellCheckpoint],
        symbols: &crate::symbols::data::StringInterner,
    ) -> Result<(), String> {
        let mut descriptors = std::collections::BTreeMap::new();
        for checkpoint in checkpoints {
            for target in std::iter::once(checkpoint.observation.target()).chain(
                checkpoint
                    .fork_provenance
                    .as_ref()
                    .map(|provenance| provenance.target()),
            ) {
                let FoundationalBranchTarget::Basis(target) = target else {
                    continue;
                };
                let commit_id = crate::history::data::CommitId(target.selected_commit_id());
                match descriptors.get(&commit_id) {
                    Some(existing) if existing != target.roots() => {
                        return Err(format!(
                            "checkpoint carries competing root descriptors for commit `{}`",
                            commit_id.0
                        ));
                    }
                    Some(_) => {}
                    None => {
                        descriptors.insert(commit_id, target.roots().clone());
                    }
                }
            }
        }
        self.rebuild_catalog_with_live_roots_and_descriptors(
            &descriptors,
            &std::collections::BTreeMap::new(),
            symbols,
        )
    }

    fn rebuild_catalog_with_live_roots_and_descriptors(
        &mut self,
        descriptors: &std::collections::BTreeMap<
            crate::history::data::CommitId,
            crate::branch::RelationalBranchRootDescriptor,
        >,
        additional_roots: &std::collections::BTreeMap<
            crate::history::data::CommitId,
            Arc<crate::branch::RelationalBranchRoot>,
        >,
        symbols: &crate::symbols::data::StringInterner,
    ) -> Result<(), String> {
        let mut roots = additional_roots.clone();
        let mut cells = self.branch_cells.take_all();
        for cell in cells.values_mut() {
            let Some(root) = cell.root() else {
                continue;
            };
            let commit_id = root
                .commit_id()
                .ok_or_else(|| "live recovery root has no commit identity".to_owned())?;
            let canonical = self
                .commit_envelopes
                .get(&commit_id)
                .cloned()
                .ok_or_else(|| format!("live recovery root `{}` has no envelope", commit_id.0))?;
            let canonical_root = if let Some(existing) = roots.get(&commit_id) {
                Arc::clone(existing)
            } else if root.links_envelope(&canonical) {
                root
            } else {
                root.relink_canonical_envelope(canonical.clone(), symbols)
                    .map_err(|denial| {
                        format!("recovery root canonical relink denied: {denial:?}")
                    })?
            };
            cell.install_root(Arc::clone(&canonical_root));
            roots.insert(commit_id, canonical_root);
        }
        self.branch_cells.restore_all(cells);
        let mut catalog = RelationalCommitCatalog::default();
        for envelope in self.commit_envelopes.values().cloned() {
            let result = match roots.get(&envelope.commit.commit_id) {
                Some(root) => catalog.append_envelope_with_root(envelope, Arc::clone(root)),
                None => match descriptors.get(&envelope.commit.commit_id) {
                    Some(descriptor) => {
                        catalog.append_envelope_with_descriptor(envelope, descriptor.clone())
                    }
                    None => catalog.append_envelope(envelope),
                },
            };
            result
                .map_err(|denial| format!("recovered catalog root linkage denied: {denial:?}"))?;
        }
        self.commit_catalog = catalog;
        Ok(())
    }

    pub(super) fn readmit_replayed_root_descriptor(
        &mut self,
        commit_id: crate::history::data::CommitId,
        descriptor: crate::branch::RelationalBranchRootDescriptor,
        replayed_root: Arc<crate::branch::RelationalBranchRoot>,
        symbols: &crate::symbols::data::StringInterner,
    ) -> Result<Arc<crate::branch::RelationalBranchRoot>, String> {
        if replayed_root.commit_id() != Some(commit_id) {
            return Err(format!(
                "recovery root `{}` was supplied for a different commit",
                commit_id.0
            ));
        }
        let canonical_root = if replayed_root.descriptor() == Some(&descriptor) {
            replayed_root
        } else {
            replayed_root
                .readmit_descriptor(descriptor, symbols)
                .map_err(|denial| format!("recovery root descriptor denied: {denial:?}"))?
        };
        let mut cells = self.branch_cells.take_all();
        for cell in cells.values_mut() {
            if cell.root().and_then(|root| root.commit_id()) == Some(commit_id) {
                cell.install_root(Arc::clone(&canonical_root));
            }
        }
        self.branch_cells.restore_all(cells);
        let mut recovered_roots = self.commit_catalog.linked_roots();
        recovered_roots.insert(commit_id, Arc::clone(&canonical_root));
        self.rebuild_catalog_with_live_roots_and_descriptors(
            &std::collections::BTreeMap::new(),
            &recovered_roots,
            symbols,
        )?;
        Ok(canonical_root)
    }

    pub(crate) fn record_recovered_commit(
        &mut self,
        envelope: &CanonicalCommitEnvelope,
        allow_reconstructed_replacement: bool,
        advance_branch_currentness: bool,
        symbols: &crate::symbols::data::StringInterner,
    ) -> Result<(), String> {
        let catalog_artifact = self.commit_catalog.get(envelope.commit.commit_id);
        if catalog_artifact.is_some_and(|artifact| artifact.envelope().as_ref() != envelope) {
            if !allow_reconstructed_replacement {
                return Err(format!(
                    "recovery commit artifact conflicts for commit {}",
                    envelope.commit.commit_id.0
                ));
            }
            self.rebuild_catalog_with_live_roots(symbols)?;
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
        self.require_recovered_branch(&envelope.branch_context)?;
        if !advance_branch_currentness {
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
        let already_current = self
            .branch_cell(&envelope.branch_context)
            .is_some_and(|cell| match cell.observation().target() {
                FoundationalBranchTarget::Basis(current) => {
                    current.selected_commit_id() == envelope.commit.commit_id.0
                        && current.version_id() == envelope.commit.version_id.0
                }
                FoundationalBranchTarget::Empty => false,
            });
        if !already_current {
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

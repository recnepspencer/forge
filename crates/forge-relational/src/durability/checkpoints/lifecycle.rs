use serde_json::json;

use crate::diagnostics::data::{
    DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope, RelationalDiagnosticsEntry,
};
use crate::durability::data::{
    CheckpointCoverage, DurabilityError, DurabilityMode, DurableCheckpoint, DurableCheckpointId,
    DurableCheckpointManifest, DurableIntegrityStatus,
};
use crate::history::data::BranchHead;
use crate::logic::runtime::RelationalRuntime;

use crate::durability::checkpoints::images::partition_to_image;
use crate::durability::log::local_store::{
    checkpoint_file_path, current_segment_ids, write_json, DurableCheckpointFile,
};

impl RelationalRuntime {
    pub(crate) fn compact_durable_log_if_needed(&mut self) {
        use crate::config::data::DurableLogRetentionMode;

        let policy = &self.config.durable_log_policy;
        if self.durability.log.len() <= policy.max_in_memory_envelopes {
            return;
        }

        match policy.retention_mode {
            DurableLogRetentionMode::RetainAllInMemory => {}
            DurableLogRetentionMode::CompactAfterCheckpoint => {
                if let Some(checkpoint) = self.durability.checkpoints.last() {
                    if let Some(commit) = checkpoint.coverage.up_to_commit.as_ref() {
                        self.durability
                            .log
                            .retain(|entry| entry.envelope.commit.commit_id > commit.commit_id);
                    }
                }
                if self.durability.log.len() > policy.max_in_memory_envelopes {
                    let overflow = self.durability.log.len() - policy.max_in_memory_envelopes;
                    self.durability.log.drain(0..overflow);
                }
            }
        }
        if self.config.durability_mode == DurabilityMode::PersistedSegmentedLocalFs
            && policy.compact_after_checkpoint
        {
            let _ = self.compact_store();
        }
    }

    pub fn checkpoint(&mut self) -> Result<DurableCheckpoint, DurabilityError> {
        let checkpoint = self.build_checkpoint_image();
        if self.config.durability_mode == DurabilityMode::PersistedSegmentedLocalFs {
            let manifest = self.persist_checkpoint_file(&checkpoint)?;
            self.push_bounded_diagnostic(
                DiagnosticsScope::History,
                DiagnosticsArtifactKind::MinimalSummary,
                vec![RelationalDiagnosticsEntry {
                    code: DiagnosticCode::CheckpointCreated,
                    message: "durable checkpoint created".to_string(),
                    fields: json!({
                        "checkpoint_id": manifest.checkpoint_id.0,
                        "up_to_commit": checkpoint.coverage.up_to_commit.as_ref().map(|commit| commit.commit_id.0),
                        "partition_count": manifest.partition_count,
                    }),
                }],
            );
        } else {
            self.push_bounded_diagnostic(
                DiagnosticsScope::History,
                DiagnosticsArtifactKind::MinimalSummary,
                vec![RelationalDiagnosticsEntry {
                    code: DiagnosticCode::CheckpointCreated,
                    message: "durable checkpoint created".to_string(),
                    fields: json!({
                        "up_to_commit": checkpoint.coverage.up_to_commit.as_ref().map(|commit| commit.commit_id.0),
                        "partition_count": checkpoint.partition_images.len(),
                    }),
                }],
            );
        }
        self.durability.checkpoints.push(checkpoint.clone());
        Ok(checkpoint)
    }

    pub fn durable_branch_heads(&self) -> Vec<BranchHead> {
        self.branches()
    }

    fn build_checkpoint_image(&self) -> DurableCheckpoint {
        DurableCheckpoint {
            coverage: CheckpointCoverage {
                up_to_commit: self.latest_commit().cloned(),
                up_to_version: self.latest_commit().map(|commit| commit.version_id),
            },
            branches: self.branches(),
            envelopes: self.history.commit_envelopes.values().cloned().collect(),
            partition_images: self
                .partitions
                .values()
                .cloned()
                .map(partition_to_image)
                .collect(),
            lineage_nodes: self.lineage.nodes.values().cloned().collect(),
            lineage_events: self.lineage.events.clone(),
            correspondence_candidates: self.lineage.correspondence_candidates.clone(),
            index_definitions: self.indexes.definitions.values().cloned().collect(),
            index_generations: self
                .indexes
                .generations
                .values()
                .flat_map(|generations| generations.iter().cloned())
                .collect(),
            symbol_table: self.symbols.snapshot(),
            runtime_name: self.config.runtime_name.clone(),
        }
    }

    fn persist_checkpoint_file(
        &mut self,
        checkpoint: &DurableCheckpoint,
    ) -> Result<DurableCheckpointManifest, DurabilityError> {
        let mut store = self.ensure_loaded_store()?;
        let checkpoint_id = DurableCheckpointId(
            store
                .checkpoints
                .last()
                .map(|manifest| manifest.checkpoint_id.0)
                .unwrap_or(0)
                + 1,
        );
        let path = checkpoint_file_path(&store.layout, checkpoint_id);
        write_json(
            &path,
            &DurableCheckpointFile {
                checkpoint: checkpoint.clone(),
            },
        )?;
        let manifest = DurableCheckpointManifest {
            checkpoint_id,
            path,
            coverage: checkpoint.coverage.clone(),
            partition_count: checkpoint.partition_images.len(),
            runtime_name: self.config.runtime_name.clone(),
            profile: self.config.profile,
            schema_version: self.primary_schema_version(),
            integrity: DurableIntegrityStatus::Verified,
        };
        store.checkpoints.push(manifest.clone());
        self.persist_store_manifest(&store)?;
        self.durability.store = Some(store);
        Ok(manifest)
    }
}

#[allow(dead_code)]
fn _keep_ids_for_clippy(store: Option<&crate::durability::data::DurableStore>) -> usize {
    current_segment_ids(store).len()
}

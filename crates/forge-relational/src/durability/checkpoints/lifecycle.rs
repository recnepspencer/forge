use serde_json::json;

use crate::capabilities::{RuntimeConfigSource, RuntimeIdentitySource, SchemaVersionSource};
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

        let policy = self.runtime_config().durability.log.clone();
        if self.durable_log_len() <= policy.max_in_memory_envelopes {
            return;
        }

        match policy.retention_mode {
            DurableLogRetentionMode::RetainAllInMemory => {}
            DurableLogRetentionMode::CompactAfterCheckpoint => {
                if let Some(checkpoint) = self.latest_durable_checkpoint() {
                    if let Some(commit) = checkpoint.coverage.up_to_commit.as_ref() {
                        self.retain_durable_log_newer_than(commit.commit_id);
                    }
                }
                if self.durable_log_len() > policy.max_in_memory_envelopes {
                    let overflow = self.durable_log_len() - policy.max_in_memory_envelopes;
                    self.drain_oldest_durable_log_entries(overflow);
                }
            }
        }
        if self.runtime_config().durability.mode == DurabilityMode::PersistedSegmentedLocalFs
            && self.runtime_config().durability.checkpoints.compact_after_checkpoint
        {
            let _ = self.compact_store();
        }
    }

    pub fn checkpoint(&mut self) -> Result<DurableCheckpoint, DurabilityError> {
        let checkpoint = self.build_checkpoint_image();
        if self.runtime_config().durability.mode == DurabilityMode::PersistedSegmentedLocalFs {
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
        self.push_durable_checkpoint(checkpoint.clone());
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
            envelopes: self.commit_envelopes_snapshot(),
            partition_images: self
                .partitions
                .values()
                .cloned()
                .map(partition_to_image)
                .collect(),
            lineage_nodes: self.lineage_nodes_snapshot(),
            lineage_events: self.lineage_events_snapshot(),
            correspondence_candidates: self.correspondence_candidates_snapshot(),
            index_definitions: self.index_definitions_snapshot(),
            index_generations: self.index_generations_snapshot(),
            symbol_table: self.symbol_table_snapshot(),
            runtime_name: self.runtime_name().to_string(),
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
            runtime_name: self.runtime_name().to_string(),
            profile: self.runtime_profile(),
            schema_version: self.primary_schema_version_id(),
            integrity: DurableIntegrityStatus::Verified,
        };
        store.checkpoints.push(manifest.clone());
        self.persist_store_manifest(&store)?;
        self.set_durable_store(Some(store));
        Ok(manifest)
    }
}

#[allow(dead_code)]
fn _keep_ids_for_clippy(store: Option<&crate::durability::data::DurableStore>) -> usize {
    current_segment_ids(store).len()
}

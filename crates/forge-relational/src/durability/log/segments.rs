use std::fs;

use serde_json::json;

use crate::capabilities::{DurabilityRead, RuntimeConfigSource, RuntimeIdentitySource};
use crate::diagnostics::data::{
    DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope, RelationalDiagnosticsEntry,
};
use crate::durability::data::{
    CompactionOutcome, CompactionPlan, DurabilityError, DurabilityMode, DurableCheckpointId,
    DurableIntegrityStatus, DurableSegmentId, DurableSegmentManifest,
};
use crate::logic::runtime::RelationalRuntime;
use crate::replay::data::CanonicalCommitEnvelope;

use crate::durability::log::local_store::{
    current_segment_ids, read_json, segment_file_path, write_json, DurableSegmentFile,
};

impl RelationalRuntime {
    pub fn compact_store(&mut self) -> Result<CompactionOutcome, DurabilityError> {
        if self.runtime_config().durability.mode != DurabilityMode::PersistedSegmentedLocalFs {
            return Ok(CompactionOutcome {
                removed_segments: Vec::new(),
                retained_segments: Vec::new(),
            });
        }
        let Some(checkpoint) = self.latest_durable_checkpoint() else {
            return Ok(CompactionOutcome {
                removed_segments: Vec::new(),
                retained_segments: current_segment_ids(self.durable_store()),
            });
        };
        let Some(up_to_commit) = checkpoint.coverage.up_to_commit.as_ref() else {
            return Ok(CompactionOutcome {
                removed_segments: Vec::new(),
                retained_segments: current_segment_ids(self.durable_store()),
            });
        };
        let mut store = self.ensure_loaded_store()?;
        let plan = CompactionPlan {
            checkpoint_id: store
                .checkpoints
                .last()
                .map(|manifest| manifest.checkpoint_id)
                .unwrap_or(DurableCheckpointId(0)),
            removable_segments: store
                .segments
                .iter()
                .filter(|segment| {
                    segment
                        .last_commit_id
                        .map(|commit_id| commit_id <= up_to_commit.commit_id)
                        .unwrap_or(false)
                })
                .map(|segment| segment.segment_id)
                .collect(),
        };
        let mut retained_segments = Vec::new();
        let mut removed_segments = Vec::new();
        store.segments.retain(|segment| {
            if plan.removable_segments.contains(&segment.segment_id) {
                let _ = fs::remove_file(&segment.path);
                removed_segments.push(segment.segment_id);
                false
            } else {
                retained_segments.push(segment.segment_id);
                true
            }
        });
        self.persist_store_manifest(&store)?;
        self.set_durable_store(Some(store));
        self.push_bounded_diagnostic(
            DiagnosticsScope::History,
            DiagnosticsArtifactKind::MinimalSummary,
            vec![RelationalDiagnosticsEntry {
                code: DiagnosticCode::DurableCompactionCompleted,
                message: "durable store compacted".to_string(),
                fields: json!({
                    "checkpoint_id": plan.checkpoint_id.0,
                    "removed_segments": removed_segments.iter().map(|id| id.0).collect::<Vec<_>>(),
                }),
            }],
        );
        Ok(CompactionOutcome {
            removed_segments,
            retained_segments,
        })
    }

    pub(crate) fn append_durable_commit(
        &mut self,
        envelope: CanonicalCommitEnvelope,
    ) -> Result<(), DurabilityError> {
        match self.runtime_config().durability.mode {
            DurabilityMode::InMemoryCanonical => {
                self.push_durable_log_entry(envelope);
                Ok(())
            }
            DurabilityMode::PersistedSegmentedLocalFs => {
                let mut store = self.ensure_loaded_store()?;
                let segment_capacity = store.layout.segment_commit_capacity.max(1);
                let segment_id = match store.segments.last() {
                    Some(segment) if segment.commit_count < segment_capacity => segment.segment_id,
                    _ => DurableSegmentId(
                        store
                            .segments
                            .last()
                            .map(|segment| segment.segment_id.0)
                            .unwrap_or(0)
                            + 1,
                    ),
                };
                let segment_path = segment_file_path(&store.layout, segment_id);
                let mut segment_entries = if segment_path.exists() {
                    read_json::<DurableSegmentFile>(&segment_path)?.entries
                } else {
                    Vec::new()
                };
                segment_entries.push(envelope.clone());
                write_json(
                    &segment_path,
                    &DurableSegmentFile {
                        entries: segment_entries.clone(),
                    },
                )?;
                let first_commit_id = segment_entries
                    .first()
                    .map(|entry| entry.commit.commit_id);
                let last_commit_id = segment_entries
                    .last()
                    .map(|entry| entry.commit.commit_id);
                if let Some(existing) = store
                    .segments
                    .iter_mut()
                    .find(|segment| segment.segment_id == segment_id)
                {
                    existing.first_commit_id = first_commit_id;
                    existing.last_commit_id = last_commit_id;
                    existing.commit_count = segment_entries.len();
                    existing.integrity = DurableIntegrityStatus::Verified;
                } else {
                    store.segments.push(DurableSegmentManifest {
                        segment_id,
                        path: segment_path,
                        first_commit_id,
                        last_commit_id,
                        commit_count: segment_entries.len(),
                        runtime_name: self.runtime_name().to_string(),
                        profile: self.runtime_profile(),
                        schema_version: self.primary_schema_version(),
                        integrity: DurableIntegrityStatus::Verified,
                    });
                }
                self.persist_store_manifest(&store)?;
                self.set_durable_store(Some(store));
                self.push_durable_log_entry(envelope);
                self.push_bounded_diagnostic(
                    DiagnosticsScope::History,
                    DiagnosticsArtifactKind::MinimalSummary,
                    vec![RelationalDiagnosticsEntry {
                        code: DiagnosticCode::DurableAppendSucceeded,
                        message: "durable segment append succeeded".to_string(),
                        fields: json!({
                            "segment_id": segment_id.0,
                            "commit_id": self.last_durable_log_commit_id().map(|commit_id| commit_id.0),
                        }),
                    }],
                );
                Ok(())
            }
        }
    }
}

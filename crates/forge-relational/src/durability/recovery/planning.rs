use crate::durability::data::{
    DurabilityMode, RecoveryCompatibilityCheck, RecoveryCursor, RecoveryIntegrityReport,
    RecoveryPlan,
};
use crate::logic::runtime::RelationalRuntime;

use crate::durability::log::local_store::{read_json, DurableCheckpointFile, DurableSegmentFile};

impl RelationalRuntime {
    pub fn recovery_plan(&self) -> RecoveryPlan {
        match self.config.durability_mode {
            DurabilityMode::InMemoryCanonical => {
                let checkpoint = self.durability.checkpoints.last().cloned();
                let tail_log = match checkpoint
                    .as_ref()
                    .and_then(|c| c.coverage.up_to_commit.as_ref())
                {
                    Some(up_to_commit) => self
                        .durability
                        .log
                        .iter()
                        .filter(|entry| entry.envelope.commit.commit_id > up_to_commit.commit_id)
                        .cloned()
                        .collect(),
                    None => self.durability.log.clone(),
                };
                RecoveryPlan {
                    config: self.config.clone(),
                    store: self.durability.store.clone(),
                    checkpoint_manifest: None,
                    checkpoint,
                    cursor: RecoveryCursor {
                        checkpoint_id: None,
                        segment_ids: Vec::new(),
                    },
                    integrity_report: RecoveryIntegrityReport {
                        selected_checkpoint_id: None,
                        skipped_corrupt_checkpoints: Vec::new(),
                        verified_segment_ids: Vec::new(),
                        corrupt_segment_id: None,
                    },
                    compatibility: RecoveryCompatibilityCheck {
                        schema_match: true,
                        profile_match: true,
                        runtime_name_match: true,
                    },
                    tail_log,
                }
            }
            DurabilityMode::PersistedSegmentedLocalFs => self.persisted_recovery_plan(),
        }
    }

    pub fn durable_log(&self) -> &[crate::durability::data::DurableCommitEnvelope] {
        &self.durability.log
    }

    fn persisted_recovery_plan(&self) -> RecoveryPlan {
        let Ok(store) = self.load_store_from_disk() else {
            return RecoveryPlan {
                config: self.config.clone(),
                store: self.durability.store.clone(),
                checkpoint_manifest: None,
                checkpoint: None,
                tail_log: Vec::new(),
                cursor: RecoveryCursor {
                    checkpoint_id: None,
                    segment_ids: Vec::new(),
                },
                integrity_report: RecoveryIntegrityReport {
                    selected_checkpoint_id: None,
                    skipped_corrupt_checkpoints: Vec::new(),
                    verified_segment_ids: Vec::new(),
                    corrupt_segment_id: None,
                },
                compatibility: RecoveryCompatibilityCheck {
                    schema_match: true,
                    profile_match: true,
                    runtime_name_match: true,
                },
            };
        };
        let mut skipped_corrupt_checkpoints = Vec::new();
        let mut selected_checkpoint = None;
        let mut selected_checkpoint_manifest = None;
        for manifest in store.checkpoints.iter().rev() {
            match read_json::<DurableCheckpointFile>(&manifest.path) {
                Ok(file) => {
                    selected_checkpoint = Some(file.checkpoint);
                    selected_checkpoint_manifest = Some(manifest.clone());
                    break;
                }
                Err(_) => skipped_corrupt_checkpoints.push(manifest.checkpoint_id),
            }
        }
        let checkpoint_commit = selected_checkpoint
            .as_ref()
            .and_then(|checkpoint| checkpoint.coverage.up_to_commit.as_ref())
            .map(|commit| commit.commit_id);
        let mut tail_log = Vec::new();
        let mut verified_segment_ids = Vec::new();
        let mut corrupt_segment_id = None;
        for manifest in &store.segments {
            if checkpoint_commit
                .is_some_and(|covered| manifest.last_commit_id.is_some_and(|last| last <= covered))
            {
                continue;
            }
            match read_json::<DurableSegmentFile>(&manifest.path) {
                Ok(file) => {
                    verified_segment_ids.push(manifest.segment_id);
                    tail_log.extend(file.entries.into_iter().filter(|entry| {
                        checkpoint_commit
                            .is_none_or(|covered| entry.envelope.commit.commit_id > covered)
                    }));
                }
                Err(_) => {
                    corrupt_segment_id = Some(manifest.segment_id);
                    break;
                }
            }
        }
        RecoveryPlan {
            config: self.config.clone(),
            store: Some(store.clone()),
            checkpoint_manifest: selected_checkpoint_manifest.clone(),
            checkpoint: selected_checkpoint.clone(),
            cursor: RecoveryCursor {
                checkpoint_id: selected_checkpoint_manifest
                    .as_ref()
                    .map(|manifest| manifest.checkpoint_id),
                segment_ids: verified_segment_ids.clone(),
            },
            integrity_report: RecoveryIntegrityReport {
                selected_checkpoint_id: selected_checkpoint_manifest
                    .as_ref()
                    .map(|manifest| manifest.checkpoint_id),
                skipped_corrupt_checkpoints,
                verified_segment_ids,
                corrupt_segment_id,
            },
            compatibility: RecoveryCompatibilityCheck {
                schema_match: selected_checkpoint_manifest
                    .as_ref()
                    .map(|manifest| manifest.schema_version == self.primary_schema_version())
                    .unwrap_or(true),
                profile_match: selected_checkpoint_manifest
                    .as_ref()
                    .map(|manifest| manifest.profile == self.config.profile)
                    .unwrap_or(true),
                runtime_name_match: selected_checkpoint_manifest
                    .as_ref()
                    .map(|manifest| manifest.runtime_name == self.config.runtime_name)
                    .unwrap_or(true),
            },
            tail_log,
        }
    }
}

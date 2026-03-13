use crate::capabilities::{
    DurabilityRead, RuntimeConfigSource, RuntimeIdentitySource, SchemaVersionSource,
};
use crate::durability::data::{
    DurabilityMode, RecoveryCompatibilityCheck, RecoveryCursor, RecoveryIntegrityReport,
    RecoveryPlan,
};
use crate::history::data::BranchHead;
use crate::logic::runtime::RelationalRuntime;

use crate::durability::log::local_store::{
    load_store_from_disk, read_json, DurableCheckpointFile, DurableSegmentFile,
};

pub struct DurabilityAccess<'runtime> {
    runtime: &'runtime RelationalRuntime,
}

impl<'runtime> DurabilityAccess<'runtime> {
    pub(crate) fn new(runtime: &'runtime RelationalRuntime) -> Self {
        Self { runtime }
    }

    pub fn recovery_plan(&self) -> RecoveryPlan {
        match self.runtime.runtime_config().durability.policy.mode {
            DurabilityMode::InMemoryCanonical => in_memory_recovery_plan(self.runtime),
            DurabilityMode::PersistedSegmentedLocalFs => self.persisted_recovery_plan(),
        }
    }

    pub fn durable_log(&self) -> &[crate::replay::data::CanonicalCommitEnvelope] {
        DurabilityRead::durable_log(self.runtime)
    }

    pub fn durable_branch_heads(&self) -> Vec<BranchHead> {
        self.runtime.history_access().branches()
    }

    fn persisted_recovery_plan(&self) -> RecoveryPlan {
        let Ok(store) = load_store_from_disk(self.runtime) else {
            return RecoveryPlan {
                config: self.runtime.runtime_config().clone(),
                store: self.runtime.durable_store().cloned(),
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
                        checkpoint_commit.is_none_or(|covered| entry.commit.commit_id > covered)
                    }));
                }
                Err(_) => {
                    corrupt_segment_id = Some(manifest.segment_id);
                    break;
                }
            }
        }
        RecoveryPlan {
            config: self.runtime.runtime_config().clone(),
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
                    .map(|manifest| {
                        manifest.schema_version == self.runtime.primary_schema_version_id()
                    })
                    .unwrap_or(true),
                profile_match: selected_checkpoint_manifest
                    .as_ref()
                    .map(|manifest| manifest.profile == self.runtime.runtime_profile())
                    .unwrap_or(true),
                runtime_name_match: selected_checkpoint_manifest
                    .as_ref()
                    .map(|manifest| manifest.runtime_name == self.runtime.runtime_name())
                    .unwrap_or(true),
            },
            tail_log,
        }
    }
}

impl RelationalRuntime {
    pub fn durability_access(&self) -> DurabilityAccess<'_> {
        DurabilityAccess::new(self)
    }
}

fn in_memory_recovery_plan(runtime: &(impl DurabilityRead + RuntimeConfigSource)) -> RecoveryPlan {
    let checkpoint = runtime.durable_checkpoints().last().cloned();
    let tail_log = match checkpoint
        .as_ref()
        .and_then(|c| c.coverage.up_to_commit.as_ref())
    {
        Some(up_to_commit) => runtime
            .durable_log()
            .iter()
            .filter(|entry| entry.commit.commit_id > up_to_commit.commit_id)
            .cloned()
            .collect(),
        None => runtime.durable_log().to_vec(),
    };
    RecoveryPlan {
        config: runtime.runtime_config().clone(),
        store: runtime.durable_store().cloned(),
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

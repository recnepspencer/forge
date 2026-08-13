use std::fs;

use crate::capabilities::{
    DurabilityRead, RuntimeConfigSource, RuntimeIdentitySource, SchemaVersionSource,
};
use crate::diagnostics::data::{DiagnosticsArtifactKind, DiagnosticsScope};
use crate::durability::checkpoints::images::partition_to_image;
use crate::durability::data::{
    CheckpointCoverage, CompactionOutcome, CompactionPlan, DurabilityError, DurabilityMode,
    DurableCheckpoint, DurableCheckpointId, DurableCheckpointManifest, DurableIntegrityStatus,
    DurableSegmentId, DurableSegmentManifest,
};
use crate::durability::log::local_store::{
    append_segment_entry, checkpoint_file_path, current_segment_ids, ensure_loaded_store,
    persist_store_manifest, segment_file_path, DurableCheckpointFile,
};
use crate::durability::log::native_file_codec::write_checkpoint_file;
use crate::lineage::data::{LineageCheckpointArtifact, LineageCheckpointDigestBasis};

use super::super::derived_index_artifacts::checkpoint_derived_index_artifacts;
use super::diagnostics::{
    durable_segment_append_succeeded, durable_store_compacted, in_memory_checkpoint_created,
    persisted_checkpoint_created,
};
use super::DurabilityAuthority;

impl<'runtime> DurabilityAuthority<'runtime> {
    pub fn checkpoint(&mut self) -> Result<DurableCheckpoint, DurabilityError> {
        let checkpoint = self.build_checkpoint_image()?;
        if self.runtime.runtime_config().durability.policy.mode
            == DurabilityMode::PersistedSegmentedLocalFs
        {
            let manifest = self.persist_checkpoint_file(&checkpoint)?;
            self.runtime
                .publication_authority()
                .push_bounded_diagnostic(
                    DiagnosticsScope::History,
                    DiagnosticsArtifactKind::MinimalSummary,
                    vec![persisted_checkpoint_created(&manifest, &checkpoint)],
                );
        } else {
            self.runtime
                .publication_authority()
                .push_bounded_diagnostic(
                    DiagnosticsScope::History,
                    DiagnosticsArtifactKind::MinimalSummary,
                    vec![in_memory_checkpoint_created(&checkpoint)],
                );
        }
        self.runtime.durability.push_checkpoint(checkpoint.clone());
        Ok(checkpoint)
    }

    pub fn compact_store(&mut self) -> Result<CompactionOutcome, DurabilityError> {
        if self.runtime.runtime_config().durability.policy.mode
            != DurabilityMode::PersistedSegmentedLocalFs
        {
            return Ok(CompactionOutcome {
                removed_segments: Vec::new(),
                retained_segments: Vec::new(),
            });
        }
        let Some(checkpoint) = self.runtime.durability.checkpoints.last() else {
            return Ok(CompactionOutcome {
                removed_segments: Vec::new(),
                retained_segments: current_segment_ids(self.runtime.durable_store()),
            });
        };
        let Some(up_to_commit) = checkpoint.coverage.up_to_commit.as_ref() else {
            return Ok(CompactionOutcome {
                removed_segments: Vec::new(),
                retained_segments: current_segment_ids(self.runtime.durable_store()),
            });
        };
        let mut store = ensure_loaded_store(self.runtime)?;
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
        persist_store_manifest(&store)?;
        self.runtime.durability.store = Some(store);
        self.runtime
            .publication_authority()
            .push_bounded_diagnostic(
                DiagnosticsScope::History,
                DiagnosticsArtifactKind::MinimalSummary,
                vec![durable_store_compacted(
                    plan.checkpoint_id,
                    &removed_segments,
                )],
            );
        Ok(CompactionOutcome {
            removed_segments,
            retained_segments,
        })
    }

    pub(crate) fn compact_log_if_needed(&mut self) {
        use crate::config::data::DurableLogRetentionMode;

        let policy = self.runtime.runtime_config().durability.policy.log.clone();
        if self.runtime.durability.log.len() <= policy.max_in_memory_envelopes {
            return;
        }

        match policy.retention_mode {
            DurableLogRetentionMode::RetainAllInMemory => {}
            DurableLogRetentionMode::CompactAfterCheckpoint => {
                if let Some(checkpoint) = self.runtime.durability.checkpoints.last() {
                    if let Some(commit) = checkpoint.coverage.up_to_commit.as_ref() {
                        self.runtime
                            .durability
                            .log
                            .retain(|entry| entry.commit.commit_id > commit.commit_id);
                        self.runtime.durability.rebuild_log_commit_index();
                    }
                }
                if self.runtime.durability.log.len() > policy.max_in_memory_envelopes {
                    let overflow =
                        self.runtime.durability.log.len() - policy.max_in_memory_envelopes;
                    self.runtime.durability.trim_log_front(overflow);
                }
            }
        }
        if self.runtime.runtime_config().durability.policy.mode
            == DurabilityMode::PersistedSegmentedLocalFs
            && self
                .runtime
                .runtime_config()
                .durability
                .policy
                .checkpoints
                .compact_after_checkpoint
        {
            let _ = self.compact_store();
        }
    }

    pub(crate) fn append_commit(
        &mut self,
        authority: super::DurableAppendAuthority,
        envelope: &crate::history::data::CanonicalCommitEnvelope,
    ) -> Result<(), DurabilityError> {
        authority.validate(self.runtime.runtime_instance_id(), envelope)?;
        match self.runtime.runtime_config().durability.policy.mode {
            DurabilityMode::InMemoryCanonical => {
                self.runtime.durability.push_log_envelope(envelope.clone());
                Ok(())
            }
            DurabilityMode::PersistedSegmentedLocalFs => {
                let mut store = ensure_loaded_store(self.runtime)?;
                let segment_capacity = store.layout.segment_commit_capacity.max(1);
                let active_segment = store.segments.last().cloned();
                let segment_id = match active_segment.as_ref() {
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
                let existing_manifest = store
                    .segments
                    .iter()
                    .find(|segment| segment.segment_id == segment_id)
                    .cloned();
                let first_commit_id = existing_manifest
                    .as_ref()
                    .and_then(|segment| segment.first_commit_id)
                    .unwrap_or(envelope.commit.commit_id);
                let last_commit_id = envelope.commit.commit_id;
                let commit_count = existing_manifest
                    .as_ref()
                    .map(|segment| segment.commit_count + 1)
                    .unwrap_or(1);
                append_segment_entry(&segment_path, envelope)?;
                if let Some(existing) = store
                    .segments
                    .iter_mut()
                    .find(|segment| segment.segment_id == segment_id)
                {
                    existing.first_commit_id = Some(first_commit_id);
                    existing.last_commit_id = Some(last_commit_id);
                    existing.commit_count = commit_count;
                    existing.integrity = DurableIntegrityStatus::Verified;
                } else {
                    store.segments.push(DurableSegmentManifest {
                        segment_id,
                        path: segment_path,
                        first_commit_id: Some(first_commit_id),
                        last_commit_id: Some(last_commit_id),
                        commit_count,
                        runtime_name: self.runtime.runtime_name().to_string(),
                        profile: self.runtime.runtime_profile(),
                        schema_version: self.runtime.primary_schema_version_id(),
                        integrity: DurableIntegrityStatus::Verified,
                    });
                }
                self.runtime.durability.store = Some(store);
                self.runtime.durability.push_log_envelope(envelope.clone());
                let latest_commit_id = self
                    .runtime
                    .durability
                    .log
                    .last()
                    .map(|entry| entry.commit.commit_id)
                    .unwrap_or(envelope.commit.commit_id);
                self.runtime
                    .publication_authority()
                    .push_bounded_diagnostic(
                        DiagnosticsScope::History,
                        DiagnosticsArtifactKind::MinimalSummary,
                        vec![durable_segment_append_succeeded(
                            segment_id,
                            latest_commit_id,
                        )],
                    );
                Ok(())
            }
        }
    }

    #[cfg(test)]
    pub(crate) fn remove_durable_envelope_for_test(
        &mut self,
        commit_id: crate::history::data::CommitId,
    ) -> bool {
        self.runtime.durability.remove_log_commit(commit_id)
    }

    fn build_checkpoint_image(&self) -> Result<DurableCheckpoint, DurabilityError> {
        let envelopes = self.runtime.history().commit_envelopes_snapshot();
        let published_lineage_commit_count = envelopes
            .iter()
            .filter(|envelope| envelope.has_lineage_authority())
            .count();
        let canonical_published_event_ids = envelopes
            .iter()
            .flat_map(|envelope| {
                envelope
                    .lineage_digest_basis()
                    .canonical_event_ids()
                    .iter()
                    .copied()
            })
            .collect();
        let published_lineage_event_count = envelopes
            .iter()
            .map(|envelope| envelope.lineage_digest_basis().lineage_event_count())
            .sum();
        let published_lineage_decision_count = envelopes
            .iter()
            .map(|envelope| envelope.lineage_digest_basis().lineage_decision_count())
            .sum();
        Ok(DurableCheckpoint {
            coverage: CheckpointCoverage {
                up_to_commit: self.runtime.history().latest_commit().cloned(),
                up_to_version: self
                    .runtime
                    .history()
                    .latest_commit()
                    .map(|commit| commit.version_id),
            },
            branches: self.runtime.history().branches(),
            envelopes,
            partition_images: self
                .runtime
                .partitions
                .values()
                .cloned()
                .map(|partition| {
                    partition_to_image(
                        partition,
                        &self.runtime.schema_contract_runtime.aspect_contract_plans,
                    )
                })
                .collect::<Result<Vec<_>, _>>()?,
            aspect_contracts: checkpoint_aspect_contracts(
                &self.runtime.schema_contract_runtime.aspect_contract_plans,
            )?,
            lineage: LineageCheckpointArtifact::new(
                LineageCheckpointDigestBasis::new(
                    published_lineage_commit_count,
                    canonical_published_event_ids,
                    published_lineage_event_count,
                    published_lineage_decision_count,
                ),
                self.runtime.lineage_access().nodes_snapshot(),
                self.runtime
                    .lineage_access()
                    .correspondence_candidates_snapshot(),
                self.runtime.lineage_access().rejected_decisions_snapshot(),
            ),
            index_definitions: self.runtime.index_access().definitions_snapshot(),
            derived_index_artifacts: checkpoint_derived_index_artifacts(self.runtime),
            symbol_table: self.runtime.services.symbols.snapshot(),
            runtime_name: self.runtime.runtime_name().to_string(),
        })
    }

    fn persist_checkpoint_file(
        &mut self,
        checkpoint: &DurableCheckpoint,
    ) -> Result<DurableCheckpointManifest, DurabilityError> {
        let mut store = ensure_loaded_store(self.runtime)?;
        let checkpoint_id = DurableCheckpointId(
            store
                .checkpoints
                .last()
                .map(|manifest| manifest.checkpoint_id.0)
                .unwrap_or(0)
                + 1,
        );
        let path = checkpoint_file_path(&store.layout, checkpoint_id);
        write_checkpoint_file(
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
            runtime_name: self.runtime.runtime_name().to_string(),
            profile: self.runtime.runtime_profile(),
            schema_version: self.runtime.primary_schema_version_id(),
            integrity: DurableIntegrityStatus::Verified,
        };
        store.checkpoints.push(manifest.clone());
        persist_store_manifest(&store)?;
        self.runtime.durability.store = Some(store);
        Ok(manifest)
    }
}

fn checkpoint_aspect_contracts(
    plans: &crate::schema::data::AspectContractPlanCatalog,
) -> Result<Vec<worth_foundational::facade::PortableAspectContract>, DurabilityError> {
    let mut contracts = std::collections::BTreeMap::new();
    for binding in plans
        .entity_plans
        .values()
        .chain(plans.relation_plans.values())
        .flat_map(|plan| plan.executable_bindings.iter())
    {
        let candidate =
            worth_foundational::facade::PortableAspectContract::from_contract(&binding.contract);
        match contracts.get(candidate.key()) {
            Some(existing) if existing == &candidate => continue,
            Some(_) => {
                return Err(DurabilityError::new(
                    crate::durability::data::RecoveryFailureClass::SchemaMismatch,
                    format!(
                        "checkpoint has conflicting active contracts for aspect `{}`",
                        candidate.key().as_str()
                    ),
                ));
            }
            None => {
                contracts.insert(candidate.key().clone(), candidate);
            }
        }
    }
    Ok(contracts.into_values().collect())
}

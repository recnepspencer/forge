use std::collections::{BTreeMap, BTreeSet};

use crate::diagnostics::data::{
    DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope, RelationalDiagnosticsEntry,
};
use crate::history::data::{BranchId, CommitId};
use crate::indexes::data::{
    DerivedIndexBuildOutcome, DerivedIndexBuildRequest, DerivedIndexCompatibility,
    DerivedIndexDefinition, DerivedIndexGeneration, DerivedIndexGenerationId, DerivedIndexId,
    DerivedIndexKind, DerivedIndexPayload, DerivedIndexPublicationStatus,
};
use crate::logic::runtime::{IndexedReadOutcome, RelationalReadView, RelationalRuntime};
use crate::payloads::data::RecordPayload;
use crate::query::data::QueryWorkPacket;
use crate::snapshots::data::SnapshotHandle;
use crate::storage::logic::state::PartitionAccess;
use serde_json::json;

impl RelationalRuntime {
    pub(crate) fn refresh_unique_field_index_for_records(
        &mut self,
        changed_records: &[crate::transactions::data::RecordRef],
        version_id: crate::identity::data::VersionId,
    ) {
        let tracked_fields = self.tracked_unique_entity_fields();
        if tracked_fields.is_empty() {
            return;
        }
        let state = self.current_state();
        let mut refreshed_values = Vec::new();
        for record in changed_records {
            let crate::transactions::data::RecordRef::Entity(entity_id) = record else {
                continue;
            };
            for field in &tracked_fields {
                if let Some(payload) = crate::validation::logic::entity_payload_for_state(
                    &state, *entity_id, version_id,
                ) {
                    if let Some(value) = payload
                        .as_json()
                        .and_then(|value| value.get(field))
                        .and_then(|value| value.as_str())
                    {
                        refreshed_values.push((field.clone(), value.to_string(), *entity_id));
                    }
                }
            }
        }
        for record in changed_records {
            let crate::transactions::data::RecordRef::Entity(entity_id) = record else {
                continue;
            };
            for field in &tracked_fields {
                if let Some(values) = self.indexes.entity_unique_field_index.get_mut(field) {
                    values.retain(|_, entity_ids| {
                        entity_ids.remove(entity_id);
                        !entity_ids.is_empty()
                    });
                }
            }
        }
        for (field, value, entity_id) in refreshed_values {
            self.indexes
                .entity_unique_field_index
                .entry(field)
                .or_default()
                .entry(value)
                .or_default()
                .insert(entity_id);
        }
    }

    #[allow(dead_code)]
    pub(crate) fn rebuild_unique_field_indexes(&mut self) {
        self.indexes.entity_unique_field_index.clear();
        let tracked_fields = self.tracked_unique_entity_fields();
        if tracked_fields.is_empty() {
            return;
        }
        let state = self.current_state();
        let version_id = self.current_version_id();
        let mut rebuilt_values = Vec::new();
        for partition_id in state.partition_ids() {
            let partition = state
                .get_partition(partition_id)
                .expect("partition for unique field rebuild");
            for slot in 0..partition.entity_arena.slot_count() {
                let Some(slot_view) = partition.entity_arena.get_slot(slot) else {
                    continue;
                };
                if slot_view.lifecycle() == crate::storage::data::RecordLifecycleState::Reusable {
                    continue;
                }
                let entity_id = crate::identity::data::EntityId::new(
                    partition_id,
                    slot as u64,
                    slot_view.generation(),
                );
                if let Some(payload) = crate::validation::logic::entity_payload_for_state(
                    &state, entity_id, version_id,
                ) {
                    for field in &tracked_fields {
                        if let Some(value) = payload
                            .as_json()
                            .and_then(|value| value.get(field))
                            .and_then(|value| value.as_str())
                        {
                            rebuilt_values.push((field.clone(), value.to_string(), entity_id));
                        }
                    }
                }
            }
        }
        for (field, value, entity_id) in rebuilt_values {
            self.indexes
                .entity_unique_field_index
                .entry(field)
                .or_default()
                .entry(value)
                .or_default()
                .insert(entity_id);
        }
    }

    #[cfg(test)]
    pub(crate) fn rebuild_unique_field_indexes_for_test(&mut self) {
        self.rebuild_unique_field_indexes();
    }

    fn tracked_unique_entity_fields(&self) -> BTreeSet<String> {
        let mut fields = BTreeSet::new();
        for rules in [
            &self.config.invariant_catalog.always_on_structural,
            &self.config.invariant_catalog.commit_boundary,
            &self.config.invariant_catalog.snapshot_audit,
            &self.config.invariant_catalog.harness_heavy,
        ] {
            for rule in rules {
                if let crate::validation::data::InvariantRule::UniqueEntityPayloadField(field) =
                    rule
                {
                    fields.insert(field.clone());
                }
            }
        }
        fields
    }

    pub fn register_index(
        &mut self,
        mut definition: DerivedIndexDefinition,
    ) -> DerivedIndexDefinition {
        definition.index_id = DerivedIndexId(self.indexes.next_index_id);
        self.indexes.next_index_id += 1;
        self.indexes
            .definitions
            .insert(definition.index_id, definition.clone());
        definition
    }

    pub fn build_indexes_for_commit(
        &mut self,
        request: DerivedIndexBuildRequest,
    ) -> DerivedIndexBuildOutcome {
        let mut generations = Vec::new();
        let mut failed_indexes = Vec::new();
        let Some(commit) = self.history.commit_envelopes.get(&request.source_commit_id) else {
            return DerivedIndexBuildOutcome {
                source_commit_id: request.source_commit_id,
                generations,
                failed_indexes: request.index_ids,
            };
        };
        let read = self.read_version(commit.commit.version_id);
        for index_id in request.index_ids {
            let Some(definition) = self.indexes.definitions.get(&index_id).cloned() else {
                failed_indexes.push(index_id);
                continue;
            };
            match self.build_index_payload(&definition, &read) {
                Some(payload) => {
                    let generation = DerivedIndexGeneration {
                        generation_id: DerivedIndexGenerationId(self.indexes.next_generation_id),
                        index_id,
                        source_commit_id: request.source_commit_id,
                        source_branch_id: request.branch_id.clone(),
                        compatibility: DerivedIndexCompatibility {
                            branch_id: request.branch_id.clone(),
                            version_id: commit.commit.version_id,
                            schema_version: self.primary_schema_version(),
                        },
                        status: DerivedIndexPublicationStatus::Published,
                        payload,
                    };
                    self.indexes.next_generation_id += 1;
                    self.indexes
                        .generations
                        .entry(index_id)
                        .or_default()
                        .push(generation.clone());
                    generations.push(generation);
                }
                None => failed_indexes.push(index_id),
            }
        }
        self.attach_index_generations_to_commit(request.source_commit_id, &generations);
        self.push_bounded_diagnostic(
            DiagnosticsScope::QueryPlanning,
            if failed_indexes.is_empty() {
                DiagnosticsArtifactKind::MinimalSummary
            } else {
                DiagnosticsArtifactKind::Failure
            },
            vec![RelationalDiagnosticsEntry {
                code: if failed_indexes.is_empty() {
                    DiagnosticCode::CommitPublished
                } else {
                    DiagnosticCode::DiagnosticsPublicationFailure
                },
                message: "derived index build completed".to_string(),
                fields: json!({
                    "source_commit_id": request.source_commit_id.0,
                    "branch_id": request.branch_id.0,
                    "generation_count": generations.len(),
                    "failed_indexes": failed_indexes.iter().map(|id| id.0).collect::<Vec<_>>(),
                }),
            }],
        );
        DerivedIndexBuildOutcome {
            source_commit_id: request.source_commit_id,
            generations,
            failed_indexes,
        }
    }

    pub fn latest_index_generation(
        &self,
        index_id: DerivedIndexId,
        branch_id: &BranchId,
    ) -> Option<&DerivedIndexGeneration> {
        let definition = self.indexes.definitions.get(&index_id)?;
        self.indexes
            .generations
            .get(&index_id)
            .and_then(|generations| {
                generations.iter().rev().find(|generation| {
                    !definition.branch_scoped || generation.compatibility.branch_id == *branch_id
                })
            })
    }

    pub fn index_generations_for_version(
        &self,
        version_id: crate::identity::data::VersionId,
    ) -> Vec<DerivedIndexGeneration> {
        let mut generations = self
            .indexes
            .generations
            .values()
            .flat_map(|generations| generations.iter())
            .filter(|generation| generation.compatibility.version_id <= version_id)
            .cloned()
            .collect::<Vec<_>>();
        generations.sort_by_key(|generation| {
            (
                generation.compatibility.branch_id.clone(),
                generation.source_commit_id,
                generation.generation_id,
            )
        });
        generations
    }

    pub fn read_with_storage_fallback(
        &self,
        handle: &SnapshotHandle,
        packet: &QueryWorkPacket,
    ) -> Option<IndexedReadOutcome> {
        let result = self.execute_read_packet(handle, packet)?;
        let branch_id = self
            .branch_id_for_version(handle.version_id)
            .unwrap_or_else(|| self.config.main_branch.clone());
        let used_index_generation = self
            .compatible_index_generations_for_version(&branch_id, handle.version_id)
            .into_iter()
            .max_by_key(|generation| generation.generation_id)
            .map(|generation| generation.generation_id);
        Some(IndexedReadOutcome {
            result,
            used_index_generation,
        })
    }

    fn build_index_payload(
        &self,
        definition: &DerivedIndexDefinition,
        read: &RelationalReadView,
    ) -> Option<DerivedIndexPayload> {
        match &definition.kind {
            DerivedIndexKind::EntityPayloadField { field } => {
                let mut map = BTreeMap::new();
                for entity in read.entities() {
                    let Some(key) = payload_field_key(&entity.payload, field) else {
                        continue;
                    };
                    map.entry(key)
                        .or_insert_with(Vec::new)
                        .push(entity.entity_id);
                }
                Some(DerivedIndexPayload::EntityField(map))
            }
            DerivedIndexKind::RelationPayloadField { field } => {
                let mut map = BTreeMap::new();
                for relation in read.relations() {
                    let Some(key) = payload_field_key_optional(&relation.payload, field) else {
                        continue;
                    };
                    map.entry(key)
                        .or_insert_with(Vec::new)
                        .push(relation.relation_id);
                }
                Some(DerivedIndexPayload::RelationField(map))
            }
        }
    }

    fn attach_index_generations_to_commit(
        &mut self,
        commit_id: CommitId,
        generations: &[DerivedIndexGeneration],
    ) {
        let ids = generations
            .iter()
            .map(|generation| generation.generation_id.0)
            .collect::<Vec<_>>();
        if let Some(envelope) = self.history.commit_envelopes.get_mut(&commit_id) {
            envelope.index_generation_ids.extend(ids.iter().copied());
        }
        if let Some(log_entry) = self
            .durability
            .log
            .iter_mut()
            .find(|entry| entry.commit.commit_id == commit_id)
        {
            log_entry.index_generation_ids.extend(ids.iter().copied());
        }
    }

    fn branch_id_for_version(
        &self,
        version_id: crate::identity::data::VersionId,
    ) -> Option<BranchId> {
        self.history
            .commit_graph
            .values()
            .find(|node| node.commit.version_id == version_id)
            .map(|node| node.commit.branch_id.clone())
    }

    fn compatible_index_generations_for_version(
        &self,
        branch_id: &BranchId,
        version_id: crate::identity::data::VersionId,
    ) -> Vec<&DerivedIndexGeneration> {
        self.indexes
            .generations
            .values()
            .flat_map(|generations| generations.iter())
            .filter(|generation| {
                generation.compatibility.version_id <= version_id
                    && self
                        .indexes
                        .definitions
                        .get(&generation.index_id)
                        .is_some_and(|definition| {
                            !definition.branch_scoped
                                || generation.compatibility.branch_id == *branch_id
                        })
            })
            .collect()
    }
}

fn payload_field_key(payload: &RecordPayload, field: &str) -> Option<String> {
    payload.as_json()?.get(field).map(|value| match value {
        serde_json::Value::String(text) => text.clone(),
        other => other.to_string(),
    })
}

fn payload_field_key_optional(payload: &Option<RecordPayload>, field: &str) -> Option<String> {
    payload
        .as_ref()
        .and_then(|payload| payload_field_key(payload, field))
}

use std::collections::BTreeMap;

use serde_json::json;

use crate::diagnostics::data::{
    DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope, RelationalDiagnosticsEntry,
};
use crate::history::data::CommitId;
use crate::indexes::data::{
    DerivedIndexBuildOutcome, DerivedIndexBuildRequest, DerivedIndexCompatibility,
    DerivedIndexDefinition, DerivedIndexGeneration, DerivedIndexGenerationId, DerivedIndexId,
    DerivedIndexKind, DerivedIndexPayload, DerivedIndexPublicationStatus,
};
use crate::logic::runtime::{RelationalReadView, RelationalRuntime};

use super::unique_field_index::{
    rebuild_unique_field_indexes, refresh_unique_field_index_for_records,
};
use super::unique_field_index::{payload_field_key, payload_field_key_optional};

pub struct IndexAuthority<'runtime> {
    runtime: &'runtime mut RelationalRuntime,
}

impl RelationalRuntime {
    pub fn index_authority(&mut self) -> IndexAuthority<'_> {
        IndexAuthority::new(self)
    }
}

impl<'runtime> IndexAuthority<'runtime> {
    pub(crate) fn new(runtime: &'runtime mut RelationalRuntime) -> Self {
        Self { runtime }
    }

    pub fn register(
        &mut self,
        mut definition: DerivedIndexDefinition,
    ) -> DerivedIndexDefinition {
        definition.index_id = DerivedIndexId(self.runtime.indexes.next_index_id);
        self.runtime.indexes.next_index_id += 1;
        self.runtime
            .indexes
            .definitions
            .insert(definition.index_id, definition.clone());
        definition
    }

    pub fn build_for_commit(
        &mut self,
        request: DerivedIndexBuildRequest,
    ) -> DerivedIndexBuildOutcome {
        let mut generations = Vec::new();
        let mut failed_indexes = Vec::new();
        let Some(commit) = self
            .runtime
            .history
            .commit_envelopes
            .get(&request.source_commit_id)
        else {
            return DerivedIndexBuildOutcome {
                source_commit_id: request.source_commit_id,
                generations,
                failed_indexes: request.index_ids,
            };
        };
        let read = self.runtime.visibility_reads().read_version(commit.commit.version_id);
        for index_id in request.index_ids {
            let Some(definition) = self.runtime.indexes.definitions.get(&index_id).cloned() else {
                failed_indexes.push(index_id);
                continue;
            };
            match build_index_payload(&definition, &read) {
                Some(payload) => {
                    let generation = DerivedIndexGeneration {
                        generation_id: DerivedIndexGenerationId(self.runtime.indexes.next_generation_id),
                        index_id,
                        source_commit_id: request.source_commit_id,
                        source_branch_id: request.branch_id.clone(),
                        compatibility: DerivedIndexCompatibility {
                            branch_id: request.branch_id.clone(),
                            version_id: commit.commit.version_id,
                            schema_version: self.runtime.primary_schema_version(),
                        },
                        status: DerivedIndexPublicationStatus::Published,
                        payload,
                    };
                    self.runtime.indexes.next_generation_id += 1;
                    self.runtime
                        .indexes
                        .generations
                        .entry(index_id)
                        .or_default()
                        .push(generation.clone());
                    generations.push(generation);
                }
                None => failed_indexes.push(index_id),
            }
        }
        self.attach_generations_to_commit(request.source_commit_id, &generations);
        self.runtime.publication_authority().push_bounded_diagnostic(
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

    pub(crate) fn refresh_unique_field_index_for_records(
        &mut self,
        changed_records: &[crate::transactions::data::RecordRef],
        version_id: crate::identity::data::VersionId,
    ) {
        refresh_unique_field_index_for_records(self.runtime, changed_records, version_id);
    }

    pub(crate) fn rebuild_unique_field_indexes(&mut self) {
        rebuild_unique_field_indexes(self.runtime);
    }

    fn attach_generations_to_commit(
        &mut self,
        commit_id: CommitId,
        generations: &[DerivedIndexGeneration],
    ) {
        let ids = generations
            .iter()
            .map(|generation| generation.generation_id.0)
            .collect::<Vec<_>>();
        if let Some(envelope) = self.runtime.history.commit_envelopes.get_mut(&commit_id) {
            envelope.index_generation_ids.extend(ids.iter().copied());
        }
        if let Some(log_entry) = self
            .runtime
            .durability
            .log
            .iter_mut()
            .find(|entry| entry.commit.commit_id == commit_id)
        {
            log_entry.index_generation_ids.extend(ids.iter().copied());
        }
    }
}

fn build_index_payload(
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
                map.entry(key).or_insert_with(Vec::new).push(entity.entity_id);
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

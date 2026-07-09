mod build_execution;
mod diagnostics;
mod packet_planning;

use crate::capabilities::SchemaVersionSource;
use crate::history::data::CommitId;
use crate::indexes::data::{
    DerivedIndexApplicability, DerivedIndexBuildOutcome, DerivedIndexBuildRequest,
    DerivedIndexDefinition, DerivedIndexGeneration, DerivedIndexGenerationId, DerivedIndexId,
    DerivedIndexPublicationStatus,
};
use crate::logic::runtime::RelationalRuntime;

use self::build_execution::{execute_index_packets, record_index_preparation_strategy_counters};
use self::diagnostics::{
    derived_index_build_artifact_kind, derived_index_build_completed, derived_index_build_scope,
};
use self::packet_planning::{
    choose_index_preparation_strategy, plan_index_packets, planned_index_definitions,
};
use super::unique_entity_aspect_field_index::{
    rebuild_unique_entity_aspect_field_indexes,
    refresh_unique_entity_aspect_field_index_for_records,
};

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

    pub fn register(&mut self, mut definition: DerivedIndexDefinition) -> DerivedIndexDefinition {
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
        let Some(version_id) = self.source_commit_version_id(request.source_commit_id) else {
            return DerivedIndexBuildOutcome {
                source_commit_id: request.source_commit_id,
                generations,
                failed_indexes: request.index_ids,
            };
        };

        let (definitions, missing_indexes) =
            planned_index_definitions(self.runtime, &request.index_ids);
        failed_indexes.extend(missing_indexes);

        let strategy = choose_index_preparation_strategy(self.runtime, definitions.len());
        record_index_preparation_strategy_counters(self.runtime, definitions.len(), &strategy);

        let schema_version = self.runtime.primary_schema_version_id();
        let projection = self.runtime.read_truth().project_version(version_id);
        let packets = plan_index_packets(&definitions);
        let results =
            execute_index_packets(self.runtime, &projection, &packets, strategy.selected_mode);

        for result in results {
            match result.entries {
                Some(entries) => {
                    let generation = DerivedIndexGeneration {
                        generation_id: DerivedIndexGenerationId(
                            self.runtime.indexes.next_generation_id,
                        ),
                        index_id: result.index_id,
                        source_commit_id: request.source_commit_id,
                        source_branch_id: request.branch_id.clone(),
                        applicability: DerivedIndexApplicability {
                            branch_id: request.branch_id.clone(),
                            version_id,
                            schema_version,
                        },
                        status: DerivedIndexPublicationStatus::Published,
                        entries,
                    };
                    self.runtime.indexes.next_generation_id += 1;
                    self.runtime
                        .indexes
                        .generations
                        .entry(result.index_id)
                        .or_default()
                        .push(generation.clone());
                    generations.push(generation);
                }
                None => failed_indexes.push(result.index_id),
            }
        }

        self.attach_generations_to_commit(request.source_commit_id, &generations);
        self.publish_build_diagnostic(&request, &generations, &failed_indexes);

        DerivedIndexBuildOutcome {
            source_commit_id: request.source_commit_id,
            generations,
            failed_indexes,
        }
    }

    pub(crate) fn refresh_unique_entity_aspect_field_index_for_records(
        &mut self,
        changed_records: &[crate::transactions::data::RecordRef],
        version_id: crate::identity::data::VersionId,
    ) {
        refresh_unique_entity_aspect_field_index_for_records(
            self.runtime,
            changed_records,
            version_id,
        );
    }

    pub(crate) fn rebuild_unique_entity_aspect_field_indexes(&mut self) {
        rebuild_unique_entity_aspect_field_indexes(self.runtime);
    }

    fn source_commit_version_id(
        &self,
        commit_id: CommitId,
    ) -> Option<crate::identity::data::VersionId> {
        self.runtime
            .history
            .commit_envelopes
            .get(&commit_id)
            .map(|commit| commit.commit.version_id)
    }

    fn attach_generations_to_commit(
        &mut self,
        commit_id: CommitId,
        generations: &[DerivedIndexGeneration],
    ) {
        self.runtime
            .history_authority()
            .append_index_generations(commit_id, generations);
        if let Some(log_entry) = self
            .runtime
            .durability
            .log
            .iter_mut()
            .find(|entry| entry.commit.commit_id == commit_id)
        {
            log_entry.append_index_generations_canonical(generations);
        }
    }

    fn publish_build_diagnostic(
        &mut self,
        request: &DerivedIndexBuildRequest,
        generations: &[DerivedIndexGeneration],
        failed_indexes: &[DerivedIndexId],
    ) {
        self.runtime
            .publication_authority()
            .push_bounded_diagnostic(
                derived_index_build_scope(),
                derived_index_build_artifact_kind(failed_indexes),
                vec![derived_index_build_completed(
                    request.source_commit_id,
                    &request.branch_id,
                    generations,
                    failed_indexes,
                )],
            );
    }
}

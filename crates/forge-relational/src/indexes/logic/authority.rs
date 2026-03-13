use std::collections::BTreeMap;

use rayon::prelude::*;
use serde_json::json;

use crate::authority::commit::preparation::packets::index::{
    IndexFragmentIdentity, IndexPreparationHeader, IndexPreparationPacket,
};
use crate::authority::commit::preparation::planning::strategy::{
    strategy_for_parallel_packets, ParallelLegality, ParallelProfitability,
    PreparationFallbackReason, PreparationStrategy, PreparationStrategySelection,
};
use crate::authority::commit::preparation::proofs::kinds::PreparationProofKind;
use crate::authority::commit::preparation::proofs::locality::{
    PreparationLocalityProof, PreparationPartitionScope, PreparationReadSetApproximation,
    PreparationRecordDomain, PreparationWriteExclusionClass,
};
use crate::authority::commit::preparation::reduction::keys::IndexReductionKey;
use crate::authority::commit::preparation::reduction::merge::{
    canonical_merge_streams, OrderedReductionStream,
};
use crate::capabilities::SchemaVersionSource;
use crate::diagnostics::data::{
    DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope, RelationalDiagnosticsEntry,
};
use crate::history::data::CommitId;
use crate::indexes::data::{
    DerivedIndexBuildOutcome, DerivedIndexBuildRequest, DerivedIndexCompatibility,
    DerivedIndexDefinition, DerivedIndexGeneration, DerivedIndexGenerationId, DerivedIndexId,
    DerivedIndexKind, DerivedIndexPayload, DerivedIndexPublicationStatus,
};
use crate::logic::planning::RelationalExecutionModel;
use crate::logic::runtime::{RelationalRuntime, VisibilityProjectionView};
use crate::validation::data::InvariantGroupSet;

use super::unique_field_index::{payload_field_key, payload_field_key_optional};
use super::unique_field_index::{
    rebuild_unique_field_indexes, refresh_unique_field_index_for_records,
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
        let Some(version_id) = self
            .runtime
            .history
            .commit_envelopes
            .get(&request.source_commit_id)
            .map(|commit| commit.commit.version_id)
        else {
            return DerivedIndexBuildOutcome {
                source_commit_id: request.source_commit_id,
                generations,
                failed_indexes: request.index_ids,
            };
        };

        let (definitions, missing_indexes) =
            planned_index_definitions(self.runtime, &request.index_ids);
        failed_indexes.extend(missing_indexes);

        self.runtime
            .performance_access()
            .count_preparation_packet_shape(
                definitions.len(),
                definitions.len(),
                usize::from(!definitions.is_empty()),
                usize::from(!definitions.is_empty()),
            );

        let strategy = choose_index_preparation_strategy(self.runtime, definitions.len());
        match strategy.parallel_legality {
            ParallelLegality::ProvenParallel => self
                .runtime
                .performance_access()
                .count_preparation_parallel_legal(),
            ParallelLegality::RequiresSerial => {}
        }
        match strategy.parallel_profitability {
            ParallelProfitability::Profitable => self
                .runtime
                .performance_access()
                .count_preparation_parallel_profitable(),
            ParallelProfitability::NotProfitable => {}
        }
        match strategy.selected_mode {
            PreparationStrategySelection::Serial => self
                .runtime
                .performance_access()
                .count_preparation_serial_strategy(),
            PreparationStrategySelection::StagedParallel => self
                .runtime
                .performance_access()
                .count_preparation_staged_parallel_strategy(),
        }

        let schema_version = self.runtime.primary_schema_version_id();
        let projection = self.runtime.visibility_reads().project_version(version_id);
        let packets = plan_index_packets(&definitions);
        let results = execute_index_packets(&projection, &packets, strategy.selected_mode);

        for result in results {
            match result.payload {
                Some(payload) => {
                    let generation = DerivedIndexGeneration {
                        generation_id: DerivedIndexGenerationId(
                            self.runtime.indexes.next_generation_id,
                        ),
                        index_id: result.index_id,
                        source_commit_id: request.source_commit_id,
                        source_branch_id: request.branch_id.clone(),
                        compatibility: DerivedIndexCompatibility {
                            branch_id: request.branch_id.clone(),
                            version_id,
                            schema_version,
                        },
                        status: DerivedIndexPublicationStatus::Published,
                        payload,
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
        self.runtime
            .publication_authority()
            .push_bounded_diagnostic(
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
        self.runtime
            .history_authority()
            .append_index_generation_ids(commit_id, &ids);
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

#[derive(Debug, Clone)]
struct IndexPreparationResult {
    index_id: DerivedIndexId,
    payload: Option<DerivedIndexPayload>,
}

fn planned_index_definitions(
    runtime: &RelationalRuntime,
    index_ids: &[DerivedIndexId],
) -> (Vec<DerivedIndexDefinition>, Vec<DerivedIndexId>) {
    let mut definitions = Vec::new();
    let mut missing_indexes = Vec::new();

    for index_id in index_ids {
        if let Some(definition) = runtime.indexes.definitions.get(index_id).cloned() {
            definitions.push(definition);
        } else {
            missing_indexes.push(*index_id);
        }
    }

    (definitions, missing_indexes)
}

fn choose_index_preparation_strategy(
    runtime: &RelationalRuntime,
    packet_count: usize,
) -> PreparationStrategy {
    if !matches!(
        runtime.config.execution.execution_model,
        RelationalExecutionModel::StagedParallelPreparation
    ) {
        return PreparationStrategy::serial(PreparationFallbackReason::ExecutionModelSerial);
    }

    strategy_for_parallel_packets(runtime.config.execution.execution_model, packet_count)
}

fn plan_index_packets(definitions: &[DerivedIndexDefinition]) -> Vec<IndexPreparationPacket> {
    definitions
        .iter()
        .cloned()
        .enumerate()
        .map(|(packet_index, definition)| {
            let record_domain = match definition.kind {
                DerivedIndexKind::EntityPayloadField { .. } => PreparationRecordDomain::Entity,
                DerivedIndexKind::RelationPayloadField { .. } => PreparationRecordDomain::Relation,
            };
            IndexPreparationPacket {
                header: IndexPreparationHeader {
                    packet_index,
                    identity: IndexFragmentIdentity {
                        index_id: definition.index_id,
                        packet_index,
                    },
                    reduction_key: IndexReductionKey::new(definition.index_id, packet_index),
                    proof_kind: PreparationProofKind::ReadOnlyShared,
                    locality: PreparationLocalityProof {
                        observation_scope:
                            crate::validation::engine::InvariantObservationKind::Committed,
                        record_domain,
                        partition_scope: PreparationPartitionScope::AllObserved,
                        invariant_group_scope: InvariantGroupSet::empty(),
                        read_set_approximation: PreparationReadSetApproximation::FullObservedScan,
                        write_exclusion: PreparationWriteExclusionClass::PublicationExcluded,
                    },
                },
                definition,
            }
        })
        .collect()
}

fn execute_index_packets(
    projection: &VisibilityProjectionView<'_>,
    packets: &[IndexPreparationPacket],
    selected_mode: PreparationStrategySelection,
) -> Vec<IndexPreparationResult> {
    let entity_packets = packets
        .iter()
        .filter_map(|packet| match &packet.definition.kind {
            DerivedIndexKind::EntityPayloadField { field } => Some((
                packet.header.reduction_key,
                packet.header.identity.index_id,
                field.clone(),
            )),
            DerivedIndexKind::RelationPayloadField { .. } => None,
        })
        .collect::<Vec<_>>();
    let relation_packets = packets
        .iter()
        .filter_map(|packet| match &packet.definition.kind {
            DerivedIndexKind::EntityPayloadField { .. } => None,
            DerivedIndexKind::RelationPayloadField { field } => Some((
                packet.header.reduction_key,
                packet.header.identity.index_id,
                field.clone(),
            )),
        })
        .collect::<Vec<_>>();

    let entity_streams = match selected_mode {
        PreparationStrategySelection::StagedParallel => {
            let records = projection.all_entity_records();
            entity_packets
                .par_iter()
                .map(|(reduction_key, index_id, field)| {
                    OrderedReductionStream::singleton(
                        reduction_key.clone(),
                        IndexPreparationResult {
                            index_id: *index_id,
                            payload: Some(DerivedIndexPayload::EntityField(
                                build_entity_index_map(&records, field),
                            )),
                        },
                    )
                })
                .collect::<Vec<_>>()
        }
        PreparationStrategySelection::Serial => {
            let records = projection.all_entity_records();
            entity_packets
                .iter()
                .map(|(reduction_key, index_id, field)| {
                    OrderedReductionStream::singleton(
                        reduction_key.clone(),
                        IndexPreparationResult {
                            index_id: *index_id,
                            payload: Some(DerivedIndexPayload::EntityField(
                                build_entity_index_map(&records, field),
                            )),
                        },
                    )
                })
                .collect::<Vec<_>>()
        }
    };
    let relation_streams = match selected_mode {
        PreparationStrategySelection::StagedParallel => {
            let records = projection.all_relation_records();
            relation_packets
                .par_iter()
                .map(|(reduction_key, index_id, field)| {
                    OrderedReductionStream::singleton(
                        reduction_key.clone(),
                        IndexPreparationResult {
                            index_id: *index_id,
                            payload: Some(DerivedIndexPayload::RelationField(
                                build_relation_index_map(&records, field),
                            )),
                        },
                    )
                })
                .collect::<Vec<_>>()
        }
        PreparationStrategySelection::Serial => {
            let records = projection.all_relation_records();
            relation_packets
                .iter()
                .map(|(reduction_key, index_id, field)| {
                    OrderedReductionStream::singleton(
                        reduction_key.clone(),
                        IndexPreparationResult {
                            index_id: *index_id,
                            payload: Some(DerivedIndexPayload::RelationField(
                                build_relation_index_map(&records, field),
                            )),
                        },
                    )
                })
                .collect::<Vec<_>>()
        }
    };

    let result_streams = entity_streams
        .into_iter()
        .chain(relation_streams)
        .collect::<Vec<_>>();

    canonical_merge_streams(result_streams)
        .into_iter()
        .map(|(_, result)| result)
        .collect()
}

fn build_entity_index_map(
    records: &[crate::storage::data::EntityReadRecord],
    field: &str,
) -> BTreeMap<String, Vec<crate::identity::data::EntityId>> {
    let mut map = BTreeMap::new();
    for entity in records {
        let Some(key) = payload_field_key(&entity.payload, field) else {
            continue;
        };
        map.entry(key)
            .or_insert_with(Vec::new)
            .push(entity.entity_id);
    }
    map
}

fn build_relation_index_map(
    records: &[crate::storage::data::RelationReadRecord],
    field: &str,
) -> BTreeMap<String, Vec<crate::identity::data::RelationId>> {
    let mut map = BTreeMap::new();
    for relation in records {
        let Some(key) = payload_field_key_optional(&relation.payload, field) else {
            continue;
        };
        map.entry(key)
            .or_insert_with(Vec::new)
            .push(relation.relation_id);
    }
    map
}

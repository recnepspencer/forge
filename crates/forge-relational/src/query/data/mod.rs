use std::sync::Arc;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::history::data::AspectFilter;
use crate::history::data::CommitId;
use crate::identity::data::{EntityId, KindId, PartitionId, VersionId};
use crate::indexes::data::DerivedIndexGenerationId;
use crate::schema::data::{DescriptorSemanticsVersion, SchemaVersionId};
use crate::snapshots::data::{SnapshotHandle, SnapshotId};
use crate::storage::data::{EntityReadRecord, RelationReadRecord};
use crate::transactions::data::RecordRef;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct DeterministicQueryPlanKey(pub u128);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct DeterministicQueryFragmentKey(pub u128);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QueryPlanContextId {
    pub runtime_instance_id: u64,
    pub snapshot_id: SnapshotId,
    pub version_id: VersionId,
    pub schema_version: SchemaVersionId,
    pub descriptor_semantics_version: DescriptorSemanticsVersion,
    pub evidence_basis: QueryPlanEvidenceBasis,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum QueryPlanEvidenceBasis {
    CanonicalCommitEnvelope { commit_id: CommitId },
    GenesisRuntimeBootstrap,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PartitionHint {
    pub partition_id: PartitionId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QueryExecutionShape {
    SingleEntity,
    BulkPacketized,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReductionDiscipline {
    DeterministicMerge,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QueryOrderingContract {
    CanonicalEntityIdOrder,
    CanonicalRelationIdOrder,
    CanonicalRecordRefOrder,
    CanonicalTraversalOrder,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QueryFallbackContract {
    StorageOnly,
    IndexAdmissibleStorageEquivalent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QueryParallelLegality {
    LegalReadOnlySnapshot,
    RequiresSerialReduction,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuerySerialReason {
    TinyPacket,
    SingleChunkSurface,
    BroadCrossPartitionCoordination,
    UnsupportedIndexPath,
    ReductionWouldDominateExecution,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum QueryParallelProfitability {
    Profitable,
    SerialPreferred { reason: QuerySerialReason },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum QueryLocalityClass {
    SinglePartition { partition_id: PartitionId },
    PartitionBounded { partitions: Arc<[PartitionId]> },
    CrossPartitionTraversal,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum QueryScope {
    ExplicitTargets {
        targets: Arc<[RecordRef]>,
    },
    EntityKindScan {
        kind_id: KindId,
        partition_scope: Option<Arc<[PartitionId]>>,
    },
    RelationKindScan {
        kind_id: KindId,
        partition_scope: Option<Arc<[PartitionId]>>,
    },
    EntityPayloadFieldEquals {
        field: String,
        value: String,
        partition_scope: Option<Arc<[PartitionId]>>,
    },
    EntityPayloadFieldAnyOf {
        field: String,
        values: Arc<[String]>,
        partition_scope: Option<Arc<[PartitionId]>>,
    },
    RelationPayloadFieldEquals {
        field: String,
        value: String,
        partition_scope: Option<Arc<[PartitionId]>>,
    },
    RelationPayloadFieldAnyOf {
        field: String,
        values: Arc<[String]>,
        partition_scope: Option<Arc<[PartitionId]>>,
    },
    AspectFilteredEntities {
        kind_id: Option<KindId>,
        aspect_filter: AspectFilter,
        partition_scope: Option<Arc<[PartitionId]>>,
    },
    AspectFilteredRelations {
        kind_id: Option<KindId>,
        aspect_filter: AspectFilter,
        partition_scope: Option<Arc<[PartitionId]>>,
    },
    OutgoingNeighborhood {
        seeds: Arc<[EntityId]>,
        relation_kind_scope: Option<Arc<[KindId]>>,
    },
    IncomingNeighborhood {
        seeds: Arc<[EntityId]>,
        relation_kind_scope: Option<Arc<[KindId]>>,
    },
    ConnectivityTraversal {
        seeds: Arc<[EntityId]>,
        relation_kind_scope: Option<Arc<[KindId]>>,
        max_depth: Option<u32>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlannedQueryPacket {
    pub label: String,
    pub context_id: QueryPlanContextId,
    pub scope: QueryScope,
    pub locality: QueryLocalityClass,
    pub ordering: QueryOrderingContract,
    pub fallback: QueryFallbackContract,
    pub execution_shape: QueryExecutionShape,
    pub reduction: ReductionDiscipline,
    pub plan_key: DeterministicQueryPlanKey,
    pub target_count_hint: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotPinnedQueryPlan {
    pub packet: PlannedQueryPacket,
    pub snapshot: SnapshotHandle,
    pub legality: QueryParallelLegality,
    pub profitability: QueryParallelProfitability,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QueryFragmentCounters {
    pub target_count: usize,
    pub entity_records_emitted: usize,
    pub relation_records_emitted: usize,
    pub touched_partitions: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TraversalEntityVisitKey {
    pub depth: u32,
    pub root_seed: EntityId,
    pub via_relation: Option<crate::identity::data::RelationId>,
    pub entity_id: EntityId,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TraversalRelationVisitKey {
    pub depth: u32,
    pub root_seed: EntityId,
    pub relation_id: crate::identity::data::RelationId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TraversalReductionBasis {
    pub entity_visit_keys: Vec<TraversalEntityVisitKey>,
    pub relation_visit_keys: Vec<TraversalRelationVisitKey>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QueryWorkerFragment {
    pub plan_key: DeterministicQueryPlanKey,
    pub fragment_key: DeterministicQueryFragmentKey,
    pub ordering: QueryOrderingContract,
    pub entities: Vec<EntityReadRecord>,
    pub relations: Vec<RelationReadRecord>,
    pub counters: QueryFragmentCounters,
    pub traversal_basis: Option<TraversalReductionBasis>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalQueryResult {
    pub execution_shape: QueryExecutionShape,
    pub ordering: QueryOrderingContract,
    pub entities: Vec<EntityReadRecord>,
    pub relations: Vec<RelationReadRecord>,
    pub reduction_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QueryComplexitySummary {
    pub packet_count: usize,
    pub fragment_count: usize,
    pub touched_partitions: usize,
    pub target_count: usize,
    pub entity_records_emitted: usize,
    pub relation_records_emitted: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QueryExecutionOutcome {
    pub plan: SnapshotPinnedQueryPlan,
    pub result: CanonicalQueryResult,
    pub complexity: QueryComplexitySummary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FallbackParityMode {
    ProductionAdmissibility,
    SampledParity,
    CertificationParity,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IndexQueryRejectionClass {
    MissingGeneration,
    IncompatibleVersion,
    IncompatibleBranch,
    CorruptPayload,
    UnsupportedScope,
    UnsupportedOrderingContract,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum QueryAccessPath {
    AuthoritativeStorage,
    DerivedIndexGeneration {
        generation_id: DerivedIndexGenerationId,
    },
    DerivedIndexRejectedStorageFallback {
        rejection: IndexQueryRejectionClass,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FallbackParityVerifiedQueryOutcome {
    pub execution: QueryExecutionOutcome,
    pub access_path: QueryAccessPath,
    pub parity_mode: FallbackParityMode,
    pub parity_basis_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReadPacketPlan {
    pub label: String,
    pub entity_chunk_indexes: Vec<usize>,
    pub relation_chunk_indexes: Vec<usize>,
    pub target_count: usize,
}

impl PlannedQueryPacket {
    pub fn explicit_targets(
        label: impl Into<String>,
        context_id: QueryPlanContextId,
        targets: Vec<RecordRef>,
    ) -> Self {
        Self::explicit_targets_with_locality(
            label,
            context_id,
            targets,
            QueryLocalityClass::CrossPartitionTraversal,
        )
    }

    pub fn explicit_targets_with_locality(
        label: impl Into<String>,
        context_id: QueryPlanContextId,
        targets: Vec<RecordRef>,
        locality: QueryLocalityClass,
    ) -> Self {
        let label = label.into();
        let execution_shape = QueryExecutionShape::BulkPacketized;
        let reduction = ReductionDiscipline::DeterministicMerge;
        let target_count_hint = targets.len();
        let scope = QueryScope::ExplicitTargets {
            targets: Arc::<[RecordRef]>::from(targets),
        };
        let ordering = QueryOrderingContract::CanonicalRecordRefOrder;
        let fallback = QueryFallbackContract::StorageOnly;
        let plan_key = deterministic_query_plan_key(
            &context_id,
            &label,
            &scope,
            &locality,
            ordering,
            fallback,
            execution_shape,
            reduction,
            target_count_hint,
        );
        Self {
            label,
            context_id,
            scope,
            locality,
            ordering,
            fallback,
            execution_shape,
            reduction,
            plan_key,
            target_count_hint,
        }
    }

    pub fn explicit_target_refs(&self) -> Option<&[RecordRef]> {
        match &self.scope {
            QueryScope::ExplicitTargets { targets } => Some(targets.as_ref()),
            _ => None,
        }
    }

    pub fn requires_serial_reduction(&self) -> bool {
        false
    }
}

fn deterministic_query_plan_key(
    context_id: &QueryPlanContextId,
    label: &str,
    scope: &QueryScope,
    locality: &QueryLocalityClass,
    ordering: QueryOrderingContract,
    fallback: QueryFallbackContract,
    execution_shape: QueryExecutionShape,
    reduction: ReductionDiscipline,
    target_count_hint: usize,
) -> DeterministicQueryPlanKey {
    let bytes = serde_json::to_vec(&(
        context_id,
        label,
        scope,
        locality,
        ordering,
        fallback,
        execution_shape,
        reduction,
        target_count_hint,
    ))
    .expect("query plan key serialization");
    let digest = Sha256::digest(bytes);
    let mut key_bytes = [0u8; 16];
    key_bytes.copy_from_slice(&digest[..16]);
    DeterministicQueryPlanKey(u128::from_be_bytes(key_bytes))
}

pub fn deterministic_query_fragment_key(
    plan_key: DeterministicQueryPlanKey,
    fragment_ordinal: u64,
) -> DeterministicQueryFragmentKey {
    let bytes = serde_json::to_vec(&(plan_key, fragment_ordinal))
        .expect("query fragment key serialization");
    let digest = Sha256::digest(bytes);
    let mut key_bytes = [0u8; 16];
    key_bytes.copy_from_slice(&digest[..16]);
    DeterministicQueryFragmentKey(u128::from_be_bytes(key_bytes))
}

pub fn reduce_query_fragments(
    execution_shape: QueryExecutionShape,
    ordering: QueryOrderingContract,
    mut fragments: Vec<QueryWorkerFragment>,
) -> CanonicalQueryResult {
    fragments.sort_by_key(|fragment| fragment.fragment_key);
    let (entities, relations) = match ordering {
        QueryOrderingContract::CanonicalEntityIdOrder => {
            let mut entities = fragments
                .into_iter()
                .flat_map(|fragment| fragment.entities.into_iter())
                .collect::<Vec<_>>();
            entities.sort_by_key(|record| record.entity_id);
            (entities, Vec::new())
        }
        QueryOrderingContract::CanonicalRelationIdOrder => {
            let mut relations = fragments
                .into_iter()
                .flat_map(|fragment| fragment.relations.into_iter())
                .collect::<Vec<_>>();
            relations.sort_by_key(|record| record.relation_id);
            (Vec::new(), relations)
        }
        QueryOrderingContract::CanonicalRecordRefOrder => {
            let mut entities = Vec::new();
            let mut relations = Vec::new();
            for fragment in fragments {
                entities.extend(fragment.entities);
                relations.extend(fragment.relations);
            }
            entities.sort_by_key(|record| record.entity_id);
            relations.sort_by_key(|record| record.relation_id);
            (entities, relations)
        }
        QueryOrderingContract::CanonicalTraversalOrder => reduce_traversal_fragments(fragments),
    };

    let reduction_digest = certification_digest(&(ordering, &entities, &relations));
    CanonicalQueryResult {
        execution_shape,
        ordering,
        entities,
        relations,
        reduction_digest,
    }
}

fn reduce_traversal_fragments(
    fragments: Vec<QueryWorkerFragment>,
) -> (Vec<EntityReadRecord>, Vec<RelationReadRecord>) {
    let entity_capacity = fragments
        .iter()
        .map(|fragment| fragment.entities.len())
        .sum();
    let relation_capacity = fragments
        .iter()
        .map(|fragment| fragment.relations.len())
        .sum();
    let mut keyed_entities = Vec::with_capacity(entity_capacity);
    let mut keyed_relations = Vec::with_capacity(relation_capacity);
    let mut fallback_entities = Vec::with_capacity(entity_capacity);
    let mut fallback_relations = Vec::with_capacity(relation_capacity);

    for fragment in fragments {
        let QueryWorkerFragment {
            entities,
            relations,
            traversal_basis,
            ..
        } = fragment;
        match traversal_basis {
            Some(traversal_basis) => {
                keyed_entities.extend(traversal_basis.entity_visit_keys.into_iter().zip(entities));
                keyed_relations.extend(
                    traversal_basis
                        .relation_visit_keys
                        .into_iter()
                        .zip(relations),
                );
            }
            None => {
                fallback_entities.extend(entities);
                fallback_relations.extend(relations);
            }
        }
    }

    (
        reduce_traversal_entities(keyed_entities, fallback_entities),
        reduce_traversal_relations(keyed_relations, fallback_relations),
    )
}

fn reduce_traversal_entities(
    mut keyed_entities: Vec<(TraversalEntityVisitKey, EntityReadRecord)>,
    fallback_entities: Vec<EntityReadRecord>,
) -> Vec<EntityReadRecord> {
    if keyed_entities.is_empty() {
        let mut seen = std::collections::BTreeSet::new();
        return fallback_entities
            .into_iter()
            .filter(|record| seen.insert(record.entity_id))
            .collect();
    }

    keyed_entities.sort_by(|left, right| {
        left.0
            .cmp(&right.0)
            .then_with(|| certification_digest(&left.1).cmp(&certification_digest(&right.1)))
    });
    let mut seen = std::collections::BTreeSet::new();
    keyed_entities
        .into_iter()
        .filter_map(|(_, record)| seen.insert(record.entity_id).then_some(record))
        .collect()
}

fn reduce_traversal_relations(
    mut keyed_relations: Vec<(TraversalRelationVisitKey, RelationReadRecord)>,
    fallback_relations: Vec<RelationReadRecord>,
) -> Vec<RelationReadRecord> {
    if keyed_relations.is_empty() {
        let mut seen = std::collections::BTreeSet::new();
        return fallback_relations
            .into_iter()
            .filter(|record| seen.insert(record.relation_id))
            .collect();
    }

    keyed_relations.sort_by(|left, right| {
        left.0
            .cmp(&right.0)
            .then_with(|| certification_digest(&left.1).cmp(&certification_digest(&right.1)))
    });
    let mut seen = std::collections::BTreeSet::new();
    keyed_relations
        .into_iter()
        .filter_map(|(_, record)| seen.insert(record.relation_id).then_some(record))
        .collect()
}

pub(crate) fn certification_digest<T: Serialize>(value: &T) -> String {
    let bytes = serde_json::to_vec(value).expect("query reduction serialization");
    let digest = Sha256::digest(bytes);
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

impl From<PartitionId> for PartitionHint {
    fn from(partition_id: PartitionId) -> Self {
        Self { partition_id }
    }
}

impl QueryScope {
    pub fn canonical_value_scope(values: &[String]) -> Arc<[String]> {
        let mut canonical = values.to_vec();
        canonical.sort();
        canonical.dedup();
        Arc::from(canonical)
    }

    pub fn target_count_hint(&self) -> usize {
        match self {
            Self::ExplicitTargets { targets } => targets.len(),
            Self::EntityKindScan { .. }
            | Self::RelationKindScan { .. }
            | Self::EntityPayloadFieldEquals { .. }
            | Self::RelationPayloadFieldEquals { .. }
            | Self::AspectFilteredEntities { .. }
            | Self::AspectFilteredRelations { .. } => 0,
            Self::EntityPayloadFieldAnyOf { values, .. }
            | Self::RelationPayloadFieldAnyOf { values, .. } => values.len(),
            Self::OutgoingNeighborhood { seeds, .. }
            | Self::IncomingNeighborhood { seeds, .. }
            | Self::ConnectivityTraversal { seeds, .. } => seeds.len(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::payloads::data::RecordPayload;
    use crate::schema::data::DescriptorSemanticsVersion;
    use crate::schema::data::{KindResolution, SchemaId, SchemaVersionId};
    use crate::storage::data::RecordLifecycleState;

    #[test]
    fn explicit_target_helper_builds_planned_packet() {
        let planned = PlannedQueryPacket::explicit_targets(
            "targets",
            QueryPlanContextId {
                runtime_instance_id: 11,
                snapshot_id: SnapshotId(5),
                version_id: VersionId(19),
                schema_version: SchemaVersionId(2),
                descriptor_semantics_version: DescriptorSemanticsVersion(1),
                evidence_basis: QueryPlanEvidenceBasis::CanonicalCommitEnvelope {
                    commit_id: CommitId(13),
                },
            },
            vec![RecordRef::Entity(EntityId::new(PartitionId(7), 3, 1))],
        );

        assert_eq!(planned.label, "targets");
        assert_eq!(
            planned.locality,
            QueryLocalityClass::CrossPartitionTraversal
        );
        assert_eq!(
            planned.ordering,
            QueryOrderingContract::CanonicalRecordRefOrder
        );
        assert_eq!(planned.fallback, QueryFallbackContract::StorageOnly);
        assert_eq!(planned.target_count_hint, 1);
        assert_eq!(
            planned.explicit_target_refs(),
            Some([RecordRef::Entity(EntityId::new(PartitionId(7), 3, 1))].as_slice())
        );
    }

    #[test]
    fn explicit_target_helper_can_bind_single_partition_locality() {
        let planned = PlannedQueryPacket::explicit_targets_with_locality(
            "partitioned",
            QueryPlanContextId {
                runtime_instance_id: 1,
                snapshot_id: SnapshotId(2),
                version_id: VersionId(3),
                schema_version: SchemaVersionId(4),
                descriptor_semantics_version: DescriptorSemanticsVersion(5),
                evidence_basis: QueryPlanEvidenceBasis::CanonicalCommitEnvelope {
                    commit_id: CommitId(21),
                },
            },
            vec![],
            QueryLocalityClass::SinglePartition {
                partition_id: PartitionId(42),
            },
        );

        assert_eq!(
            planned.locality,
            QueryLocalityClass::SinglePartition {
                partition_id: PartitionId(42)
            }
        );
    }

    #[test]
    fn explicit_target_helper_generates_non_zero_deterministic_plan_key() {
        let context = QueryPlanContextId {
            runtime_instance_id: 11,
            snapshot_id: SnapshotId(7),
            version_id: VersionId(9),
            schema_version: SchemaVersionId(2),
            descriptor_semantics_version: DescriptorSemanticsVersion(1),
            evidence_basis: QueryPlanEvidenceBasis::CanonicalCommitEnvelope {
                commit_id: CommitId(34),
            },
        };

        let first = PlannedQueryPacket::explicit_targets(
            "legacy",
            context.clone(),
            vec![RecordRef::Entity(EntityId::new(PartitionId(7), 3, 1))],
        );
        let second = PlannedQueryPacket::explicit_targets(
            "legacy",
            context,
            vec![RecordRef::Entity(EntityId::new(PartitionId(7), 3, 1))],
        );

        assert_ne!(first.plan_key, DeterministicQueryPlanKey(0));
        assert_eq!(first.plan_key, second.plan_key);
    }

    #[test]
    fn traversal_reduction_uses_visit_keys_for_cross_fragment_determinism() {
        let seed = EntityId::new(PartitionId(1), 1, 1);
        let mid = EntityId::new(PartitionId(1), 2, 1);
        let leaf = EntityId::new(PartitionId(1), 3, 1);
        let relation = crate::identity::data::RelationId::new(PartitionId(1), 1, 1);
        let kind = KindResolution {
            kind_id: KindId(1),
            kind_name: "test.entity".to_string(),
            schema_id: SchemaId("test".to_string()),
            schema_version_id: SchemaVersionId(1),
        };
        let relation_kind = KindResolution {
            kind_id: KindId(2),
            kind_name: "test.relation".to_string(),
            schema_id: SchemaId("test".to_string()),
            schema_version_id: SchemaVersionId(1),
        };

        let reduced = reduce_query_fragments(
            QueryExecutionShape::BulkPacketized,
            QueryOrderingContract::CanonicalTraversalOrder,
            vec![
                QueryWorkerFragment {
                    plan_key: DeterministicQueryPlanKey(1),
                    fragment_key: DeterministicQueryFragmentKey(2),
                    ordering: QueryOrderingContract::CanonicalTraversalOrder,
                    entities: vec![
                        EntityReadRecord {
                            entity_id: mid,
                            lineage_id: None,
                            kind: kind.clone(),
                            lifecycle: RecordLifecycleState::Live,
                            created_at_version: VersionId(1),
                            retired_at_version: None,
                            payload: RecordPayload::StructuredJson(
                                serde_json::json!({"name":"mid"}),
                            ),
                        },
                        EntityReadRecord {
                            entity_id: leaf,
                            lineage_id: None,
                            kind: kind.clone(),
                            lifecycle: RecordLifecycleState::Live,
                            created_at_version: VersionId(1),
                            retired_at_version: None,
                            payload: RecordPayload::StructuredJson(
                                serde_json::json!({"name":"leaf"}),
                            ),
                        },
                    ],
                    relations: vec![RelationReadRecord {
                        relation_id: relation,
                        kind: relation_kind,
                        lifecycle: RecordLifecycleState::Live,
                        created_at_version: VersionId(1),
                        retired_at_version: None,
                        source: seed,
                        target: mid,
                        payload: Some(RecordPayload::StructuredJson(
                            serde_json::json!({"label":"edge"}),
                        )),
                    }],
                    counters: QueryFragmentCounters {
                        target_count: 1,
                        entity_records_emitted: 2,
                        relation_records_emitted: 1,
                        touched_partitions: 1,
                    },
                    traversal_basis: Some(TraversalReductionBasis {
                        entity_visit_keys: vec![
                            TraversalEntityVisitKey {
                                depth: 1,
                                root_seed: seed,
                                via_relation: Some(relation),
                                entity_id: mid,
                            },
                            TraversalEntityVisitKey {
                                depth: 2,
                                root_seed: seed,
                                via_relation: Some(relation),
                                entity_id: leaf,
                            },
                        ],
                        relation_visit_keys: vec![TraversalRelationVisitKey {
                            depth: 0,
                            root_seed: seed,
                            relation_id: relation,
                        }],
                    }),
                },
                QueryWorkerFragment {
                    plan_key: DeterministicQueryPlanKey(1),
                    fragment_key: DeterministicQueryFragmentKey(1),
                    ordering: QueryOrderingContract::CanonicalTraversalOrder,
                    entities: vec![
                        EntityReadRecord {
                            entity_id: seed,
                            lineage_id: None,
                            kind,
                            lifecycle: RecordLifecycleState::Live,
                            created_at_version: VersionId(1),
                            retired_at_version: None,
                            payload: RecordPayload::StructuredJson(
                                serde_json::json!({"name":"seed"}),
                            ),
                        },
                        EntityReadRecord {
                            entity_id: mid,
                            lineage_id: None,
                            kind: KindResolution {
                                kind_id: KindId(1),
                                kind_name: "test.entity".to_string(),
                                schema_id: SchemaId("test".to_string()),
                                schema_version_id: SchemaVersionId(1),
                            },
                            lifecycle: RecordLifecycleState::Live,
                            created_at_version: VersionId(1),
                            retired_at_version: None,
                            payload: RecordPayload::StructuredJson(
                                serde_json::json!({"name":"mid-duplicate"}),
                            ),
                        },
                    ],
                    relations: vec![],
                    counters: QueryFragmentCounters {
                        target_count: 1,
                        entity_records_emitted: 2,
                        relation_records_emitted: 0,
                        touched_partitions: 1,
                    },
                    traversal_basis: Some(TraversalReductionBasis {
                        entity_visit_keys: vec![
                            TraversalEntityVisitKey {
                                depth: 0,
                                root_seed: seed,
                                via_relation: None,
                                entity_id: seed,
                            },
                            TraversalEntityVisitKey {
                                depth: 1,
                                root_seed: seed,
                                via_relation: Some(relation),
                                entity_id: mid,
                            },
                        ],
                        relation_visit_keys: vec![],
                    }),
                },
            ],
        );

        assert_eq!(
            reduced
                .entities
                .iter()
                .map(|record| record.entity_id)
                .collect::<Vec<_>>(),
            vec![seed, mid, leaf]
        );
        assert_eq!(
            reduced
                .relations
                .iter()
                .map(|record| record.relation_id)
                .collect::<Vec<_>>(),
            vec![relation]
        );
        assert_eq!(
            reduced.entities[1].payload,
            RecordPayload::StructuredJson(serde_json::json!({"name":"mid"}))
        );
    }
}

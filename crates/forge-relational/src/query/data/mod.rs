use std::sync::Arc;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::history::data::CommitId;
use crate::history::data::AspectFilter;
use crate::indexes::data::DerivedIndexGenerationId;
use crate::identity::data::{EntityId, KindId, PartitionId, VersionId};
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
    CanonicalCommitEnvelope {
        commit_id: CommitId,
    },
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
    SerialPreferred {
        reason: QuerySerialReason,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum QueryLocalityClass {
    SinglePartition {
        partition_id: PartitionId,
    },
    PartitionBounded {
        partitions: Arc<[PartitionId]>,
    },
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QueryWorkerFragment {
    pub plan_key: DeterministicQueryPlanKey,
    pub fragment_key: DeterministicQueryFragmentKey,
    pub ordering: QueryOrderingContract,
    pub entities: Vec<EntityReadRecord>,
    pub relations: Vec<RelationReadRecord>,
    pub counters: QueryFragmentCounters,
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
pub struct QueryWorkPacket {
    pub label: String,
    pub partition_hint: Option<PartitionHint>,
    pub execution_shape: QueryExecutionShape,
    pub reduction: ReductionDiscipline,
    pub targets: Vec<RecordRef>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReadPacketPlan {
    pub label: String,
    pub entity_chunk_indexes: Vec<usize>,
    pub relation_chunk_indexes: Vec<usize>,
    pub target_count: usize,
}

impl QueryWorkPacket {
    pub fn bulk(label: impl Into<String>, targets: Vec<RecordRef>) -> Self {
        Self {
            label: label.into(),
            partition_hint: None,
            execution_shape: QueryExecutionShape::BulkPacketized,
            reduction: ReductionDiscipline::DeterministicMerge,
            targets,
        }
    }

    pub fn planned_with_context(self, context_id: QueryPlanContextId) -> PlannedQueryPacket {
        PlannedQueryPacket::from_legacy_packet(self, context_id)
    }
}

impl PlannedQueryPacket {
    pub fn from_legacy_packet(packet: QueryWorkPacket, context_id: QueryPlanContextId) -> Self {
        let QueryWorkPacket {
            label,
            partition_hint,
            execution_shape,
            reduction,
            targets,
        } = packet;
        let locality = match partition_hint {
            Some(PartitionHint { partition_id }) => QueryLocalityClass::SinglePartition {
                partition_id,
            },
            None => QueryLocalityClass::CrossPartitionTraversal,
        };
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

    pub fn explicit_targets(&self) -> Option<&[RecordRef]> {
        match &self.scope {
            QueryScope::ExplicitTargets { targets } => Some(targets.as_ref()),
            _ => None,
        }
    }

    pub fn requires_serial_reduction(&self) -> bool {
        matches!(
            self.scope,
            QueryScope::OutgoingNeighborhood { .. }
                | QueryScope::IncomingNeighborhood { .. }
                | QueryScope::ConnectivityTraversal { .. }
        )
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
    let mut entities = Vec::new();
    let mut relations = Vec::new();
    for fragment in fragments {
        entities.extend(fragment.entities);
        relations.extend(fragment.relations);
    }

    match ordering {
        QueryOrderingContract::CanonicalEntityIdOrder => {
            entities.sort_by_key(|record| record.entity_id);
        }
        QueryOrderingContract::CanonicalRelationIdOrder => {
            relations.sort_by_key(|record| record.relation_id);
        }
        QueryOrderingContract::CanonicalRecordRefOrder => {
            entities.sort_by_key(|record| record.entity_id);
            relations.sort_by_key(|record| record.relation_id);
        }
        QueryOrderingContract::CanonicalTraversalOrder => {}
    }

    let reduction_digest = certification_digest(&(ordering, &entities, &relations));
    CanonicalQueryResult {
        execution_shape,
        ordering,
        entities,
        relations,
        reduction_digest,
    }
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
    pub fn target_count_hint(&self) -> usize {
        match self {
            Self::ExplicitTargets { targets } => targets.len(),
            Self::EntityKindScan { .. }
            | Self::RelationKindScan { .. }
            | Self::EntityPayloadFieldEquals { .. }
            | Self::AspectFilteredEntities { .. }
            | Self::AspectFilteredRelations { .. } => 0,
            Self::OutgoingNeighborhood { seeds, .. }
            | Self::IncomingNeighborhood { seeds, .. }
            | Self::ConnectivityTraversal { seeds, .. } => seeds.len(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::data::DescriptorSemanticsVersion;

    #[test]
    fn legacy_query_packet_converts_to_planned_explicit_targets_packet() {
        let legacy = QueryWorkPacket::bulk(
            "targets",
            vec![RecordRef::Entity(EntityId::new(PartitionId(7), 3, 1))],
        );
        let planned = legacy.planned_with_context(QueryPlanContextId {
            runtime_instance_id: 11,
            snapshot_id: SnapshotId(5),
            version_id: VersionId(19),
            schema_version: SchemaVersionId(2),
            descriptor_semantics_version: DescriptorSemanticsVersion(1),
            evidence_basis: QueryPlanEvidenceBasis::CanonicalCommitEnvelope {
                commit_id: CommitId(13),
            },
        });

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
            planned.explicit_targets(),
            Some([RecordRef::Entity(EntityId::new(PartitionId(7), 3, 1))].as_slice())
        );
    }

    #[test]
    fn legacy_query_packet_partition_hint_converts_to_single_partition_locality() {
        let legacy = QueryWorkPacket {
            label: "partitioned".to_string(),
            partition_hint: Some(PartitionHint::from(PartitionId(42))),
            execution_shape: QueryExecutionShape::BulkPacketized,
            reduction: ReductionDiscipline::DeterministicMerge,
            targets: vec![],
        };

        let planned = PlannedQueryPacket::from_legacy_packet(
            legacy,
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
        );

        assert_eq!(
            planned.locality,
            QueryLocalityClass::SinglePartition {
                partition_id: PartitionId(42)
            }
        );
    }

    #[test]
    fn legacy_query_packet_conversion_generates_non_zero_deterministic_plan_key() {
        let packet = QueryWorkPacket::bulk(
            "legacy",
            vec![RecordRef::Entity(EntityId::new(PartitionId(7), 3, 1))],
        );
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

        let first = PlannedQueryPacket::from_legacy_packet(packet.clone(), context.clone());
        let second = PlannedQueryPacket::from_legacy_packet(packet, context);

        assert_ne!(first.plan_key, DeterministicQueryPlanKey(0));
        assert_eq!(first.plan_key, second.plan_key);
    }
}

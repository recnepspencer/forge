use std::sync::Arc;

use serde::{Deserialize, Serialize};

use forge_foundational::facade::{AspectValue, FieldKey};

use crate::history::data::{AspectFilter, CommitId};
use crate::identity::data::{EntityId, KindId, PartitionId, VersionId};
use crate::schema::data::{DescriptorSemanticsVersion, SchemaVersionId};
use crate::snapshots::data::{SnapshotHandle, SnapshotId};
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
    EntityFieldEquals {
        field: FieldKey,
        value: AspectValue,
        partition_scope: Option<Arc<[PartitionId]>>,
    },
    EntityFieldAnyOf {
        field: FieldKey,
        values: Arc<[AspectValue]>,
        partition_scope: Option<Arc<[PartitionId]>>,
    },
    RelationFieldEquals {
        field: FieldKey,
        value: AspectValue,
        partition_scope: Option<Arc<[PartitionId]>>,
    },
    RelationFieldAnyOf {
        field: FieldKey,
        values: Arc<[AspectValue]>,
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
    super::canonical_digest::deterministic_query_plan_key_from_canonical_bytes(
        context_id,
        label,
        scope,
        locality,
        ordering,
        fallback,
        execution_shape,
        reduction,
        target_count_hint,
    )
}

pub fn deterministic_query_fragment_key(
    plan_key: DeterministicQueryPlanKey,
    fragment_ordinal: u64,
) -> DeterministicQueryFragmentKey {
    super::canonical_digest::deterministic_query_fragment_key_from_canonical_bytes(
        plan_key,
        fragment_ordinal,
    )
}

impl From<PartitionId> for PartitionHint {
    fn from(partition_id: PartitionId) -> Self {
        Self { partition_id }
    }
}

impl QueryScope {
    pub fn canonical_value_scope(values: &[AspectValue]) -> Arc<[AspectValue]> {
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
            | Self::EntityFieldEquals { .. }
            | Self::RelationFieldEquals { .. }
            | Self::AspectFilteredEntities { .. }
            | Self::AspectFilteredRelations { .. } => 0,
            Self::EntityFieldAnyOf { values, .. } | Self::RelationFieldAnyOf { values, .. } => {
                values.len()
            }
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
    use crate::schema::data::SchemaVersionId;
    use forge_foundational::facade::InternedString;

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
    fn field_predicate_plan_keys_preserve_aspect_value_family() {
        let context = QueryPlanContextId {
            runtime_instance_id: 11,
            snapshot_id: SnapshotId(7),
            version_id: VersionId(9),
            schema_version: SchemaVersionId(2),
            descriptor_semantics_version: DescriptorSemanticsVersion(1),
            evidence_basis: QueryPlanEvidenceBasis::GenesisRuntimeBootstrap,
        };
        let field = FieldKey::new("count").expect("valid field key");

        let int_plan_key = field_equals_plan_key(&context, field.clone(), AspectValue::Int64(1));
        let string_plan_key = field_equals_plan_key(
            &context,
            field,
            AspectValue::String(InternedString::Raw("1".to_string())),
        );

        assert_ne!(int_plan_key, string_plan_key);
    }

    fn field_equals_plan_key(
        context: &QueryPlanContextId,
        field: FieldKey,
        value: AspectValue,
    ) -> DeterministicQueryPlanKey {
        deterministic_query_plan_key(
            context,
            "field",
            &QueryScope::EntityFieldEquals {
                field,
                value,
                partition_scope: None,
            },
            &QueryLocalityClass::CrossPartitionTraversal,
            QueryOrderingContract::CanonicalEntityIdOrder,
            QueryFallbackContract::StorageOnly,
            QueryExecutionShape::BulkPacketized,
            ReductionDiscipline::DeterministicMerge,
            0,
        )
    }
}

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::config::data::CascadeDeletePolicy;
use crate::identity::data::{EntityId, KindId, PartitionId, VersionId};
use crate::schema::data::{
    ContractId, EndpointDeletionIntegrityMode, SymmetryMode, UniquenessScope,
};

use super::execution::InvariantClass;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InvariantViolationFields {
    None,
    MergedIntentLimit {
        merged_intent_count: usize,
        limit: usize,
    },
    SnapshotEntityLimit {
        version_id: VersionId,
        visible_entities: usize,
        limit: usize,
    },
    UniqueEntityPayloadField {
        field: String,
        value: String,
    },
    SidecarConsistency {
        partition_id: PartitionId,
        slot: usize,
        missing_label: String,
    },
    RelationEndpointKindMismatch {
        contract_id: ContractId,
        relation_kind_id: KindId,
        source: EntityId,
        target: EntityId,
        source_kind_id: KindId,
        target_kind_id: KindId,
        boundary: RelationEndpointBoundary,
    },
    RelationEndpointKindSelfEdge {
        contract_id: ContractId,
        relation_kind_id: KindId,
        source: EntityId,
        target: EntityId,
        self_edge: bool,
    },
    RelationEndpointKindCrossContext {
        contract_id: ContractId,
        relation_kind_id: KindId,
        source_partition_id: PartitionId,
        target_partition_id: PartitionId,
    },
    RelationCardinalityEndpoint {
        contract_id: ContractId,
        relation_kind_id: KindId,
        entity_id: EntityId,
        boundary: RelationCardinalityBoundary,
        count: usize,
        limit: u64,
    },
    RelationCardinalityPair {
        contract_id: ContractId,
        relation_kind_id: KindId,
        source: EntityId,
        target: EntityId,
        count: usize,
        limit: u64,
    },
    RelationUniqueness {
        contract_id: ContractId,
        relation_kind_id: KindId,
        scope: UniquenessScope,
        source: EntityId,
        target: EntityId,
        count: usize,
    },
    RelationSymmetry {
        contract_id: ContractId,
        relation_kind_id: KindId,
        source: EntityId,
        target: EntityId,
        mode: SymmetryMode,
    },
    RelationEndpointDeletionIntegrity {
        contract_id: ContractId,
        relation_kind_id: KindId,
        entity_id: EntityId,
        remaining_relation_endpoint_count: usize,
        mode: EndpointDeletionIntegrityMode,
        cascade_delete_policy: Option<CascadeDeletePolicy>,
    },
    StorageInconsistency {
        entity_id: Option<EntityId>,
        partition_id: Option<PartitionId>,
        slot: Option<usize>,
        missing_label: Option<String>,
        scan: Option<String>,
        lookup: Option<String>,
        failure: Option<String>,
    },
    RelationIntegrityScopeBudgetExceeded {
        limit_name: String,
        limit: usize,
        observed: usize,
        relation_kind_count: usize,
        touched_entity_count: usize,
        deleted_entity_count: usize,
        scanned_relation_count: usize,
        planned_edge_count: usize,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelationEndpointBoundary {
    Source,
    Target,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelationCardinalityBoundary {
    Source,
    Target,
    Pair,
}

impl InvariantViolationFields {
    pub fn to_json_value(&self) -> Value {
        match self {
            Self::None => json!({}),
            Self::MergedIntentLimit {
                merged_intent_count,
                limit,
            } => json!({
                "merged_intent_count": merged_intent_count,
                "limit": limit,
            }),
            Self::SnapshotEntityLimit {
                version_id,
                visible_entities,
                limit,
            } => json!({
                "version_id": version_id.0,
                "visible_entities": visible_entities,
                "limit": limit,
            }),
            Self::UniqueEntityPayloadField { field, value } => json!({
                "field": field,
                "value": value,
            }),
            Self::SidecarConsistency {
                partition_id,
                slot,
                missing_label,
            } => json!({
                "partition_id": partition_id.0,
                "slot": slot,
                "missing_label": missing_label,
            }),
            Self::RelationEndpointKindMismatch {
                contract_id,
                relation_kind_id,
                source,
                target,
                source_kind_id,
                target_kind_id,
                boundary: _,
            } => json!({
                "contract_id": contract_id,
                "relation_kind_id": relation_kind_id.0,
                "source": source,
                "target": target,
                "source_kind_id": source_kind_id.0,
                "target_kind_id": target_kind_id.0,
            }),
            Self::RelationEndpointKindSelfEdge {
                contract_id,
                relation_kind_id,
                source,
                target,
                self_edge,
            } => json!({
                "contract_id": contract_id,
                "relation_kind_id": relation_kind_id.0,
                "source": source,
                "target": target,
                "self_edge": self_edge,
            }),
            Self::RelationEndpointKindCrossContext {
                contract_id,
                relation_kind_id,
                source_partition_id,
                target_partition_id,
            } => json!({
                "contract_id": contract_id,
                "relation_kind_id": relation_kind_id.0,
                "source_partition_id": source_partition_id.0,
                "target_partition_id": target_partition_id.0,
            }),
            Self::RelationCardinalityEndpoint {
                contract_id,
                relation_kind_id,
                entity_id,
                boundary,
                count,
                limit,
            } => json!({
                "contract_id": contract_id,
                "relation_kind_id": relation_kind_id.0,
                "entity_id": entity_id,
                "boundary": match boundary {
                    RelationCardinalityBoundary::Source => "source",
                    RelationCardinalityBoundary::Target => "target",
                    RelationCardinalityBoundary::Pair => "pair",
                },
                "count": count,
                "limit": limit,
            }),
            Self::RelationCardinalityPair {
                contract_id,
                relation_kind_id,
                source,
                target,
                count,
                limit,
            } => json!({
                "contract_id": contract_id,
                "relation_kind_id": relation_kind_id.0,
                "source": source,
                "target": target,
                "count": count,
                "limit": limit,
            }),
            Self::RelationUniqueness {
                contract_id,
                relation_kind_id,
                scope,
                source,
                target,
                count,
            } => json!({
                "contract_id": contract_id,
                "relation_kind_id": relation_kind_id.0,
                "scope": match scope {
                    UniquenessScope::DirectedSemanticEdge => "directed",
                    UniquenessScope::NormalizedSymmetricEdge => "normalized",
                },
                "source": source,
                "target": target,
                "count": count,
            }),
            Self::RelationSymmetry {
                contract_id,
                relation_kind_id,
                source,
                target,
                mode,
            } => json!({
                "contract_id": contract_id,
                "relation_kind_id": relation_kind_id.0,
                "source": source,
                "target": target,
                "mode": match mode {
                    SymmetryMode::CanonicalUndirected => "canonical_undirected",
                    SymmetryMode::PairedInverseRequired | SymmetryMode::PairedTwinRequired => "paired",
                    SymmetryMode::InverseProhibited => "inverse_prohibited",
                },
            }),
            Self::RelationEndpointDeletionIntegrity {
                contract_id,
                relation_kind_id,
                entity_id,
                remaining_relation_endpoint_count,
                mode,
                cascade_delete_policy,
            } => {
                let mut value = json!({
                    "contract_id": contract_id,
                    "relation_kind_id": relation_kind_id.0,
                    "entity_id": entity_id,
                    "remaining_relation_endpoint_count": remaining_relation_endpoint_count,
                    "mode": match mode {
                        EndpointDeletionIntegrityMode::RejectDeleteWithLiveRelations => "reject_delete_with_live_relations",
                        EndpointDeletionIntegrityMode::RequireRelationDeletionInSameCommit => "require_relation_deletion_in_same_commit",
                        EndpointDeletionIntegrityMode::RequireRelationRetirement => "require_relation_retirement",
                    },
                });
                if let Some(cascade_delete_policy) = cascade_delete_policy {
                    value["cascade_delete_policy"] = json!(match cascade_delete_policy {
                        CascadeDeletePolicy::CascadeDeleteRelations => "cascade_delete_relations",
                        CascadeDeletePolicy::RetainDanglingForAudit => "retain_dangling_for_audit",
                    });
                }
                value
            }
            Self::StorageInconsistency {
                entity_id,
                partition_id,
                slot,
                missing_label,
                scan,
                lookup,
                failure,
            } => {
                let mut value = json!({});
                if let Some(entity_id) = entity_id {
                    value["entity_id"] = json!(entity_id);
                }
                if let Some(partition_id) = partition_id {
                    value["partition_id"] = json!(partition_id.0);
                }
                if let Some(slot) = slot {
                    value["slot"] = json!(slot);
                }
                if let Some(missing_label) = missing_label {
                    value["missing_label"] = json!(missing_label);
                }
                if let Some(scan) = scan {
                    value["scan"] = json!(scan);
                }
                if let Some(lookup) = lookup {
                    value["lookup"] = json!(lookup);
                }
                if let Some(failure) = failure {
                    value["failure"] = json!(failure);
                }
                value
            }
            Self::RelationIntegrityScopeBudgetExceeded {
                limit_name,
                limit,
                observed,
                relation_kind_count,
                touched_entity_count,
                deleted_entity_count,
                scanned_relation_count,
                planned_edge_count,
            } => json!({
                "limit_name": limit_name,
                "limit": limit,
                "observed": observed,
                "relation_kind_count": relation_kind_count,
                "touched_entity_count": touched_entity_count,
                "deleted_entity_count": deleted_entity_count,
                "scanned_relation_count": scanned_relation_count,
                "planned_edge_count": planned_edge_count,
            }),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvariantViolation {
    pub class: InvariantClass,
    pub code: crate::diagnostics::data::DiagnosticCode,
    pub detail: String,
    pub fields: InvariantViolationFields,
}

impl InvariantViolation {
    pub fn fields_json(&self) -> Value {
        self.fields.to_json_value()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InvariantAdvisory {
    AuditOnly,
}

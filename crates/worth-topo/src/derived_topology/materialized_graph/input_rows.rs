use forge_relational::facade::identity::EntityId;
use forge_relational::facade::runtime::{EntityReadRecord, RelationReadRecord};
use schema::facade::platform::entities::EntityKind;
use schema::facade::platform::relations::RelationKind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MaterializationEntityRow {
    pub entity_id: EntityId,
    pub kind: EntityKind,
    pub label: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MaterializationRelationRow {
    pub kind: RelationKind,
    pub source: EntityId,
    pub target: EntityId,
}

impl MaterializationEntityRow {
    pub(crate) fn from_truth_record(record: &EntityReadRecord) -> Option<Self> {
        let kind = EntityKind::from_kind_id(record.kind.kind_id)?;
        Some(Self {
            entity_id: record.entity_id,
            kind,
            label: crate::derived_topology::materialized_graph::entity_labels::entity_label(record),
        })
    }
}

impl MaterializationRelationRow {
    pub(crate) fn from_truth_record(record: &RelationReadRecord) -> Option<Self> {
        let kind = RelationKind::from_kind_id(record.kind.kind_id)?;
        Some(Self {
            kind,
            source: record.source,
            target: record.target,
        })
    }
}

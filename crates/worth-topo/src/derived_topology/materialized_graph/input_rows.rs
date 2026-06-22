use forge_query::facade::ForgeQueryEntity;
use forge_relational::facade::identity::EntityId;
#[cfg(test)]
use forge_relational::facade::runtime::{EntityReadRecord, RelationReadRecord};
use schema::facade::platform::entities::EntityKind;
use schema::facade::platform::relations::RelationKind;

use crate::derived_topology::materialized_graph::query_input_decode::{
    entity_id_from_query_identity, parse_entity_identity, parse_entity_kind, parse_relation_kind,
    required_text,
};
use crate::derived_topology::materialized_graph::TopologyMaterializationError;

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
    #[cfg(test)]
    pub(crate) fn from_truth_record(record: &EntityReadRecord) -> Option<Self> {
        let kind = EntityKind::from_kind_id(record.kind.kind_id)?;
        Some(Self {
            entity_id: record.entity_id,
            kind,
            label: crate::derived_topology::materialized_graph::entity_labels::entity_label(record),
        })
    }

    pub(crate) fn from_query_row(
        row: &ForgeQueryEntity,
    ) -> Result<Self, TopologyMaterializationError> {
        let external_row = row.external_row();
        let entity_id = entity_id_from_query_identity(row.identity())?;
        let kind = parse_entity_kind(required_text(external_row, "topology.kind")?)?;
        let label = external_row
            .get("topology")
            .and_then(|value| value.get("structure"))
            .and_then(serde_json::Value::as_str)
            .or_else(|| {
                external_row
                    .get("naming")
                    .and_then(|value| value.get("persistent_name"))
                    .and_then(serde_json::Value::as_str)
            })
            .unwrap_or_else(|| kind.kind_name())
            .to_string();
        Ok(Self {
            entity_id,
            kind,
            label,
        })
    }
}

impl MaterializationRelationRow {
    #[cfg(test)]
    pub(crate) fn from_truth_record(record: &RelationReadRecord) -> Option<Self> {
        let kind = RelationKind::from_kind_id(record.kind.kind_id)?;
        Some(Self {
            kind,
            source: record.source,
            target: record.target,
        })
    }

    pub(crate) fn from_query_row(
        row: &ForgeQueryEntity,
    ) -> Result<Self, TopologyMaterializationError> {
        Ok(Self {
            kind: parse_relation_kind(required_text(row.external_row(), "topology.kind")?)?,
            source: parse_entity_identity(required_text(
                row.external_row(),
                "topology.source_identity",
            )?)?,
            target: parse_entity_identity(required_text(
                row.external_row(),
                "topology.target_identity",
            )?)?,
        })
    }
}

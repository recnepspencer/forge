use forge_query::facade::ForgeQueryEntity;
use forge_relational::facade::identity::EntityId;
use forge_relational::facade::runtime::{EntityReadRecord, RelationReadRecord};
use worth_schema::facade::{WorthEntityKind, WorthRelationKind};

use crate::materialization::WorthTopologyMaterializationError;
use crate::query::{parse_entity_identity, parse_entity_kind, parse_relation_kind, required_text};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MaterializationEntityRow {
    pub entity_id: EntityId,
    pub kind: WorthEntityKind,
    pub label: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MaterializationRelationRow {
    pub kind: WorthRelationKind,
    pub source: EntityId,
    pub target: EntityId,
}

impl MaterializationEntityRow {
    pub(crate) fn from_truth_record(record: &EntityReadRecord) -> Option<Self> {
        let kind = WorthEntityKind::from_kind_id(record.kind.kind_id)?;
        Some(Self {
            entity_id: record.entity_id,
            kind,
            label: crate::materialization::entity_labels::entity_label(record),
        })
    }

    pub(crate) fn from_query_row(
        row: &ForgeQueryEntity,
    ) -> Result<Self, WorthTopologyMaterializationError> {
        let entity_id = parse_entity_identity(&row.identity)?;
        let kind = parse_entity_kind(required_text(&row.payload, "topology.kind")?)?;
        let label = row
            .payload
            .get("topology")
            .and_then(|value| value.get("structure"))
            .and_then(serde_json::Value::as_str)
            .or_else(|| {
                row.payload
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
    pub(crate) fn from_truth_record(record: &RelationReadRecord) -> Option<Self> {
        let kind = WorthRelationKind::from_kind_id(record.kind.kind_id)?;
        Some(Self {
            kind,
            source: record.source,
            target: record.target,
        })
    }

    pub(crate) fn from_query_row(
        row: &ForgeQueryEntity,
    ) -> Result<Self, WorthTopologyMaterializationError> {
        Ok(Self {
            kind: parse_relation_kind(required_text(&row.payload, "topology.kind")?)?,
            source: parse_entity_identity(required_text(
                &row.payload,
                "topology.source_identity",
            )?)?,
            target: parse_entity_identity(required_text(
                &row.payload,
                "topology.target_identity",
            )?)?,
        })
    }
}

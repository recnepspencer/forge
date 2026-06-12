//! Query-native naming truth helpers for  topology.
//!
//! This module keeps persistent-name truth as a first-class query surface so
//! query-native certification and inspection can reason about attachment and
//! orphan posture without reopening relational truth out-of-band.

use std::collections::{BTreeMap, BTreeSet};

use forge_query::facade::{
    ForgeQueryEntity, ForgeQueryEntityIdentity, ForgeQueryLiveView, ForgeQueryLiveViewBuilder,
    ForgeQueryRuntimeError, ForgeQueryWorkspace, ForgeQueryWorkspaceLiveViewDeclaration,
};
use forge_relational::facade::identity::EntityId;
use forge_runtime_bridge::facade::RelationalBridgeRecordIdentityKind;
use schema::facade::platform::entities::{EntityKind, NamingEntityKind, TopologyEntityKind};
use schema::facade::{QueryAspectPath, QueryCollection, QueryLiveField, QuerySchemaBasis};
use serde::{Deserialize, Serialize};

use crate::projection::{entity_id_from_query_identity, required_text};

use super::TopologyQuerySurfaceError;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NamingAttachmentRow {
    pub topology_entity_id: EntityId,
    pub topology_kind_name: String,
    pub attached_persistent_name_ids: Vec<EntityId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NamingAttachmentReport {
    pub fully_named: bool,
    pub orphan_persistent_name_ids: Vec<EntityId>,
    pub attachments: Vec<NamingAttachmentRow>,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct TopologyNamingAttachmentInput<'a> {
    entity_rows: &'a [ForgeQueryEntity],
    persistent_name_rows: &'a [ForgeQueryEntity],
}

impl<'a> TopologyNamingAttachmentInput<'a> {
    pub(crate) fn new(
        entity_rows: &'a [ForgeQueryEntity],
        persistent_name_rows: &'a [ForgeQueryEntity],
    ) -> Self {
        Self {
            entity_rows,
            persistent_name_rows,
        }
    }

    fn entity_rows(&self) -> &'a [ForgeQueryEntity] {
        self.entity_rows
    }

    fn persistent_name_rows(&self) -> &'a [ForgeQueryEntity] {
        self.persistent_name_rows
    }
}

pub(crate) fn persistent_name_live_view_declaration(
    surface_name: impl Into<String>,
) -> Result<ForgeQueryWorkspaceLiveViewDeclaration, ForgeQueryRuntimeError> {
    ForgeQueryLiveViewBuilder::surface(surface_name)
        .select([
            QueryLiveField::IdentityId.delivered_name(),
            QueryLiveField::LineageProvenance.delivered_name(),
            QueryAspectPath::NAMING_PERSISTENT_NAME.as_str(),
            QueryLiveField::NamingTargetIdentity.delivered_name(),
        ])
        .order_by(QueryLiveField::IdentityId.delivered_name())
        .from(QueryCollection::PersistentName.as_str())
        .schema_basis(QuerySchemaBasis::PersistentNameLiveView.as_str())
        .build()
}

pub(crate) fn declare_persistent_name_live_view<T>(
    workspace: &mut ForgeQueryWorkspace,
    surface_name: impl Into<String>,
) -> Result<ForgeQueryLiveView<T>, ForgeQueryRuntimeError> {
    let surface_name = surface_name.into();
    let declaration = persistent_name_live_view_declaration(surface_name.clone())?;
    let request = declaration.request().clone();
    let schema_view = declaration.schema_view().clone();
    workspace.live_view_request(surface_name, request, schema_view)
}

pub(crate) fn naming_attachment_report_from_query_input(
    input: TopologyNamingAttachmentInput<'_>,
) -> Result<NamingAttachmentReport, TopologyQuerySurfaceError> {
    let topology_kind_names: BTreeSet<_> = TopologyEntityKind::WRAPPED_ALL
        .into_iter()
        .map(|kind| kind.kind_name())
        .collect();

    let mut topology_entities = Vec::new();
    let mut topology_identities = BTreeMap::new();
    for row in input.entity_rows() {
        let external_row = row.external_row();
        let entity_id = entity_id_from_query_identity(row.identity())
            .map_err(|error| TopologyQuerySurfaceError::new(error.to_string()))?;
        let kind_name = required_text(external_row, "topology.kind")
            .map_err(|error| TopologyQuerySurfaceError::new(error.to_string()))?;
        let entity_kind = external_row
            .get("topology")
            .and_then(|value| value.get("kind"))
            .and_then(serde_json::Value::as_str)
            .unwrap_or(kind_name);
        if !topology_kind_names.contains(entity_kind) {
            continue;
        }
        topology_identities.insert(query_identity_label(row.identity())?, entity_id);
        topology_entities.push((entity_id, kind_name.to_string()));
    }

    let mut attachments = BTreeMap::<_, Vec<_>>::new();
    let persistent_name_kind = EntityKind::Naming(NamingEntityKind::PersistentName);
    let mut orphan_persistent_name_ids = Vec::new();
    for row in input.persistent_name_rows() {
        let external_row = row.external_row();
        let persistent_name_id = entity_id_from_query_identity(row.identity())
            .map_err(|error| TopologyQuerySurfaceError::new(error.to_string()))?;
        let kind_name = required_text(external_row, "topology.kind")
            .map_err(|error| TopologyQuerySurfaceError::new(error.to_string()))?;
        if kind_name != persistent_name_kind.kind_name() {
            return Err(TopologyQuerySurfaceError::new(format!(
                "query persistent-name surface expected `{}`, found `{kind_name}`",
                persistent_name_kind.kind_name()
            )));
        }
        required_text(external_row, "naming.persistent_name")
            .map_err(|error| TopologyQuerySurfaceError::new(error.to_string()))?;
        if external_row
            .get("lineage")
            .and_then(|value| value.get("provenance"))
            .is_none()
        {
            return Err(TopologyQuerySurfaceError::new(format!(
                "query persistent-name row `{}` is missing lineage.provenance",
                row.identity()
            )));
        }
        let target_identity = external_row
            .get("naming")
            .and_then(|value| value.get("target_identity"))
            .and_then(serde_json::Value::as_str);
        match target_identity {
            Some(identity) => {
                let Some(target_entity_id) = topology_identities.get(identity) else {
                    return Err(TopologyQuerySurfaceError::new(format!(
                        "query persistent-name row `{}` targets unknown topology identity `{identity}`",
                        row.identity()
                    )));
                };
                attachments
                    .entry(*target_entity_id)
                    .or_default()
                    .push(persistent_name_id);
            }
            None => orphan_persistent_name_ids.push(persistent_name_id),
        }
    }

    let attachment_rows = topology_entities
        .into_iter()
        .map(
            |(topology_entity_id, topology_kind_name)| NamingAttachmentRow {
                topology_entity_id,
                topology_kind_name,
                attached_persistent_name_ids: attachments
                    .get(&topology_entity_id)
                    .cloned()
                    .unwrap_or_default(),
            },
        )
        .collect::<Vec<_>>();
    let named_entity_ids = attachment_rows
        .iter()
        .filter(|row| !row.attached_persistent_name_ids.is_empty())
        .map(|row| row.topology_entity_id)
        .collect::<BTreeSet<_>>();

    Ok(NamingAttachmentReport {
        fully_named: attachment_rows.len() == named_entity_ids.len()
            && orphan_persistent_name_ids.is_empty(),
        orphan_persistent_name_ids,
        attachments: attachment_rows,
    })
}

fn query_identity_label(
    identity: &ForgeQueryEntityIdentity,
) -> Result<String, TopologyQuerySurfaceError> {
    let parts = identity.relational_record_parts().ok_or_else(|| {
        TopologyQuerySurfaceError::new(format!(
            "persistent naming requires relational query identity, got `{identity}`"
        ))
    })?;
    let kind = match parts.kind() {
        RelationalBridgeRecordIdentityKind::Entity => "entity",
        RelationalBridgeRecordIdentityKind::Relation => "relation",
    };
    Ok(format!(
        "{kind}:{}:{}:{}",
        parts.partition_id(),
        parts.local_slot(),
        parts.generation()
    ))
}

#[cfg(test)]
mod tests {
    use forge_runtime_bridge::facade::RelationalBridgeRecordIdentityParts;
    use schema::facade::platform::entities::TopologyEntityKind;
    use serde_json::json;

    use super::*;

    #[test]
    fn naming_attachment_report_rejects_unknown_query_target_identity() {
        let entity_rows = vec![ForgeQueryEntity::from_external_projection(
            ForgeQueryEntityIdentity::from_relational_record(
                RelationalBridgeRecordIdentityParts::entity(0, 1, 0),
            ),
            json!({
                "topology": {
                    "kind": TopologyEntityKind::Vertex.kind_name(),
                    "structure": "vertex-a",
                }
            }),
        )];
        let persistent_name_rows = vec![ForgeQueryEntity::from_external_projection(
            ForgeQueryEntityIdentity::from_relational_record(
                RelationalBridgeRecordIdentityParts::entity(0, 2, 0),
            ),
            json!({
                "topology": {
                    "kind": EntityKind::Naming(NamingEntityKind::PersistentName).kind_name(),
                },
                "lineage": {
                    "provenance": "entity:0:2:0",
                },
                "naming": {
                    "persistent_name": "vertex-a",
                    "target_identity": "entity:0:99:0",
                }
            }),
        )];

        let error = naming_attachment_report_from_query_input(TopologyNamingAttachmentInput::new(
            &entity_rows,
            &persistent_name_rows,
        ))
        .expect_err("unknown query target identities must fail closed");

        assert!(error
            .to_string()
            .contains("targets unknown topology identity"));
    }
}

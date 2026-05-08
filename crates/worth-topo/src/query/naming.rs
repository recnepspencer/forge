//! Query-native naming truth helpers for  topology.
//!
//! This module keeps persistent-name truth as a first-class query surface so
//! query-native certification and inspection can reason about attachment and
//! orphan posture without reopening relational truth out-of-band.

use std::collections::{BTreeMap, BTreeSet};

use forge_query::facade::{
    ForgeQueryEntity, ForgeQueryLiveView, ForgeQueryRuntimeError, ForgeQueryWorkspace,
    ForgeQueryWorkspaceError, ForgeQueryWorkspaceLiveViewDeclaration,
};
use schema::facade::{
    EntityKind, NamingEntityKind, QueryAspectPath, QueryCollection, QueryDeclarationError,
    QueryLiveDeclarationBuilder, QueryLiveField, QuerySchemaBasis, TopologyEntityKind,
};

use crate::facade::{NamingAttachmentReport, NamingAttachmentRow};
use crate::query::{parse_entity_identity, required_text};

use super::TopologyQuerySurfaceError;

pub fn persistent_name_live_view_declaration(
    surface_name: impl Into<String>,
) -> Result<ForgeQueryWorkspaceLiveViewDeclaration, QueryDeclarationError> {
    QueryLiveDeclarationBuilder::new(
        surface_name,
        QueryCollection::PersistentName,
        QuerySchemaBasis::PersistentNameLiveView,
    )
    .select_fields([
        QueryLiveField::IdentityId,
        QueryLiveField::LineageProvenance,
        QueryAspectPath::NAMING_PERSISTENT_NAME.into(),
        QueryLiveField::NamingTargetIdentity,
    ])
    .order_by_field(QueryLiveField::IdentityId)
    .build()
}

pub fn declare_persistent_name_live_view<T>(
    workspace: &mut ForgeQueryWorkspace,
    surface_name: impl Into<String>,
) -> Result<ForgeQueryLiveView<T>, ForgeQueryRuntimeError> {
    let surface_name = surface_name.into();
    let declaration =
        persistent_name_live_view_declaration(surface_name.clone()).map_err(|error| {
            ForgeQueryRuntimeError::Workspace(ForgeQueryWorkspaceError::new(error.to_string()))
        })?;
    let request = declaration.request().clone();
    let schema_view = declaration.schema_view().clone();
    workspace.live_view_request(surface_name, request, schema_view)
}

pub fn naming_attachment_report_from_query_rows(
    entity_rows: &[ForgeQueryEntity],
    persistent_name_rows: &[ForgeQueryEntity],
) -> Result<NamingAttachmentReport, TopologyQuerySurfaceError> {
    let topology_kind_names: BTreeSet<_> = TopologyEntityKind::WRAPPED_ALL
        .into_iter()
        .map(|kind| kind.kind_name())
        .collect();

    let mut topology_entities = Vec::new();
    let mut topology_identities = BTreeMap::new();
    for row in entity_rows {
        let entity_id = parse_entity_identity(&row.identity)
            .map_err(|error| TopologyQuerySurfaceError::new(error.to_string()))?;
        let kind_name = required_text(&row.payload, "topology.kind")
            .map_err(|error| TopologyQuerySurfaceError::new(error.to_string()))?;
        let entity_kind = row
            .payload
            .get("topology")
            .and_then(|value| value.get("kind"))
            .and_then(serde_json::Value::as_str)
            .unwrap_or(kind_name);
        if !topology_kind_names.contains(entity_kind) {
            continue;
        }
        topology_identities.insert(row.identity.clone(), entity_id);
        topology_entities.push((entity_id, kind_name.to_string()));
    }

    let mut attachments = BTreeMap::<_, Vec<_>>::new();
    let persistent_name_kind = EntityKind::Naming(NamingEntityKind::PersistentName);
    let mut orphan_persistent_name_ids = Vec::new();
    for row in persistent_name_rows {
        let persistent_name_id = parse_entity_identity(&row.identity)
            .map_err(|error| TopologyQuerySurfaceError::new(error.to_string()))?;
        let kind_name = required_text(&row.payload, "topology.kind")
            .map_err(|error| TopologyQuerySurfaceError::new(error.to_string()))?;
        if kind_name != persistent_name_kind.kind_name() {
            return Err(TopologyQuerySurfaceError::new(format!(
                "query persistent-name surface expected `{}`, found `{kind_name}`",
                persistent_name_kind.kind_name()
            )));
        }
        required_text(&row.payload, "naming.persistent_name")
            .map_err(|error| TopologyQuerySurfaceError::new(error.to_string()))?;
        if row
            .payload
            .get("lineage")
            .and_then(|value| value.get("provenance"))
            .is_none()
        {
            return Err(TopologyQuerySurfaceError::new(format!(
                "query persistent-name row `{}` is missing lineage.provenance",
                row.identity
            )));
        }
        let target_identity = row
            .payload
            .get("naming")
            .and_then(|value| value.get("target_identity"))
            .and_then(serde_json::Value::as_str);
        match target_identity {
            Some(identity) => {
                let Some(target_entity_id) = topology_identities.get(identity) else {
                    return Err(TopologyQuerySurfaceError::new(format!(
                        "query persistent-name row `{}` targets unknown topology identity `{identity}`",
                        row.identity
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

#[cfg(test)]
mod tests {
    use schema::facade::TopologyEntityKind;
    use serde_json::json;

    use super::*;

    #[test]
    fn naming_attachment_report_rejects_unknown_query_target_identity() {
        let entity_rows = vec![ForgeQueryEntity {
            identity: "entity:0:1:0".to_string(),
            payload: json!({
                "topology": {
                    "kind": TopologyEntityKind::Vertex.kind_name(),
                    "structure": "vertex-a",
                }
            }),
        }];
        let persistent_name_rows = vec![ForgeQueryEntity {
            identity: "entity:0:2:0".to_string(),
            payload: json!({
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
        }];

        let error = naming_attachment_report_from_query_rows(&entity_rows, &persistent_name_rows)
            .expect_err("unknown query target identities must fail closed");

        assert!(error
            .to_string()
            .contains("targets unknown topology identity"));
    }
}

use forge_query::facade::{
    ForgeQueryDerivedPatch, ForgeQueryDerivedView, ForgeQueryDerivedViewHandle,
    ForgeQueryDerivedViewMaintainer, ForgeQueryDerivedViewMaterialization, ForgeQueryEntity,
    ForgeQueryLiveView, ForgeQueryRuntimeError, ForgeQueryWorkspace, ForgeQueryWorkspaceError,
    ForgeQueryWorkspaceLiveViewDeclaration,
};
use forge_relational::facade::identity::{EntityId, PartitionId, RelationId, VersionId};
use forge_relational::facade::payloads::RecordPayload;
use forge_relational::facade::runtime::{EntityReadRecord, RelationReadRecord};
use forge_relational::facade::schema::{KindResolution, SchemaId, SchemaVersionId};
use forge_relational::facade::storage::RecordLifecycleState;
use serde_json::{json, Value};
use worth_schema::facade::{
    WorthEntityKind, WorthQueryAspectPath, WorthQueryCollection,
    WorthQueryComputedDeclarationBuilder, WorthQueryDeclarationError,
    WorthQueryLiveDeclarationBuilder, WorthQueryLiveField, WorthQuerySchemaBasis,
    WorthRelationKind,
};

use crate::materialization::{
    MaterializedTopologyView, WorthTopologyMaterializationError, WorthTopologyMaterializer,
};

use super::QUERY_SURFACE_FAILURE_ROW_KEY;

#[derive(Debug, Clone)]
pub struct WorthTopologyMaterializedMaintainer {
    entity_view_name: String,
    relation_view_name: String,
}

impl WorthTopologyMaterializedMaintainer {
    pub fn new(entity_view_name: impl Into<String>, relation_view_name: impl Into<String>) -> Self {
        Self {
            entity_view_name: entity_view_name.into(),
            relation_view_name: relation_view_name.into(),
        }
    }
}

impl ForgeQueryDerivedViewMaintainer for WorthTopologyMaterializedMaintainer {
    fn maintain(
        &mut self,
        view: &ForgeQueryDerivedView,
        delta: &forge_query::facade::ForgeQueryMutationDelta,
        materialization: &mut ForgeQueryDerivedViewMaterialization,
    ) -> ForgeQueryDerivedPatch {
        let payload = json!({
            QUERY_SURFACE_FAILURE_ROW_KEY: format!(
                "incremental delivery reached `{}` for `{}`; whole-refresh fallback was expected",
                delta.collection,
                view.name(),
            ),
        });
        materialization.replace_rows([payload.clone()]);
        ForgeQueryDerivedPatch::incremental(
            view.name(),
            "worth-topology-materialized-incremental-unexpected",
            delta.entity_identity.clone(),
            if view.produced_aspects().is_empty() {
                delta.aspect_paths.clone()
            } else {
                view.produced_aspects().to_vec()
            },
            payload,
        )
    }

    fn refresh_from_upstreams(
        &mut self,
        view: &ForgeQueryDerivedView,
        _mutation: &forge_query::facade::ForgeQueryRetainedMutationContext,
        upstreams: &forge_query::facade::ForgeQueryRetainedUpstreamInputs,
        materialization: &mut ForgeQueryDerivedViewMaterialization,
    ) -> Option<ForgeQueryDerivedPatch> {
        let entity_rows = upstreams.live_rows(&self.entity_view_name).unwrap_or(&[]);
        let relation_rows = upstreams.live_rows(&self.relation_view_name).unwrap_or(&[]);

        let payload = match materialized_topology_from_query_rows(entity_rows, relation_rows) {
            Ok(materialized) => serde_json::to_value(materialized)
                .expect("materialized topology view must serialize"),
            Err(error) => json!({ QUERY_SURFACE_FAILURE_ROW_KEY: error.to_string() }),
        };
        materialization.replace_rows([payload.clone()]);
        Some(ForgeQueryDerivedPatch::whole_refresh_materialized(
            view.name(),
            "worth-topology-materialized",
            if view.produced_aspects().is_empty() {
                view.dependency_aspects().to_vec()
            } else {
                view.produced_aspects().to_vec()
            },
            payload,
            "worth-topology-retained-live-rebuild",
        ))
    }
}

pub fn worth_topology_entity_live_view_declaration(
    surface_name: impl Into<String>,
) -> Result<ForgeQueryWorkspaceLiveViewDeclaration, WorthQueryDeclarationError> {
    WorthQueryLiveDeclarationBuilder::new(
        surface_name,
        WorthQueryCollection::TopologyEntity,
        WorthQuerySchemaBasis::TopologyEntityLiveView,
    )
    .select_fields([
        WorthQueryLiveField::IdentityId,
        WorthQueryLiveField::TopologyKind,
        WorthQueryAspectPath::LINEAGE_PROVENANCE.into(),
        WorthQueryAspectPath::TOPOLOGY_STRUCTURE.into(),
        WorthQueryAspectPath::NAMING_PERSISTENT_NAME.into(),
    ])
    .order_by_field(WorthQueryLiveField::IdentityId)
    .build()
}

pub fn worth_topology_relation_live_view_declaration(
    surface_name: impl Into<String>,
) -> Result<ForgeQueryWorkspaceLiveViewDeclaration, WorthQueryDeclarationError> {
    WorthQueryLiveDeclarationBuilder::new(
        surface_name,
        WorthQueryCollection::TopologyRelation,
        WorthQuerySchemaBasis::TopologyRelationLiveView,
    )
    .select([
        WorthQueryAspectPath::LINEAGE_PROVENANCE,
        WorthQueryAspectPath::TOPOLOGY_OWNERSHIP,
        WorthQueryAspectPath::TOPOLOGY_BOUNDARY,
        WorthQueryAspectPath::TOPOLOGY_RADIAL,
    ])
    .select_fields([
        WorthQueryLiveField::IdentityId,
        WorthQueryLiveField::TopologyKind,
        WorthQueryLiveField::TopologySourceIdentity,
        WorthQueryLiveField::TopologyTargetIdentity,
    ])
    .order_by_field(WorthQueryLiveField::IdentityId)
    .build()
}

pub fn worth_topology_materialized_computed_declaration(
    surface_name: impl Into<String>,
) -> Result<ForgeQueryDerivedView, WorthQueryDeclarationError> {
    WorthQueryComputedDeclarationBuilder::new(surface_name)
        .reads([
            WorthQueryAspectPath::TOPOLOGY_STRUCTURE,
            WorthQueryAspectPath::TOPOLOGY_OWNERSHIP,
            WorthQueryAspectPath::TOPOLOGY_BOUNDARY,
            WorthQueryAspectPath::TOPOLOGY_RADIAL,
            WorthQueryAspectPath::NAMING_PERSISTENT_NAME,
        ])
        .produces([
            WorthQueryAspectPath::TOPOLOGY_STRUCTURE,
            WorthQueryAspectPath::TOPOLOGY_OWNERSHIP,
            WorthQueryAspectPath::TOPOLOGY_BOUNDARY,
            WorthQueryAspectPath::TOPOLOGY_RADIAL,
        ])
        .whole_refresh_fallback()
        .build()
}

pub fn declare_worth_topology_entity_live_view<T>(
    workspace: &mut ForgeQueryWorkspace,
    surface_name: impl Into<String>,
) -> Result<ForgeQueryLiveView<T>, ForgeQueryRuntimeError> {
    let surface_name = surface_name.into();
    let declaration =
        worth_topology_entity_live_view_declaration(surface_name.clone()).map_err(|error| {
            ForgeQueryRuntimeError::Workspace(ForgeQueryWorkspaceError::new(error.to_string()))
        })?;
    let request = declaration.request().clone();
    let schema_view = declaration.schema_view().clone();
    workspace.live_view_request(surface_name, request, schema_view)
}

pub fn declare_worth_topology_relation_live_view<T>(
    workspace: &mut ForgeQueryWorkspace,
    surface_name: impl Into<String>,
) -> Result<ForgeQueryLiveView<T>, ForgeQueryRuntimeError> {
    let surface_name = surface_name.into();
    let declaration =
        worth_topology_relation_live_view_declaration(surface_name.clone()).map_err(|error| {
            ForgeQueryRuntimeError::Workspace(ForgeQueryWorkspaceError::new(error.to_string()))
        })?;
    let request = declaration.request().clone();
    let schema_view = declaration.schema_view().clone();
    workspace.live_view_request(surface_name, request, schema_view)
}

pub fn declare_worth_topology_materialized_surface<T, E, R>(
    workspace: &mut ForgeQueryWorkspace,
    surface_name: impl Into<String>,
    entity_view: &ForgeQueryLiveView<E>,
    relation_view: &ForgeQueryLiveView<R>,
) -> Result<ForgeQueryDerivedViewHandle<T>, ForgeQueryRuntimeError> {
    let surface_name = surface_name.into();
    let view = worth_topology_materialized_computed_declaration(surface_name)
        .map_err(|error| {
            ForgeQueryRuntimeError::Workspace(ForgeQueryWorkspaceError::new(error.to_string()))
        })?
        .depends_on_live_name(entity_view.name())
        .depends_on_live_name(relation_view.name());
    workspace.computed_view(
        view,
        WorthTopologyMaterializedMaintainer::new(entity_view.name(), relation_view.name()),
    )
}

pub fn materialized_topology_from_query_rows(
    entity_rows: &[ForgeQueryEntity],
    relation_rows: &[ForgeQueryEntity],
) -> Result<MaterializedTopologyView, WorthTopologyMaterializationError> {
    let entity_records = entity_rows
        .iter()
        .map(entity_record_from_query_row)
        .collect::<Result<Vec<_>, _>>()?;
    let relation_records = relation_rows
        .iter()
        .enumerate()
        .map(|(index, row)| relation_record_from_query_row(index as u64, row))
        .collect::<Result<Vec<_>, _>>()?;
    WorthTopologyMaterializer::materialize_from_records(&entity_records, &relation_records)
}

fn entity_record_from_query_row(
    row: &ForgeQueryEntity,
) -> Result<EntityReadRecord, WorthTopologyMaterializationError> {
    let entity_id = parse_entity_identity(&row.identity)?;
    let kind_name = required_text(&row.payload, "topology.kind")?;
    let kind = parse_entity_kind(kind_name)?;
    let payload = json!({
        "topology": {
            "structure": row
                .payload
                .get("topology")
                .and_then(|value| value.get("structure"))
                .cloned()
                .unwrap_or_else(|| Value::String(kind.kind_name().to_string())),
        },
        "naming": {
            "persistent_name": row
                .payload
                .get("naming")
                .and_then(|value| value.get("persistent_name"))
                .cloned()
                .unwrap_or(Value::Null),
        }
    });
    Ok(EntityReadRecord {
        entity_id,
        lineage_id: None,
        kind: entity_kind_resolution(kind),
        lifecycle: RecordLifecycleState::Live,
        created_at_version: VersionId(1),
        retired_at_version: None,
        payload: RecordPayload::from(payload),
    })
}

fn relation_record_from_query_row(
    ordinal: u64,
    row: &ForgeQueryEntity,
) -> Result<RelationReadRecord, WorthTopologyMaterializationError> {
    let kind_name = required_text(&row.payload, "topology.kind")?;
    let kind = parse_relation_kind(kind_name)?;
    Ok(RelationReadRecord {
        relation_id: RelationId::new(PartitionId::main(), ordinal + 1, 0),
        kind: relation_kind_resolution(kind),
        lifecycle: RecordLifecycleState::Live,
        created_at_version: VersionId(1),
        retired_at_version: None,
        source: parse_entity_identity(required_text(&row.payload, "topology.source_identity")?)?,
        target: parse_entity_identity(required_text(&row.payload, "topology.target_identity")?)?,
        payload: None,
    })
}

fn parse_entity_kind(
    kind_name: &str,
) -> Result<WorthEntityKind, WorthTopologyMaterializationError> {
    WorthEntityKind::ALL
        .into_iter()
        .find(|kind| kind.kind_name() == kind_name)
        .ok_or_else(|| {
            WorthTopologyMaterializationError::new(format!(
                "unknown worth topology entity kind `{kind_name}` in query row"
            ))
        })
}

pub(crate) fn parse_relation_kind(
    kind_name: &str,
) -> Result<WorthRelationKind, WorthTopologyMaterializationError> {
    WorthRelationKind::ALL
        .into_iter()
        .find(|kind| kind.kind_name() == kind_name)
        .ok_or_else(|| {
            WorthTopologyMaterializationError::new(format!(
                "unknown worth topology relation kind `{kind_name}` in query row"
            ))
        })
}

pub(crate) fn topology_relation_dependency_path(kind: WorthRelationKind) -> Option<&'static str> {
    match kind {
        WorthRelationKind::Topology(
            worth_schema::facade::WorthTopologyRelationKind::ModelOwnsBody
            | worth_schema::facade::WorthTopologyRelationKind::BodyOwnsLump
            | worth_schema::facade::WorthTopologyRelationKind::LumpOwnsRegion
            | worth_schema::facade::WorthTopologyRelationKind::RegionOwnsShell
            | worth_schema::facade::WorthTopologyRelationKind::ShellOwnsFace
            | worth_schema::facade::WorthTopologyRelationKind::WireOwnsHalfEdge,
        ) => Some(WorthQueryAspectPath::TOPOLOGY_OWNERSHIP.as_str()),
        WorthRelationKind::Topology(
            worth_schema::facade::WorthTopologyRelationKind::FaceOuterLoop
            | worth_schema::facade::WorthTopologyRelationKind::FaceInnerLoop
            | worth_schema::facade::WorthTopologyRelationKind::LoopOwnsHalfEdge
            | worth_schema::facade::WorthTopologyRelationKind::HalfEdgeNext
            | worth_schema::facade::WorthTopologyRelationKind::HalfEdgePrev
            | worth_schema::facade::WorthTopologyRelationKind::HalfEdgeUsesEdge
            | worth_schema::facade::WorthTopologyRelationKind::HalfEdgeStartsAtVertex
            | worth_schema::facade::WorthTopologyRelationKind::HalfEdgeEndsAtVertex,
        ) => Some(WorthQueryAspectPath::TOPOLOGY_BOUNDARY.as_str()),
        WorthRelationKind::Topology(
            worth_schema::facade::WorthTopologyRelationKind::HalfEdgeRadialNext,
        ) => Some(WorthQueryAspectPath::TOPOLOGY_RADIAL.as_str()),
        _ => None,
    }
}

fn entity_kind_resolution(kind: WorthEntityKind) -> KindResolution {
    KindResolution {
        kind_id: kind.kind_id(),
        kind_name: kind.kind_name().to_string(),
        schema_id: SchemaId(worth_schema::facade::WORTH_SCHEMA_ID.to_string()),
        schema_version_id: SchemaVersionId(worth_schema::facade::WORTH_SCHEMA_VERSION_ID),
    }
}

fn relation_kind_resolution(kind: WorthRelationKind) -> KindResolution {
    KindResolution {
        kind_id: kind.kind_id(),
        kind_name: kind.kind_name().to_string(),
        schema_id: SchemaId(worth_schema::facade::WORTH_SCHEMA_ID.to_string()),
        schema_version_id: SchemaVersionId(worth_schema::facade::WORTH_SCHEMA_VERSION_ID),
    }
}

pub(crate) fn required_text<'a>(
    payload: &'a Value,
    path: &str,
) -> Result<&'a str, WorthTopologyMaterializationError> {
    let mut current = payload;
    for part in path.split('.') {
        current = current.get(part).ok_or_else(|| {
            WorthTopologyMaterializationError::new(format!(
                "query truth row is missing required field `{path}`"
            ))
        })?;
    }
    current.as_str().ok_or_else(|| {
        WorthTopologyMaterializationError::new(format!(
            "query truth row field `{path}` must be a string"
        ))
    })
}

pub(crate) fn parse_entity_identity(
    identity: &str,
) -> Result<EntityId, WorthTopologyMaterializationError> {
    let mut parts = identity.split(':');
    if parts.next() != Some("entity") {
        return Err(WorthTopologyMaterializationError::new(format!(
            "expected forge-query entity identity, found `{identity}`"
        )));
    }
    let partition = parse_identity_part(parts.next(), "partition", identity)?;
    let slot = parse_identity_part(parts.next(), "slot", identity)?;
    let generation = parse_identity_part(parts.next(), "generation", identity)?;
    if parts.next().is_some() {
        return Err(WorthTopologyMaterializationError::new(format!(
            "unexpected trailing forge-query identity data in `{identity}`"
        )));
    }
    Ok(EntityId::new(PartitionId(partition), slot, generation))
}

fn parse_identity_part<T>(
    part: Option<&str>,
    label: &str,
    identity: &str,
) -> Result<T, WorthTopologyMaterializationError>
where
    T: std::str::FromStr,
{
    let value = part.ok_or_else(|| {
        WorthTopologyMaterializationError::new(format!(
            "missing {label} component in forge-query identity `{identity}`"
        ))
    })?;
    value.parse::<T>().map_err(|_| {
        WorthTopologyMaterializationError::new(format!(
            "invalid {label} component in forge-query identity `{identity}`"
        ))
    })
}

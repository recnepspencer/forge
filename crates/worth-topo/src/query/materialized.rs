use forge_query::facade::{
    ForgeQueryDerivedPatch, ForgeQueryDerivedView, ForgeQueryDerivedViewHandle,
    ForgeQueryDerivedViewMaintainer, ForgeQueryDerivedViewMaterialization, ForgeQueryLiveView,
    ForgeQueryRuntimeError, ForgeQueryWorkspace, ForgeQueryWorkspaceError,
    ForgeQueryWorkspaceLiveViewDeclaration,
};
use serde_json::json;
use worth_schema::facade::{
    WorthQueryAspectPath, WorthQueryCollection, WorthQueryComputedDeclarationBuilder,
    WorthQueryDeclarationError, WorthQueryLiveDeclarationBuilder, WorthQueryLiveField,
    WorthQuerySchemaBasis, WorthRelationKind,
};

use crate::materialization::WorthTopologyMaterializer;

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

        let payload = match WorthTopologyMaterializer::materialize_from_query_rows(
            entity_rows,
            relation_rows,
        ) {
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

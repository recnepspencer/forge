use forge_query::facade::{
    ForgeQueryComputedBuilder, ForgeQueryDerivedPatch, ForgeQueryDerivedView,
    ForgeQueryDerivedViewHandle, ForgeQueryDerivedViewMaintainer,
    ForgeQueryDerivedViewMaterialization, ForgeQueryLiveView, ForgeQueryLiveViewBuilder,
    ForgeQueryRuntimeError, ForgeQueryWorkspace, ForgeQueryWorkspaceLiveViewDeclaration,
};
use schema::facade::{QueryAspectPath, QueryCollection, QueryLiveField, QuerySchemaBasis};
use serde_json::json;

use super::super::retained_payload::{
    incremental_patch_touches, publish_retained_payload, refresh_patch_touches,
};
use super::super::QUERY_SURFACE_FAILURE_ROW_KEY;
use crate::query_native_runtime_boundary::{
    query_aspect_field_key, query_aspect_touch, query_live_field_key, TopologyNativeQueryRowField,
};

#[derive(Debug, Clone)]
pub(crate) struct TopologyMaterializedMaintainer;

impl TopologyMaterializedMaintainer {
    pub(crate) fn new() -> Self {
        Self
    }
}

impl ForgeQueryDerivedViewMaintainer for TopologyMaterializedMaintainer {
    fn maintain(
        &mut self,
        view: &ForgeQueryDerivedView,
        delta: &forge_query::facade::ForgeQueryMutationDelta,
        materialization: &mut ForgeQueryDerivedViewMaterialization,
    ) -> ForgeQueryDerivedPatch {
        let payload = json!({
            QUERY_SURFACE_FAILURE_ROW_KEY: format!(
                "incremental delivery reached `{}` for `{}`; whole-refresh fallback was expected",
                delta.collection(),
                view.name(),
            ),
        });
        let patch_payload = publish_retained_payload(view.name(), materialization, &payload);
        ForgeQueryDerivedPatch::incremental(
            view.name(),
            crate::projection::runtime_boundary::query_support::derived_surface_commit_identity(
                "topology-materialized-incremental-unexpected",
            ),
            delta.entity_identity().clone(),
            incremental_patch_touches(view, delta),
            patch_payload,
        )
    }

    fn refresh_from_upstreams(
        &mut self,
        view: &ForgeQueryDerivedView,
        _refresh: &forge_query::facade::ForgeQueryRetainedRefreshContext,
        _upstreams: &forge_query::facade::ForgeQueryRetainedUpstreamInputs,
        materialization: &mut ForgeQueryDerivedViewMaterialization,
    ) -> Option<ForgeQueryDerivedPatch> {
        let payload = json!({
            QUERY_SURFACE_FAILURE_ROW_KEY:
                "topology materialized surface requires Query-native retained materialization support; local materialization fallback is denied",
        });
        let patch_payload = publish_retained_payload(view.name(), materialization, &payload);
        Some(ForgeQueryDerivedPatch::whole_refresh_materialized(
            view.name(),
            crate::projection::runtime_boundary::query_support::derived_surface_commit_identity(
                "topology-materialized",
            ),
            refresh_patch_touches(view),
            patch_payload,
            "topology-retained-live-rebuild",
        ))
    }
}

pub(crate) fn topology_entity_live_view_declaration(
    surface_name: impl Into<String>,
) -> Result<ForgeQueryWorkspaceLiveViewDeclaration, ForgeQueryRuntimeError> {
    ForgeQueryLiveViewBuilder::surface(surface_name)
        .select([
            query_live_field_key(QueryLiveField::IdentityId),
            query_live_field_key(QueryLiveField::TopologyKind),
            query_aspect_field_key(QueryAspectPath::LINEAGE_PROVENANCE),
            query_aspect_field_key(QueryAspectPath::TOPOLOGY_STRUCTURE),
            query_aspect_field_key(QueryAspectPath::NAMING_PERSISTENT_NAME),
        ])
        .order_by(query_live_field_key(QueryLiveField::IdentityId))
        .from(QueryCollection::TopologyEntity.as_str())
        .schema_basis(QuerySchemaBasis::TopologyEntityLiveView.as_str())
        .build()
}

pub(crate) fn topology_relation_live_view_declaration(
    surface_name: impl Into<String>,
) -> Result<ForgeQueryWorkspaceLiveViewDeclaration, ForgeQueryRuntimeError> {
    ForgeQueryLiveViewBuilder::surface(surface_name)
        .select([
            query_aspect_field_key(QueryAspectPath::LINEAGE_PROVENANCE),
            query_aspect_field_key(QueryAspectPath::TOPOLOGY_OWNERSHIP),
            query_aspect_field_key(QueryAspectPath::TOPOLOGY_BOUNDARY),
            query_aspect_field_key(QueryAspectPath::TOPOLOGY_RADIAL),
            query_live_field_key(QueryLiveField::IdentityId),
            query_live_field_key(QueryLiveField::TopologyKind),
            query_live_field_key(QueryLiveField::TopologySourceIdentity),
            query_live_field_key(QueryLiveField::TopologyTargetIdentity),
        ])
        .order_by(query_live_field_key(QueryLiveField::IdentityId))
        .from(QueryCollection::TopologyRelation.as_str())
        .schema_basis(QuerySchemaBasis::TopologyRelationLiveView.as_str())
        .build()
}

pub(crate) fn declare_topology_entity_live_view<T>(
    workspace: &mut ForgeQueryWorkspace,
    surface_name: impl Into<String>,
) -> Result<ForgeQueryLiveView<T>, ForgeQueryRuntimeError> {
    let surface_name = surface_name.into();
    let declaration = topology_entity_live_view_declaration(surface_name.clone())?;
    let request = declaration.request().clone();
    let schema_view = declaration.schema_view().clone();
    workspace.live_view_request(surface_name, request, schema_view)
}

pub(crate) fn declare_topology_relation_live_view<T>(
    workspace: &mut ForgeQueryWorkspace,
    surface_name: impl Into<String>,
) -> Result<ForgeQueryLiveView<T>, ForgeQueryRuntimeError> {
    let surface_name = surface_name.into();
    let declaration = topology_relation_live_view_declaration(surface_name.clone())?;
    let request = declaration.request().clone();
    let schema_view = declaration.schema_view().clone();
    workspace.live_view_request(surface_name, request, schema_view)
}

pub(crate) fn declare_topology_materialized_surface<T, E, R>(
    workspace: &mut ForgeQueryWorkspace,
    surface_name: impl Into<String>,
    entity_view: &ForgeQueryLiveView<E>,
    relation_view: &ForgeQueryLiveView<R>,
) -> Result<ForgeQueryDerivedViewHandle<T>, ForgeQueryRuntimeError> {
    let surface_name = surface_name.into();
    let view = ForgeQueryComputedBuilder::surface(surface_name)
        .reads([
            TopologyNativeQueryRowField::TopologyKind.touch(),
            query_aspect_touch(QueryAspectPath::TOPOLOGY_STRUCTURE),
            query_aspect_touch(QueryAspectPath::TOPOLOGY_OWNERSHIP),
            query_aspect_touch(QueryAspectPath::TOPOLOGY_BOUNDARY),
            query_aspect_touch(QueryAspectPath::TOPOLOGY_RADIAL),
            query_aspect_touch(QueryAspectPath::NAMING_PERSISTENT_NAME),
        ])
        .produces([
            query_aspect_touch(QueryAspectPath::TOPOLOGY_STRUCTURE),
            query_aspect_touch(QueryAspectPath::TOPOLOGY_OWNERSHIP),
            query_aspect_touch(QueryAspectPath::TOPOLOGY_BOUNDARY),
            query_aspect_touch(QueryAspectPath::TOPOLOGY_RADIAL),
        ])
        .depends_on_live(entity_view)
        .depends_on_live(relation_view)
        .whole_refresh_fallback()
        .build()?;
    workspace.computed_view(view, TopologyMaterializedMaintainer::new())
}

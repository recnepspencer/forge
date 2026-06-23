use forge_query::facade::{
    ForgeQueryComputedBuilder, ForgeQueryDerivedPatch, ForgeQueryDerivedView,
    ForgeQueryDerivedViewHandle, ForgeQueryDerivedViewMaintainer,
    ForgeQueryDerivedViewMaterialization, ForgeQueryLiveView, ForgeQueryLiveViewBuilder,
    ForgeQueryRuntimeError, ForgeQueryWorkspace, ForgeQueryWorkspaceLiveViewDeclaration,
};
use schema::facade::{QueryAspectPath, QueryCollection, QueryLiveField, QuerySchemaBasis};
use serde_json::json;

use crate::derived_topology::materialized_graph::{
    TopologyMaterializer, TopologyQueryMaterializationInput,
};

use super::QUERY_SURFACE_FAILURE_ROW_KEY;

#[derive(Debug, Clone)]
pub(crate) struct TopologyMaterializedMaintainer {
    entity_view_name: String,
    relation_view_name: String,
}

impl TopologyMaterializedMaintainer {
    pub(crate) fn new(
        entity_view_name: impl Into<String>,
        relation_view_name: impl Into<String>,
    ) -> Self {
        Self {
            entity_view_name: entity_view_name.into(),
            relation_view_name: relation_view_name.into(),
        }
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
        materialization.replace_rows([payload.clone()]);
        ForgeQueryDerivedPatch::incremental(
            view.name(),
            crate::projection::runtime_boundary::query_support::derived_surface_commit_identity(
                "topology-materialized-incremental-unexpected",
            ),
            delta.entity_identity().clone(),
            if view.produced_aspects().is_empty() {
                delta.aspect_paths().to_vec()
            } else {
                view.produced_aspects().to_vec()
            },
            payload,
        )
    }

    fn refresh_from_upstreams(
        &mut self,
        view: &ForgeQueryDerivedView,
        _refresh: &forge_query::facade::ForgeQueryRetainedRefreshContext,
        upstreams: &forge_query::facade::ForgeQueryRetainedUpstreamInputs,
        materialization: &mut ForgeQueryDerivedViewMaterialization,
    ) -> Option<ForgeQueryDerivedPatch> {
        let entity_rows = upstreams.live_rows(&self.entity_view_name).unwrap_or(&[]);
        let relation_rows = upstreams.live_rows(&self.relation_view_name).unwrap_or(&[]);

        let payload = match TopologyQueryMaterializationInput::decode(entity_rows, relation_rows) {
            Ok(input) => match TopologyMaterializer::materialize_query_input(&input) {
                Ok(materialized) => serde_json::to_value(materialized)
                    .expect("materialized topology view must serialize"),
                Err(error) => json!({ QUERY_SURFACE_FAILURE_ROW_KEY: error.to_string() }),
            },
            Err(error) => json!({ QUERY_SURFACE_FAILURE_ROW_KEY: error.to_string() }),
        };
        materialization.replace_rows([payload.clone()]);
        Some(ForgeQueryDerivedPatch::whole_refresh_materialized(
            view.name(),
            crate::projection::runtime_boundary::query_support::derived_surface_commit_identity(
                "topology-materialized",
            ),
            if view.produced_aspects().is_empty() {
                view.dependency_aspects().to_vec()
            } else {
                view.produced_aspects().to_vec()
            },
            payload,
            "topology-retained-live-rebuild",
        ))
    }
}

pub(crate) fn topology_entity_live_view_declaration(
    surface_name: impl Into<String>,
) -> Result<ForgeQueryWorkspaceLiveViewDeclaration, ForgeQueryRuntimeError> {
    ForgeQueryLiveViewBuilder::surface(surface_name)
        .select([
            QueryLiveField::IdentityId.delivered_name(),
            QueryLiveField::TopologyKind.delivered_name(),
            QueryAspectPath::LINEAGE_PROVENANCE.as_str(),
            QueryAspectPath::TOPOLOGY_STRUCTURE.as_str(),
            QueryAspectPath::NAMING_PERSISTENT_NAME.as_str(),
        ])
        .order_by(QueryLiveField::IdentityId.delivered_name())
        .from(QueryCollection::TopologyEntity.as_str())
        .schema_basis(QuerySchemaBasis::TopologyEntityLiveView.as_str())
        .build()
}

pub(crate) fn topology_relation_live_view_declaration(
    surface_name: impl Into<String>,
) -> Result<ForgeQueryWorkspaceLiveViewDeclaration, ForgeQueryRuntimeError> {
    ForgeQueryLiveViewBuilder::surface(surface_name)
        .select([
            QueryAspectPath::LINEAGE_PROVENANCE.as_str(),
            QueryAspectPath::TOPOLOGY_OWNERSHIP.as_str(),
            QueryAspectPath::TOPOLOGY_BOUNDARY.as_str(),
            QueryAspectPath::TOPOLOGY_RADIAL.as_str(),
            QueryLiveField::IdentityId.delivered_name(),
            QueryLiveField::TopologyKind.delivered_name(),
            QueryLiveField::TopologySourceIdentity.delivered_name(),
            QueryLiveField::TopologyTargetIdentity.delivered_name(),
        ])
        .order_by(QueryLiveField::IdentityId.delivered_name())
        .from(QueryCollection::TopologyRelation.as_str())
        .schema_basis(QuerySchemaBasis::TopologyRelationLiveView.as_str())
        .build()
}

pub(crate) fn topology_materialized_computed_declaration(
    surface_name: impl Into<String>,
) -> Result<ForgeQueryDerivedView, ForgeQueryRuntimeError> {
    ForgeQueryComputedBuilder::surface(surface_name)
        .reads([
            QueryAspectPath::TOPOLOGY_STRUCTURE.as_str(),
            QueryAspectPath::TOPOLOGY_OWNERSHIP.as_str(),
            QueryAspectPath::TOPOLOGY_BOUNDARY.as_str(),
            QueryAspectPath::TOPOLOGY_RADIAL.as_str(),
            QueryAspectPath::NAMING_PERSISTENT_NAME.as_str(),
        ])
        .produces([
            QueryAspectPath::TOPOLOGY_STRUCTURE.as_str(),
            QueryAspectPath::TOPOLOGY_OWNERSHIP.as_str(),
            QueryAspectPath::TOPOLOGY_BOUNDARY.as_str(),
            QueryAspectPath::TOPOLOGY_RADIAL.as_str(),
        ])
        .whole_refresh_fallback()
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
    let view = topology_materialized_computed_declaration(surface_name)?
        .depends_on_live_name(entity_view.name())
        .depends_on_live_name(relation_view.name());
    workspace.computed_view(
        view,
        TopologyMaterializedMaintainer::new(entity_view.name(), relation_view.name()),
    )
}

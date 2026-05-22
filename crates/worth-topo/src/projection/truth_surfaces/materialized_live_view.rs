use forge_query::facade::{
    ForgeQueryDerivedPatch, ForgeQueryDerivedView, ForgeQueryDerivedViewHandle,
    ForgeQueryDerivedViewMaintainer, ForgeQueryDerivedViewMaterialization, ForgeQueryLiveView,
    ForgeQueryRuntimeError, ForgeQueryWorkspace, ForgeQueryWorkspaceError,
    ForgeQueryWorkspaceLiveViewDeclaration,
};
use schema::facade::{
    QueryAspectPath, QueryCollection, QueryComputedDeclarationBuilder, QueryDeclarationError,
    QueryLiveDeclarationBuilder, QueryLiveField, QuerySchemaBasis,
};
use serde_json::json;

use crate::derived_topology::materialized_graph::{
    TopologyMaterializer, TopologyQueryMaterializationInput,
};

use super::QUERY_SURFACE_FAILURE_ROW_KEY;

#[derive(Debug, Clone)]
pub struct TopologyMaterializedMaintainer {
    entity_view_name: String,
    relation_view_name: String,
}

impl TopologyMaterializedMaintainer {
    pub fn new(entity_view_name: impl Into<String>, relation_view_name: impl Into<String>) -> Self {
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
                delta.collection,
                view.name(),
            ),
        });
        materialization.replace_rows([payload.clone()]);
        ForgeQueryDerivedPatch::incremental(
            view.name(),
            "topology-materialized-incremental-unexpected",
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
            "topology-materialized",
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

pub fn topology_entity_live_view_declaration(
    surface_name: impl Into<String>,
) -> Result<ForgeQueryWorkspaceLiveViewDeclaration, QueryDeclarationError> {
    QueryLiveDeclarationBuilder::new(
        surface_name,
        QueryCollection::TopologyEntity,
        QuerySchemaBasis::TopologyEntityLiveView,
    )
    .select_fields([
        QueryLiveField::IdentityId,
        QueryLiveField::TopologyKind,
        QueryAspectPath::LINEAGE_PROVENANCE.into(),
        QueryAspectPath::TOPOLOGY_STRUCTURE.into(),
        QueryAspectPath::NAMING_PERSISTENT_NAME.into(),
    ])
    .order_by_field(QueryLiveField::IdentityId)
    .build()
}

pub fn topology_relation_live_view_declaration(
    surface_name: impl Into<String>,
) -> Result<ForgeQueryWorkspaceLiveViewDeclaration, QueryDeclarationError> {
    QueryLiveDeclarationBuilder::new(
        surface_name,
        QueryCollection::TopologyRelation,
        QuerySchemaBasis::TopologyRelationLiveView,
    )
    .select([
        QueryAspectPath::LINEAGE_PROVENANCE,
        QueryAspectPath::TOPOLOGY_OWNERSHIP,
        QueryAspectPath::TOPOLOGY_BOUNDARY,
        QueryAspectPath::TOPOLOGY_RADIAL,
    ])
    .select_fields([
        QueryLiveField::IdentityId,
        QueryLiveField::TopologyKind,
        QueryLiveField::TopologySourceIdentity,
        QueryLiveField::TopologyTargetIdentity,
    ])
    .order_by_field(QueryLiveField::IdentityId)
    .build()
}

pub fn topology_materialized_computed_declaration(
    surface_name: impl Into<String>,
) -> Result<ForgeQueryDerivedView, QueryDeclarationError> {
    QueryComputedDeclarationBuilder::new(surface_name)
        .reads([
            QueryAspectPath::TOPOLOGY_STRUCTURE,
            QueryAspectPath::TOPOLOGY_OWNERSHIP,
            QueryAspectPath::TOPOLOGY_BOUNDARY,
            QueryAspectPath::TOPOLOGY_RADIAL,
            QueryAspectPath::NAMING_PERSISTENT_NAME,
        ])
        .produces([
            QueryAspectPath::TOPOLOGY_STRUCTURE,
            QueryAspectPath::TOPOLOGY_OWNERSHIP,
            QueryAspectPath::TOPOLOGY_BOUNDARY,
            QueryAspectPath::TOPOLOGY_RADIAL,
        ])
        .whole_refresh_fallback()
        .build()
}

pub fn declare_topology_entity_live_view<T>(
    workspace: &mut ForgeQueryWorkspace,
    surface_name: impl Into<String>,
) -> Result<ForgeQueryLiveView<T>, ForgeQueryRuntimeError> {
    let surface_name = surface_name.into();
    let declaration =
        topology_entity_live_view_declaration(surface_name.clone()).map_err(|error| {
            ForgeQueryRuntimeError::Workspace(ForgeQueryWorkspaceError::new(error.to_string()))
        })?;
    let request = declaration.request().clone();
    let schema_view = declaration.schema_view().clone();
    workspace.live_view_request(surface_name, request, schema_view)
}

pub fn declare_topology_relation_live_view<T>(
    workspace: &mut ForgeQueryWorkspace,
    surface_name: impl Into<String>,
) -> Result<ForgeQueryLiveView<T>, ForgeQueryRuntimeError> {
    let surface_name = surface_name.into();
    let declaration =
        topology_relation_live_view_declaration(surface_name.clone()).map_err(|error| {
            ForgeQueryRuntimeError::Workspace(ForgeQueryWorkspaceError::new(error.to_string()))
        })?;
    let request = declaration.request().clone();
    let schema_view = declaration.schema_view().clone();
    workspace.live_view_request(surface_name, request, schema_view)
}

pub fn declare_topology_materialized_surface<T, E, R>(
    workspace: &mut ForgeQueryWorkspace,
    surface_name: impl Into<String>,
    entity_view: &ForgeQueryLiveView<E>,
    relation_view: &ForgeQueryLiveView<R>,
) -> Result<ForgeQueryDerivedViewHandle<T>, ForgeQueryRuntimeError> {
    let surface_name = surface_name.into();
    let view = topology_materialized_computed_declaration(surface_name)
        .map_err(|error| {
            ForgeQueryRuntimeError::Workspace(ForgeQueryWorkspaceError::new(error.to_string()))
        })?
        .depends_on_live_name(entity_view.name())
        .depends_on_live_name(relation_view.name());
    workspace.computed_view(
        view,
        TopologyMaterializedMaintainer::new(entity_view.name(), relation_view.name()),
    )
}

use std::fmt;

use forge_query::facade::{
    ForgeQueryComputedBuilder, ForgeQueryDerivedPatch, ForgeQueryDerivedView,
    ForgeQueryDerivedViewHandle, ForgeQueryDerivedViewMaintainer,
    ForgeQueryDerivedViewMaterialization, ForgeQueryRuntimeError, ForgeQueryWorkspace,
};
use schema::facade::QueryAspectPath;
use serde_json::{json, Value};

use crate::derived_topology::materialized_graph::MaterializedTopologyView;
use crate::derived_topology::traversal_views::{interpret_topology_view, InterpretedTopologyView};
use crate::validation::{
    validate_interpreted_topology, DerivedTopologyValidationReport, TopologyValidationError,
};

use super::QUERY_SURFACE_FAILURE_ROW_KEY;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TopologyQuerySurfaceError {
    message: String,
}

impl TopologyQuerySurfaceError {
    pub(crate) fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for TopologyQuerySurfaceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.message.fmt(f)
    }
}

impl std::error::Error for TopologyQuerySurfaceError {}

#[derive(Debug, Clone)]
pub(crate) struct TopologyInterpretedMaintainer {
    materialized_view_name: String,
}

impl TopologyInterpretedMaintainer {
    pub(crate) fn new(materialized_view_name: impl Into<String>) -> Self {
        Self {
            materialized_view_name: materialized_view_name.into(),
        }
    }
}

impl ForgeQueryDerivedViewMaintainer for TopologyInterpretedMaintainer {
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
                "topology-interpreted-incremental-unexpected",
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
        let payload =
            match interpreted_topology_from_upstreams(upstreams, &self.materialized_view_name) {
                Ok(interpreted) => serde_json::to_value(interpreted)
                    .expect("interpreted topology view must serialize"),
                Err(error) => json!({ QUERY_SURFACE_FAILURE_ROW_KEY: error.to_string() }),
            };
        materialization.replace_rows([payload.clone()]);
        Some(ForgeQueryDerivedPatch::whole_refresh_materialized(
            view.name(),
            crate::projection::runtime_boundary::query_support::derived_surface_commit_identity(
                "topology-interpreted",
            ),
            if view.produced_aspects().is_empty() {
                view.dependency_aspects().to_vec()
            } else {
                view.produced_aspects().to_vec()
            },
            payload,
            "topology-materialized-interpretation",
        ))
    }
}

#[derive(Debug, Clone)]
pub(crate) struct TopologyValidationMaintainer {
    materialized_view_name: String,
    interpreted_view_name: String,
}

impl TopologyValidationMaintainer {
    pub(crate) fn new(
        materialized_view_name: impl Into<String>,
        interpreted_view_name: impl Into<String>,
    ) -> Self {
        Self {
            materialized_view_name: materialized_view_name.into(),
            interpreted_view_name: interpreted_view_name.into(),
        }
    }
}

impl ForgeQueryDerivedViewMaintainer for TopologyValidationMaintainer {
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
                "topology-validation-incremental-unexpected",
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
        let payload = match validation_report_from_upstreams(
            upstreams,
            &self.materialized_view_name,
            &self.interpreted_view_name,
        ) {
            Ok(validation) => {
                serde_json::to_value(validation).expect("validation report must serialize")
            }
            Err(error) => json!({ QUERY_SURFACE_FAILURE_ROW_KEY: error.to_string() }),
        };
        materialization.replace_rows([payload.clone()]);
        Some(ForgeQueryDerivedPatch::whole_refresh_materialized(
            view.name(),
            crate::projection::runtime_boundary::query_support::derived_surface_commit_identity(
                "topology-validation",
            ),
            if view.produced_aspects().is_empty() {
                view.dependency_aspects().to_vec()
            } else {
                view.produced_aspects().to_vec()
            },
            payload,
            "topology-interpreted-validation",
        ))
    }
}

pub(crate) fn topology_interpreted_computed_declaration(
    surface_name: impl Into<String>,
) -> Result<ForgeQueryDerivedView, ForgeQueryRuntimeError> {
    ForgeQueryComputedBuilder::surface(surface_name)
        .reads([
            QueryAspectPath::TOPOLOGY_STRUCTURE.as_str(),
            QueryAspectPath::TOPOLOGY_OWNERSHIP.as_str(),
            QueryAspectPath::TOPOLOGY_BOUNDARY.as_str(),
            QueryAspectPath::TOPOLOGY_RADIAL.as_str(),
        ])
        .produces([QueryAspectPath::DIAGNOSTICS_INTERPRETATIONS.as_str()])
        .whole_refresh_fallback()
        .build()
}

pub(crate) fn topology_validation_computed_declaration(
    surface_name: impl Into<String>,
) -> Result<ForgeQueryDerivedView, ForgeQueryRuntimeError> {
    ForgeQueryComputedBuilder::surface(surface_name)
        .reads([
            QueryAspectPath::TOPOLOGY_STRUCTURE.as_str(),
            QueryAspectPath::TOPOLOGY_OWNERSHIP.as_str(),
            QueryAspectPath::TOPOLOGY_BOUNDARY.as_str(),
            QueryAspectPath::TOPOLOGY_RADIAL.as_str(),
            QueryAspectPath::DIAGNOSTICS_INTERPRETATIONS.as_str(),
        ])
        .produces([QueryAspectPath::DIAGNOSTICS_DECISIONS.as_str()])
        .whole_refresh_fallback()
        .build()
}

pub(crate) fn declare_topology_interpreted_surface<T, M>(
    workspace: &mut ForgeQueryWorkspace,
    surface_name: impl Into<String>,
    materialized_view: &ForgeQueryDerivedViewHandle<M>,
) -> Result<ForgeQueryDerivedViewHandle<T>, ForgeQueryRuntimeError> {
    let surface_name = surface_name.into();
    let view = topology_interpreted_computed_declaration(surface_name)?
        .depends_on_derived_name(materialized_view.name());
    workspace.computed_view(
        view,
        TopologyInterpretedMaintainer::new(materialized_view.name()),
    )
}

pub(crate) fn declare_topology_validation_surface<T, M, I>(
    workspace: &mut ForgeQueryWorkspace,
    surface_name: impl Into<String>,
    materialized_view: &ForgeQueryDerivedViewHandle<M>,
    interpreted_view: &ForgeQueryDerivedViewHandle<I>,
) -> Result<ForgeQueryDerivedViewHandle<T>, ForgeQueryRuntimeError> {
    let surface_name = surface_name.into();
    let view = topology_validation_computed_declaration(surface_name)?
        .depends_on_derived_name(materialized_view.name())
        .depends_on_derived_name(interpreted_view.name());
    workspace.computed_view(
        view,
        TopologyValidationMaintainer::new(materialized_view.name(), interpreted_view.name()),
    )
}

#[cfg_attr(not(test), allow(dead_code))]
pub(crate) fn interpreted_topology_from_materialized_rows(
    materialized_rows: &[Value],
) -> Result<InterpretedTopologyView, TopologyQuerySurfaceError> {
    let materialized: MaterializedTopologyView =
        decode_query_surface_row(materialized_rows, "materialized topology")?;
    Ok(interpret_topology_view(&materialized))
}

#[cfg_attr(not(test), allow(dead_code))]
pub(crate) fn validation_report_from_query_rows(
    materialized_rows: &[Value],
    interpreted_rows: &[Value],
) -> Result<DerivedTopologyValidationReport, TopologyQuerySurfaceError> {
    let materialized: MaterializedTopologyView =
        decode_query_surface_row(materialized_rows, "materialized topology")?;
    let interpreted: InterpretedTopologyView =
        decode_query_surface_row(interpreted_rows, "interpreted topology")?;
    validate_interpreted_topology(&materialized, &interpreted).map_err(validation_surface_error)
}

fn interpreted_topology_from_upstreams(
    upstreams: &forge_query::facade::ForgeQueryRetainedUpstreamInputs,
    materialized_view_name: &str,
) -> Result<InterpretedTopologyView, TopologyQuerySurfaceError> {
    let materialized: MaterializedTopologyView = upstreams
        .decode_single_computed_row(materialized_view_name)
        .map_err(|error| TopologyQuerySurfaceError::new(error.to_string()))?;
    Ok(interpret_topology_view(&materialized))
}

fn validation_report_from_upstreams(
    upstreams: &forge_query::facade::ForgeQueryRetainedUpstreamInputs,
    materialized_view_name: &str,
    interpreted_view_name: &str,
) -> Result<DerivedTopologyValidationReport, TopologyQuerySurfaceError> {
    let materialized: MaterializedTopologyView = upstreams
        .decode_single_computed_row(materialized_view_name)
        .map_err(|error| TopologyQuerySurfaceError::new(error.to_string()))?;
    let interpreted: InterpretedTopologyView = upstreams
        .decode_single_computed_row(interpreted_view_name)
        .map_err(|error| TopologyQuerySurfaceError::new(error.to_string()))?;
    validate_interpreted_topology(&materialized, &interpreted).map_err(validation_surface_error)
}

pub(crate) fn decode_query_surface_row<T>(
    rows: &[Value],
    view_name: &str,
) -> Result<T, TopologyQuerySurfaceError>
where
    T: serde::de::DeserializeOwned,
{
    let upstreams = forge_query::facade::ForgeQueryRetainedUpstreamInputs::new(
        Vec::<(String, Vec<forge_query::facade::ForgeQueryEntity>)>::new(),
        [(view_name.to_string(), rows.to_vec())],
    );
    upstreams
        .decode_single_computed_row(view_name)
        .map_err(|error| TopologyQuerySurfaceError::new(error.to_string()))
}

fn validation_surface_error(error: TopologyValidationError) -> TopologyQuerySurfaceError {
    TopologyQuerySurfaceError::new(format!("query-derived validation refresh failed: {error}"))
}

use forge_query::facade::{
    ForgeQueryComputedBuilder, ForgeQueryDerivedPatch, ForgeQueryDerivedView,
    ForgeQueryDerivedViewHandle, ForgeQueryDerivedViewMaintainer,
    ForgeQueryDerivedViewMaterialization, ForgeQueryRuntimeError, ForgeQueryWorkspace,
};
use schema::facade::QueryAspectPath;
use serde_json::json;
#[cfg(test)]
use serde_json::Value;

#[cfg(test)]
use crate::derived_topology::materialized_graph::MaterializedTopologyView;
#[cfg(test)]
use crate::derived_topology::traversal_views::{
    bootstrap_topology_interpretation, InterpretedTopologyView,
};
#[cfg(test)]
use crate::projection::diagnostic_surfaces::derived_read_diagnostics::derive_topology_validation_report;
#[cfg(test)]
use crate::validation::{DerivedTopologyValidationReport, TopologyValidationError};

use super::super::retained_payload::{
    incremental_patch_touches, publish_retained_payload, refresh_patch_touches,
};
#[cfg(test)]
use super::super::TopologyQuerySurfaceError;
use super::super::QUERY_SURFACE_FAILURE_ROW_KEY;
use crate::query_native_runtime_boundary::query_aspect_touch;

#[derive(Debug, Clone)]
pub(crate) struct TopologyInterpretedMaintainer;

impl TopologyInterpretedMaintainer {
    pub(crate) fn new() -> Self {
        Self
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
        let patch_payload = publish_retained_payload(view.name(), materialization, &payload);
        ForgeQueryDerivedPatch::incremental(
            view.name(),
            crate::projection::runtime_boundary::query_support::derived_surface_commit_identity(
                "topology-interpreted-incremental-unexpected",
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
        let payload = json! {
            {
                QUERY_SURFACE_FAILURE_ROW_KEY:
                    "topology interpreted surface requires Query-native traversal product receipts; local interpretation fallback is denied",
            }
        };
        let patch_payload = publish_retained_payload(view.name(), materialization, &payload);
        Some(ForgeQueryDerivedPatch::whole_refresh_materialized(
            view.name(),
            crate::projection::runtime_boundary::query_support::derived_surface_commit_identity(
                "topology-interpreted",
            ),
            refresh_patch_touches(view),
            patch_payload,
            "topology-materialized-interpretation",
        ))
    }
}

#[derive(Debug, Clone)]
pub(crate) struct TopologyValidationMaintainer;

impl TopologyValidationMaintainer {
    pub(crate) fn new() -> Self {
        Self
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
        let patch_payload = publish_retained_payload(view.name(), materialization, &payload);
        ForgeQueryDerivedPatch::incremental(
            view.name(),
            crate::projection::runtime_boundary::query_support::derived_surface_commit_identity(
                "topology-validation-incremental-unexpected",
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
        let payload = json! {
            {
                QUERY_SURFACE_FAILURE_ROW_KEY:
                    "topology validation surface requires selected validator enforcement receipts; local validation fallback is denied",
            }
        };
        let patch_payload = publish_retained_payload(view.name(), materialization, &payload);
        Some(ForgeQueryDerivedPatch::whole_refresh_materialized(
            view.name(),
            crate::projection::runtime_boundary::query_support::derived_surface_commit_identity(
                "topology-validation",
            ),
            refresh_patch_touches(view),
            patch_payload,
            "topology-interpreted-validation",
        ))
    }
}

pub(crate) fn declare_topology_interpreted_surface<T, M>(
    workspace: &mut ForgeQueryWorkspace,
    surface_name: impl Into<String>,
    materialized_view: &ForgeQueryDerivedViewHandle<M>,
) -> Result<ForgeQueryDerivedViewHandle<T>, ForgeQueryRuntimeError> {
    let surface_name = surface_name.into();
    let view = ForgeQueryComputedBuilder::surface(surface_name)
        .reads([
            query_aspect_touch(QueryAspectPath::TOPOLOGY_STRUCTURE),
            query_aspect_touch(QueryAspectPath::TOPOLOGY_OWNERSHIP),
            query_aspect_touch(QueryAspectPath::TOPOLOGY_BOUNDARY),
            query_aspect_touch(QueryAspectPath::TOPOLOGY_RADIAL),
        ])
        .produces([query_aspect_touch(
            QueryAspectPath::DIAGNOSTICS_INTERPRETATIONS,
        )])
        .depends_on_computed(materialized_view)
        .whole_refresh_fallback()
        .build()?;
    workspace.computed_view(view, TopologyInterpretedMaintainer::new())
}

pub(crate) fn declare_topology_validation_surface<T, M, I>(
    workspace: &mut ForgeQueryWorkspace,
    surface_name: impl Into<String>,
    materialized_view: &ForgeQueryDerivedViewHandle<M>,
    interpreted_view: &ForgeQueryDerivedViewHandle<I>,
) -> Result<ForgeQueryDerivedViewHandle<T>, ForgeQueryRuntimeError> {
    let surface_name = surface_name.into();
    let view = ForgeQueryComputedBuilder::surface(surface_name)
        .reads([
            query_aspect_touch(QueryAspectPath::TOPOLOGY_STRUCTURE),
            query_aspect_touch(QueryAspectPath::TOPOLOGY_OWNERSHIP),
            query_aspect_touch(QueryAspectPath::TOPOLOGY_BOUNDARY),
            query_aspect_touch(QueryAspectPath::TOPOLOGY_RADIAL),
            query_aspect_touch(QueryAspectPath::DIAGNOSTICS_INTERPRETATIONS),
        ])
        .produces([query_aspect_touch(QueryAspectPath::DIAGNOSTICS_DECISIONS)])
        .depends_on_computed(materialized_view)
        .depends_on_computed(interpreted_view)
        .whole_refresh_fallback()
        .build()?;
    workspace.computed_view(view, TopologyValidationMaintainer::new())
}

#[cfg(test)]
pub(crate) fn interpreted_topology_from_materialized_rows(
    materialized_rows: &[Value],
) -> Result<InterpretedTopologyView, TopologyQuerySurfaceError> {
    let materialized: MaterializedTopologyView =
        decode_query_surface_row(materialized_rows, "materialized topology")?;
    Ok(bootstrap_topology_interpretation(&materialized))
}

#[cfg(test)]
pub(crate) fn validation_report_from_query_rows(
    materialized_rows: &[Value],
    interpreted_rows: &[Value],
) -> Result<DerivedTopologyValidationReport, TopologyQuerySurfaceError> {
    let materialized: MaterializedTopologyView =
        decode_query_surface_row(materialized_rows, "materialized topology")?;
    let interpreted: InterpretedTopologyView =
        decode_query_surface_row(interpreted_rows, "interpreted topology")?;
    derive_topology_validation_report(&materialized, &interpreted).map_err(validation_surface_error)
}

#[cfg(test)]
pub(crate) fn decode_query_surface_row<T>(
    rows: &[Value],
    view_name: &str,
) -> Result<T, TopologyQuerySurfaceError>
where
    T: serde::de::DeserializeOwned,
{
    let row = match rows {
        [] => {
            return Err(TopologyQuerySurfaceError::new(format!(
                "query surface `{view_name}` expected one row, found none"
            )));
        }
        [row] => row,
        rows => {
            return Err(TopologyQuerySurfaceError::new(format!(
                "query surface `{view_name}` expected one row, found {}",
                rows.len()
            )));
        }
    };
    serde_json::from_value(row.clone()).map_err(|error| {
        TopologyQuerySurfaceError::new(format!(
            "query surface `{view_name}` row failed to decode: {error}"
        ))
    })
}

#[cfg(test)]
fn validation_surface_error(error: TopologyValidationError) -> TopologyQuerySurfaceError {
    TopologyQuerySurfaceError::new(format!("query-derived validation refresh failed: {error}"))
}

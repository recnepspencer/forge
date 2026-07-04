mod equivalence_contract;
mod evidence;
mod retained_diagnostics;

use crate::query_native_runtime_boundary::query_aspect_touch;
use forge_query::facade::{
    ForgeQueryComputedBuilder, ForgeQueryDerivedPatch, ForgeQueryDerivedView,
    ForgeQueryDerivedViewHandle, ForgeQueryDerivedViewMaintainer,
    ForgeQueryDerivedViewMaterialization, ForgeQueryRetainedRefreshContext,
    ForgeQueryRetainedUpstreamInputs, ForgeQueryRuntimeError, ForgeQueryWorkspace,
};
use schema::facade::QueryAspectPath;
use serde_json::json;

use super::retained_payload::{
    incremental_patch_touches, publish_retained_payload, refresh_patch_touches,
};
use super::QUERY_SURFACE_FAILURE_ROW_KEY;
pub(crate) use equivalence_contract::declare_topology_equivalence_contract_surface;
#[cfg(test)]
pub(crate) use equivalence_contract::equivalence_contract_from_diagnostics_rows;
pub(crate) use evidence::TopologyHistoricalReadBasisMetadata;
pub(crate) use evidence::TopologyQueryMutationEvidence;
use retained_diagnostics::derived_read_diagnostics_from_upstreams;

#[derive(Debug, Clone)]
pub(crate) struct TopologyDiagnosticsMaintainer;

impl TopologyDiagnosticsMaintainer {
    pub(crate) fn new() -> Self {
        Self
    }
}

impl ForgeQueryDerivedViewMaintainer for TopologyDiagnosticsMaintainer {
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
                "topology-diagnostics-incremental-unexpected",
            ),
            delta.entity_identity().clone(),
            incremental_patch_touches(view, delta),
            patch_payload,
        )
    }

    fn refresh_from_upstreams(
        &mut self,
        view: &ForgeQueryDerivedView,
        refresh: &ForgeQueryRetainedRefreshContext,
        upstreams: &ForgeQueryRetainedUpstreamInputs,
        materialization: &mut ForgeQueryDerivedViewMaterialization,
    ) -> Option<ForgeQueryDerivedPatch> {
        let payload = match derived_read_diagnostics_from_upstreams(refresh, view, upstreams) {
            Ok(diagnostics) => {
                serde_json::to_value(diagnostics).expect("derived diagnostics must serialize")
            }
            Err(error) => error.failure_payload(),
        };
        let patch_payload = publish_retained_payload(view.name(), materialization, &payload);
        Some(ForgeQueryDerivedPatch::whole_refresh_materialized(
            view.name(),
            crate::projection::runtime_boundary::query_support::derived_surface_commit_identity(
                "topology-diagnostics",
            ),
            refresh_patch_touches(view),
            patch_payload,
            "topology-derived-diagnostics",
        ))
    }
}

pub(crate) fn declare_topology_diagnostics_surface<T, M, I, V>(
    workspace: &mut ForgeQueryWorkspace,
    surface_name: impl Into<String>,
    materialized_view: &ForgeQueryDerivedViewHandle<M>,
    interpreted_view: &ForgeQueryDerivedViewHandle<I>,
    validation_view: &ForgeQueryDerivedViewHandle<V>,
) -> Result<ForgeQueryDerivedViewHandle<T>, ForgeQueryRuntimeError> {
    let surface_name = surface_name.into();
    let view = ForgeQueryComputedBuilder::surface(surface_name)
        .reads([
            query_aspect_touch(QueryAspectPath::TOPOLOGY_STRUCTURE),
            query_aspect_touch(QueryAspectPath::TOPOLOGY_OWNERSHIP),
            query_aspect_touch(QueryAspectPath::TOPOLOGY_BOUNDARY),
            query_aspect_touch(QueryAspectPath::TOPOLOGY_RADIAL),
            query_aspect_touch(QueryAspectPath::NAMING_PERSISTENT_NAME),
            query_aspect_touch(QueryAspectPath::DIAGNOSTICS_INTERPRETATIONS),
            query_aspect_touch(QueryAspectPath::DIAGNOSTICS_DECISIONS),
        ])
        .depends_on_computed(materialized_view)
        .depends_on_computed(interpreted_view)
        .depends_on_computed(validation_view)
        .whole_refresh_fallback()
        .build()?;
    workspace.computed_view(view, TopologyDiagnosticsMaintainer::new())
}

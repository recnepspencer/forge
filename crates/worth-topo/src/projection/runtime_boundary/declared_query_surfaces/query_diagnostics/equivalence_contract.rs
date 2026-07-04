use forge_query::facade::{
    ForgeQueryComputedBuilder, ForgeQueryDerivedPatch, ForgeQueryDerivedView,
    ForgeQueryDerivedViewHandle, ForgeQueryDerivedViewMaintainer,
    ForgeQueryDerivedViewMaterialization, ForgeQueryRetainedRefreshContext,
    ForgeQueryRetainedUpstreamInputs, ForgeQueryRuntimeError, ForgeQueryWorkspace,
};
use schema::facade::QueryAspectPath;
use serde_json::{json, Value};

use crate::derived_topology::compiled_product_consumer_cutover::DerivedEquivalenceContractReport;
use crate::projection::runtime_boundary::diagnostic_projection::DerivedReadDiagnostics;
use crate::query_native_runtime_boundary::query_aspect_touch;

#[cfg(test)]
use super::super::derived_surfaces::decode_query_surface_row;
use super::super::retained_payload::{
    decode_single_retained_payload_row, incremental_patch_touches, publish_retained_payload,
    refresh_patch_touches,
};
use super::super::{
    decode_query_surface_failure_payload, TopologyQuerySurfaceError, TopologyQuerySurfaceErrorKind,
    QUERY_SURFACE_FAILURE_ROW_KEY,
};

#[derive(Debug, Clone)]
pub(crate) struct TopologyEquivalenceContractMaintainer;

impl TopologyEquivalenceContractMaintainer {
    pub(crate) fn new() -> Self {
        Self
    }
}

impl ForgeQueryDerivedViewMaintainer for TopologyEquivalenceContractMaintainer {
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
                "topology-equivalence-incremental-unexpected",
            ),
            delta.entity_identity().clone(),
            incremental_patch_touches(view, delta),
            patch_payload,
        )
    }

    fn refresh_from_upstreams(
        &mut self,
        view: &ForgeQueryDerivedView,
        _refresh: &ForgeQueryRetainedRefreshContext,
        upstreams: &ForgeQueryRetainedUpstreamInputs,
        materialization: &mut ForgeQueryDerivedViewMaterialization,
    ) -> Option<ForgeQueryDerivedPatch> {
        let payload = match equivalence_contract_report_from_upstreams(view, upstreams) {
            Ok(report) => serde_json::to_value(report)
                .expect("derived equivalence contract report must serialize"),
            Err(error) => error.failure_payload(),
        };
        let patch_payload = publish_retained_payload(view.name(), materialization, &payload);
        Some(ForgeQueryDerivedPatch::whole_refresh_materialized(
            view.name(),
            crate::projection::runtime_boundary::query_support::derived_surface_commit_identity(
                "topology-equivalence-contract",
            ),
            refresh_patch_touches(view),
            patch_payload,
            "topology-derived-equivalence-contract",
        ))
    }
}

pub(crate) fn declare_topology_equivalence_contract_surface<T, V>(
    workspace: &mut ForgeQueryWorkspace,
    surface_name: impl Into<String>,
    diagnostics_view: &ForgeQueryDerivedViewHandle<V>,
) -> Result<ForgeQueryDerivedViewHandle<T>, ForgeQueryRuntimeError> {
    let surface_name = surface_name.into();
    let view = ForgeQueryComputedBuilder::surface(surface_name)
        .reads([
            query_aspect_touch(QueryAspectPath::DIAGNOSTICS_INTERPRETATIONS),
            query_aspect_touch(QueryAspectPath::DIAGNOSTICS_DECISIONS),
        ])
        .depends_on_computed(diagnostics_view)
        .whole_refresh_fallback()
        .build()?;
    workspace.computed_view(view, TopologyEquivalenceContractMaintainer::new())
}

fn equivalence_contract_report_from_upstreams(
    view: &ForgeQueryDerivedView,
    upstreams: &ForgeQueryRetainedUpstreamInputs,
) -> Result<DerivedEquivalenceContractReport, TopologyQuerySurfaceError> {
    let diagnostics_payload: Value = decode_single_retained_payload_row(
        upstreams
            .declared_retained_computed_row_sets(view)
            .next()
            .unwrap_or_default(),
        "topology diagnostics",
    )?;
    if let Some(error) =
        decode_query_surface_failure_payload(&diagnostics_payload, "topology diagnostics")
    {
        return Err(error);
    }
    let diagnostics: DerivedReadDiagnostics =
        serde_json::from_value(diagnostics_payload).map_err(|error| {
            TopologyQuerySurfaceError::with_kind(
                TopologyQuerySurfaceErrorKind::RetainedPayloadDecodeFailed,
                format!(
                    "retained surface `topology diagnostics` payload failed to decode: {error}"
                ),
            )
        })?;
    Ok(diagnostics.equivalence_contract_report)
}

#[cfg(test)]
pub(crate) fn equivalence_contract_from_diagnostics_rows(
    diagnostics_rows: &[serde_json::Value],
) -> Result<DerivedEquivalenceContractReport, TopologyQuerySurfaceError> {
    let diagnostics: DerivedReadDiagnostics =
        decode_query_surface_row(diagnostics_rows, "derived read diagnostics")?;
    Ok(diagnostics.equivalence_contract_report)
}

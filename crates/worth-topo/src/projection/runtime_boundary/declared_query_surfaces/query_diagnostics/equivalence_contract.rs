use forge_query::facade::{
    ForgeQueryComputedBuilder, ForgeQueryDerivedPatch, ForgeQueryDerivedView,
    ForgeQueryDerivedViewHandle, ForgeQueryDerivedViewMaintainer,
    ForgeQueryDerivedViewMaterialization, ForgeQueryRetainedRefreshContext,
    ForgeQueryRetainedUpstreamInputs, ForgeQueryRuntimeError, ForgeQueryWorkspace,
};
use schema::facade::QueryAspectPath;
use serde_json::{json, Value};

use crate::projection::diagnostic_surfaces::{
    derived_read_diagnostics::DerivedReadDiagnostics, DerivedEquivalenceContractReport,
};

use super::super::derived_surfaces::{decode_query_surface_row, TopologyQuerySurfaceError};
use super::super::QUERY_SURFACE_FAILURE_ROW_KEY;

#[derive(Debug, Clone)]
pub(crate) struct TopologyEquivalenceContractMaintainer {
    diagnostics_view_name: String,
}

impl TopologyEquivalenceContractMaintainer {
    pub(crate) fn new(diagnostics_view_name: impl Into<String>) -> Self {
        Self {
            diagnostics_view_name: diagnostics_view_name.into(),
        }
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
        materialization.replace_rows([payload.clone()]);
        ForgeQueryDerivedPatch::incremental(
            view.name(),
            crate::projection::runtime_boundary::query_support::derived_surface_commit_identity(
                "topology-equivalence-incremental-unexpected",
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
        _refresh: &ForgeQueryRetainedRefreshContext,
        upstreams: &ForgeQueryRetainedUpstreamInputs,
        materialization: &mut ForgeQueryDerivedViewMaterialization,
    ) -> Option<ForgeQueryDerivedPatch> {
        let payload = match equivalence_contract_from_diagnostics_rows(
            upstreams
                .computed_rows(&self.diagnostics_view_name)
                .unwrap_or(&[]),
        ) {
            Ok(report) => serde_json::to_value(report)
                .expect("derived equivalence contract report must serialize"),
            Err(error) => json!({ QUERY_SURFACE_FAILURE_ROW_KEY: error.to_string() }),
        };
        materialization.replace_rows([payload.clone()]);
        Some(ForgeQueryDerivedPatch::whole_refresh_materialized(
            view.name(),
            crate::projection::runtime_boundary::query_support::derived_surface_commit_identity(
                "topology-equivalence-contract",
            ),
            if view.produced_aspects().is_empty() {
                view.dependency_aspects().to_vec()
            } else {
                view.produced_aspects().to_vec()
            },
            payload,
            "topology-derived-equivalence-contract",
        ))
    }
}

pub(crate) fn topology_equivalence_contract_computed_declaration(
    surface_name: impl Into<String>,
) -> Result<ForgeQueryDerivedView, ForgeQueryRuntimeError> {
    ForgeQueryComputedBuilder::surface(surface_name)
        .reads([
            QueryAspectPath::DIAGNOSTICS_INTERPRETATIONS.as_str(),
            QueryAspectPath::DIAGNOSTICS_DECISIONS.as_str(),
        ])
        .whole_refresh_fallback()
        .build()
}

pub(crate) fn declare_topology_equivalence_contract_surface<T, D>(
    workspace: &mut ForgeQueryWorkspace,
    surface_name: impl Into<String>,
    diagnostics_view: &ForgeQueryDerivedViewHandle<D>,
) -> Result<ForgeQueryDerivedViewHandle<T>, ForgeQueryRuntimeError> {
    let surface_name = surface_name.into();
    let view = topology_equivalence_contract_computed_declaration(surface_name)?
        .depends_on_derived_name(diagnostics_view.name());
    workspace.computed_view(
        view,
        TopologyEquivalenceContractMaintainer::new(diagnostics_view.name()),
    )
}

pub(crate) fn equivalence_contract_from_diagnostics_rows(
    diagnostics_rows: &[Value],
) -> Result<DerivedEquivalenceContractReport, TopologyQuerySurfaceError> {
    let diagnostics: DerivedReadDiagnostics =
        decode_query_surface_row(diagnostics_rows, "derived read diagnostics")?;
    Ok(diagnostics.equivalence_contract_report)
}

mod equivalence_contract;
mod evidence;

use forge_query::facade::{
    ForgeQueryComputedBuilder, ForgeQueryDerivedPatch, ForgeQueryDerivedView,
    ForgeQueryDerivedViewHandle, ForgeQueryDerivedViewMaintainer,
    ForgeQueryDerivedViewMaterialization, ForgeQueryRetainedRefreshContext,
    ForgeQueryRetainedUpstreamInputs, ForgeQueryRuntimeError, ForgeQueryWorkspace,
};
use schema::facade::QueryAspectPath;
use serde_json::{json, Value};

use crate::derived_topology::materialized_graph::MaterializedTopologyView;
use crate::derived_topology::traversal_views::InterpretedTopologyView;
use crate::projection::diagnostic_surfaces::{
    build_derived_equivalence_contract_report,
    derived_read_diagnostics::{
        build_derived_fallback_report_from_counts, build_derived_invalidation_report_from_aspects,
        DerivedReadDiagnostics,
    },
};
use crate::validation::DerivedTopologyValidationReport;

use super::derived_surfaces::{decode_query_surface_row, TopologyQuerySurfaceError};
use super::QUERY_SURFACE_FAILURE_ROW_KEY;
pub(crate) use equivalence_contract::declare_topology_equivalence_contract_surface;
#[cfg(test)]
pub(crate) use equivalence_contract::equivalence_contract_from_diagnostics_rows;
pub(crate) use evidence::TopologyQueryMutationEvidence;

#[derive(Debug, Clone)]
pub(crate) struct TopologyDiagnosticsMaintainer {
    materialized_view_name: String,
    interpreted_view_name: String,
    validation_view_name: String,
}

impl TopologyDiagnosticsMaintainer {
    pub(crate) fn new(
        materialized_view_name: impl Into<String>,
        interpreted_view_name: impl Into<String>,
        validation_view_name: impl Into<String>,
    ) -> Self {
        Self {
            materialized_view_name: materialized_view_name.into(),
            interpreted_view_name: interpreted_view_name.into(),
            validation_view_name: validation_view_name.into(),
        }
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
        materialization.replace_rows([payload.clone()]);
        ForgeQueryDerivedPatch::incremental(
            view.name(),
            crate::projection::runtime_boundary::query_support::derived_surface_commit_identity(
                "topology-diagnostics-incremental-unexpected",
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
        refresh: &ForgeQueryRetainedRefreshContext,
        upstreams: &ForgeQueryRetainedUpstreamInputs,
        materialization: &mut ForgeQueryDerivedViewMaterialization,
    ) -> Option<ForgeQueryDerivedPatch> {
        let payload = match derived_read_diagnostics_from_upstreams(
            refresh,
            upstreams,
            &self.materialized_view_name,
            &self.interpreted_view_name,
            &self.validation_view_name,
        ) {
            Ok(diagnostics) => {
                serde_json::to_value(diagnostics).expect("derived diagnostics must serialize")
            }
            Err(error) => json!({ QUERY_SURFACE_FAILURE_ROW_KEY: error.to_string() }),
        };
        materialization.replace_rows([payload.clone()]);
        Some(ForgeQueryDerivedPatch::whole_refresh_materialized(
            view.name(),
            crate::projection::runtime_boundary::query_support::derived_surface_commit_identity(
                "topology-diagnostics",
            ),
            if view.produced_aspects().is_empty() {
                view.dependency_aspects().to_vec()
            } else {
                view.produced_aspects().to_vec()
            },
            payload,
            "topology-derived-diagnostics",
        ))
    }
}

pub(crate) fn topology_diagnostics_computed_declaration(
    surface_name: impl Into<String>,
) -> Result<ForgeQueryDerivedView, ForgeQueryRuntimeError> {
    ForgeQueryComputedBuilder::surface(surface_name)
        .reads([
            QueryAspectPath::TOPOLOGY_STRUCTURE.as_str(),
            QueryAspectPath::TOPOLOGY_OWNERSHIP.as_str(),
            QueryAspectPath::TOPOLOGY_BOUNDARY.as_str(),
            QueryAspectPath::TOPOLOGY_RADIAL.as_str(),
            QueryAspectPath::NAMING_PERSISTENT_NAME.as_str(),
            QueryAspectPath::DIAGNOSTICS_INTERPRETATIONS.as_str(),
            QueryAspectPath::DIAGNOSTICS_DECISIONS.as_str(),
        ])
        .whole_refresh_fallback()
        .build()
}

pub(crate) fn declare_topology_diagnostics_surface<T, M, I, V>(
    workspace: &mut ForgeQueryWorkspace,
    surface_name: impl Into<String>,
    materialized_view: &ForgeQueryDerivedViewHandle<M>,
    interpreted_view: &ForgeQueryDerivedViewHandle<I>,
    validation_view: &ForgeQueryDerivedViewHandle<V>,
) -> Result<ForgeQueryDerivedViewHandle<T>, ForgeQueryRuntimeError> {
    let surface_name = surface_name.into();
    let view = topology_diagnostics_computed_declaration(surface_name)?
        .depends_on_derived_name(materialized_view.name())
        .depends_on_derived_name(interpreted_view.name())
        .depends_on_derived_name(validation_view.name());
    workspace.computed_view(
        view,
        TopologyDiagnosticsMaintainer::new(
            materialized_view.name(),
            interpreted_view.name(),
            validation_view.name(),
        ),
    )
}

#[allow(dead_code)]
pub(crate) fn derived_read_diagnostics_from_query_rows(
    refresh: &ForgeQueryRetainedRefreshContext,
    materialized_rows: &[Value],
    interpreted_rows: &[Value],
    validation_rows: &[Value],
) -> Result<DerivedReadDiagnostics, TopologyQuerySurfaceError> {
    let evidence = TopologyQueryMutationEvidence::from_refresh(refresh)?;
    let touched_aspects = evidence.touched_aspects()?;
    let materialized: MaterializedTopologyView =
        decode_query_surface_row(materialized_rows, "materialized topology")?;
    let interpreted: InterpretedTopologyView =
        decode_query_surface_row(interpreted_rows, "interpreted topology")?;
    let validation: DerivedTopologyValidationReport =
        decode_query_surface_row(validation_rows, "topology validation")?;

    Ok(DerivedReadDiagnostics {
        invalidation_report: build_derived_invalidation_report_from_aspects(
            touched_aspects.iter().copied(),
        ),
        rebuild_report: crate::projection::diagnostic_surfaces::derived_read_diagnostics::build_derived_rebuild_report(
            &materialized,
            &interpreted,
            &validation,
        ),
        fallback_report: build_derived_fallback_report_from_counts(
            evidence.precision_fallback_count,
            evidence.precision_budget_fallback_count,
            &materialized,
        ),
        equivalence_contract_report: build_derived_equivalence_contract_report(
            evidence.authority_snapshot_id,
            evidence.authority_branch_id,
            evidence.authoritative_mutation_origin,
            evidence.derivation_origin,
            evidence.truth_basis_digest_hex,
            touched_aspects.len(),
            crate::projection::diagnostic_surfaces::derived_read_diagnostics::triggered_invalidation_targets_from_aspects(
                touched_aspects.iter().copied(),
            ),
            evidence.precision_fallback_count,
            evidence.precision_budget_fallback_count,
            &materialized,
            &interpreted,
            &validation,
        ),
    })
}

fn derived_read_diagnostics_from_upstreams(
    refresh: &ForgeQueryRetainedRefreshContext,
    upstreams: &ForgeQueryRetainedUpstreamInputs,
    materialized_view_name: &str,
    interpreted_view_name: &str,
    validation_view_name: &str,
) -> Result<DerivedReadDiagnostics, TopologyQuerySurfaceError> {
    let evidence = TopologyQueryMutationEvidence::from_refresh(refresh)?;
    let touched_aspects = evidence.touched_aspects()?;
    let materialized: MaterializedTopologyView = upstreams
        .decode_single_computed_row(materialized_view_name)
        .map_err(|error| TopologyQuerySurfaceError::new(error.to_string()))?;
    let interpreted: InterpretedTopologyView = upstreams
        .decode_single_computed_row(interpreted_view_name)
        .map_err(|error| TopologyQuerySurfaceError::new(error.to_string()))?;
    let validation: DerivedTopologyValidationReport = upstreams
        .decode_single_computed_row(validation_view_name)
        .map_err(|error| TopologyQuerySurfaceError::new(error.to_string()))?;

    Ok(DerivedReadDiagnostics {
        invalidation_report: build_derived_invalidation_report_from_aspects(
            touched_aspects.iter().copied(),
        ),
        rebuild_report: crate::projection::diagnostic_surfaces::derived_read_diagnostics::build_derived_rebuild_report(
            &materialized,
            &interpreted,
            &validation,
        ),
        fallback_report: build_derived_fallback_report_from_counts(
            evidence.precision_fallback_count,
            evidence.precision_budget_fallback_count,
            &materialized,
        ),
        equivalence_contract_report: build_derived_equivalence_contract_report(
            evidence.authority_snapshot_id,
            evidence.authority_branch_id,
            evidence.authoritative_mutation_origin,
            evidence.derivation_origin,
            evidence.truth_basis_digest_hex,
            touched_aspects.len(),
            crate::projection::diagnostic_surfaces::derived_read_diagnostics::triggered_invalidation_targets_from_aspects(
                touched_aspects.iter().copied(),
            ),
            evidence.precision_fallback_count,
            evidence.precision_budget_fallback_count,
            &materialized,
            &interpreted,
            &validation,
        ),
    })
}

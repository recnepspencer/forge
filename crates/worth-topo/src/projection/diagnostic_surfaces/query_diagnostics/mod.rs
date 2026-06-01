use forge_query::facade::{
    ForgeQueryComputedBuilder, ForgeQueryDerivedPatch, ForgeQueryDerivedView,
    ForgeQueryDerivedViewHandle, ForgeQueryDerivedViewMaintainer,
    ForgeQueryDerivedViewMaterialization, ForgeQueryRetainedMutationContext,
    ForgeQueryRetainedUpstreamInputs, ForgeQueryRuntimeError, ForgeQueryWorkspace,
};
use schema::facade::platform::aspects::Aspect;
use schema::facade::platform::authority::MutationOrigin;
use schema::facade::topology_authoring::DerivedTopologyReadBasis;
use schema::facade::{query_aspect_paths_from_set, QueryAspectPath};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::certification::support::parity::build_derived_equivalence_contract_report;
use crate::facade::{
    DerivedEquivalenceContractReport, DerivedTopologyValidationReport, InterpretedTopologyView,
    MaterializedTopologyView,
};
use crate::projection::diagnostic_surfaces::{
    build_derived_fallback_report_from_counts, build_derived_invalidation_report_from_aspects,
    DerivedReadDiagnostics,
};

use super::QUERY_SURFACE_FAILURE_ROW_KEY;
use crate::projection::derived_surfaces::{decode_single_computed_row, TopologyQuerySurfaceError};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopologyQueryMutationEvidence {
    pub authority_snapshot_id: u64,
    pub authority_branch_id: String,
    pub authoritative_mutation_origin: MutationOrigin,
    pub derivation_origin: MutationOrigin,
    pub truth_basis_digest_hex: String,
    pub touched_aspect_paths: Vec<String>,
    pub precision_fallback_count: usize,
    pub precision_budget_fallback_count: usize,
}

impl TopologyQueryMutationEvidence {
    pub const fn metadata_key() -> &'static str {
        ".topology.read_basis"
    }

    pub fn from_read_basis(read_basis: &DerivedTopologyReadBasis) -> Self {
        Self {
            authority_snapshot_id: read_basis.snapshot().snapshot_id.0,
            authority_branch_id: read_basis.branch_id().0.clone(),
            authoritative_mutation_origin: read_basis.authoritative_mutation_origin(),
            derivation_origin: read_basis.derivation_origin(),
            truth_basis_digest_hex: read_basis
                .authority
                .truth_basis_identity
                .mutation_digest_hex
                .clone(),
            touched_aspect_paths: query_aspect_paths_from_set(read_basis.touched_aspects())
                .into_iter()
                .map(|aspect| aspect.as_str().to_string())
                .collect(),
            precision_fallback_count: read_basis.precision_fallbacks.len(),
            precision_budget_fallback_count: read_basis.precision_budget_fallbacks.len(),
        }
    }

    fn from_mutation(
        mutation: &ForgeQueryRetainedMutationContext,
    ) -> Result<Self, TopologyQuerySurfaceError> {
        let Some(value) = mutation.mutation_metadata().get(Self::metadata_key()) else {
            return Err(TopologyQuerySurfaceError::new(format!(
                "query-derived mutation context is missing `{}` metadata",
                Self::metadata_key()
            )));
        };
        serde_json::from_value(value.clone()).map_err(|error| {
            TopologyQuerySurfaceError::new(format!(
                "query-derived mutation metadata `{}` failed to decode: {error}",
                Self::metadata_key()
            ))
        })
    }

    fn touched_aspects(&self) -> Result<Vec<Aspect>, TopologyQuerySurfaceError> {
        self.touched_aspect_paths
            .iter()
            .map(|path| {
                let path = QueryAspectPath::from_str(path).ok_or_else(|| {
                    TopologyQuerySurfaceError::new(format!(
                        "query-derived mutation metadata declared unsupported touched aspect `{path}`"
                    ))
                })?;
                Ok(path.into_aspect())
            })
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct TopologyDiagnosticsMaintainer {
    materialized_view_name: String,
    interpreted_view_name: String,
    validation_view_name: String,
}

impl TopologyDiagnosticsMaintainer {
    pub fn new(
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
                delta.collection,
                view.name(),
            ),
        });
        materialization.replace_rows([payload.clone()]);
        ForgeQueryDerivedPatch::incremental(
            view.name(),
            "topology-diagnostics-incremental-unexpected",
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
        mutation: &ForgeQueryRetainedMutationContext,
        upstreams: &ForgeQueryRetainedUpstreamInputs,
        materialization: &mut ForgeQueryDerivedViewMaterialization,
    ) -> Option<ForgeQueryDerivedPatch> {
        let payload = match derived_read_diagnostics_from_query_rows(
            mutation,
            upstreams
                .computed_rows(&self.materialized_view_name)
                .unwrap_or(&[]),
            upstreams
                .computed_rows(&self.interpreted_view_name)
                .unwrap_or(&[]),
            upstreams
                .computed_rows(&self.validation_view_name)
                .unwrap_or(&[]),
        ) {
            Ok(diagnostics) => {
                serde_json::to_value(diagnostics).expect("derived diagnostics must serialize")
            }
            Err(error) => json!({ QUERY_SURFACE_FAILURE_ROW_KEY: error.to_string() }),
        };
        materialization.replace_rows([payload.clone()]);
        Some(ForgeQueryDerivedPatch::whole_refresh_materialized(
            view.name(),
            "topology-diagnostics",
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

#[derive(Debug, Clone)]
pub struct TopologyEquivalenceContractMaintainer {
    diagnostics_view_name: String,
}

impl TopologyEquivalenceContractMaintainer {
    pub fn new(diagnostics_view_name: impl Into<String>) -> Self {
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
                delta.collection,
                view.name(),
            ),
        });
        materialization.replace_rows([payload.clone()]);
        ForgeQueryDerivedPatch::incremental(
            view.name(),
            "topology-equivalence-incremental-unexpected",
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
        _mutation: &ForgeQueryRetainedMutationContext,
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
            "topology-equivalence-contract",
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

pub fn topology_diagnostics_computed_declaration(
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

pub fn topology_equivalence_contract_computed_declaration(
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

pub fn declare_topology_diagnostics_surface<T, M, I, V>(
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

pub fn declare_topology_equivalence_contract_surface<T, D>(
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

pub fn derived_read_diagnostics_from_query_rows(
    mutation: &ForgeQueryRetainedMutationContext,
    materialized_rows: &[Value],
    interpreted_rows: &[Value],
    validation_rows: &[Value],
) -> Result<DerivedReadDiagnostics, TopologyQuerySurfaceError> {
    let evidence = TopologyQueryMutationEvidence::from_mutation(mutation)?;
    let touched_aspects = evidence.touched_aspects()?;
    let materialized: MaterializedTopologyView =
        decode_single_computed_row(materialized_rows, "materialized topology")?;
    let interpreted: InterpretedTopologyView =
        decode_single_computed_row(interpreted_rows, "interpreted topology")?;
    let validation: DerivedTopologyValidationReport =
        decode_single_computed_row(validation_rows, "topology validation")?;

    Ok(DerivedReadDiagnostics {
        invalidation_report: build_derived_invalidation_report_from_aspects(
            touched_aspects.iter().copied(),
        ),
        rebuild_report: crate::projection::diagnostic_surfaces::build_derived_rebuild_report(
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
            crate::projection::diagnostic_surfaces::triggered_invalidation_targets_from_aspects(
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

pub fn equivalence_contract_from_diagnostics_rows(
    diagnostics_rows: &[Value],
) -> Result<DerivedEquivalenceContractReport, TopologyQuerySurfaceError> {
    let diagnostics: DerivedReadDiagnostics =
        decode_single_computed_row(diagnostics_rows, "derived read diagnostics")?;
    Ok(diagnostics.equivalence_contract_report)
}

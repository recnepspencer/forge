use forge_query::facade::{ForgeQueryDerivedArtifactBinding, ForgeQueryWorkspace};

use crate::derived_topology::materialized_graph::MaterializedTopologyView;
use crate::derived_topology::traversal_views::InterpretedTopologyView;
use crate::projection::diagnostic_surfaces::DerivedEquivalenceContractReport;
use crate::projection::diagnostic_surfaces::DerivedReadDiagnostics;
use crate::validation::DerivedTopologyValidationReport;

use super::{
    materialize_declared_query_surface_binding, TopologyDeclaredQuerySurfaces,
    TopologyQuerySurfaceError,
};

const HISTORICAL_TRUTH_ARTIFACT_NAME: &str = "topology.historical.truth";
const HISTORICAL_DERIVED_SNAPSHOT_ARTIFACT_NAME: &str = "topology.historical.derived_snapshot";
const EQUIVALENCE_ALIGNMENT_FIELDS: [(&str, &str); 18] = [
    (
        "equivalence_contract_report.authority_snapshot_id",
        "authority_snapshot_id",
    ),
    (
        "equivalence_contract_report.authority_branch_id",
        "authority_branch_id",
    ),
    (
        "equivalence_contract_report.authoritative_mutation_origin",
        "authoritative_mutation_origin",
    ),
    (
        "equivalence_contract_report.derivation_origin",
        "derivation_origin",
    ),
    (
        "equivalence_contract_report.truth_basis_digest_hex",
        "truth_basis_digest_hex",
    ),
    (
        "equivalence_contract_report.touched_aspect_count",
        "touched_aspect_count",
    ),
    (
        "equivalence_contract_report.triggered_invalidation_targets",
        "triggered_invalidation_targets",
    ),
    (
        "equivalence_contract_report.precision_fallback_count",
        "precision_fallback_count",
    ),
    (
        "equivalence_contract_report.precision_budget_fallback_count",
        "precision_budget_fallback_count",
    ),
    (
        "equivalence_contract_report.materialized_topology_digest.algorithm",
        "materialized_topology_digest.algorithm",
    ),
    (
        "equivalence_contract_report.materialized_topology_digest.digest_hex",
        "materialized_topology_digest.digest_hex",
    ),
    (
        "equivalence_contract_report.materialized_topology_digest.row_count",
        "materialized_topology_digest.row_count",
    ),
    (
        "equivalence_contract_report.interpreted_topology_digest.algorithm",
        "interpreted_topology_digest.algorithm",
    ),
    (
        "equivalence_contract_report.interpreted_topology_digest.digest_hex",
        "interpreted_topology_digest.digest_hex",
    ),
    (
        "equivalence_contract_report.interpreted_topology_digest.row_count",
        "interpreted_topology_digest.row_count",
    ),
    (
        "equivalence_contract_report.derived_validation_digest.algorithm",
        "derived_validation_digest.algorithm",
    ),
    (
        "equivalence_contract_report.derived_validation_digest.digest_hex",
        "derived_validation_digest.digest_hex",
    ),
    (
        "equivalence_contract_report.derived_validation_digest.row_count",
        "derived_validation_digest.row_count",
    ),
];

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TopologyHistoricalTruthArtifact {
    materialized: MaterializedTopologyView,
    interpreted: InterpretedTopologyView,
    validation: DerivedTopologyValidationReport,
}

impl TopologyHistoricalTruthArtifact {
    pub(crate) fn materialized(&self) -> &MaterializedTopologyView {
        &self.materialized
    }

    pub(crate) fn interpreted(&self) -> &InterpretedTopologyView {
        &self.interpreted
    }

    pub(crate) fn validation(&self) -> &DerivedTopologyValidationReport {
        &self.validation
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TopologyHistoricalDerivedSurfaceSnapshot {
    materialized: MaterializedTopologyView,
    interpreted: InterpretedTopologyView,
    validation: DerivedTopologyValidationReport,
    diagnostics: DerivedReadDiagnostics,
    equivalence_contract: DerivedEquivalenceContractReport,
}

impl TopologyHistoricalDerivedSurfaceSnapshot {
    pub(crate) fn materialized(&self) -> &MaterializedTopologyView {
        &self.materialized
    }

    pub(crate) fn interpreted(&self) -> &InterpretedTopologyView {
        &self.interpreted
    }

    pub(crate) fn validation(&self) -> &DerivedTopologyValidationReport {
        &self.validation
    }

    pub(crate) fn diagnostics(&self) -> &DerivedReadDiagnostics {
        &self.diagnostics
    }

    pub(crate) fn equivalence_contract(&self) -> &DerivedEquivalenceContractReport {
        &self.equivalence_contract
    }
}

pub(crate) fn materialize_topology_historical_truth_artifact(
    surfaces: &TopologyDeclaredQuerySurfaces,
    workspace: &mut ForgeQueryWorkspace,
) -> Result<TopologyHistoricalTruthArtifact, TopologyQuerySurfaceError> {
    let bundle = materialize_declared_query_surface_binding(
        workspace,
        HISTORICAL_TRUTH_ARTIFACT_NAME,
        [
            surfaces.materialized().into(),
            surfaces.interpreted().into(),
            surfaces.validation().into(),
        ],
    )?;
    let (materialized, interpreted, validation) = bundle
        .decode_row_triple(
            surfaces.materialized(),
            surfaces.interpreted(),
            surfaces.validation(),
        )
        .map_err(bundle_decode_error)?;
    Ok(TopologyHistoricalTruthArtifact {
        materialized,
        interpreted,
        validation,
    })
}

pub(crate) fn materialize_topology_historical_derived_surface_snapshot(
    surfaces: &TopologyDeclaredQuerySurfaces,
    workspace: &mut ForgeQueryWorkspace,
) -> Result<TopologyHistoricalDerivedSurfaceSnapshot, TopologyQuerySurfaceError> {
    let truth_artifact = materialize_topology_historical_truth_artifact(surfaces, workspace)?;
    let bundle = materialize_declared_query_surface_binding(
        workspace,
        HISTORICAL_DERIVED_SNAPSHOT_ARTIFACT_NAME,
        [
            surfaces.diagnostics().into(),
            surfaces.equivalence_contract().into(),
        ],
    )?;
    let (diagnostics, equivalence_contract) = derived_surface_rows_from_bundle(&bundle, surfaces)?;
    Ok(TopologyHistoricalDerivedSurfaceSnapshot {
        materialized: truth_artifact.materialized().clone(),
        interpreted: truth_artifact.interpreted().clone(),
        validation: truth_artifact.validation().clone(),
        diagnostics,
        equivalence_contract,
    })
}

fn derived_surface_rows_from_bundle(
    bundle: &ForgeQueryDerivedArtifactBinding,
    surfaces: &TopologyDeclaredQuerySurfaces,
) -> Result<(DerivedReadDiagnostics, DerivedEquivalenceContractReport), TopologyQuerySurfaceError> {
    ensure_diagnostics_equivalence_alignment(bundle, surfaces)?;
    bundle
        .decode_row_pair(surfaces.diagnostics(), surfaces.equivalence_contract())
        .map_err(bundle_decode_error)
}

fn ensure_diagnostics_equivalence_alignment(
    bundle: &ForgeQueryDerivedArtifactBinding,
    surfaces: &TopologyDeclaredQuerySurfaces,
) -> Result<(), TopologyQuerySurfaceError> {
    bundle
        .verify_scalar_alignment(
            surfaces.diagnostics(),
            surfaces.equivalence_contract(),
            EQUIVALENCE_ALIGNMENT_FIELDS,
        )
        .map(|_| ())
        .map_err(bundle_decode_error)
}

fn bundle_decode_error(
    error: forge_query::facade::ForgeQueryRuntimeError,
) -> TopologyQuerySurfaceError {
    TopologyQuerySurfaceError::new(error.to_string())
}

use crate::derived_topology::compiled_product_consumer_cutover::DerivedEquivalenceContractReport;
use crate::derived_topology::materialized_graph::MaterializedTopologyView;
use crate::derived_topology::traversal_views::InterpretedTopologyView;
use crate::projection::runtime_boundary::diagnostic_projection::DerivedReadDiagnostics;
use crate::validation::DerivedTopologyValidationReport;

use super::TopologyQuerySurfaceError;

#[cfg(test)]
use forge_query::facade::{ForgeQueryDerivedArtifactBinding, ForgeQueryWorkspace};
#[cfg(test)]
use serde_json::Value;
#[cfg(test)]
use super::{
    decode_query_surface_failure_payload, materialize_declared_query_surface_binding,
    retained_payload, TopologyDeclaredQuerySurfaces, TopologyQuerySurfaceErrorKind,
};

#[cfg(test)]
const HISTORICAL_TRUTH_ARTIFACT_NAME: &str = "topology.historical.truth";
#[cfg(test)]
const HISTORICAL_DERIVED_SNAPSHOT_ARTIFACT_NAME: &str = "topology.historical.derived_snapshot";

#[cfg(test)]
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TopologyHistoricalTruthArtifact {
    materialized: MaterializedTopologyView,
    interpreted: InterpretedTopologyView,
    validation: DerivedTopologyValidationReport,
}

#[cfg(test)]
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

#[cfg(test)]
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
    let materialized = decode_bundle_row(&bundle, surfaces.materialized(), "materialized")?;
    let interpreted = decode_bundle_row(&bundle, surfaces.interpreted(), "interpreted")?;
    let validation = decode_bundle_row(&bundle, surfaces.validation(), "validation")?;
    Ok(TopologyHistoricalTruthArtifact {
        materialized,
        interpreted,
        validation,
    })
}

#[cfg(test)]
pub(crate) fn materialize_topology_historical_derived_surface_snapshot(
    surfaces: &TopologyDeclaredQuerySurfaces,
    workspace: &mut ForgeQueryWorkspace,
) -> Result<TopologyHistoricalDerivedSurfaceSnapshot, TopologyQuerySurfaceError> {
    let truth_artifact = materialize_topology_historical_truth_artifact(surfaces, workspace)?;
    let (diagnostics, equivalence_contract) =
        materialize_topology_historical_derived_reports(surfaces, workspace)?;
    build_topology_historical_derived_surface_snapshot(
        truth_artifact.materialized().clone(),
        truth_artifact.interpreted().clone(),
        truth_artifact.validation().clone(),
        diagnostics,
        equivalence_contract,
    )
}

#[cfg(test)]
pub(crate) fn materialize_topology_historical_derived_reports(
    surfaces: &TopologyDeclaredQuerySurfaces,
    workspace: &mut ForgeQueryWorkspace,
) -> Result<(DerivedReadDiagnostics, DerivedEquivalenceContractReport), TopologyQuerySurfaceError> {
    let bundle = materialize_declared_query_surface_binding(
        workspace,
        HISTORICAL_DERIVED_SNAPSHOT_ARTIFACT_NAME,
        [
            surfaces.diagnostics().into(),
            surfaces.equivalence_contract().into(),
        ],
    )?;
    derived_surface_rows_from_bundle(&bundle, surfaces)
}

pub(crate) fn build_topology_historical_derived_surface_snapshot(
    materialized: MaterializedTopologyView,
    interpreted: InterpretedTopologyView,
    validation: DerivedTopologyValidationReport,
    diagnostics: DerivedReadDiagnostics,
    equivalence_contract: DerivedEquivalenceContractReport,
) -> Result<TopologyHistoricalDerivedSurfaceSnapshot, TopologyQuerySurfaceError> {
    if diagnostics.equivalence_contract_report != equivalence_contract {
        return Err(TopologyQuerySurfaceError::new(
            "derived diagnostics and equivalence contract retained artifacts diverged",
        ));
    }
    Ok(TopologyHistoricalDerivedSurfaceSnapshot {
        materialized,
        interpreted,
        validation,
        diagnostics,
        equivalence_contract,
    })
}

#[cfg(test)]
fn derived_surface_rows_from_bundle(
    bundle: &ForgeQueryDerivedArtifactBinding,
    surfaces: &TopologyDeclaredQuerySurfaces,
) -> Result<(DerivedReadDiagnostics, DerivedEquivalenceContractReport), TopologyQuerySurfaceError> {
    let diagnostics_payload: Value =
        decode_bundle_row(bundle, surfaces.diagnostics(), "diagnostics")?;
    if let Some(error) = decode_query_surface_failure_payload(&diagnostics_payload, "diagnostics") {
        return Err(error);
    }
    let diagnostics: DerivedReadDiagnostics =
        serde_json::from_value(diagnostics_payload).map_err(|error| {
            TopologyQuerySurfaceError::with_kind(
                TopologyQuerySurfaceErrorKind::RetainedPayloadDecodeFailed,
                format!("retained surface `diagnostics` payload failed to decode: {error}"),
            )
        })?;
    let equivalence_contract = decode_bundle_row(
        bundle,
        surfaces.equivalence_contract(),
        "equivalence contract",
    )
    .unwrap_or_else(|_| diagnostics.equivalence_contract_report.clone());
    Ok((diagnostics, equivalence_contract))
}

#[cfg(test)]
fn decode_bundle_row<T>(
    bundle: &ForgeQueryDerivedArtifactBinding,
    view: &forge_query::facade::ForgeQueryDerivedViewHandle<serde_json::Value>,
    label: &str,
) -> Result<T, TopologyQuerySurfaceError>
where
    T: serde::de::DeserializeOwned,
{
    bundle
        .materialization(view)
        .map_err(bundle_decode_error)
        .and_then(|materialization| {
            materialization
                .single_retained_row()
                .map_err(bundle_decode_error)
                .and_then(|row| retained_payload::decode_retained_payload_row(row, label))
        })
}

#[cfg(test)]
fn bundle_decode_error(
    error: forge_query::facade::ForgeQueryRuntimeError,
) -> TopologyQuerySurfaceError {
    TopologyQuerySurfaceError::new(error.to_string())
}

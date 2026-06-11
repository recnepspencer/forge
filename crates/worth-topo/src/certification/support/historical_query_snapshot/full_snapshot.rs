use crate::certification::support::read_basis_query_runtime::HistoricalReadBasisQueryRuntime;
use crate::certification::{DerivedEquivalenceContractReport, NamingAttachmentReport};
use crate::derived_topology::materialized_graph::MaterializedTopologyView;
use crate::derived_topology::traversal_views::InterpretedTopologyView;
use crate::projection::diagnostic_surfaces::DerivedReadDiagnostics;
use crate::projection::runtime_boundary::declared_query_surfaces::truth_surfaces::{
    naming_attachment_report_from_query_input, TopologyNamingAttachmentInput,
};
use crate::projection::runtime_boundary::declared_query_surfaces::{
    read_declared_query_surface_binding, TopologyDeclaredQuerySurfaces, TopologyQuerySurfaceError,
};
use crate::validation::DerivedTopologyValidationReport;

use super::derived_snapshot::historical_derived_surface_snapshot_for_read_basis;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct HistoricalTopologyQuerySnapshot {
    naming_attachments: NamingAttachmentReport,
    materialized: MaterializedTopologyView,
    interpreted: InterpretedTopologyView,
    validation: DerivedTopologyValidationReport,
    diagnostics: DerivedReadDiagnostics,
    equivalence_contract: DerivedEquivalenceContractReport,
}

impl HistoricalTopologyQuerySnapshot {
    pub(crate) fn naming_attachments(&self) -> &NamingAttachmentReport {
        &self.naming_attachments
    }

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

pub(crate) fn historical_query_snapshot_for_read_basis(
    runtime: &mut HistoricalReadBasisQueryRuntime,
) -> Result<HistoricalTopologyQuerySnapshot, TopologyQuerySurfaceError> {
    let surfaces = runtime.surfaces().clone();
    let naming_attachments = historical_naming_attachments(runtime.workspace(), &surfaces)?;
    let derived_snapshot = historical_derived_surface_snapshot_for_read_basis(runtime)?;

    Ok(HistoricalTopologyQuerySnapshot {
        naming_attachments,
        materialized: derived_snapshot.materialized().clone(),
        interpreted: derived_snapshot.interpreted().clone(),
        validation: derived_snapshot.validation().clone(),
        diagnostics: derived_snapshot.diagnostics().clone(),
        equivalence_contract: derived_snapshot.equivalence_contract().clone(),
    })
}

fn historical_naming_attachments(
    workspace: &mut forge_query::facade::ForgeQueryWorkspace,
    surfaces: &TopologyDeclaredQuerySurfaces,
) -> Result<NamingAttachmentReport, TopologyQuerySurfaceError> {
    let binding = read_declared_query_surface_binding(
        workspace,
        "topology.historical.naming_attachments",
        [
            surfaces.entities().into(),
            surfaces.persistent_names().into(),
        ],
    )?;
    let entity_rows = binding
        .read(surfaces.entities())
        .map_err(|error| TopologyQuerySurfaceError::new(error.to_string()))?;
    let persistent_name_rows = binding
        .read(surfaces.persistent_names())
        .map_err(|error| TopologyQuerySurfaceError::new(error.to_string()))?;
    naming_attachment_report_from_query_input(TopologyNamingAttachmentInput::new(
        entity_rows.rows(),
        persistent_name_rows.rows(),
    ))
}

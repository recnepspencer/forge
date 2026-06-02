use schema::facade::topology_authoring::DerivedTopologyReadBasis;

use crate::certification::support::read_basis_query_runtime::HistoricalReadBasisQueryRuntime;
use crate::facade::{
    DerivedEquivalenceContractReport, DerivedReadDiagnostics, DerivedTopologyValidationReport,
    InterpretedTopologyView, MaterializedTopologyView,
};
use crate::projection::runtime_boundary::declared_query_surfaces::TopologyQuerySurfaceError;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct HistoricalDerivedSurfaceSnapshot {
    materialized: MaterializedTopologyView,
    interpreted: InterpretedTopologyView,
    validation: DerivedTopologyValidationReport,
    diagnostics: DerivedReadDiagnostics,
    equivalence_contract: DerivedEquivalenceContractReport,
}

impl HistoricalDerivedSurfaceSnapshot {
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

pub(crate) fn historical_derived_surface_snapshot_for_read_basis(
    runtime: &mut HistoricalReadBasisQueryRuntime,
) -> Result<HistoricalDerivedSurfaceSnapshot, TopologyQuerySurfaceError> {
    let read_basis = runtime.read_basis().clone();
    let equivalence_read_basis_facts = runtime.historical_equivalence_read_basis_facts()?;
    let snapshot = runtime.historical_derived_surface_snapshot()?;
    ensure_snapshot_matches_read_basis(&equivalence_read_basis_facts, &read_basis)?;

    Ok(HistoricalDerivedSurfaceSnapshot {
        materialized: snapshot.materialized().clone(),
        interpreted: snapshot.interpreted().clone(),
        validation: snapshot.validation().clone(),
        diagnostics: snapshot.diagnostics().clone(),
        equivalence_contract: snapshot.equivalence_contract().clone(),
    })
}

fn ensure_snapshot_matches_read_basis(
    equivalence_read_basis_facts: &forge_query::facade::ForgeQueryRetainedScalarFactSet,
    read_basis: &DerivedTopologyReadBasis,
) -> Result<(), TopologyQuerySurfaceError> {
    super::ensure_snapshot_matches_read_basis(equivalence_read_basis_facts, read_basis)?;
    Ok(())
}

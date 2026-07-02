use forge_query::facade::{
    ForgeQueryComputedInspectionEvidence, ForgeQueryInspection, ForgeQueryRuntimeStateKind,
    ForgeQueryWorkspace,
};
use forge_relational::facade::runtime::RelationalRuntime;
use schema::facade::topology_authoring::DerivedTopologyReadBasis;

use crate::certification::MilestoneOneCertificationError;
use crate::derived_topology::compiled_product_consumer_cutover::DerivedEquivalenceContractReport;
use crate::derived_topology::materialized_graph::MaterializedTopologyView;
use crate::derived_topology::traversal_views::InterpretedTopologyView;
use crate::projection::diagnostic_surfaces::DerivedReadDiagnostics;
use crate::projection::planner_owned_routing::diagnostic_projection_input::build_derived_read_diagnostics;
use crate::projection::runtime_boundary::declared_query_surfaces::retained_artifacts::{
    build_topology_historical_derived_surface_snapshot, TopologyHistoricalDerivedSurfaceSnapshot,
};
use crate::projection::runtime_boundary::declared_query_surfaces::{
    declare_topology_query_surfaces, TopologyDeclaredQuerySurfaces, TopologyQuerySurfaceError,
};
use crate::projection::runtime_boundary::query_runtime::{
    topology_runtime, TopologyRuntimeAdapters,
};
use crate::projection::runtime_boundary::read_stage::{
    open_topology_read_view, stage_topology_read_from_view,
};
use crate::validation::validate_named_topology_truth;

pub(crate) struct HistoricalReadBasisQueryRuntime {
    read_basis: DerivedTopologyReadBasis,
    workspace: ForgeQueryWorkspace,
    surfaces: TopologyDeclaredQuerySurfaces,
    materialized: MaterializedTopologyView,
    interpreted: InterpretedTopologyView,
    validation: crate::validation::DerivedTopologyValidationReport,
}

pub(crate) struct HistoricalQuerySurfaceEvidence {
    #[cfg(test)]
    validation_state: forge_query::facade::ForgeQueryRuntimeStateSnapshot,
    #[cfg(test)]
    equivalence_state: forge_query::facade::ForgeQueryRuntimeStateSnapshot,
    validation_inspection: ForgeQueryComputedInspectionEvidence,
    equivalence_inspection: ForgeQueryComputedInspectionEvidence,
}

impl HistoricalQuerySurfaceEvidence {
    #[cfg(test)]
    pub(crate) fn validation_state(&self) -> &forge_query::facade::ForgeQueryRuntimeStateSnapshot {
        &self.validation_state
    }

    #[cfg(test)]
    pub(crate) fn equivalence_state(&self) -> &forge_query::facade::ForgeQueryRuntimeStateSnapshot {
        &self.equivalence_state
    }

    pub(crate) fn validation_inspection(&self) -> &ForgeQueryComputedInspectionEvidence {
        &self.validation_inspection
    }

    pub(crate) fn equivalence_inspection(&self) -> &ForgeQueryComputedInspectionEvidence {
        &self.equivalence_inspection
    }
}

impl HistoricalReadBasisQueryRuntime {
    pub(crate) fn open(
        runtime: &RelationalRuntime,
        read_basis: DerivedTopologyReadBasis,
        workspace_name: &str,
    ) -> Result<Self, MilestoneOneCertificationError> {
        let read_view = open_topology_read_view(runtime, &read_basis)
            .map_err(|error| MilestoneOneCertificationError::ReadView(error.to_string()))?;
        validate_named_topology_truth(&read_view)?;
        let staged = stage_topology_read_from_view(&read_view)
            .map_err(|error| MilestoneOneCertificationError::ReadView(error.to_string()))?;
        let adapters =
            TopologyRuntimeAdapters::snapshot_historical_basis(read_view, read_basis.clone());
        let mut workspace = topology_runtime(adapters, workspace_name)
            .map_err(|error| MilestoneOneCertificationError::Query(error.to_string()))?;
        let surfaces = declare_topology_query_surfaces(&mut workspace)
            .map_err(|error| MilestoneOneCertificationError::Query(error.to_string()))?;
        Ok(Self {
            read_basis,
            workspace,
            surfaces,
            materialized: staged.materialized().clone(),
            interpreted: staged.interpreted().clone(),
            validation: staged.validation().clone(),
        })
    }

    pub(crate) fn read_basis(&self) -> &DerivedTopologyReadBasis {
        &self.read_basis
    }

    pub(crate) fn surfaces(&self) -> &TopologyDeclaredQuerySurfaces {
        &self.surfaces
    }

    pub(crate) fn workspace(&mut self) -> &mut ForgeQueryWorkspace {
        &mut self.workspace
    }

    pub(crate) fn historical_derived_surface_snapshot(
        &mut self,
    ) -> Result<TopologyHistoricalDerivedSurfaceSnapshot, TopologyQuerySurfaceError> {
        let diagnostics = self.local_derived_read_diagnostics();
        let equivalence_contract = diagnostics.equivalence_contract_report.clone();
        build_topology_historical_derived_surface_snapshot(
            self.materialized.clone(),
            self.interpreted.clone(),
            self.validation.clone(),
            diagnostics,
            equivalence_contract,
        )
    }

    pub(crate) fn historical_equivalence_read_basis_facts(
        &mut self,
    ) -> Result<DerivedEquivalenceContractReport, TopologyQuerySurfaceError> {
        Ok(self
            .local_derived_read_diagnostics()
            .equivalence_contract_report)
    }

    pub(crate) fn query_surface_evidence(
        &mut self,
    ) -> Result<HistoricalQuerySurfaceEvidence, MilestoneOneCertificationError> {
        let validation = query_surface_evidence(
            &mut self.workspace,
            self.surfaces.validation(),
            ".topology.validation",
        )?;
        let equivalence = query_surface_evidence(
            &mut self.workspace,
            self.surfaces.equivalence_contract(),
            ".topology.equivalence_contract",
        )?;
        Ok(HistoricalQuerySurfaceEvidence {
            #[cfg(test)]
            validation_state: validation.0,
            validation_inspection: validation.1,
            #[cfg(test)]
            equivalence_state: equivalence.0,
            equivalence_inspection: equivalence.1,
        })
    }
}

impl HistoricalReadBasisQueryRuntime {
    fn local_derived_read_diagnostics(&self) -> DerivedReadDiagnostics {
        build_derived_read_diagnostics(
            &self.read_basis,
            &self.materialized,
            &self.interpreted,
            &self.validation,
        )
    }
}

fn query_surface_evidence<T>(
    workspace: &mut ForgeQueryWorkspace,
    view: &forge_query::facade::ForgeQueryDerivedViewHandle<T>,
    expected_name: &str,
) -> Result<
    (
        forge_query::facade::ForgeQueryRuntimeStateSnapshot,
        ForgeQueryComputedInspectionEvidence,
    ),
    MilestoneOneCertificationError,
> {
    let state = workspace
        .state(view)
        .map_err(|error| MilestoneOneCertificationError::Query(error.to_string()))?;
    ensure_query_surface_ready(expected_name, &state)?;
    let inspection = match workspace
        .inspect(view)
        .map_err(|error| MilestoneOneCertificationError::Query(error.to_string()))?
    {
        ForgeQueryInspection::DerivedView(inspection) => {
            if inspection.name() != expected_name {
                return Err(MilestoneOneCertificationError::Query(format!(
                    "query inspection returned derived surface `{}` while `{expected_name}` was expected",
                    inspection.name()
                )));
            }
            inspection
        }
        other => {
            return Err(MilestoneOneCertificationError::Query(format!(
                "query inspection for `{expected_name}` returned wrong artifact family: {other:?}"
            )));
        }
    };
    Ok((state, inspection))
}

fn ensure_query_surface_ready(
    surface_name: &str,
    state: &forge_query::facade::ForgeQueryRuntimeStateSnapshot,
) -> Result<(), MilestoneOneCertificationError> {
    if state.kind() != ForgeQueryRuntimeStateKind::Ready {
        return Err(MilestoneOneCertificationError::Query(format!(
            "query certification surface `{surface_name}` is `{}` instead of `ready`: {}",
            state.kind(),
            state.explanation()
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests;

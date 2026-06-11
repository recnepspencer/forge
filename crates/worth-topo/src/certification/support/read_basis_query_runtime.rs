use forge_query::facade::{
    ForgeQueryComputedInspectionEvidence, ForgeQueryInspection, ForgeQueryRetainedScalarFactSet,
    ForgeQueryRuntimeStateKind, ForgeQueryWorkspace,
};
use forge_relational::facade::runtime::RelationalRuntime;
use schema::facade::topology_authoring::DerivedTopologyReadBasis;

use crate::certification::MilestoneOneCertificationError;
use crate::projection::runtime_boundary::declared_query_surfaces::retained_artifacts::{
    materialize_topology_historical_derived_surface_snapshot,
    TopologyHistoricalDerivedSurfaceSnapshot,
};
use crate::projection::runtime_boundary::declared_query_surfaces::{
    declare_topology_query_surfaces, materialize_declared_query_surface_binding,
    TopologyDeclaredQuerySurfaces, TopologyQuerySurfaceError,
};
use crate::projection::runtime_boundary::query_runtime::{
    topology_runtime, TopologyRuntimeAdapters,
};
use crate::validation::validate_named_topology_truth;

pub(crate) struct HistoricalReadBasisQueryRuntime {
    read_basis: DerivedTopologyReadBasis,
    workspace: ForgeQueryWorkspace,
    surfaces: TopologyDeclaredQuerySurfaces,
}

const READ_BASIS_EQUIVALENCE_FIELDS: [&str; 6] = [
    "authority_snapshot_id",
    "authority_branch_id",
    "authoritative_mutation_origin",
    "derivation_origin",
    "truth_basis_digest_hex",
    "touched_aspect_count",
];

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
        let read_view = runtime
            .read_truth()
            .read_snapshot(read_basis.snapshot())
            .ok_or_else(|| {
                MilestoneOneCertificationError::ReadView(format!(
                    " certification could not open snapshot {:?}",
                    read_basis.snapshot()
                ))
            })?;
        validate_named_topology_truth(&read_view)?;
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
        materialize_topology_historical_derived_surface_snapshot(
            &self.surfaces,
            &mut self.workspace,
        )
    }

    pub(crate) fn historical_equivalence_read_basis_facts(
        &mut self,
    ) -> Result<ForgeQueryRetainedScalarFactSet, TopologyQuerySurfaceError> {
        materialize_declared_query_surface_binding(
            &mut self.workspace,
            "topology.historical.read_basis_equivalence",
            [self.surfaces.equivalence_contract().into()],
        )?
        .consume_scalar_fields(
            self.surfaces.equivalence_contract(),
            READ_BASIS_EQUIVALENCE_FIELDS,
        )
        .map_err(|error| TopologyQuerySurfaceError::new(error.to_string()))
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

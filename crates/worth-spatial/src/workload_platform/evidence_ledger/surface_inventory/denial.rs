use std::marker::PhantomData;

use super::model::{SpatialEvidenceSurfaceAuthorityCategory, SpatialEvidenceSurfaceDeletionAction};
use super::rows::spatial_evidence_surface_deletion_ledger;
use crate::workload_platform::evidence_ledger::{
    WorkloadEvidenceBacking, WorkloadEvidenceRow, WorkloadEvidenceStage,
};
use topology::facade::{TopologyDeclaredTouchedGraphBasisProof, TopologyTouchedGraphBasis};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialEvidenceTopologySubstitutionSurface {
    TopologyTouchedGraphBasis,
    TopologyDeclaredTouchedGraphBasisProof,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialEvidenceSubstitutionDenial {
    ManualEvidenceRow {
        stage: WorkloadEvidenceStage,
        backing: WorkloadEvidenceBacking,
    },
    TopologyAuthorityCannotSatisfySpatialEvidence {
        surface: SpatialEvidenceTopologySubstitutionSurface,
    },
}

pub fn deny_manual_evidence_row_as_spatial_touch_authority(
    row: &WorkloadEvidenceRow,
) -> Result<(), SpatialEvidenceSubstitutionDenial> {
    if row.backing() == WorkloadEvidenceBacking::Manual {
        return Err(SpatialEvidenceSubstitutionDenial::ManualEvidenceRow {
            stage: row.stage(),
            backing: row.backing(),
        });
    }

    Ok(())
}

pub fn deny_topology_laundering_as_spatial_touch_authority(
    surface: SpatialEvidenceTopologySubstitutionSurface,
) -> SpatialEvidenceSubstitutionDenial {
    let surface_name = match surface {
        SpatialEvidenceTopologySubstitutionSurface::TopologyTouchedGraphBasis => {
            "TopologyTouchedGraphBasis"
        }
        SpatialEvidenceTopologySubstitutionSurface::TopologyDeclaredTouchedGraphBasisProof => {
            "TopologyDeclaredTouchedGraphBasisProof"
        }
    };
    let covered = spatial_evidence_surface_deletion_ledger()
        .iter()
        .any(|row| {
            row.authority_category()
                == SpatialEvidenceSurfaceAuthorityCategory::TopologySubstitutionBoundary
                && row.surface_name() == surface_name
                && row.deletion_action() == SpatialEvidenceSurfaceDeletionAction::CertificationOnly
        });
    debug_assert!(
        covered,
        "topology laundering row missing from deletion ledger"
    );

    SpatialEvidenceSubstitutionDenial::TopologyAuthorityCannotSatisfySpatialEvidence { surface }
}

pub fn deny_topology_touched_graph_basis_as_spatial_touch_authority(
    _: PhantomData<TopologyTouchedGraphBasis>,
) -> SpatialEvidenceSubstitutionDenial {
    deny_topology_laundering_as_spatial_touch_authority(
        SpatialEvidenceTopologySubstitutionSurface::TopologyTouchedGraphBasis,
    )
}

pub fn deny_topology_declared_touched_graph_basis_proof_as_spatial_touch_authority(
    _: PhantomData<TopologyDeclaredTouchedGraphBasisProof>,
) -> SpatialEvidenceSubstitutionDenial {
    deny_topology_laundering_as_spatial_touch_authority(
        SpatialEvidenceTopologySubstitutionSurface::TopologyDeclaredTouchedGraphBasisProof,
    )
}

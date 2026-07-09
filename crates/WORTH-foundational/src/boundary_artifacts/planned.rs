use super::materialization::FoundationalMaterializedBoundaryArtifact;
use super::roles::FoundationalBoundaryArtifactRole;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoundationalPlannedWorkBoundaryArtifactDenial {
    BoundaryRoleMustBePlannedWork,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalPlannedWorkBoundaryArtifact<Surface> {
    artifact: FoundationalMaterializedBoundaryArtifact<Surface>,
}

impl<Surface> FoundationalPlannedWorkBoundaryArtifact<Surface> {
    fn new(artifact: FoundationalMaterializedBoundaryArtifact<Surface>) -> Self {
        Self { artifact }
    }

    pub const fn artifact(&self) -> &FoundationalMaterializedBoundaryArtifact<Surface> {
        &self.artifact
    }

    pub const fn surface(&self) -> &Surface {
        self.artifact.surface()
    }
}

pub fn admit_planned_work_boundary_artifact<Surface>(
    artifact: FoundationalMaterializedBoundaryArtifact<Surface>,
) -> Result<
    FoundationalPlannedWorkBoundaryArtifact<Surface>,
    FoundationalPlannedWorkBoundaryArtifactDenial,
> {
    if artifact.role() != FoundationalBoundaryArtifactRole::PlannedWork {
        return Err(FoundationalPlannedWorkBoundaryArtifactDenial::BoundaryRoleMustBePlannedWork);
    }

    Ok(FoundationalPlannedWorkBoundaryArtifact::new(artifact))
}

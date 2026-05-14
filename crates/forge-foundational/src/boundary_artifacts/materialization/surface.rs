use super::derivation::build_boundary_materialization_plan;
use super::model::{
    FoundationalBoundaryMaterializationDenial, FoundationalBoundaryMaterializationInput,
    FoundationalBoundaryMaterializationPlan, FoundationalMaterializedBoundaryArtifact,
};
use super::vocabulary::{
    FoundationalBoundaryMaterializationSeam, FoundationalBoundaryMaterializationSource,
    FoundationalBoundaryPlanningDenial,
};
use crate::boundary_artifacts::{
    FoundationalAuthoritativeBoundaryClaim, FoundationalBoundaryArtifactCategory,
    FoundationalBoundaryArtifactRole, FoundationalBoundaryCategorySurface,
    FoundationalBoundaryRoleClaim, FoundationalBoundaryRoleMarker,
};
use crate::profiles::MaterializedFoundationalProfileSet;

pub fn plan_descriptive_boundary_materialization<Surface, Role>(
    claim: FoundationalBoundaryRoleClaim<Surface, Role>,
    source: FoundationalBoundaryMaterializationSource,
    seam: FoundationalBoundaryMaterializationSeam,
    profile: MaterializedFoundationalProfileSet,
) -> Result<FoundationalBoundaryMaterializationPlan<Surface>, FoundationalBoundaryPlanningDenial>
where
    Surface: FoundationalBoundaryCategorySurface,
    Role: FoundationalBoundaryRoleMarker,
{
    let (surface, category, role) = claim.into_parts();
    let input =
        FoundationalBoundaryMaterializationInput::new(surface, category, role, source, false);
    build_boundary_materialization_plan(input, seam, profile)
}

pub fn plan_authoritative_boundary_materialization<Surface>(
    claim: FoundationalAuthoritativeBoundaryClaim<Surface>,
    source: FoundationalBoundaryMaterializationSource,
    seam: FoundationalBoundaryMaterializationSeam,
    profile: MaterializedFoundationalProfileSet,
) -> Result<FoundationalBoundaryMaterializationPlan<Surface>, FoundationalBoundaryPlanningDenial>
where
    Surface: FoundationalBoundaryCategorySurface,
{
    let input = FoundationalBoundaryMaterializationInput::new(
        claim.into_surface(),
        FoundationalBoundaryArtifactCategory::Artifact,
        FoundationalBoundaryArtifactRole::AuthoritativeCurrent,
        source,
        true,
    );
    build_boundary_materialization_plan(input, seam, profile)
}

pub fn materialize_descriptive_boundary_surface<Surface, Role>(
    claim: FoundationalBoundaryRoleClaim<Surface, Role>,
    source: FoundationalBoundaryMaterializationSource,
    seam: FoundationalBoundaryMaterializationSeam,
    profile: MaterializedFoundationalProfileSet,
) -> Result<
    FoundationalMaterializedBoundaryArtifact<Surface>,
    FoundationalBoundaryMaterializationDenial,
>
where
    Surface: FoundationalBoundaryCategorySurface,
    Role: FoundationalBoundaryRoleMarker,
{
    plan_descriptive_boundary_materialization(claim, source, seam, profile)
        .map_err(FoundationalBoundaryMaterializationDenial::Planning)?
        .materialize()
}

pub fn materialize_authoritative_boundary_surface<Surface>(
    claim: FoundationalAuthoritativeBoundaryClaim<Surface>,
    source: FoundationalBoundaryMaterializationSource,
    seam: FoundationalBoundaryMaterializationSeam,
    profile: MaterializedFoundationalProfileSet,
) -> Result<
    FoundationalMaterializedBoundaryArtifact<Surface>,
    FoundationalBoundaryMaterializationDenial,
>
where
    Surface: FoundationalBoundaryCategorySurface,
{
    plan_authoritative_boundary_materialization(claim, source, seam, profile)
        .map_err(FoundationalBoundaryMaterializationDenial::Planning)?
        .materialize()
}

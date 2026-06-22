use forge_query::facade::ForgeQueryApplicationFacade;
use worth_spatial::facade::planar_motion_posture::{
    PlanarMotionPostureQueryDomain, PlanarMotionPostureQueryWorld,
};
use worth_spatial::facade::planar_retained_facts::{
    RetainedPlanarFactsQueryDomain, RetainedPlanarFactsQueryWorld,
};
use worth_spatial::facade::planar_structural_identity::{
    PlanarStructuralIdentityQueryDomain, PlanarStructuralIdentityQueryWorld,
};

pub(crate) fn retained_planar_handle(
    world: &'static str,
) -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
    RetainedPlanarFactsQueryDomain,
    RetainedPlanarFactsQueryWorld,
> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(RetainedPlanarFactsQueryDomain)
        .with_operating_context(RetainedPlanarFactsQueryWorld::new(world))
        .validate()
        .expect("validated retained planar facts test domain")
        .admit()
        .expect("admitted retained planar facts test domain")
}

pub(crate) fn motion_posture_handle(
    world: &'static str,
) -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
    PlanarMotionPostureQueryDomain,
    PlanarMotionPostureQueryWorld,
> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(PlanarMotionPostureQueryDomain)
        .with_operating_context(PlanarMotionPostureQueryWorld::new(world))
        .validate()
        .expect("validated motion posture test domain")
        .admit()
        .expect("admitted motion posture test domain")
}

pub(crate) fn structural_identity_handle(
    world: &'static str,
) -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
    PlanarStructuralIdentityQueryDomain,
    PlanarStructuralIdentityQueryWorld,
> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(PlanarStructuralIdentityQueryDomain)
        .with_operating_context(PlanarStructuralIdentityQueryWorld::new(world))
        .validate()
        .expect("validated structural identity test domain")
        .admit()
        .expect("admitted structural identity test domain")
}

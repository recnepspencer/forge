use forge_query::facade::ForgeQueryApplicationFacade;
use worth_spatial::facade::planar_recovery::{
    PlanarRecoveryPostureQueryDomain, PlanarRecoveryPostureQueryWorld,
};

pub(crate) fn recovery_handle(
    world: &'static str,
) -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
    PlanarRecoveryPostureQueryDomain,
    PlanarRecoveryPostureQueryWorld,
> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(PlanarRecoveryPostureQueryDomain)
        .with_operating_context(PlanarRecoveryPostureQueryWorld::new(world))
        .validate()
        .expect("validated planar recovery test domain")
        .admit()
        .expect("admitted planar recovery test domain")
}

use forge_query::facade::ForgeQueryApplicationFacade;
use worth_spatial::facade::planar_clean_fail_boundary::{
    PlanarCleanFailBoundaryQueryDomain, PlanarCleanFailBoundaryQueryWorld,
};
use worth_spatial::facade::planar_diagnostics::{
    PlanarDiagnosticBundleQueryDomain, PlanarDiagnosticBundleQueryWorld,
};
use worth_spatial::facade::planar_recovery::{
    PlanarRecoveryPostureQueryDomain, PlanarRecoveryPostureQueryWorld,
};

pub(crate) fn clean_fail_handle(
    world: &'static str,
) -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
    PlanarCleanFailBoundaryQueryDomain,
    PlanarCleanFailBoundaryQueryWorld,
> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(PlanarCleanFailBoundaryQueryDomain)
        .with_operating_context(PlanarCleanFailBoundaryQueryWorld::new(world))
        .validate()
        .expect("validated clean-fail boundary domain")
        .admit()
        .expect("admitted clean-fail boundary domain")
}

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
        .expect("validated recovery domain")
        .admit()
        .expect("admitted recovery domain")
}

pub(crate) fn diagnostic_handle(
    world: &'static str,
) -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
    PlanarDiagnosticBundleQueryDomain,
    PlanarDiagnosticBundleQueryWorld,
> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(PlanarDiagnosticBundleQueryDomain)
        .with_operating_context(PlanarDiagnosticBundleQueryWorld::new(world))
        .validate()
        .expect("validated diagnostic domain")
        .admit()
        .expect("admitted diagnostic domain")
}

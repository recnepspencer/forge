use forge_query::facade::ForgeQueryApplicationFacade;
use worth_spatial::facade::bindings::{
    PrimitiveBindingQueryDomain, PrimitiveBindingQueryWorld, PrimitiveRebindingQueryDomain,
    PrimitiveRebindingQueryWorld,
};
use worth_spatial::facade::planar_diagnostics::{
    PlanarDiagnosticBundleQueryDomain, PlanarDiagnosticBundleQueryWorld,
};
use worth_spatial::facade::planar_local_rebuild_parity::{
    PlanarLocalRebuildParityQueryDomain, PlanarLocalRebuildParityQueryWorld,
};
use worth_spatial::facade::planar_motion_posture::{
    PlanarMotionPostureQueryDomain, PlanarMotionPostureQueryWorld,
};
use worth_spatial::facade::planar_recovery::{
    PlanarRecoveryPostureQueryDomain, PlanarRecoveryPostureQueryWorld,
};

pub(crate) fn local_rebuild_handle(
    world: &'static str,
) -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
    PlanarLocalRebuildParityQueryDomain,
    PlanarLocalRebuildParityQueryWorld,
> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(PlanarLocalRebuildParityQueryDomain)
        .with_operating_context(PlanarLocalRebuildParityQueryWorld::new(world))
        .validate()
        .expect("validated local rebuild parity domain")
        .admit()
        .expect("admitted local rebuild parity domain")
}

pub(crate) fn binding_handle(
    world: &'static str,
) -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
    PrimitiveBindingQueryDomain,
    PrimitiveBindingQueryWorld,
> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(PrimitiveBindingQueryDomain)
        .with_operating_context(PrimitiveBindingQueryWorld::new(world))
        .validate()
        .expect("validated binding domain")
        .admit()
        .expect("admitted binding domain")
}

pub(crate) fn rebinding_handle(
    world: &'static str,
) -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
    PrimitiveRebindingQueryDomain,
    PrimitiveRebindingQueryWorld,
> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(PrimitiveRebindingQueryDomain)
        .with_operating_context(PrimitiveRebindingQueryWorld::new(world))
        .validate()
        .expect("validated rebinding domain")
        .admit()
        .expect("admitted rebinding domain")
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
        .expect("validated motion posture domain")
        .admit()
        .expect("admitted motion posture domain")
}

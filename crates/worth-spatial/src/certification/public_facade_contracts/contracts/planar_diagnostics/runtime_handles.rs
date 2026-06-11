use forge_query::facade::ForgeQueryApplicationFacade;
use worth_spatial::facade::planar_diagnostics::{
    PlanarDiagnosticBundleQueryDomain, PlanarDiagnosticBundleQueryWorld,
};

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
        .expect("validated planar diagnostic test domain")
        .admit()
        .expect("admitted planar diagnostic test domain")
}

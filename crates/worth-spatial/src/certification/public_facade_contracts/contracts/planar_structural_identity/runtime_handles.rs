use forge_query::facade::ForgeQueryApplicationFacade;
use worth_spatial::facade::planar_structural_identity::{
    PlanarStructuralIdentityQueryDomain, PlanarStructuralIdentityQueryWorld,
};

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

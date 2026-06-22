use forge_query::facade::ForgeQueryApplicationFacade;
use worth_spatial::facade::planar_predicate_consumption::{
    PredicateCertificateConsumptionQueryDomain, PredicateCertificateConsumptionQueryWorld,
};

pub(crate) fn predicate_consumption_handle(
    world: &'static str,
) -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
    PredicateCertificateConsumptionQueryDomain,
    PredicateCertificateConsumptionQueryWorld,
> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(PredicateCertificateConsumptionQueryDomain)
        .with_operating_context(PredicateCertificateConsumptionQueryWorld::new(world))
        .validate()
        .expect("validated predicate consumption domain")
        .admit()
        .expect("admitted predicate consumption domain")
}

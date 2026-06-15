use forge_query::facade::ForgeQueryApplicationFacade;
use worth_spatial::facade::planar_topology_contract::{
    PlanarTopologyContractCompletenessQueryDomain, PlanarTopologyContractCompletenessQueryWorld,
};

pub(crate) fn topology_contract_handle(
    world: &'static str,
) -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
    PlanarTopologyContractCompletenessQueryDomain,
    PlanarTopologyContractCompletenessQueryWorld,
> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(PlanarTopologyContractCompletenessQueryDomain)
        .with_operating_context(PlanarTopologyContractCompletenessQueryWorld::new(world))
        .validate()
        .expect("validated topology contract test domain")
        .admit()
        .expect("admitted topology contract test domain")
}

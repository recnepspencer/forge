use bank_domain::estate::BankEstateWorld;
use bank_domain::schema::BankSchema;
use worth_query_host::facade::primary_graph::{
    WorthQueryPrimaryGraphBootstrap, WorthQueryPrimaryGraphInstallationDenial,
};

mod capability_relations;
mod case_relations;
mod emergency_relations;
mod entities;
mod keys;
mod relation_seed;

pub(super) fn bind_estate_world(
    graph: &mut WorthQueryPrimaryGraphBootstrap<BankSchema>,
    world: &BankEstateWorld,
) -> Result<(), WorthQueryPrimaryGraphInstallationDenial> {
    entities::bind(graph, world)?;
    case_relations::bind(graph, world)?;
    capability_relations::bind(graph, world)?;
    emergency_relations::bind(graph, world)
}

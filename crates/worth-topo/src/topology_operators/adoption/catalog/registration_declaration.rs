use forge_query::facade::consumer_kit::{
    ForgeQueryGraphObligationConsumerKitError,
    ForgeQueryGraphObligationConsumerRegistrationDeclaration,
};
use forge_query::facade::ForgeQueryGraphObligationRegistration;

use super::{
    topology_operator_graph_obligation_catalog, TOPOLOGY_OPERATOR_GRAPH_OBLIGATION_FAMILY,
};

pub fn topology_operator_graph_obligation_registration_declaration() -> Result<
    ForgeQueryGraphObligationConsumerRegistrationDeclaration,
    ForgeQueryGraphObligationConsumerKitError,
> {
    ForgeQueryGraphObligationConsumerRegistrationDeclaration::for_runtime_family(
        TOPOLOGY_OPERATOR_GRAPH_OBLIGATION_FAMILY,
        topology_operator_graph_obligation_catalog().registrations(),
    )
}

pub(crate) fn topology_operator_runtime_graph_obligation_registrations(
) -> Vec<ForgeQueryGraphObligationRegistration> {
    topology_operator_graph_obligation_catalog().runtime_graph_composition_registrations()
}

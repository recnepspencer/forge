use forge_query::facade::consumer_kit::{
    graph_obligation_consumer_kit, ForgeQueryGraphObligationAdoptionProof,
    ForgeQueryGraphObligationConsumerKitError,
};
use forge_query::facade::ForgeQueryGraphObligationOperatingWorldDescriptor;

use super::catalog::{
    topology_operator_graph_obligation_registration_declaration,
    topology_operator_graph_obligation_selector_coverage,
    topology_operator_graph_obligation_support_matrix,
    topology_operator_graph_obligation_support_pin, topology_operator_relation_touch_descriptor,
};
use super::residue::topology_operator_graph_obligation_local_ceremony_audit;
use super::residue::topology_operator_graph_obligation_residue_manifest;

pub fn topology_operator_graph_obligation_adoption_proof(
) -> Result<ForgeQueryGraphObligationAdoptionProof, ForgeQueryGraphObligationConsumerKitError> {
    let touch_descriptor = topology_operator_relation_touch_descriptor()
        .expect("topology operator touch descriptor is static and non-empty");
    graph_obligation_consumer_kit("worth-topo.operator-catalog")
        .register_obligations(topology_operator_graph_obligation_registration_declaration()?)
        .declare_selector_coverage(topology_operator_graph_obligation_selector_coverage())
        .pin_support(topology_operator_graph_obligation_support_pin())
        .against_support_matrix(topology_operator_graph_obligation_support_matrix())
        .audit_local_ceremony(topology_operator_graph_obligation_local_ceremony_audit())
        .account_for_residue(topology_operator_graph_obligation_residue_manifest()?)
        .prove_in_memory_selection(
            &touch_descriptor,
            &ForgeQueryGraphObligationOperatingWorldDescriptor::configured_domain_handle(),
        )?
        .prove_adoption()
}

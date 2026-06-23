use forge_query::facade::consumer_kit::{
    graph_obligation_consumer_kit, ForgeQueryGraphObligationAdoptionProof,
    ForgeQueryGraphObligationConsumerKitError,
};
use forge_query::facade::ForgeQueryGraphObligationOperatingWorldDescriptor;

use super::catalog::{
    primitive_construction_birth_touch_descriptor,
    primitive_construction_graph_obligation_registration_declaration,
    primitive_construction_graph_obligation_selector_coverage,
    primitive_construction_graph_obligation_support_matrix,
    primitive_construction_graph_obligation_support_pin,
};
use super::residue::{
    primitive_construction_graph_obligation_local_ceremony_audit,
    primitive_construction_graph_obligation_residue_manifest,
};

pub(crate) fn primitive_construction_graph_obligation_adoption_proof(
) -> Result<ForgeQueryGraphObligationAdoptionProof, ForgeQueryGraphObligationConsumerKitError> {
    let touch_descriptor = primitive_construction_birth_touch_descriptor()
        .expect("primitive construction birth touch descriptor is static and non-empty");
    graph_obligation_consumer_kit("worth-kernel.primitive-construction")
        .register_obligations(primitive_construction_graph_obligation_registration_declaration()?)
        .declare_selector_coverage(primitive_construction_graph_obligation_selector_coverage())
        .pin_support(primitive_construction_graph_obligation_support_pin())
        .against_support_matrix(primitive_construction_graph_obligation_support_matrix())
        .audit_local_ceremony(primitive_construction_graph_obligation_local_ceremony_audit())
        .account_for_residue(primitive_construction_graph_obligation_residue_manifest()?)
        .prove_in_memory_selection(
            &touch_descriptor,
            &ForgeQueryGraphObligationOperatingWorldDescriptor::configured_domain_handle(),
        )?
        .prove_adoption()
}

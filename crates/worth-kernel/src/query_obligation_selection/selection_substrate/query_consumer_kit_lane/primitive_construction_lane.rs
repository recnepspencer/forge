use forge_query::facade::consumer_kit::{
    graph_obligation_consumer_kit, ForgeQueryGraphObligationExecutionBackedAdoptionProof,
};

use crate::construction::graph_obligation_adoption::{
    primitive_construction_graph_obligation_registration_declaration,
    primitive_construction_graph_obligation_residue_manifest,
    primitive_construction_graph_obligation_selector_coverage,
    primitive_construction_graph_obligation_support_matrix,
    primitive_construction_graph_obligation_support_pin,
};
use crate::query_obligation_selection::selection_substrate::{
    QueryObligationSelectionError, QueryObligationSelectionInput,
};

use super::local_ceremony_audit::selection_substrate_local_ceremony_audit;

const PRIMITIVE_CONSTRUCTION_CONSUMER_NAME: &str =
    "worth-kernel.query-obligation-selection.primitive-construction";

pub fn prove_primitive_construction_query_selection(
    input: &QueryObligationSelectionInput,
) -> Result<ForgeQueryGraphObligationExecutionBackedAdoptionProof, QueryObligationSelectionError> {
    graph_obligation_consumer_kit(PRIMITIVE_CONSTRUCTION_CONSUMER_NAME)
        .register_obligations(primitive_construction_graph_obligation_registration_declaration()?)
        .declare_selector_coverage(primitive_construction_graph_obligation_selector_coverage())
        .pin_support(primitive_construction_graph_obligation_support_pin())
        .against_support_matrix(primitive_construction_graph_obligation_support_matrix())
        .audit_local_ceremony(selection_substrate_local_ceremony_audit())
        .account_for_residue(primitive_construction_graph_obligation_residue_manifest()?)
        .prove_execution_with(input.touch_descriptor(), input.operating_world())?
        .prove_adoption_with_execution()
        .map_err(QueryObligationSelectionError::from)
}

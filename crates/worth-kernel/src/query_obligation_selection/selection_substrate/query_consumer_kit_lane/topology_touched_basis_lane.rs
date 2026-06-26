use forge_query::facade::consumer_kit::{
    graph_obligation_consumer_kit, ForgeQueryGraphObligationExecutionBackedAdoptionProof,
};
use topology::facade::{
    topology_operator_graph_obligation_registration_declaration,
    topology_operator_graph_obligation_residue_manifest,
    topology_operator_graph_obligation_selector_coverage,
    topology_operator_graph_obligation_support_matrix,
    topology_operator_graph_obligation_support_pin,
    TOPOLOGY_PRIMITIVE_CONSTRUCTION_BIRTH_COMPOSE_COLLECTION,
};

use super::local_ceremony_audit::selection_substrate_local_ceremony_audit;
use super::primitive_construction_lane::prove_primitive_construction_query_selection;
use crate::query_obligation_selection::selection_substrate::{
    QueryObligationSelectionError, QueryObligationSelectionInput,
};

const TOPOLOGY_CONSUMER_NAME: &str = "worth-kernel.query-obligation-selection.topology";

enum TopologyTouchedBasisSelectionLane {
    PrimitiveConstructionBirth,
    TopologyOperator,
}

pub fn prove_topology_touched_basis_query_selection(
    input: &QueryObligationSelectionInput,
) -> Result<ForgeQueryGraphObligationExecutionBackedAdoptionProof, QueryObligationSelectionError> {
    match classify_topology_touched_basis_selection_lane(input) {
        TopologyTouchedBasisSelectionLane::PrimitiveConstructionBirth => {
            prove_primitive_construction_query_selection(input)
        }
        TopologyTouchedBasisSelectionLane::TopologyOperator => {
            prove_topology_operator_query_selection(input)
        }
    }
}

fn classify_topology_touched_basis_selection_lane(
    input: &QueryObligationSelectionInput,
) -> TopologyTouchedBasisSelectionLane {
    if input.touch_descriptor().rows().iter().any(|row| {
        row.declared_collection() == Some(TOPOLOGY_PRIMITIVE_CONSTRUCTION_BIRTH_COMPOSE_COLLECTION)
    }) {
        return TopologyTouchedBasisSelectionLane::PrimitiveConstructionBirth;
    }
    TopologyTouchedBasisSelectionLane::TopologyOperator
}

fn prove_topology_operator_query_selection(
    input: &QueryObligationSelectionInput,
) -> Result<ForgeQueryGraphObligationExecutionBackedAdoptionProof, QueryObligationSelectionError> {
    graph_obligation_consumer_kit(TOPOLOGY_CONSUMER_NAME)
        .register_obligations(topology_operator_graph_obligation_registration_declaration()?)
        .declare_selector_coverage(topology_operator_graph_obligation_selector_coverage())
        .pin_support(topology_operator_graph_obligation_support_pin())
        .against_support_matrix(topology_operator_graph_obligation_support_matrix())
        .audit_local_ceremony(selection_substrate_local_ceremony_audit())
        .account_for_residue(topology_operator_graph_obligation_residue_manifest()?)
        .prove_execution_with(input.touch_descriptor(), input.operating_world())?
        .prove_adoption_with_execution()
        .map_err(QueryObligationSelectionError::from)
}

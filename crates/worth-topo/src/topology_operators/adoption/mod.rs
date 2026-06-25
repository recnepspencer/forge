mod catalog;
mod proof;
mod residue;

#[cfg(test)]
#[path = "../adoption_tests/mod.rs"]
mod tests;

pub(crate) use catalog::topology_operator_runtime_graph_obligation_registrations;
pub(crate) use catalog::topology_rewire_loop_successor_registration;
pub use catalog::{
    topology_operator_command_batch_equivalent_touch_descriptor,
    topology_operator_graph_obligation_catalog,
    topology_operator_graph_obligation_registration_declaration,
    topology_operator_graph_obligation_selector_coverage,
    topology_operator_graph_obligation_support_matrix,
    topology_operator_graph_obligation_support_pin, topology_operator_relation_touch_descriptor,
    TopologyOperatorGraphObligationAdoptionStatus, TopologyOperatorGraphObligationCatalog,
    TopologyOperatorGraphObligationCatalogRow, TopologyOperatorGraphObligationLoweringPath,
    TOPOLOGY_OPERATOR_GRAPH_OBLIGATION_FAMILY, TOPOLOGY_OPERATOR_RELATION_COLLECTION,
    TOPOLOGY_REWIRE_LOOP_SUCCESSOR_ASPECT_OPERATION, TOPOLOGY_REWIRE_LOOP_SUCCESSOR_ASPECT_PATH,
};
#[cfg(test)]
pub(crate) use catalog::{
    topology_rewire_loop_successor_aspect_operation, topology_rewire_loop_successor_aspect_touch,
};
pub use proof::topology_operator_graph_obligation_adoption_proof;
pub use residue::topology_operator_graph_obligation_residue_manifest;

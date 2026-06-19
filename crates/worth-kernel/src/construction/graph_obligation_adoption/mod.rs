mod catalog;
#[cfg(test)]
mod family_execution_matrix;
mod proof;
mod residue;
#[cfg(test)]
mod selector_matrix;

#[cfg(test)]
mod tests;

pub(crate) use catalog::{
    primitive_construction_birth_touch_descriptor, primitive_construction_graph_obligation_catalog,
    primitive_construction_graph_obligation_selector_coverage,
    primitive_construction_graph_obligation_support_matrix,
    primitive_construction_graph_obligation_support_pin,
};
#[cfg(test)]
pub(crate) use family_execution_matrix::{
    primitive_construction_graph_obligation_execution_matrix,
    primitive_construction_graph_obligation_replay_pair,
};
pub(crate) use proof::primitive_construction_graph_obligation_adoption_proof;
pub(crate) use residue::{
    primitive_construction_graph_obligation_audit_sources,
    primitive_construction_graph_obligation_local_ceremony_audit,
    primitive_construction_graph_obligation_residue_manifest,
    primitive_construction_phase_eighteen_family_count_gap,
    PHASE_EIGHTEEN_SPEC_PRIMITIVE_FAMILY_COUNT,
};
#[cfg(test)]
pub(crate) use selector_matrix::primitive_construction_graph_obligation_selector_precision_matrix;

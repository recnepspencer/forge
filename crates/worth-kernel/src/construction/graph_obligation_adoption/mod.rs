mod catalog;
#[cfg(test)]
mod family_execution_matrix;
mod residue;
mod selector_matrix;

#[cfg(test)]
mod primitive_construction_touched_basis_fixture;
#[cfg(test)]
mod tests;

#[cfg(test)]
pub(crate) use catalog::{
    primitive_construction_birth_touch_descriptor, primitive_construction_graph_obligation_catalog,
};
pub(crate) use catalog::{
    primitive_construction_graph_obligation_registration_declaration,
    primitive_construction_graph_obligation_selector_coverage,
    primitive_construction_graph_obligation_support_matrix,
    primitive_construction_graph_obligation_support_pin,
};
#[cfg(test)]
pub(crate) use family_execution_matrix::{
    primitive_construction_graph_obligation_execution_matrix,
    primitive_construction_graph_obligation_replay_pair,
};
pub(crate) use residue::{
    primitive_construction_family_cardinality_closeout,
    primitive_construction_graph_obligation_residue_contract,
    primitive_construction_graph_obligation_residue_manifest,
};
#[cfg(test)]
pub(crate) use residue::{
    primitive_construction_graph_obligation_audit_sources,
    primitive_construction_graph_obligation_local_ceremony_audit,
    primitive_construction_phase_eighteen_family_count_gap,
    PHASE_EIGHTEEN_SPEC_PRIMITIVE_FAMILY_COUNT,
};
pub(crate) use selector_matrix::primitive_construction_graph_obligation_selector_precision_matrix;

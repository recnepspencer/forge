mod adoption_inventory;
mod adoption_residue_report;
mod query_boundary_sources;
mod residue_assertions;
mod seeded_query_bypass;

pub(crate) use adoption_inventory::{
    evaluate_reference_support_pins, evaluate_test_backend_residue_audit,
    reference_consumer_enforcement_adoption_report, test_backend_adoption_posture,
    worth_domain_hygiene_classification_report, TestBackendAdoptionPosture,
};
pub(crate) use adoption_residue_report::ReferenceConsumerAdoptionResidueReport;
pub(crate) use query_boundary_sources::{
    worth_kernel_query_boundary_inventory, worth_kernel_query_boundary_source_count,
    worth_kernel_query_boundary_sources,
};
pub(crate) use residue_assertions::{
    assert_no_hand_assembled_test_backend_residue, assert_no_query_enforcement_folklore_residue,
};
pub(crate) use seeded_query_bypass::seeded_query_bypass_source_sets;

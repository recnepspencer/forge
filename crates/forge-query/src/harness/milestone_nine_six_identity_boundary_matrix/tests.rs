use crate::application::MILESTONE_NINE_SIX_SUITE_NAME;

use super::{
    identity_boundary_hostile_matrix_digest, MILESTONE_NINE_SIX_REQUIRED_CANONICAL_ROW_NAMES,
    MILESTONE_NINE_SIX_REQUIRED_REJECTION_ROW_NAMES,
};

#[test]
fn milestone_nine_six_adapter_emits_named_matrix() {
    assert_eq!(
        MILESTONE_NINE_SIX_SUITE_NAME,
        "Milestone 9.6 Identity And Stop-Class Hostile Certification Matrix"
    );
    assert!(!identity_boundary_hostile_matrix_digest().is_empty());
}

#[test]
fn milestone_nine_six_matrix_exports_required_row_names() {
    assert_eq!(MILESTONE_NINE_SIX_REQUIRED_CANONICAL_ROW_NAMES.len(), 5);
    assert_eq!(MILESTONE_NINE_SIX_REQUIRED_REJECTION_ROW_NAMES.len(), 2);
}

#[test]
fn milestone_nine_six_matrix_digest_is_stable() {
    let first = identity_boundary_hostile_matrix_digest();
    let second = identity_boundary_hostile_matrix_digest();
    assert_eq!(first, second);
}

#[test]
fn milestone_nine_six_requirements_match_exported_row_names() {
    let requirements = crate::harness::certification::milestone_nine_six_requirements();
    assert_eq!(requirements.suite_name, MILESTONE_NINE_SIX_SUITE_NAME);
    assert_eq!(
        requirements.required_canonical_rows,
        MILESTONE_NINE_SIX_REQUIRED_CANONICAL_ROW_NAMES
    );
    assert_eq!(
        requirements.required_rejection_rows,
        MILESTONE_NINE_SIX_REQUIRED_REJECTION_ROW_NAMES
    );
}

use super::*;
use crate::tests::milestone_6_certification::{
    suite_rows_foundation::*,
    suite_rows_overlap_and_corruption::*,
    suite_rows_rebuild::*,
};

fn canonical_row_by_name<'a, T: Eq + serde::Serialize, E: Eq + serde::Serialize>(
    suite: &'a CertificationSuite<T, E>,
    name: &str,
) -> &'a CanonicalRow<T> {
    suite
        .canonical_rows()
        .iter()
        .find(|row| row.name() == name)
        .unwrap_or_else(|| panic!("missing canonical row `{name}`"))
}

fn rejection_row_by_name<'a, T: Eq + serde::Serialize, E: Eq + serde::Serialize>(
    suite: &'a CertificationSuite<T, E>,
    name: &str,
) -> &'a RejectionRow<E> {
    suite
        .rejection_rows()
        .iter()
        .find(|row| row.name() == name)
        .unwrap_or_else(|| panic!("missing rejection row `{name}`"))
}

pub(super) fn milestone_6_suite() -> CertificationSuite<String, String> {
    CertificationSuite::new(ASPECT_LAYOUT_PHYSICAL_CERTIFICATION_TEST.suite_name)
        .with_canonical_row(CanonicalRow::new(
            "admitted_layout_truth_parity",
            admitted_truth_parity(),
            &[AssertionClass::Equality],
        ))
        .with_canonical_row(CanonicalRow::new(
            "admitted_layout_counter_contract_parity",
            admitted_counter_parity(),
            &[AssertionClass::Equality, AssertionClass::ExactCounter],
        ))
        .with_canonical_row(CanonicalRow::new(
            "admitted_layout_artifact_parity",
            admitted_artifact_parity(),
            &[AssertionClass::Equality],
        ))
        .with_canonical_row(CanonicalRow::new(
            "authority_rebuild_preserves_layout_identity",
            authority_rebuild_parity(),
            &[AssertionClass::Equality],
        ))
        .with_canonical_row(CanonicalRow::new(
            "authority_rebuild_preserves_execution_surfaces",
            authority_rebuild_execution_parity(),
            &[AssertionClass::Equality],
        ))
        .with_canonical_row(CanonicalRow::new(
            "dedup_control_overlap_branch_parity",
            dedup_control_overlap_branch_parity(),
            &[AssertionClass::Equality],
        ))
        .with_canonical_row(CanonicalRow::new(
            "chunk_export_rebuild_parity",
            chunk_export_rebuild_parity(),
            &[AssertionClass::Equality],
        ))
        .with_canonical_row(CanonicalRow::new(
            "sqlite_legacy_seed_migration_parity",
            sqlite_legacy_seed_migration_parity(),
            &[AssertionClass::Equality],
        ))
        .with_canonical_row(CanonicalRow::new(
            "scope_shape_changes_physical_truth",
            scope_shape_divergence(),
            &[AssertionClass::Inequality],
        ))
        .with_rejection_row(RejectionRow::new(
            "generalized_scope_requires_explicit_fallback",
            generalized_scope_rejection(),
            &[AssertionClass::TypedFailure, AssertionClass::ExactCounter],
        ))
        .with_rejection_row(RejectionRow::new(
            "commit_coupled_seed_corruption_requires_typed_failure",
            commit_coupled_seed_corruption(),
            &[AssertionClass::TypedFailure],
        ))
        .with_rejection_row(RejectionRow::new(
            "chunk_export_corruption_requires_typed_failure",
            chunk_export_corruption(),
            &[AssertionClass::TypedFailure],
        ))
        .with_rejection_row(RejectionRow::new(
            "chunk_export_boundary_mismatch_requires_typed_failure",
            chunk_export_boundary_mismatch(),
            &[AssertionClass::TypedFailure],
        ))
}

#[test]
fn milestone_6_certification_harness_scaffolds_layout_suite() {
    let suite = milestone_6_suite();
    assert_all_equal(canonical_row_by_name(
        &suite,
        "admitted_layout_truth_parity",
    ));
    assert_all_equal(canonical_row_by_name(
        &suite,
        "admitted_layout_counter_contract_parity",
    ));
    assert_all_equal(canonical_row_by_name(
        &suite,
        "admitted_layout_artifact_parity",
    ));
    assert_all_equal(canonical_row_by_name(
        &suite,
        "authority_rebuild_preserves_layout_identity",
    ));
    assert_all_equal(canonical_row_by_name(
        &suite,
        "authority_rebuild_preserves_execution_surfaces",
    ));
    assert_all_equal(canonical_row_by_name(
        &suite,
        "dedup_control_overlap_branch_parity",
    ));
    assert_all_equal(canonical_row_by_name(
        &suite,
        "chunk_export_rebuild_parity",
    ));
    assert_all_equal(canonical_row_by_name(
        &suite,
        "sqlite_legacy_seed_migration_parity",
    ));
    assert_any_not_equal(canonical_row_by_name(
        &suite,
        "scope_shape_changes_physical_truth",
    ));
    assert_rejection_payloads_present(rejection_row_by_name(
        &suite,
        "generalized_scope_requires_explicit_fallback",
    ));
    assert_rejection_payloads_present(rejection_row_by_name(
        &suite,
        "commit_coupled_seed_corruption_requires_typed_failure",
    ));
    assert_rejection_payloads_present(rejection_row_by_name(
        &suite,
        "chunk_export_corruption_requires_typed_failure",
    ));
    assert_rejection_payloads_present(rejection_row_by_name(
        &suite,
        "chunk_export_boundary_mismatch_requires_typed_failure",
    ));

    let completeness =
        evaluate_completeness(&suite, &ASPECT_LAYOUT_PHYSICAL_CERTIFICATION_TEST);
    assert!(
        completeness.missing_rows().is_empty()
            && completeness.missing_assertion_classes().is_empty(),
        "milestone 6 layout certification suite is incomplete: {:?}",
        completeness
    );
}

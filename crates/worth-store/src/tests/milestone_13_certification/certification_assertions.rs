use crate::WORTHStoreBuilder;

use super::super::harness::certification::{
    assertions::{assert_all_equal, assert_any_not_equal},
    requirements::{evaluate_completeness, TIERING_AND_WORKING_SET_NON_AUTHORITY_TEST},
};
use super::suite::milestone_13_suite;
use super::tiering_lanes::execute_tiering_batch;
use super::world::build_store;

#[test]
fn milestone_13_certification_suite_is_complete_and_truth_equal() {
    let suite = milestone_13_suite();
    assert_all_equal(&suite.canonical_rows()[0]);
    assert_all_equal(&suite.canonical_rows()[1]);
    let completeness = evaluate_completeness(&suite, &TIERING_AND_WORKING_SET_NON_AUTHORITY_TEST);
    assert!(completeness.missing_rows().is_empty());
    assert!(completeness.missing_assertion_classes().is_empty());
}

#[test]
fn milestone_13_certification_diagnostics_diverge_while_truth_stays_equal() {
    let suite = milestone_13_suite();
    assert_all_equal(&suite.canonical_rows()[0]);
    assert_all_equal(&suite.canonical_rows()[1]);
    assert_any_not_equal(&suite.canonical_rows()[2]);
}

#[test]
fn milestone_13_certification_counters_match_expected_batch() {
    let suite = milestone_13_suite();
    assert_all_equal(&suite.canonical_rows()[3]);
}

#[test]
fn milestone_13_certification_bundle_summary_flags_are_adversarially_meaningful() {
    let (control_store, _) = build_store(WORTHStoreBuilder::new().in_memory());
    let control_export = control_store.export_authoritative_records();
    let control_bundle = control_store
        .milestone_13_certification_bundle(&control_export)
        .unwrap();

    let (mut moved_store, moved_snapshot_id) = build_store(WORTHStoreBuilder::new().in_memory());
    execute_tiering_batch(&mut moved_store, moved_snapshot_id);
    let moved_bundle = moved_store
        .milestone_13_certification_bundle(&control_export)
        .unwrap();

    assert!(
        moved_bundle
            .certification_summary
            .truth_matches_control_lane
    );
    assert!(
        moved_bundle
            .certification_summary
            .no_tier_truth_parity_failures
    );
    assert!(
        moved_bundle
            .certification_summary
            .no_tier_restore_parity_failures
    );
    assert!(moved_bundle.certification_summary.no_tier_recall_failures);
    assert!(
        moved_bundle
            .certification_summary
            .no_residual_residency_ambiguity
    );
    assert_eq!(
        moved_bundle
            .artifact_report
            .residual_residency_ambiguity_count,
        0
    );
    assert_eq!(moved_bundle.truth_digest, control_bundle.truth_digest);
    assert_eq!(moved_bundle.artifact_digest, control_bundle.artifact_digest);
    assert_ne!(
        moved_bundle.diagnostics_digest,
        control_bundle.diagnostics_digest
    );
    assert!(
        moved_bundle.certification_summary.verified_path_count > 0,
        "certification summary should count verified paths"
    );
    assert!(
        moved_bundle.certification_summary.debt_path_count > 0,
        "coalesced-only moved lane should keep unexercised recall execution explicit as debt"
    );
}

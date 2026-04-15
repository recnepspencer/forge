use crate::{ForgeStoreBuilder, StoreErrorKind};

use super::harness::{
    certification::{
        assertions::assert_all_equal,
        core::{AssertionClass, CanonicalRow, CertificationSuite},
        lanes::run_store_lanes,
        requirements::{
            evaluate_completeness, ADVERSARIAL_CRASH_RECOVERY_SOURCE_PRECEDENCE_TEST,
            DURABLE_MEDIA_WRITE_PATH_CERTIFICATION_TEST,
            SNAPSHOT_PLUS_TAIL_RESTORE_EQUIVALENCE_TEST,
        },
    },
    corruption::authoritative::corrupt_local_file_commit_digest,
    fixtures::{
        artifacts::append_two_mainline_commits,
        stores::{build_store_for_lane, unique_test_store_path, StoreLane},
    },
};

#[test]
fn lane_runner_executes_store_scenarios_across_backend_families() {
    let rows = run_store_lanes(
        &[StoreLane::InMemory, StoreLane::LocalFile, StoreLane::Sqlite],
        |lane| {
            let mut store = build_store_for_lane(lane, "forge-store-harness-lane-runner");
            append_two_mainline_commits(&mut store);
            store.export_authoritative_records().canonical_json()
        },
    );
    let row = CanonicalRow::new("authority_parity", rows, &[AssertionClass::Equality]);
    assert_all_equal(&row);
}

#[test]
fn requirements_registry_reports_missing_rows_and_assertions_deterministically() {
    let suite: CertificationSuite<String, String> =
        CertificationSuite::new(SNAPSHOT_PLUS_TAIL_RESTORE_EQUIVALENCE_TEST.suite_name)
            .with_canonical_row(CanonicalRow::new(
                "restore_rebuild_equivalence",
                vec![],
                &[AssertionClass::Equality],
            ));

    let completeness = evaluate_completeness(&suite, &SNAPSHOT_PLUS_TAIL_RESTORE_EQUIVALENCE_TEST);
    assert_eq!(
        completeness.missing_rows(),
        vec![
            "backend_variation_delete_rebuild".to_string(),
            "typed_snapshot_failure".to_string()
        ]
    );
    assert_eq!(
        completeness.missing_assertion_classes(),
        &[AssertionClass::TypedFailure, AssertionClass::ExactCounter]
    );
}

#[test]
fn corruption_modules_mutate_intended_artifact_scope() {
    let path = unique_test_store_path("forge-store-harness-corruption");
    {
        let mut store = ForgeStoreBuilder::new()
            .local_file(path.clone())
            .build()
            .unwrap();
        append_two_mainline_commits(&mut store);
    }

    corrupt_local_file_commit_digest(&path);
    let error = ForgeStoreBuilder::new()
        .local_file(path)
        .build()
        .unwrap_err();
    assert_eq!(error.kind(), &StoreErrorKind::BackendIntegrityViolation);
}

#[test]
fn certification_core_supports_named_rows_for_future_publication_suites() {
    let suite: CertificationSuite<String, String> =
        CertificationSuite::new(DURABLE_MEDIA_WRITE_PATH_CERTIFICATION_TEST.suite_name)
            .with_canonical_row(CanonicalRow::new(
                "publication_family_equivalence",
                vec![],
                &[AssertionClass::Equality],
            ));
    let completeness = evaluate_completeness(&suite, &DURABLE_MEDIA_WRITE_PATH_CERTIFICATION_TEST);
    assert_eq!(
        completeness.missing_rows(),
        vec![
            "publication_gap_classification".to_string(),
            "typed_media_failures".to_string()
        ]
    );
}

#[test]
fn certification_core_supports_named_rows_for_future_recovery_precedence_suites() {
    let suite: CertificationSuite<String, String> =
        CertificationSuite::new(ADVERSARIAL_CRASH_RECOVERY_SOURCE_PRECEDENCE_TEST.suite_name)
            .with_canonical_row(CanonicalRow::new(
                "authoritative_truth_outranks_residue",
                vec![],
                &[AssertionClass::Equality],
            ));
    let completeness =
        evaluate_completeness(&suite, &ADVERSARIAL_CRASH_RECOVERY_SOURCE_PRECEDENCE_TEST);
    assert_eq!(
        completeness.missing_rows(),
        vec![
            "interrupted_snapshot_publication".to_string(),
            "retained_without_ack_lane".to_string(),
            "quiescent_second_restart".to_string(),
            "quarantine_required_lane".to_string(),
        ]
    );
}

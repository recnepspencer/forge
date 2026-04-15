use crate::{
    evidence::{Milestone3CertificationBundle, ObservedRecoveryFailure},
    ForgeStoreBuilder, StoreErrorKind,
};

use super::harness::{
    certification::{
        assertions::{assert_all_equal, assert_rejection_payloads_present},
        core::{AssertionClass, CanonicalRow, CertificationSuite, LaneResult, RejectionRow},
        requirements::{evaluate_completeness, WAL_CRASH_BOUNDARY_EXACTNESS_TEST},
    },
    corruption::wal::corrupt_first_sqlite_wal_record_digest,
    fixtures::{runtime::runtime_with_demo_schema, stores::unique_test_sqlite_path},
    scenarios::recovery::recovery_and_rebuild_equivalence,
};

fn stable_failure_digest(failures: &[ObservedRecoveryFailure]) -> String {
    use sha2::{Digest, Sha256};

    let bytes = serde_json::to_vec(failures).expect("failure digest serialization");
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}

fn milestone_3_suite() -> CertificationSuite<String, String> {
    let scenario = recovery_and_rebuild_equivalence();
    let recovered_export = scenario.recovered.store().export_authoritative_records();
    let path = unique_test_sqlite_path("forge-store-m3-failure-certification");
    {
        let mut durable = ForgeStoreBuilder::new()
            .sqlite_file(path.clone())
            .durable_mode(runtime_with_demo_schema())
            .build()
            .unwrap();
        durable
            .execute_mutation(crate::DurableMutationRequest::new(
                "create-alpha",
                super::harness::scenarios::recovery::create_alpha_commit,
            ))
            .unwrap();
    }
    corrupt_first_sqlite_wal_record_digest(&path);
    let failure = ForgeStoreBuilder::new()
        .sqlite_file(path)
        .durable_mode(runtime_with_demo_schema())
        .build_pending()
        .unwrap_err();
    let observed_failure = ObservedRecoveryFailure::from_error(&failure);

    CertificationSuite::new(WAL_CRASH_BOUNDARY_EXACTNESS_TEST.suite_name)
        .with_canonical_row(CanonicalRow::new(
            "recovery_rebuild_equivalence",
            vec![
                LaneResult::new("recovered", recovered_export.canonical_json()),
                LaneResult::new("rebuild", scenario.rebuilt_export_json),
            ],
            &[AssertionClass::Equality, AssertionClass::ExactCounter],
        ))
        .with_rejection_row(RejectionRow::new(
            "typed_recovery_failure",
            vec![LaneResult::new(
                "corrupted_sqlite",
                stable_failure_digest(std::slice::from_ref(&observed_failure)),
            )],
            &[AssertionClass::TypedFailure],
        ))
}

#[test]
fn milestone_3_certification_bundle_proves_recovery_and_rebuild_equivalence() {
    let scenario = recovery_and_rebuild_equivalence();
    let recovered_export = scenario.recovered.store().export_authoritative_records();
    let rebuilt = crate::ForgeStore::restore_from_authoritative_export(
        recovered_export.clone().admit_restore(),
    )
    .expect("rebuild should succeed");
    let rebuilt_export = rebuilt.export_authoritative_records();

    let bundle = Milestone3CertificationBundle::new(
        &recovered_export,
        &rebuilt_export,
        scenario.recovered.store().counters(),
        &[],
    );

    assert_eq!(bundle.truth_digest, bundle.restore_digest);
    assert_eq!(bundle.failure_digest, stable_failure_digest(&[]));
    assert_eq!(bundle.counter_snapshot.durable_commit_recovered_count, 1);
    assert_eq!(
        bundle
            .counter_snapshot
            .durable_commit_duplicate_suppression_count,
        1
    );
    assert!(bundle.canonical_json().contains(&bundle.truth_digest));

    let suite: CertificationSuite<String, String> = CertificationSuite::new(
        WAL_CRASH_BOUNDARY_EXACTNESS_TEST.suite_name,
    )
    .with_canonical_row(CanonicalRow::new(
        "recovery_rebuild_equivalence",
        vec![
            LaneResult::new("recovered", recovered_export.canonical_json()),
            LaneResult::new("rebuild", rebuilt_export.canonical_json()),
        ],
        &[AssertionClass::Equality, AssertionClass::ExactCounter],
    ));
    assert_all_equal(&suite.canonical_rows()[0]);
}

#[test]
fn milestone_3_certification_bundle_captures_typed_recovery_failure() {
    let path = unique_test_sqlite_path("forge-store-m3-failure-certification");
    {
        let mut durable = ForgeStoreBuilder::new()
            .sqlite_file(path.clone())
            .durable_mode(runtime_with_demo_schema())
            .build()
            .unwrap();
        durable
            .execute_mutation(crate::DurableMutationRequest::new(
                "create-alpha",
                super::harness::scenarios::recovery::create_alpha_commit,
            ))
            .unwrap();
    }

    corrupt_first_sqlite_wal_record_digest(&path);

    let failure = ForgeStoreBuilder::new()
        .sqlite_file(path)
        .durable_mode(runtime_with_demo_schema())
        .build_pending()
        .expect_err("corrupted wal should fail before recovery starts");
    assert_eq!(failure.kind(), &StoreErrorKind::WalDigestMismatch);

    let observed_failure = ObservedRecoveryFailure::from_error(&failure);
    let empty_store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    let export = empty_store.export_authoritative_records();
    let bundle = Milestone3CertificationBundle::new(
        &export,
        &export,
        empty_store.counters(),
        std::slice::from_ref(&observed_failure),
    );

    assert_eq!(
        bundle.failure_digest,
        stable_failure_digest(std::slice::from_ref(&observed_failure))
    );
    assert_ne!(bundle.failure_digest, stable_failure_digest(&[]));
    assert!(bundle.canonical_json().contains(&bundle.failure_digest));

    let suite = milestone_3_suite();
    assert_rejection_payloads_present(&suite.rejection_rows()[0]);
    let completeness = evaluate_completeness(&suite, &WAL_CRASH_BOUNDARY_EXACTNESS_TEST);
    assert!(completeness.missing_rows().is_empty());
    assert!(completeness.missing_assertion_classes().is_empty());
}

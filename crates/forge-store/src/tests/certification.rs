use crate::{ForgeStore, ForgeStoreBuilder};

use super::harness::{
    certification::{
        assertions::{assert_all_equal, assert_any_not_equal},
        core::{AssertionClass, CanonicalRow, CertificationSuite, LaneResult},
        requirements::{evaluate_completeness, DURABLE_ARTIFACT_AUTHORITY_EQUIVALENCE_TEST},
    },
    fixtures::{
        runtime::{
            create_entity, latest_envelope, runtime_with_demo_schema, update_entity_on_branch,
        },
        stores::{unique_test_sqlite_path, unique_test_store_path},
    },
};

fn milestone_1_suite() -> CertificationSuite<String, String> {
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let first = latest_envelope(&runtime);
    update_entity_on_branch(&mut runtime, entity_id, "beta", None);
    let second = latest_envelope(&runtime);

    let mut in_memory_store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    in_memory_store
        .append_canonical_commit(first.clone())
        .unwrap();
    in_memory_store
        .append_canonical_commit(second.clone())
        .unwrap();

    let file_bundle = {
        let mut file_store = ForgeStoreBuilder::new()
            .local_file(unique_test_store_path("forge-store-certification"))
            .build()
            .unwrap();
        file_store.append_canonical_commit(first.clone()).unwrap();
        file_store.append_canonical_commit(second.clone()).unwrap();
        (
            file_store.export_authoritative_records().canonical_json(),
            file_store
                .milestone_1_certification_bundle()
                .semantic_json(),
        )
    };

    let sqlite_bundle = {
        let mut sqlite_store = ForgeStoreBuilder::new()
            .sqlite_file(unique_test_sqlite_path("forge-store-certification-sqlite"))
            .build()
            .unwrap();
        sqlite_store.append_canonical_commit(first.clone()).unwrap();
        sqlite_store
            .append_canonical_commit(second.clone())
            .unwrap();
        (
            sqlite_store.export_authoritative_records().canonical_json(),
            sqlite_store
                .milestone_1_certification_bundle()
                .semantic_json(),
        )
    };

    let rebuilt = ForgeStore::restore_from_authoritative_export(
        in_memory_store
            .export_authoritative_records()
            .admit_restore(),
    )
    .unwrap();

    let in_memory_export_json = in_memory_store
        .export_authoritative_records()
        .canonical_json();
    let in_memory_bundle = in_memory_store.milestone_1_certification_bundle();
    let in_memory_bundle_json = in_memory_bundle.semantic_json();
    let rebuilt_bundle = rebuilt.milestone_1_certification_bundle();

    CertificationSuite::new(DURABLE_ARTIFACT_AUTHORITY_EQUIVALENCE_TEST.suite_name)
        .with_canonical_row(CanonicalRow::new(
            "semantic_parity",
            vec![
                LaneResult::new("in_memory", in_memory_bundle_json.clone()),
                LaneResult::new("local_file", file_bundle.1.clone()),
                LaneResult::new("sqlite", sqlite_bundle.1.clone()),
                LaneResult::new("rebuild", rebuilt_bundle.semantic_json()),
            ],
            &[AssertionClass::Equality],
        ))
        .with_canonical_row(CanonicalRow::new(
            "export_json_parity",
            vec![
                LaneResult::new("in_memory", in_memory_export_json.clone()),
                LaneResult::new("local_file", file_bundle.0),
                LaneResult::new("sqlite", sqlite_bundle.0),
                LaneResult::new(
                    "rebuild",
                    rebuilt.export_authoritative_records().canonical_json(),
                ),
            ],
            &[AssertionClass::Equality],
        ))
        .with_canonical_row(CanonicalRow::new(
            "lane_local_counter_divergence",
            vec![
                LaneResult::new(
                    "operational",
                    serde_json::to_string(&in_memory_bundle.counter_snapshot).unwrap(),
                ),
                LaneResult::new(
                    "rebuild",
                    serde_json::to_string(&rebuilt_bundle.counter_snapshot).unwrap(),
                ),
            ],
            &[AssertionClass::Inequality],
        ))
}

#[test]
fn durable_artifact_authority_equivalence_bundle_matches_across_backend_and_rebuild_lanes() {
    let suite = milestone_1_suite();
    assert_all_equal(&suite.canonical_rows()[0]);
    let completeness = evaluate_completeness(&suite, &DURABLE_ARTIFACT_AUTHORITY_EQUIVALENCE_TEST);
    assert!(completeness.missing_rows().is_empty());
    assert!(completeness.missing_assertion_classes().is_empty());
}

#[test]
fn authoritative_export_and_certification_json_are_identical_across_equivalent_lanes() {
    let suite = milestone_1_suite();
    assert_all_equal(&suite.canonical_rows()[0]);
    assert_all_equal(&suite.canonical_rows()[1]);
}

#[test]
fn certification_counters_remain_lane_local_while_semantic_evidence_stays_equal() {
    let suite = milestone_1_suite();
    assert_any_not_equal(&suite.canonical_rows()[2]);
}

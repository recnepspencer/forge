use super::support::counter_value;
use super::*;

#[test]
fn traced_certification_read_view_surfaces_schema_owned_trace() {
    let mut runtime = crate::facade::worth_milestone_one_runtime_builder()
        .expect("worth milestone one runtime builder")
        .build();

    let seeded =
        seeded_bootstrap(&mut runtime, "cert-traced-surface").expect("seed worth topology");
    let read_view = runtime
        .read_truth()
        .read_snapshot(&seeded.snapshot)
        .expect("worth snapshot read");

    let traced = certify_milestone_one_read_view_traced(&read_view, seeded.read_basis.clone())
        .expect("traced milestone one certification");

    assert_eq!(
        traced.integrity_markers().truth_basis_identity,
        Some(seeded.read_basis.authority.truth_basis_identity.clone())
    );
    assert_eq!(
        traced
            .decision_trace()
            .derived
            .as_ref()
            .expect("derived trace")
            .invalidation_target_count,
        traced
            .primary_result()
            .derived_invalidation_report
            .triggered_target_count
    );
    assert!(traced
        .performance_accounting()
        .counters
        .iter()
        .any(|counter| counter.name == "certification.commit_boundary_validator_count"));
    assert_eq!(
        counter_value(
            traced.performance_accounting(),
            "certification.query.affected_live_view_count",
        ),
        Some(3),
    );
    assert_eq!(
        counter_value(
            traced.performance_accounting(),
            "certification.query.affected_derived_view_count",
        ),
        Some(5),
    );
    assert!(
        counter_value(
            traced.performance_accounting(),
            "certification.query.considered_computed_view_count",
        )
        .expect("milestone one certification should expose considered computed count")
            >= 5
    );
    assert!(
        counter_value(
            traced.performance_accounting(),
            "certification.query.mutation_metadata_key_count",
        )
        .expect("milestone one certification should expose aggregated mutation metadata count")
            > 1
    );
}

#[test]
fn milestone_one_read_certification_fails_closed_on_query_import_gap() {
    let mut runtime = crate::facade::worth_milestone_one_runtime_builder()
        .expect("worth milestone one runtime builder")
        .build();

    let seeded =
        seeded_bootstrap(&mut runtime, "cert-m1-query-denial").expect("seed worth topology");
    let mut read_view_json = serde_json::to_value(
        runtime
            .read_truth()
            .read_snapshot(&seeded.snapshot)
            .expect("worth snapshot read"),
    )
    .expect("relational read view should serialize");

    let relations = read_view_json
        .get_mut("relations")
        .and_then(Value::as_array_mut)
        .expect("serialized read view should expose relations");
    let relation = relations
        .iter_mut()
        .find(|relation| {
            relation
                .get("kind")
                .and_then(|kind| kind.get("kind_name"))
                .and_then(Value::as_str)
                != Some("PersistentNameTargetsEntity")
        })
        .expect("seeded topology should contain non-naming relation");
    relation["target"] = serde_json::json!({
        "partition_id": 0,
        "local_slot": 999999u64,
        "generation": 1,
    });

    let corrupted: forge_relational::facade::runtime::RelationalReadView =
        serde_json::from_value(read_view_json)
            .expect("corrupted read view should deserialize for hostile denial");

    let failure = certify_milestone_one_read_view_traced(&corrupted, seeded.read_basis)
        .expect_err("milestone one query certification should fail closed");

    match failure.error() {
        crate::facade::WorthMilestoneOneCertificationError::Query(detail) => {
            assert!(detail.contains("missing imported query identity mapping"));
        }
        other => panic!("expected query certification failure, got {other:?}"),
    }
}

#[test]
fn traced_milestone_two_read_view_reuses_certification_trace_packet() {
    let mut runtime = crate::facade::worth_milestone_one_runtime_builder()
        .expect("worth milestone one runtime builder")
        .build();

    let seeded = seeded_bootstrap(&mut runtime, "cert-m2-traced").expect("seed worth topology");
    let read_view = runtime
        .read_truth()
        .read_snapshot(&seeded.snapshot)
        .expect("worth snapshot read");

    let traced = certify_milestone_two_read_view_traced(&read_view, seeded.read_basis)
        .expect("traced milestone two read certification");

    assert!(traced.decision_trace().derived.is_some());
    assert!(traced
        .performance_accounting()
        .counters
        .iter()
        .any(|counter| counter.name == "certification.derived_invalidation_target_count"));
    assert_eq!(
        counter_value(
            traced.performance_accounting(),
            "certification.query.affected_live_view_count",
        ),
        Some(3),
    );
    assert_eq!(
        counter_value(
            traced.performance_accounting(),
            "certification.query.affected_derived_view_count",
        ),
        Some(5),
    );
    assert!(
        counter_value(
            traced.performance_accounting(),
            "certification.query.considered_computed_view_count",
        )
        .expect("query certification should expose considered computed view count")
            >= 5
    );
    assert!(
        counter_value(
            traced.performance_accounting(),
            "certification.query.mutation_metadata_key_count",
        )
        .expect("query certification should expose aggregated mutation metadata count")
            > 1
    );
    assert_eq!(
        traced
            .primary_result()
            .derived_equivalence_contract_report
            .materialized_topology_digest
            .digest_hex,
        traced
            .decision_trace()
            .derived
            .as_ref()
            .expect("derived trace")
            .equivalence_digest
            .clone()
            .expect("equivalence digest")
    );
}

#[test]
fn milestone_two_read_certification_fails_closed_on_query_import_gap() {
    let mut runtime = crate::facade::worth_milestone_one_runtime_builder()
        .expect("worth milestone one runtime builder")
        .build();

    let seeded =
        seeded_bootstrap(&mut runtime, "cert-m2-query-denial").expect("seed worth topology");
    let mut read_view_json = serde_json::to_value(
        runtime
            .read_truth()
            .read_snapshot(&seeded.snapshot)
            .expect("worth snapshot read"),
    )
    .expect("relational read view should serialize");

    let relations = read_view_json
        .get_mut("relations")
        .and_then(Value::as_array_mut)
        .expect("serialized read view should expose relations");
    let relation = relations
        .iter_mut()
        .find(|relation| {
            relation
                .get("kind")
                .and_then(|kind| kind.get("kind_name"))
                .and_then(Value::as_str)
                != Some("PersistentNameTargetsEntity")
        })
        .expect("seeded topology should contain non-naming relation");
    let original_target = relation
        .get("target")
        .cloned()
        .expect("seeded topology should contain relation target");
    relation["target"] = serde_json::json!({
        "partition_id": 0,
        "local_slot": 999999u64,
        "generation": 1,
    });
    assert_ne!(
        relation
            .get("target")
            .expect("corrupted target should remain present"),
        &original_target
    );

    let corrupted: forge_relational::facade::runtime::RelationalReadView =
        serde_json::from_value(read_view_json)
            .expect("corrupted read view should deserialize for hostile denial");

    let failure = certify_milestone_two_read_view_traced(&corrupted, seeded.read_basis)
        .expect_err("milestone two query certification should fail closed");

    match failure.error() {
        crate::facade::WorthMilestoneOneCertificationError::Query(detail) => {
            assert!(detail.contains("missing imported query identity mapping"));
        }
        other => panic!("expected query certification failure, got {other:?}"),
    }
}

#[test]
fn verified_commit_earns_direct_milestone_two_read_report() {
    let mut runtime = crate::facade::worth_milestone_one_runtime_builder()
        .expect("worth milestone one runtime builder")
        .build();

    let verified = verified_primitive(
        &mut runtime,
        "cert-m2-verified",
        &WorthMilestoneOnePrimitiveCase::WireBranch { branch_count: 4 },
    )
    .expect("verified primitive");

    let report = certify_milestone_two_verified_topology_commit_traced(&mut runtime, &verified)
        .expect("milestone two verified certification should succeed")
        .into_primary_result();

    assert!(report.materialized_topology_digest.row_count > 0);
    assert!(report.interpreted_topology_digest.row_count > 0);
    assert!(report.derived_validation_digest.row_count > 0);
    assert!(
        report
            .derived_replay_parity_report
            .relational_replay_checked
    );
}

#[test]
fn default_primitive_corpus_earns_direct_milestone_two_derived_corpus_report() {
    let report = certify_milestone_two_default_derived_corpus(
        || {
            crate::facade::worth_milestone_one_runtime_builder()
                .expect("worth milestone one runtime builder")
                .build()
        },
        "cert-m2-corpus",
    )
    .expect("milestone two derived corpus");

    assert!(report.materialized_topology_digest.row_count > 0);
    assert!(report.interpreted_topology_digest.row_count > 0);
    assert!(report.derived_validation_digest.row_count > 0);
    assert!(report
        .derived_family_coverage_matrix
        .rows
        .iter()
        .any(|row| row.family == "WireOpen(n)" && row.coverage_complete));
    assert!(report
        .derived_family_parity_matrix
        .rows
        .iter()
        .any(|row| row.family == "WireBranch(k)" && row.parity_complete));
    assert!(report.bridge_routing_digest.row_count > 0);
    assert!(report.bridge_historical_evaluation_digest.row_count > 0);
    assert!(report.milestone_2_counter_report.derived_read_count > 0);
}

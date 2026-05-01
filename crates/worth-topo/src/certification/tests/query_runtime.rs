use super::support::counter_value;
use super::*;

#[test]
fn traced_certification_read_view_surfaces_schema_owned_trace() {
    let mut runtime = crate::facade::worth_milestone_one_runtime_builder()
        .expect("worth milestone one runtime builder")
        .build();

    let seeded =
        seeded_bootstrap(&mut runtime, "cert-traced-surface").expect("seed worth topology");
    let traced = certify_milestone_one_read_basis_traced(&mut runtime, seeded.read_basis.clone())
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
        Some(0),
    );
    assert_eq!(
        counter_value(
            traced.performance_accounting(),
            "certification.query.affected_derived_view_count",
        ),
        Some(0),
    );
    assert_eq!(
        counter_value(
            traced.performance_accounting(),
            "certification.query.considered_computed_view_count",
        ),
        Some(0),
    );
    assert_eq!(
        counter_value(
            traced.performance_accounting(),
            "certification.query.mutation_metadata_key_count",
        ),
        Some(0),
    );
}

#[test]
fn traced_milestone_two_read_view_reuses_certification_trace_packet() {
    let mut runtime = crate::facade::worth_milestone_one_runtime_builder()
        .expect("worth milestone one runtime builder")
        .build();

    let seeded = seeded_bootstrap(&mut runtime, "cert-m2-traced").expect("seed worth topology");
    let traced = certify_milestone_two_read_basis_traced(&mut runtime, seeded.read_basis)
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
        Some(0),
    );
    assert_eq!(
        counter_value(
            traced.performance_accounting(),
            "certification.query.affected_derived_view_count",
        ),
        Some(0),
    );
    assert_eq!(
        counter_value(
            traced.performance_accounting(),
            "certification.query.considered_computed_view_count",
        ),
        Some(0),
    );
    assert_eq!(
        counter_value(
            traced.performance_accounting(),
            "certification.query.mutation_metadata_key_count",
        ),
        Some(0),
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

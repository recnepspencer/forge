use crate::workload_platform::planar_boolean_loop_reconstruction::test_support::{
    prepared_phase_fourteen_subject, LoopFixtureEntryOrder,
};
use crate::workload_platform::planar_boolean_loop_reconstruction::{
    PlanarBooleanLoopDecisionKind, PlanarBooleanLoopDecisionLog,
    PlanarBooleanLoopDecisionLogDenialKind, PlanarBooleanLoopDecisionLogInput,
    PlanarBooleanLoopIdentityMap, PlanarBooleanLoopRoleOutcome, PlanarBooleanLoopRoleOutcomeKind,
    PlanarBooleanLoopRoleOutcomeSet,
};

#[test]
fn loop_decision_log_is_replay_stable_for_real_phase_thirteen_products() {
    let canonical = prepared_phase_fourteen_subject(LoopFixtureEntryOrder::Canonical);
    let replayed = prepared_phase_fourteen_subject(LoopFixtureEntryOrder::Replayed);

    let canonical_log = PlanarBooleanLoopDecisionLog::record(canonical.decision_log_input())
        .expect("canonical phase-thirteen products should record decisions");
    let replayed_log = PlanarBooleanLoopDecisionLog::record(replayed.decision_log_input())
        .expect("replayed phase-thirteen products should record decisions");

    assert!(!canonical
        .reconstructed_boundary
        .reconstructed_loops()
        .rows()
        .is_empty());
    assert!(!canonical_log.rows().is_empty());
    assert_eq!(canonical_log.rows(), replayed_log.rows());
    assert_eq!(
        canonical_log.decision_log_identity(),
        replayed_log.decision_log_identity()
    );
    assert_eq!(
        canonical_log.request_identity(),
        replayed_log.request_identity()
    );
    assert_eq!(
        canonical_log.split_ledger_receipt_identity(),
        replayed_log.split_ledger_receipt_identity()
    );
    assert_eq!(canonical_log.counters(), replayed_log.counters());
    assert_eq!(
        canonical_log.counters().decision_rows_emitted(),
        canonical_log.rows().len()
    );
    assert_eq!(
        canonical_log.counters().lookup_index_entries(),
        canonical_log.rows().len()
    );
    assert_eq!(
        canonical_log.counters().continuation_rows_consumed(),
        canonical.continuation_index.rows().len()
    );
    assert_eq!(
        canonical_log.counters().walk_outcomes_consumed(),
        canonical.walk_outcomes.rows().len()
    );
    assert_eq!(
        canonical_log.counters().loop_candidates_consumed(),
        canonical
            .loop_candidate_boundary
            .loop_candidates()
            .rows()
            .len()
    );
    assert_eq!(
        canonical_log.counters().denied_loop_candidates_consumed(),
        canonical
            .loop_candidate_boundary
            .denied_loop_candidates()
            .rows()
            .len()
    );
    assert_eq!(
        canonical_log.counters().reconstructed_loops_consumed(),
        canonical
            .reconstructed_boundary
            .reconstructed_loops()
            .rows()
            .len()
    );
    assert_eq!(
        canonical_log.counters().born_loops_consumed(),
        canonical.reconstructed_boundary.born_loops().rows().len()
    );
    assert_eq!(
        canonical_log.counters().island_rows_consumed(),
        canonical.island_partition.rows().len()
    );
    assert_eq!(
        canonical_log.counters().split_attribution_rows_consumed(),
        canonical.split_attribution.rows().len()
    );
    assert_eq!(
        canonical_log.counters().role_rows_consumed(),
        canonical.role_boundary.role_outcomes().rows().len()
    );
    assert_eq!(
        canonical_log.counters().degenerate_rows_consumed(),
        canonical.degenerate_boundary.outcomes().rows().len()
    );
    assert_eq!(
        canonical_log.counters().identity_rows_consumed(),
        canonical.identity_boundary.loop_identity_map().rows().len()
    );
    assert_eq!(
        canonical_log.counters().propagated_name_rows_consumed(),
        canonical
            .identity_boundary
            .persistent_name_propagation_map()
            .rows()
            .len()
    );
    assert_eq!(
        canonical_log
            .counters()
            .propagated_signature_rows_consumed(),
        canonical
            .identity_boundary
            .subshape_signature_map()
            .rows()
            .len()
    );
    assert_eq!(
        canonical_log
            .counters()
            .duplicate_decision_identity_denials(),
        0
    );
    assert_eq!(
        canonical_log.counters().request_identity_mismatch_denials(),
        0
    );
}

#[test]
fn loop_decision_log_denies_request_mismatch() {
    let fixture = prepared_phase_fourteen_subject(LoopFixtureEntryOrder::Canonical);
    let foreign_identity_map = PlanarBooleanLoopIdentityMap::new(
        fixture
            .identity_boundary
            .loop_identity_map()
            .map_identity()
            .to_string(),
        "foreign-request".to_string(),
        fixture
            .identity_boundary
            .loop_identity_map()
            .rows()
            .to_vec(),
    );
    let input = PlanarBooleanLoopDecisionLogInput::from_phase_thirteen_products(
        &fixture.request,
        &fixture.continuation_index,
        &fixture.walk_outcomes,
        &fixture.loop_candidate_boundary.loop_candidates(),
        fixture.loop_candidate_boundary.denied_loop_candidates(),
        fixture.reconstructed_boundary.reconstructed_loops(),
        fixture.reconstructed_boundary.born_loops(),
        &fixture.island_partition,
        &fixture.split_attribution,
        fixture.role_boundary.role_outcomes(),
        fixture.degenerate_boundary.outcomes(),
        &foreign_identity_map,
        fixture.identity_boundary.persistent_name_propagation_map(),
        fixture.identity_boundary.subshape_signature_map(),
    );

    let denial = PlanarBooleanLoopDecisionLog::record(input)
        .expect_err("foreign request identities must deny decision-log assembly");

    assert_eq!(
        denial.kind(),
        PlanarBooleanLoopDecisionLogDenialKind::RequestIdentityMismatch
    );
    assert_eq!(denial.counters().request_identity_mismatch_denials(), 1);
}

#[test]
fn loop_decision_log_denies_duplicate_decision_identity() {
    let fixture = prepared_phase_fourteen_subject(LoopFixtureEntryOrder::Canonical);
    let mut duplicated_rows = fixture.role_boundary.role_outcomes().rows().to_vec();
    duplicated_rows.push(
        duplicated_rows
            .first()
            .cloned()
            .expect("fixture should emit at least one role outcome"),
    );
    let duplicated_role_outcomes = PlanarBooleanLoopRoleOutcomeSet::new(
        fixture
            .role_boundary
            .role_outcomes()
            .role_outcome_set_identity()
            .to_string(),
        fixture
            .role_boundary
            .role_outcomes()
            .request_identity()
            .to_string(),
        duplicated_rows,
    );
    let input = PlanarBooleanLoopDecisionLogInput::from_phase_thirteen_products(
        &fixture.request,
        &fixture.continuation_index,
        &fixture.walk_outcomes,
        fixture.loop_candidate_boundary.loop_candidates(),
        fixture.loop_candidate_boundary.denied_loop_candidates(),
        fixture.reconstructed_boundary.reconstructed_loops(),
        fixture.reconstructed_boundary.born_loops(),
        &fixture.island_partition,
        &fixture.split_attribution,
        &duplicated_role_outcomes,
        fixture.degenerate_boundary.outcomes(),
        fixture.identity_boundary.loop_identity_map(),
        fixture.identity_boundary.persistent_name_propagation_map(),
        fixture.identity_boundary.subshape_signature_map(),
    );

    let denial = PlanarBooleanLoopDecisionLog::record(input)
        .expect_err("duplicate role decisions must deny decision-log assembly");

    assert_eq!(
        denial.kind(),
        PlanarBooleanLoopDecisionLogDenialKind::DuplicateDecisionIdentity
    );
    assert_eq!(denial.counters().duplicate_decision_identity_denials(), 1);
}

#[test]
fn loop_decision_log_localizes_failures_and_reports_related_decisions() {
    let fixture = prepared_phase_fourteen_subject(LoopFixtureEntryOrder::Canonical);
    let denied_role = fixture
        .role_boundary
        .role_outcomes()
        .rows()
        .first()
        .map(|row| {
            PlanarBooleanLoopRoleOutcome::new(
                row.role_outcome_identity().to_string(),
                row.loop_identity().to_string(),
                row.loop_kind(),
                row.island_identities().to_vec(),
                row.source_loop_identities().to_vec(),
                row.preserved_source_role(),
                PlanarBooleanLoopRoleOutcomeKind::BornLoopRoleAmbiguous,
            )
        })
        .expect("fixture should emit one role outcome");
    let denied_role_outcomes = PlanarBooleanLoopRoleOutcomeSet::new(
        fixture
            .role_boundary
            .role_outcomes()
            .role_outcome_set_identity()
            .to_string(),
        fixture
            .role_boundary
            .role_outcomes()
            .request_identity()
            .to_string(),
        vec![denied_role],
    );
    let input = PlanarBooleanLoopDecisionLogInput::from_phase_thirteen_products(
        &fixture.request,
        &fixture.continuation_index,
        &fixture.walk_outcomes,
        fixture.loop_candidate_boundary.loop_candidates(),
        fixture.loop_candidate_boundary.denied_loop_candidates(),
        fixture.reconstructed_boundary.reconstructed_loops(),
        fixture.reconstructed_boundary.born_loops(),
        &fixture.island_partition,
        &fixture.split_attribution,
        &denied_role_outcomes,
        fixture.degenerate_boundary.outcomes(),
        fixture.identity_boundary.loop_identity_map(),
        fixture.identity_boundary.persistent_name_propagation_map(),
        fixture.identity_boundary.subshape_signature_map(),
    );
    let log = PlanarBooleanLoopDecisionLog::record(input)
        .expect("diagnostic fixture should still record a decision log");
    let denied_row = log
        .rows()
        .iter()
        .find(|row| row.kind() == PlanarBooleanLoopDecisionKind::Denied)
        .expect("diagnostic fixture should emit a denied decision row");
    let admitted_row = log
        .rows()
        .iter()
        .find(|row| row.kind() == PlanarBooleanLoopDecisionKind::Admitted)
        .expect("diagnostic fixture should also emit an admitted decision row");

    assert_eq!(
        log.decision_by_identity(denied_row.decision_identity()),
        Some(denied_row)
    );
    assert!(log
        .localize_failure(admitted_row.decision_identity())
        .is_none());

    let related_rows = log.decisions_for_artifact(denied_row.affected_artifact_identity());
    assert!(!related_rows.is_empty());
    assert!(related_rows
        .iter()
        .any(|row| { row.decision_identity() == denied_row.decision_identity() }));

    let localization = log
        .localize_failure(denied_row.decision_identity())
        .expect("denied rows should localize");
    assert_eq!(localization.kind(), denied_row.kind());
    assert_eq!(
        localization.affected_artifact(),
        denied_row.affected_artifact()
    );
    assert_eq!(
        localization.affected_artifact_identity(),
        denied_row.affected_artifact_identity()
    );
    assert_eq!(localization.human_reason(), denied_row.human_reason());

    let report = log.structured_failure_report(&localization);
    assert_eq!(report.localization(), &localization);
    assert_eq!(
        report.related_decision_identities(),
        related_rows
            .iter()
            .map(|row| row.decision_identity().to_string())
            .collect::<Vec<_>>()
    );
}

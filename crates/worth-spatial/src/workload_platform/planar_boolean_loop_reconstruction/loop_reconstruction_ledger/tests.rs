mod support;

use self::support::admitted_identity_products;
use crate::workload_platform::planar_boolean_loop_reconstruction::test_support::{
    prepared_phase_fourteen_subject, LoopFixtureEntryOrder,
};
use crate::workload_platform::planar_boolean_loop_reconstruction::{
    PlanarBooleanDegenerateLoopOutcomeSet, PlanarBooleanLoopClassifiedProductKind,
    PlanarBooleanLoopDecisionLog, PlanarBooleanLoopIdentityMap, PlanarBooleanLoopIdentityRow,
    PlanarBooleanLoopReconstructionLedger, PlanarBooleanLoopReconstructionLedgerDenialKind,
    PlanarBooleanLoopReconstructionLedgerInput, PlanarBooleanLoopRoleOutcomeSet,
};

#[test]
fn loop_reconstruction_ledger_is_replay_stable_for_real_phase_fourteen_products() {
    let canonical = prepared_phase_fourteen_subject(LoopFixtureEntryOrder::Canonical);
    let replayed = prepared_phase_fourteen_subject(LoopFixtureEntryOrder::Replayed);

    let canonical_log = PlanarBooleanLoopDecisionLog::record(canonical.decision_log_input())
        .expect("canonical products should admit decision-log recording");
    let replayed_log = PlanarBooleanLoopDecisionLog::record(replayed.decision_log_input())
        .expect("replayed products should admit decision-log recording");
    let (canonical_identity_map, canonical_name_map, canonical_signature_map) =
        admitted_identity_products(&canonical);
    let (replayed_identity_map, replayed_name_map, replayed_signature_map) =
        admitted_identity_products(&replayed);

    let (canonical_ledger, canonical_receipt) = PlanarBooleanLoopReconstructionLedger::assemble(
        PlanarBooleanLoopReconstructionLedgerInput::from_decision_log_and_loop_products(
            &canonical.request,
            &canonical_log,
            &canonical_identity_map,
            &canonical_name_map,
            &canonical_signature_map,
            canonical.reconstructed_boundary.reconstructed_loops(),
            canonical.reconstructed_boundary.born_loops(),
            &canonical.island_partition,
            &canonical.split_attribution,
            canonical.role_boundary.role_outcomes(),
            canonical.degenerate_boundary.outcomes(),
        ),
    )
    .expect("canonical products should assemble the loop ledger");
    let (replayed_ledger, replayed_receipt) = PlanarBooleanLoopReconstructionLedger::assemble(
        PlanarBooleanLoopReconstructionLedgerInput::from_decision_log_and_loop_products(
            &replayed.request,
            &replayed_log,
            &replayed_identity_map,
            &replayed_name_map,
            &replayed_signature_map,
            replayed.reconstructed_boundary.reconstructed_loops(),
            replayed.reconstructed_boundary.born_loops(),
            &replayed.island_partition,
            &replayed.split_attribution,
            replayed.role_boundary.role_outcomes(),
            replayed.degenerate_boundary.outcomes(),
        ),
    )
    .expect("replayed products should assemble the loop ledger");

    assert!(!canonical_identity_map.rows().is_empty());
    assert!(!canonical_ledger.rows().is_empty());
    assert_eq!(canonical_ledger.rows(), replayed_ledger.rows());
    assert_eq!(
        canonical_ledger.ledger_identity(),
        replayed_ledger.ledger_identity()
    );
    assert_eq!(
        canonical_ledger.request_identity(),
        replayed_ledger.request_identity()
    );
    assert_eq!(
        canonical_ledger.decision_log_identity(),
        replayed_ledger.decision_log_identity()
    );
    assert_eq!(
        canonical_ledger.loop_identity_map_identity(),
        replayed_ledger.loop_identity_map_identity()
    );
    assert_eq!(
        canonical_ledger.persistent_name_map_identity(),
        replayed_ledger.persistent_name_map_identity()
    );
    assert_eq!(
        canonical_ledger.subshape_signature_map_identity(),
        replayed_ledger.subshape_signature_map_identity()
    );
    assert_eq!(
        canonical_ledger.reconstructed_loop_set_identity(),
        replayed_ledger.reconstructed_loop_set_identity()
    );
    assert_eq!(
        canonical_ledger.born_loop_set_identity(),
        replayed_ledger.born_loop_set_identity()
    );
    assert_eq!(
        canonical_ledger.island_partition_identity(),
        replayed_ledger.island_partition_identity()
    );
    assert_eq!(
        canonical_ledger.split_attribution_identity(),
        replayed_ledger.split_attribution_identity()
    );
    assert_eq!(
        canonical_ledger.role_outcome_set_identity(),
        replayed_ledger.role_outcome_set_identity()
    );
    assert_eq!(
        canonical_ledger.degenerate_outcome_set_identity(),
        replayed_ledger.degenerate_outcome_set_identity()
    );
    assert_eq!(canonical_ledger.counters(), replayed_ledger.counters());
    assert_eq!(
        canonical_receipt.receipt_identity(),
        replayed_receipt.receipt_identity()
    );
    assert_eq!(
        canonical_receipt.ledger_identity(),
        replayed_receipt.ledger_identity()
    );
    assert_eq!(
        canonical_receipt.downstream_consumption_identity(),
        replayed_receipt.downstream_consumption_identity()
    );
    assert_eq!(
        canonical_receipt.request_identity(),
        replayed_receipt.request_identity()
    );
    assert_eq!(
        canonical_receipt.decision_log_identity(),
        replayed_receipt.decision_log_identity()
    );
    assert_eq!(
        canonical_receipt.loop_identity_map_identity(),
        replayed_receipt.loop_identity_map_identity()
    );
    assert_eq!(
        canonical_receipt.persistent_name_map_identity(),
        replayed_receipt.persistent_name_map_identity()
    );
    assert_eq!(
        canonical_receipt.subshape_signature_map_identity(),
        replayed_receipt.subshape_signature_map_identity()
    );
    assert_eq!(
        canonical_receipt.ledger_row_identities(),
        replayed_receipt.ledger_row_identities()
    );
    assert_eq!(canonical_receipt.counters(), replayed_receipt.counters());
    assert!(!canonical_receipt.receipt_identity().is_empty());
    assert!(!canonical_receipt
        .downstream_consumption_identity()
        .is_empty());
    assert_eq!(
        canonical_receipt.counters().ledger_rows_emitted(),
        canonical_ledger.rows().len()
    );
    assert_eq!(
        canonical_receipt.counters().identity_rows_consumed(),
        canonical_identity_map.rows().len()
    );
    assert_eq!(
        canonical_receipt.counters().decision_rows_consumed(),
        canonical_log.rows().len()
    );
    assert_eq!(
        canonical_receipt.counters().propagated_name_rows_consumed(),
        canonical_name_map.rows().len()
    );
    assert_eq!(
        canonical_receipt
            .counters()
            .propagated_signature_rows_consumed(),
        canonical_signature_map.rows().len()
    );
    assert_eq!(
        canonical_receipt.counters().downstream_identities_emitted(),
        1
    );

    for row in canonical_ledger.rows() {
        assert!(!row.ledger_row_identity().is_empty());
        assert!(!row.canonical_loop_identity().is_empty());
        assert!(!row.tracked_loop_identity().is_empty());
        assert!(!row.role_outcome_identity().is_empty());
        assert!(!row.degenerate_outcome_identity().is_empty());
        assert!(!row.decision_identities().is_empty());
        assert!(!row.source_loop_identities().is_empty());
    }
}

#[test]
fn loop_reconstruction_ledger_denies_request_mismatch() {
    let fixture = prepared_phase_fourteen_subject(LoopFixtureEntryOrder::Canonical);
    let decision_log = PlanarBooleanLoopDecisionLog::record(fixture.decision_log_input())
        .expect("fixture should admit decision-log recording");
    let foreign_identity_map = PlanarBooleanLoopIdentityMap::new(
        "foreign-request-identity-map".to_string(),
        "foreign-request".to_string(),
        Vec::new(),
    );
    let input = PlanarBooleanLoopReconstructionLedgerInput::from_decision_log_and_loop_products(
        &fixture.request,
        &decision_log,
        &foreign_identity_map,
        fixture.identity_boundary.persistent_name_propagation_map(),
        fixture.identity_boundary.subshape_signature_map(),
        fixture.reconstructed_boundary.reconstructed_loops(),
        fixture.reconstructed_boundary.born_loops(),
        &fixture.island_partition,
        &fixture.split_attribution,
        fixture.role_boundary.role_outcomes(),
        fixture.degenerate_boundary.outcomes(),
    );

    let denial = PlanarBooleanLoopReconstructionLedger::assemble(input)
        .expect_err("foreign request lineage must deny ledger assembly");

    assert_eq!(
        denial.kind(),
        PlanarBooleanLoopReconstructionLedgerDenialKind::RequestIdentityMismatch
    );
    assert_eq!(denial.counters().request_identity_mismatch_denials(), 1);
}

#[test]
fn loop_reconstruction_ledger_denies_split_ledger_lineage_mismatch() {
    let fixture = prepared_phase_fourteen_subject(LoopFixtureEntryOrder::Canonical);
    let decision_log = PlanarBooleanLoopDecisionLog::record(fixture.decision_log_input())
        .expect("fixture should admit decision-log recording");
    let foreign_decision_log =
        decision_log.with_split_ledger_receipt_identity_for_tests("foreign-split-ledger");
    let input = PlanarBooleanLoopReconstructionLedgerInput::from_decision_log_and_loop_products(
        &fixture.request,
        &foreign_decision_log,
        fixture.identity_boundary.loop_identity_map(),
        fixture.identity_boundary.persistent_name_propagation_map(),
        fixture.identity_boundary.subshape_signature_map(),
        fixture.reconstructed_boundary.reconstructed_loops(),
        fixture.reconstructed_boundary.born_loops(),
        &fixture.island_partition,
        &fixture.split_attribution,
        fixture.role_boundary.role_outcomes(),
        fixture.degenerate_boundary.outcomes(),
    );

    let denial = PlanarBooleanLoopReconstructionLedger::assemble(input)
        .expect_err("foreign split-ledger lineage must deny ledger assembly");

    assert_eq!(
        denial.kind(),
        PlanarBooleanLoopReconstructionLedgerDenialKind::SplitLedgerLineageMismatch
    );
    assert_eq!(denial.counters().split_ledger_lineage_mismatch_denials(), 1);
}

#[test]
fn loop_reconstruction_ledger_denies_missing_tracked_loop() {
    let fixture = prepared_phase_fourteen_subject(LoopFixtureEntryOrder::Canonical);
    let decision_log = PlanarBooleanLoopDecisionLog::record(fixture.decision_log_input())
        .expect("fixture should admit decision-log recording");
    let (_, name_map, signature_map) = admitted_identity_products(&fixture);
    let reconstructed = fixture
        .reconstructed_boundary
        .reconstructed_loops()
        .rows()
        .first()
        .expect("fixture should reconstruct one loop");
    let role_outcome = fixture
        .role_boundary
        .role_outcomes()
        .rows()
        .first()
        .expect("fixture should expose one role outcome");
    let degenerate_outcome = fixture
        .degenerate_boundary
        .outcomes()
        .rows()
        .first()
        .expect("fixture should expose one degenerate outcome");
    let missing_tracked_identity_map = PlanarBooleanLoopIdentityMap::new(
        "missing-tracked-identity-map".to_string(),
        fixture.request.request_identity().to_string(),
        vec![PlanarBooleanLoopIdentityRow::new(
            "missing-tracked-row".to_string(),
            format!("missing:{}", reconstructed.reconstructed_loop_identity()),
            "canonical:missing-tracked".to_string(),
            PlanarBooleanLoopClassifiedProductKind::ReconstructedLoop,
            vec![reconstructed.source_loop_identity().to_string()],
            reconstructed.fragment_identities().to_vec(),
            reconstructed.split_vertex_identities().to_vec(),
            role_outcome.role_outcome_identity().to_string(),
            degenerate_outcome
                .degenerate_loop_outcome_identity()
                .to_string(),
        )],
    );
    let input = PlanarBooleanLoopReconstructionLedgerInput::from_decision_log_and_loop_products(
        &fixture.request,
        &decision_log,
        &missing_tracked_identity_map,
        &name_map,
        &signature_map,
        fixture.reconstructed_boundary.reconstructed_loops(),
        fixture.reconstructed_boundary.born_loops(),
        &fixture.island_partition,
        &fixture.split_attribution,
        fixture.role_boundary.role_outcomes(),
        fixture.degenerate_boundary.outcomes(),
    );

    let denial = PlanarBooleanLoopReconstructionLedger::assemble(input)
        .expect_err("foreign tracked loop identities must deny ledger assembly");

    assert_eq!(
        denial.kind(),
        PlanarBooleanLoopReconstructionLedgerDenialKind::MissingTrackedLoop
    );
    assert_eq!(denial.counters().missing_tracked_loop_denials(), 1);
}

#[test]
fn loop_reconstruction_ledger_denies_missing_role_outcome() {
    let fixture = prepared_phase_fourteen_subject(LoopFixtureEntryOrder::Canonical);
    let decision_log = PlanarBooleanLoopDecisionLog::record(fixture.decision_log_input())
        .expect("fixture should admit decision-log recording");
    let (identity_map, name_map, signature_map) = admitted_identity_products(&fixture);
    let missing_role_outcomes = PlanarBooleanLoopRoleOutcomeSet::new(
        "missing-role-outcomes".to_string(),
        fixture.request.request_identity().to_string(),
        Vec::new(),
    );
    let input = PlanarBooleanLoopReconstructionLedgerInput::from_decision_log_and_loop_products(
        &fixture.request,
        &decision_log,
        &identity_map,
        &name_map,
        &signature_map,
        fixture.reconstructed_boundary.reconstructed_loops(),
        fixture.reconstructed_boundary.born_loops(),
        &fixture.island_partition,
        &fixture.split_attribution,
        &missing_role_outcomes,
        fixture.degenerate_boundary.outcomes(),
    );

    let denial = PlanarBooleanLoopReconstructionLedger::assemble(input)
        .expect_err("missing role evidence must deny ledger assembly");

    assert_eq!(
        denial.kind(),
        PlanarBooleanLoopReconstructionLedgerDenialKind::MissingRoleOutcome
    );
    assert_eq!(denial.counters().missing_role_outcome_denials(), 1);
}

#[test]
fn loop_reconstruction_ledger_denies_missing_degenerate_outcome() {
    let fixture = prepared_phase_fourteen_subject(LoopFixtureEntryOrder::Canonical);
    let decision_log = PlanarBooleanLoopDecisionLog::record(fixture.decision_log_input())
        .expect("fixture should admit decision-log recording");
    let (identity_map, name_map, signature_map) = admitted_identity_products(&fixture);
    let missing_degenerate_outcomes = PlanarBooleanDegenerateLoopOutcomeSet::new(
        "missing-degenerate-outcomes".to_string(),
        fixture.request.request_identity().to_string(),
        Vec::new(),
    );
    let input = PlanarBooleanLoopReconstructionLedgerInput::from_decision_log_and_loop_products(
        &fixture.request,
        &decision_log,
        &identity_map,
        &name_map,
        &signature_map,
        fixture.reconstructed_boundary.reconstructed_loops(),
        fixture.reconstructed_boundary.born_loops(),
        &fixture.island_partition,
        &fixture.split_attribution,
        fixture.role_boundary.role_outcomes(),
        &missing_degenerate_outcomes,
    );

    let denial = PlanarBooleanLoopReconstructionLedger::assemble(input)
        .expect_err("missing degeneracy posture must deny ledger assembly");

    assert_eq!(
        denial.kind(),
        PlanarBooleanLoopReconstructionLedgerDenialKind::MissingDegenerateOutcome
    );
    assert_eq!(denial.counters().missing_degenerate_outcome_denials(), 1);
}

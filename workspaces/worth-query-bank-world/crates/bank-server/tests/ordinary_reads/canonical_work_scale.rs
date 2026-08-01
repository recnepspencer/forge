use std::time::Instant;

use bank_server::{queries, BankReadControls};
use worth_query_host::facade::domain::{
    WorthQueryCanonicalWorkEvidence, WorthQueryCanonicalWorkPhases,
};
use worth_query_host::facade::primary_graph::WorthQueryApplicationQueryControls;

use super::canonical_scale_fixture::canonical_scale_world;
use super::fixture::{
    ordinary_read_world_with_pending_payments, over_budget_discovery_world, APPROVER,
};
use crate::support::request_scope;

#[test]
fn result_and_graph_fanout_do_not_increase_canonical_work() {
    let fixture = canonical_scale_world();
    let baseline_actor = fixture.authenticate_baseline();
    let expanded_actor = fixture.authenticate_expanded();

    let baseline_result = fixture
        .world
        .runtime
        .query(queries::accounts())
        .as_principal(&baseline_actor)
        .controls(controls(192))
        .execute()
        .expect("baseline account discovery should execute");
    let expanded_result = fixture
        .world
        .runtime
        .query(queries::accounts())
        .as_principal(&expanded_actor)
        .controls(controls(192))
        .execute()
        .expect("expanded account discovery should execute");

    assert_eq!(baseline_result.rows().len(), 1);
    assert_eq!(expanded_result.rows().len(), 192);
    assert!(
        expanded_result.receipt().edge_scan_count() > baseline_result.receipt().edge_scan_count()
    );
    assert!(
        expanded_result.receipt().projected_record_count()
            > baseline_result.receipt().projected_record_count()
    );
    assert!(
        expanded_result.receipt().projected_field_count()
            > baseline_result.receipt().projected_field_count()
    );
    assert_fanout_invariant(
        baseline_result.receipt().canonical_work(),
        expanded_result.receipt().canonical_work(),
    );
}

#[test]
fn guarded_candidate_fanout_does_not_increase_canonical_work() {
    const EXPANDED_CANDIDATE_COUNT: usize = 64;

    let baseline_fixture = ordinary_read_world_with_pending_payments(
        "candidate-scale",
        EXPANDED_CANDIDATE_COUNT - 1,
        1,
    );
    let expanded_fixture =
        ordinary_read_world_with_pending_payments("candidate-scale", 0, EXPANDED_CANDIDATE_COUNT);
    let baseline_approver = baseline_fixture.authenticate(APPROVER);
    let expanded_approver = expanded_fixture.authenticate(APPROVER);

    let baseline_result = baseline_fixture
        .world
        .runtime
        .query(queries::pending_payments())
        .as_principal(&baseline_approver)
        .controls(controls(EXPANDED_CANDIDATE_COUNT))
        .execute()
        .expect("baseline guarded query should execute");
    let expanded_result = expanded_fixture
        .world
        .runtime
        .query(queries::pending_payments())
        .as_principal(&expanded_approver)
        .controls(controls(EXPANDED_CANDIDATE_COUNT))
        .execute()
        .expect("expanded guarded query should execute");

    assert_eq!(baseline_result.rows().len(), 1);
    assert_eq!(expanded_result.rows().len(), EXPANDED_CANDIDATE_COUNT);
    assert!(
        expanded_result.receipt().examined_candidate_count()
            > baseline_result.receipt().examined_candidate_count()
    );
    assert!(
        expanded_result.receipt().work().predicate_work_units()
            > baseline_result.receipt().work().predicate_work_units()
    );
    assert_fanout_invariant(
        baseline_result.receipt().canonical_work(),
        expanded_result.receipt().canonical_work(),
    );
}

#[test]
fn policy_fact_fanout_is_counted_without_increasing_canonical_work() {
    let fixture = canonical_scale_world();
    let baseline_actor = fixture.authenticate_baseline();
    let expanded_actor = fixture.authenticate_expanded();
    let baseline_request = request_scope();
    let expanded_request = request_scope();

    let baseline_result = fixture
        .world
        .runtime
        .account_activity(fixture.baseline_account)
        .as_principal(&baseline_actor)
        .execute(WorthQueryApplicationQueryControls::current_one_shot(
            std::num::NonZeroUsize::new(1).unwrap(),
            std::num::NonZeroUsize::new(100_000).unwrap(),
            &baseline_request,
        ))
        .expect("baseline guarded account query should execute");
    let expanded_result = fixture
        .world
        .runtime
        .account_activity(fixture.expanded_account)
        .as_principal(&expanded_actor)
        .execute(WorthQueryApplicationQueryControls::current_one_shot(
            std::num::NonZeroUsize::new(1).unwrap(),
            std::num::NonZeroUsize::new(100_000).unwrap(),
            &expanded_request,
        ))
        .expect("expanded guarded account query should execute");

    assert_eq!(baseline_result.rows().len(), 1);
    assert_eq!(expanded_result.rows().len(), 1);
    let baseline_work = baseline_result.receipt().authorization_work();
    let expanded_work = expanded_result.receipt().authorization_work();
    assert_eq!(baseline_work.requirement_count(), 1);
    assert_eq!(expanded_work.requirement_count(), 1);
    assert_eq!(
        baseline_work.paths_evaluated(),
        expanded_work.paths_evaluated()
    );
    assert!(expanded_work.adjacency_edges_inspected() > baseline_work.adjacency_edges_inspected());
    assert!(expanded_work.signal_dependency_count() > baseline_work.signal_dependency_count());
    assert_fanout_invariant(
        baseline_result.receipt().canonical_work(),
        expanded_result.receipt().canonical_work(),
    );
}

#[test]
#[ignore = "scheduled high-operation speed probe; run explicitly with --ignored --nocapture"]
fn high_operation_speed_probe_keeps_warm_digest_work_at_exact_zero() {
    const QUERY_COUNT: usize = 512;
    const RESULT_COUNT: usize = 64;

    let fixture = over_budget_discovery_world("canonical-work-high-operation", RESULT_COUNT);
    let actor = fixture.authenticate();
    let started = Instant::now();
    let mut expected = None;
    let mut observed_rows = 0usize;

    for _ in 0..QUERY_COUNT {
        let result = fixture
            .world
            .runtime
            .query(queries::accounts())
            .as_principal(&actor)
            .controls(controls(RESULT_COUNT))
            .execute()
            .expect("high-operation account discovery should execute");
        observed_rows = observed_rows
            .checked_add(result.rows().len())
            .expect("observed row count remains bounded");
        let phases = result.receipt().canonical_work();
        assert_phase_posture(phases);
        if let Some(expected) = expected {
            assert_fanout_invariant(expected, phases);
        } else {
            expected = Some(phases);
        }
    }

    let elapsed = started.elapsed();
    let seconds = elapsed.as_secs_f64();
    let queries_per_second = QUERY_COUNT as f64 / seconds;
    let rows_per_second = observed_rows as f64 / seconds;
    assert_eq!(observed_rows, QUERY_COUNT * RESULT_COUNT);
    eprintln!(
        "profile={} queries={QUERY_COUNT} rows={observed_rows} elapsed={elapsed:?} \
         queries_per_second={queries_per_second:.2} rows_per_second={rows_per_second:.2}",
        if cfg!(debug_assertions) {
            "debug"
        } else {
            "release"
        }
    );
}

fn assert_phase_posture(phases: WorthQueryCanonicalWorkPhases) {
    assert!(phases.installation().basis_preparations() > 0);
    assert!(phases.installation().canonical_encoded_bytes() > 0);
    assert!(
        phases.installation().canonical_material_allocation_bytes()
            >= phases.installation().canonical_encoded_bytes()
    );
    assert!(phases.installation().sha256_input_bytes() > 0);
    assert!(phases.admission().basis_preparations() > 0);
    assert!(phases.admission().digest_derivations() > 0);
    assert_zero(phases.execution());
    assert_zero(phases.provider_commit());
    assert_zero(phases.projection());
    assert_zero(phases.live_delivery());
    assert_zero(phases.retry_resolution());
    assert_zero(phases.recovery_inspection());
    assert_zero(phases.publication());
}

fn assert_fanout_invariant(
    baseline: WorthQueryCanonicalWorkPhases,
    expanded: WorthQueryCanonicalWorkPhases,
) {
    assert_eq!(baseline.installation(), expanded.installation());
    let baseline_admission = baseline.admission();
    let expanded_admission = expanded.admission();
    assert_eq!(
        baseline_admission.basis_preparations(),
        expanded_admission.basis_preparations()
    );
    assert_eq!(
        baseline_admission.digest_derivations(),
        expanded_admission.digest_derivations()
    );
    assert_eq!(
        baseline_admission.canonical_entries(),
        expanded_admission.canonical_entries()
    );
    assert_eq!(
        baseline_admission.sha256_compression_blocks(),
        expanded_admission.sha256_compression_blocks()
    );
    assert_eq!(
        baseline_admission.digest_text_materializations(),
        expanded_admission.digest_text_materializations()
    );
    assert_phase_posture(baseline);
    assert_phase_posture(expanded);
}

fn assert_zero(work: WorthQueryCanonicalWorkEvidence) {
    assert_eq!(work.basis_preparations(), 0);
    assert_eq!(work.digest_derivations(), 0);
    assert_eq!(work.canonical_entries(), 0);
    assert_eq!(work.canonical_encoded_bytes(), 0);
    assert_eq!(work.canonical_material_allocation_bytes(), 0);
    assert_eq!(work.sha256_input_bytes(), 0);
    assert_eq!(work.sha256_compression_blocks(), 0);
    assert_eq!(work.digest_text_materializations(), 0);
}

fn controls(maximum_results: usize) -> BankReadControls {
    BankReadControls::current(request_scope(), maximum_results, 100_000)
        .expect("high-operation controls are bounded")
}

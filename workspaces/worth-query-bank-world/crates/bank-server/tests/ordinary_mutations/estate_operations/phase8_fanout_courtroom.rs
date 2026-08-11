//! Courtroom row 12 / R8.14 — lineage fan-out through the production Bank path.
//!
//! Two sizes (1 vs 100 lineage edges). Posting slope (10 vs 1000) is proved by
//! the execution unit twin that discards those counts from the undo/redo basis;
//! this Bank scenario proves the production undo-admission counters stay 1/1/0
//! when lineage length actually grows on the runtime under test.

use bank_server::BankMutationCommitOutcome;
use worth_query_host::facade::primary_graph::WorthQueryApplicationIdempotencyBinding;

use super::disburse_estate::fixture::disbursement_world;
use crate::support::request_scope;

#[test]
fn courtroom_row_12_history_fanout_twins_leave_section_8_counters_unchanged() {
    let small = measure_undo_admission_after_history(1, 40);
    let large = measure_undo_admission_after_history(100, 41);
    assert_eq!(small.observed_commits, 1);
    assert_eq!(large.observed_commits, 100);
    assert_eq!(small.basis_preparations, 1);
    assert_eq!(small.digest_derivations, 1);
    assert_eq!(small.digest_text_materializations, 0);
    assert_eq!(small.basis_preparations, large.basis_preparations);
    assert_eq!(small.digest_derivations, large.digest_derivations);
    assert_eq!(
        small.digest_text_materializations,
        large.digest_text_materializations
    );
}

#[derive(Debug)]
struct FanoutObservation {
    observed_commits: usize,
    basis_preparations: u32,
    digest_derivations: u32,
    digest_text_materializations: u32,
}

fn measure_undo_admission_after_history(history_len: usize, key: u8) -> FanoutObservation {
    let fixture = disbursement_world(&format!("fanout-history-{history_len}"), 5_000);
    let specialist = fixture.authenticate_actor();
    let mut committed = Vec::with_capacity(history_len);
    for i in 0..history_len {
        let mut original = [0u8; 32];
        let mut intent = [0u8; 32];
        original[0] = key;
        original[1] = (i & 0xff) as u8;
        original[2] = ((i >> 8) & 0xff) as u8;
        intent[0] = key.wrapping_add(1);
        intent[1] = (i & 0xff) as u8;
        intent[2] = ((i >> 8) & 0xff) as u8;
        let outcome = fixture
            .world
            .runtime
            .disburse_estate(
                &specialist,
                fixture.action(1),
                WorthQueryApplicationIdempotencyBinding::new(original, intent),
                &request_scope(),
            )
            .expect("tiny disburse for lineage seed");
        let BankMutationCommitOutcome::Committed(receipt) = outcome else {
            panic!("disburse must commit");
        };
        committed.push(receipt);
    }
    assert_eq!(committed.len(), history_len);

    let outcome = fixture
        .world
        .runtime
        .disburse_estate(
            &specialist,
            fixture.action(10),
            WorthQueryApplicationIdempotencyBinding::new([key; 32], [key.wrapping_add(10); 32]),
            &request_scope(),
        )
        .expect("measured disburse");
    let BankMutationCommitOutcome::Committed(receipt) = outcome else {
        panic!("measured disburse must commit");
    };
    let handle = fixture
        .world
        .runtime
        .open_commit_recovery(&receipt)
        .expect("mint");
    let admission = fixture
        .world
        .runtime
        .admit_undo_disbursement_recovery(handle, &specialist, &request_scope())
        .expect("admit");
    let work = admission.undo_admission_work();
    FanoutObservation {
        observed_commits: committed.len(),
        basis_preparations: work.basis_preparations(),
        digest_derivations: work.digest_derivations(),
        digest_text_materializations: work.digest_text_materializations(),
    }
}

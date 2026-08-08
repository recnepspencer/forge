//! Shared Gate 8.5 redo courtroom helpers — prove through production path.

use bank_domain::proposals::BankIdempotencyKey;
use bank_server::{BankAuthenticatedPrincipal, BankMutationCommitOutcome};
use worth_query_host::facade::primary_graph::WorthQueryApplicationIdempotencyBinding;
use worth_query_host::facade::provisional_aftermath::{
    WorthQueryProvedUndo, WorthQueryRedoIntent, WorthQueryRedoRecovery,
};

use super::disburse_estate::fixture::DisbursementFixture;
pub(super) use super::phase8_undo_denial_support::graph_snapshot;
use crate::support::request_scope;

pub(super) struct ProvedUndoFixture {
    pub specialist: BankAuthenticatedPrincipal,
    pub recovery: WorthQueryRedoRecovery,
    pub intent: WorthQueryRedoIntent,
}

impl ProvedUndoFixture {
    pub(super) const fn proved(&self) -> &WorthQueryProvedUndo {
        self.recovery.proved()
    }
}

pub(super) fn commit_and_prove_undo(fixture: &DisbursementFixture, key: u8) -> ProvedUndoFixture {
    let specialist = fixture.authenticate_actor();
    let outcome = fixture
        .world
        .runtime
        .disburse_estate(
            &specialist,
            fixture.action(100),
            WorthQueryApplicationIdempotencyBinding::new([key; 32], [key.wrapping_add(1); 32]),
            &request_scope(),
        )
        .expect("disburse");
    let BankMutationCommitOutcome::Committed(original) = outcome else {
        panic!("disburse must commit: {outcome:?}");
    };
    let handle = fixture
        .world
        .runtime
        .open_commit_recovery(&original)
        .expect("mint");
    let admission = fixture
        .world
        .runtime
        .admit_undo_disbursement_recovery(handle, &specialist, &request_scope())
        .expect("admit undo");
    let compensation_key =
        BankIdempotencyKey::new(format!("redo-support-undo-{key}")).expect("key");
    let compensated = fixture
        .world
        .runtime
        .progress_undo_commit_recovery(admission, &specialist, &compensation_key, &request_scope())
        .expect("undo commit");
    let (compensated, proved) = compensated.into_parts();
    let _undo = match compensated {
        BankMutationCommitOutcome::Committed(r)
        | BankMutationCommitOutcome::AlreadyCommitted(r) => r,
        other => panic!("undo must commit: {other:?}"),
    };
    let recovery = proved.expect("committed undo seals causal evidence");
    let intent = fixture
        .world
        .runtime
        .derive_redo_intent(recovery.proved())
        .expect("derive redo intent");
    ProvedUndoFixture {
        specialist,
        recovery,
        intent,
    }
}

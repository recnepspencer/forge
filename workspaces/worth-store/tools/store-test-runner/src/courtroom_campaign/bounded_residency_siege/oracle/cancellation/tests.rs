use super::verify;
use crate::courtroom_campaign::bounded_residency_siege::protocol::{
    BoundedCancellationCaseObservation, BoundedCancellationDispatch, BoundedCancellationObligation,
    BoundedCancellationObservation, BoundedCancellationRecovery, BoundedCancellationSeam,
    BoundedCancellationSignal, BoundedCancellationTerminal, BoundedResidencyWorkEffectFate,
    BoundedResidencyWorkFamily, BoundedResidencyWorkReconciliationObservation,
    BoundedResidencyWorkRecordObservation, BoundedResidencyWorkRecovery,
    BoundedResidencyWorkTerminalFate,
};

#[path = "tests/accepted_evidence.rs"]
mod accepted_evidence;

use accepted_evidence::{
    cancellation as accepted_cancellation, work as accepted_work, GENERATION, RUNTIME, STORE,
};

#[test]
fn cancellation_oracle_accepts_only_the_two_exact_lifecycle_seams() {
    verify(
        accepted_cancellation(),
        &accepted_work(),
        STORE,
        RUNTIME,
        GENERATION,
    )
    .unwrap();
}

#[test]
fn pre_dispatch_cancellation_rejects_each_semantic_substitution_exactly() {
    let accepted = accepted_cancellation();
    for (hostile, denial) in [
        (
            BoundedCancellationCaseObservation {
                obligation: BoundedCancellationObligation::SettlementContinues,
                ..accepted.pre_dispatch
            },
            "A16 pre-dispatch cancellation lost NotDispatched",
        ),
        (
            BoundedCancellationCaseObservation {
                signal: BoundedCancellationSignal::ReconciledFromPhysicalTruth,
                ..accepted.pre_dispatch
            },
            "A16 pre-dispatch cancellation lacked Signal cancellation",
        ),
        (
            BoundedCancellationCaseObservation {
                dispatch: BoundedCancellationDispatch::WriteCompleted,
                ..accepted.pre_dispatch
            },
            "A16 pre-dispatch work was not denied as ConsumerCancelled",
        ),
        (
            BoundedCancellationCaseObservation {
                recovery: BoundedCancellationRecovery::ContinueSettlement,
                ..accepted.pre_dispatch
            },
            "A16 pre-dispatch cancellation fabricated settlement recovery",
        ),
        (
            BoundedCancellationCaseObservation {
                terminal: BoundedCancellationTerminal::ContinuedAfterConsumerCancellation,
                ..accepted.pre_dispatch
            },
            "A16 pre-dispatch cancellation reached the wrong terminal fate",
        ),
        (
            BoundedCancellationCaseObservation {
                terminal_media_effects: 1,
                ..accepted.pre_dispatch
            },
            "A16 pre-dispatch cancellation reached media",
        ),
    ] {
        let observation = BoundedCancellationObservation {
            pre_dispatch: hostile,
            ..accepted
        };
        assert_eq!(
            verify(observation, &accepted_work(), STORE, RUNTIME, GENERATION).unwrap_err(),
            denial
        );
    }
}

#[test]
fn post_dispatch_cancellation_rejects_each_semantic_substitution_exactly() {
    let accepted = accepted_cancellation();
    for (hostile, denial) in [
        (
            BoundedCancellationCaseObservation {
                obligation: BoundedCancellationObligation::NotDispatched,
                ..accepted.post_dispatch
            },
            "A16 post-dispatch cancellation lost SettlementContinues",
        ),
        (
            BoundedCancellationCaseObservation {
                signal: BoundedCancellationSignal::RequestCancelled,
                ..accepted.post_dispatch
            },
            "A16 post-dispatch settlement did not reconcile from physical truth",
        ),
        (
            BoundedCancellationCaseObservation {
                dispatch: BoundedCancellationDispatch::DeniedConsumerCancelled,
                ..accepted.post_dispatch
            },
            "A16 post-dispatch work did not complete its write",
        ),
        (
            BoundedCancellationCaseObservation {
                recovery: BoundedCancellationRecovery::NoSettlement,
                ..accepted.post_dispatch
            },
            "A16 post-dispatch work lost ContinueSettlement",
        ),
        (
            BoundedCancellationCaseObservation {
                terminal: BoundedCancellationTerminal::CancelledBeforeDispatch,
                ..accepted.post_dispatch
            },
            "A16 post-dispatch work reached the wrong terminal fate",
        ),
        (
            BoundedCancellationCaseObservation {
                cancellation_media_effects: 1,
                ..accepted.post_dispatch
            },
            "A16 post-dispatch media and cancellation deltas did not reconcile",
        ),
    ] {
        let observation = BoundedCancellationObservation {
            post_dispatch: hostile,
            ..accepted
        };
        assert_eq!(
            verify(observation, &accepted_work(), STORE, RUNTIME, GENERATION).unwrap_err(),
            denial
        );
    }
}

#[test]
fn cancellation_oracle_rejects_identity_and_raw_record_fraud_exactly() {
    let accepted = accepted_cancellation();
    let foreign = BoundedCancellationObservation {
        pre_dispatch: BoundedCancellationCaseObservation {
            runtime: RUNTIME + 1,
            ..accepted.pre_dispatch
        },
        ..accepted
    };
    assert_eq!(
        verify(foreign, &accepted_work(), STORE, RUNTIME, GENERATION).unwrap_err(),
        "A16 pre-dispatch cancellation carried a foreign physical work identity"
    );

    let reused = BoundedCancellationObservation {
        post_dispatch: BoundedCancellationCaseObservation {
            operation: accepted.pre_dispatch.operation,
            ..accepted.post_dispatch
        },
        ..accepted
    };
    assert_eq!(
        verify(reused, &accepted_work(), STORE, RUNTIME, GENERATION).unwrap_err(),
        "A16 cancellation seams reused one physical work identity"
    );

    let mut missing = accepted_work();
    missing.records = Box::new([]);
    assert_eq!(
        verify(accepted, &missing, STORE, RUNTIME, GENERATION).unwrap_err(),
        "A16 post-dispatch cancellation lost its causal work record"
    );

    let mut wrong_receipt = accepted_work();
    wrong_receipt.records[0].backend_operation += 1;
    assert_eq!(
        verify(accepted, &wrong_receipt, STORE, RUNTIME, GENERATION).unwrap_err(),
        "A16 post-dispatch cancellation did not join its exact write settlement"
    );
}

#[test]
fn cancellation_oracle_rejects_each_identity_axis_and_seam_label() {
    let accepted = accepted_cancellation();
    let work = accepted_work();
    let mut hostile = accepted;
    hostile.pre_dispatch.seam = BoundedCancellationSeam::PostDispatch;
    assert_eq!(
        denial(hostile, &work),
        "A16 pre-dispatch evidence named the wrong seam"
    );
    hostile = accepted;
    hostile.pre_dispatch.store = [8; 16];
    assert_eq!(
        denial(hostile, &work),
        "A16 pre-dispatch cancellation carried a foreign physical work identity"
    );
    hostile = accepted;
    hostile.pre_dispatch.generation += 1;
    assert_eq!(
        denial(hostile, &work),
        "A16 pre-dispatch cancellation carried a foreign physical work identity"
    );
    hostile = accepted;
    hostile.pre_dispatch.operation = 0;
    assert_eq!(
        denial(hostile, &work),
        "A16 pre-dispatch cancellation carried a foreign physical work identity"
    );
    hostile = accepted;
    hostile.post_dispatch.seam = BoundedCancellationSeam::PreDispatch;
    assert_eq!(
        denial(hostile, &work),
        "A16 post-dispatch evidence named the wrong seam"
    );
    hostile = accepted;
    hostile.post_dispatch.runtime += 1;
    assert_eq!(
        denial(hostile, &work),
        "A16 post-dispatch cancellation carried a foreign physical work identity"
    );
}

#[test]
fn cancellation_oracle_rejects_each_media_and_receipt_substitution() {
    let accepted = accepted_cancellation();
    let work = accepted_work();
    let mut hostile = accepted;
    hostile.pre_dispatch.media_before_cancellation = 1;
    assert_eq!(
        denial(hostile, &work),
        "A16 pre-dispatch cancellation reached media"
    );
    hostile = accepted;
    hostile.pre_dispatch.cancellation_media_effects = 1;
    assert_eq!(
        denial(hostile, &work),
        "A16 pre-dispatch cancellation reached media"
    );
    hostile = accepted;
    hostile.pre_dispatch.terminal_media_effects = 1;
    assert_eq!(
        denial(hostile, &work),
        "A16 pre-dispatch cancellation reached media"
    );
    hostile = accepted;
    hostile.pre_dispatch.backend_receipt = Some(500);
    assert_eq!(
        denial(hostile, &work),
        "A16 pre-dispatch cancellation reached media"
    );
    hostile = accepted;
    hostile.post_dispatch.media_before_cancellation = 0;
    assert_eq!(
        denial(hostile, &work),
        "A16 post-dispatch media and cancellation deltas did not reconcile"
    );
    hostile = accepted;
    hostile.post_dispatch.cancellation_media_effects = 1;
    assert_eq!(
        denial(hostile, &work),
        "A16 post-dispatch media and cancellation deltas did not reconcile"
    );
    hostile = accepted;
    hostile.post_dispatch.terminal_media_effects = 0;
    assert_eq!(
        denial(hostile, &work),
        "A16 post-dispatch media and cancellation deltas did not reconcile"
    );
    hostile = accepted;
    hostile.post_dispatch.backend_receipt = None;
    assert_eq!(
        denial(hostile, &work),
        "A16 post-dispatch media and cancellation deltas did not reconcile"
    );
}

#[test]
fn cancellation_oracle_rejects_each_raw_work_join_substitution() {
    let accepted = accepted_cancellation();
    let mut work = accepted_work();
    work.records = Box::new([
        BoundedResidencyWorkRecordObservation {
            operation: accepted.pre_dispatch.operation,
            ..work.records[0]
        },
        work.records[0],
    ]);
    assert_eq!(
        denial(accepted, &work),
        "A16 pre-dispatch cancellation produced a causal media record"
    );

    work = accepted_work();
    work.records = Box::new([work.records[0], work.records[0]]);
    assert_eq!(
        denial(accepted, &work),
        "A16 post-dispatch cancellation duplicated its causal work record"
    );
    work = accepted_work();
    work.records[0].family = BoundedResidencyWorkFamily::ArtifactRangeRead;
    assert_eq!(denial(accepted, &work), exact_join_denial());
    work = accepted_work();
    work.records[0].effect_fate = BoundedResidencyWorkEffectFate::ReadCompleted;
    assert_eq!(denial(accepted, &work), exact_join_denial());
    work = accepted_work();
    work.records[0].recovery = BoundedResidencyWorkRecovery::NoEffect;
    assert_eq!(denial(accepted, &work), exact_join_denial());
    work = accepted_work();
    work.records[0].terminal = BoundedResidencyWorkTerminalFate::Settled;
    assert_eq!(denial(accepted, &work), exact_join_denial());
    work = accepted_work();
    work.records[0].backend_operation += 1;
    assert_eq!(denial(accepted, &work), exact_join_denial());
    work = accepted_work();
    work.continued_terminal_fates = 0;
    assert_eq!(
        denial(accepted, &work),
        "A16 post-dispatch cancellation did not own the one continued terminal fate"
    );
}

fn denial(
    cancellation: BoundedCancellationObservation,
    work: &BoundedResidencyWorkReconciliationObservation,
) -> String {
    verify(cancellation, work, STORE, RUNTIME, GENERATION).unwrap_err()
}

fn exact_join_denial() -> &'static str {
    "A16 post-dispatch cancellation did not join its exact write settlement"
}

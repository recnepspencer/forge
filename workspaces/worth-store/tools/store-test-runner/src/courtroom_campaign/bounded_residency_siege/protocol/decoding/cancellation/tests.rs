use super::parse;
use crate::courtroom_campaign::bounded_residency_siege::protocol::{
    BoundedCancellationDispatch, BoundedCancellationObligation, BoundedCancellationRecovery,
    BoundedCancellationSignal, BoundedCancellationTerminal,
};

const PRE: &str = "BOUNDED_RESIDENCY_CANCELLATION pre-dispatch \
09090909090909090909090909090909 11 13 70 not-dispatched request-cancelled \
denied-consumer-cancelled no-settlement cancelled-before-dispatch 0 0 0 0";
const POST: &str = "BOUNDED_RESIDENCY_CANCELLATION post-dispatch \
09090909090909090909090909090909 11 13 71 settlement-continues \
reconciled-from-physical-truth write-completed continue-settlement \
continued-after-consumer-cancellation 1 0 1 501";

#[test]
fn cancellation_decoder_requires_two_distinct_named_seams() {
    let observation = parse(&[PRE.to_owned(), POST.to_owned()]).unwrap();

    assert_eq!(
        observation.pre_dispatch.obligation,
        BoundedCancellationObligation::NotDispatched
    );
    assert_eq!(
        observation.pre_dispatch.signal,
        BoundedCancellationSignal::RequestCancelled
    );
    assert_eq!(
        observation.pre_dispatch.dispatch,
        BoundedCancellationDispatch::DeniedConsumerCancelled
    );
    assert_eq!(
        observation.pre_dispatch.recovery,
        BoundedCancellationRecovery::NoSettlement
    );
    assert_eq!(
        observation.pre_dispatch.terminal,
        BoundedCancellationTerminal::CancelledBeforeDispatch
    );
    assert_eq!(observation.post_dispatch.backend_receipt, Some(501));
}

#[test]
fn cancellation_decoder_rejects_missing_duplicate_and_malformed_seams_exactly() {
    assert_eq!(
        parse(&[PRE.to_owned()]).unwrap_err(),
        "missing bounded-residency post-dispatch cancellation seam"
    );
    assert_eq!(
        parse(&[PRE.to_owned(), PRE.to_owned(), POST.to_owned()]).unwrap_err(),
        "duplicate bounded-residency pre-dispatch cancellation seam"
    );
    assert_eq!(
        parse(&[PRE.replace("pre-dispatch", "unknown"), POST.to_owned()]).unwrap_err(),
        "unknown bounded-residency cancellation seam unknown"
    );
    let malformed = format!("{PRE} extra");
    assert_eq!(
        parse(&[malformed.clone(), POST.to_owned()]).unwrap_err(),
        format!("malformed Courtroom C marker `{malformed}`")
    );
}

#[test]
fn cancellation_decoder_rejects_each_untyped_lifecycle_token_exactly() {
    for (hostile, denial) in [
        (
            PRE.replace("not-dispatched", "unknown-obligation"),
            "unknown cancellation obligation unknown-obligation",
        ),
        (
            PRE.replace("request-cancelled", "unknown-signal"),
            "unknown cancellation Signal outcome unknown-signal",
        ),
        (
            PRE.replace("denied-consumer-cancelled", "unknown-dispatch"),
            "unknown cancellation dispatch outcome unknown-dispatch",
        ),
        (
            PRE.replace("no-settlement", "unknown-recovery"),
            "unknown cancellation recovery outcome unknown-recovery",
        ),
        (
            PRE.replace("cancelled-before-dispatch", "unknown-terminal"),
            "unknown cancellation terminal fate unknown-terminal",
        ),
        (
            PRE.replace("09090909090909090909090909090909", "09"),
            "cancellation Store identity must contain 32 hex characters",
        ),
    ] {
        assert_eq!(
            parse(&[hostile, POST.to_owned()]).unwrap_err(),
            denial,
            "hostile cancellation marker escaped typed decoding"
        );
    }
}

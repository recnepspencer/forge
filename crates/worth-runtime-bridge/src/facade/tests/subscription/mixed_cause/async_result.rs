use super::*;
use crate::facade::tests::source::support::denied_request_response_completion_after_cancellation;
use crate::facade::{
    BridgeAsyncCompletionDenialClass, BridgeAsyncCompletionState, BridgeMixedCauseAsyncResultCause,
    BridgeMixedCauseAsyncResultDisposition,
};

#[test]
fn ordinary_denied_async_completion_remains_an_ordered_result_transition() {
    let runtime = runtime(BridgeRuntimePolicy::development());
    let report = denied_request_response_completion_after_cancellation(
        &runtime,
        NodeId::new(244, 0),
        BridgeAsyncRequestTruthViewBasis::authoritative(
            crate::truth_identity_fixtures::truth_branch_fixture("truth-main"),
            crate::truth_identity_fixtures::truth_commit_fixture("commit-a"),
            crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
        ),
    );
    let denied = report
        .denied_completion()
        .expect("Signal cancellation must yield a denied Bridge completion")
        .clone();
    let ordering = runtime.order_mixed_causes(&BridgeMixedCauseOrderingRequest::new(
        BridgeMixedCauseOrderingLaneKind::Authoritative,
        vec![BridgeMixedCauseOrderingInput::AsyncDeniedCompletion(denied)],
    ));

    assert_eq!(ordering.ordered().len(), 1);
    assert!(ordering.denied().is_empty());
    assert_eq!(
        ordering.ordered()[0].family_kind(),
        BridgeMixedCauseOrderFamilyKind::AsyncDeniedCompletion
    );
    assert!(matches!(
        ordering.async_result_transitions(),
        [transition]
            if transition.cause()
                == BridgeMixedCauseAsyncResultCause::Completion(
                    BridgeAsyncCompletionState::Denied(
                        BridgeAsyncCompletionDenialClass::Cancelled
                    )
                )
                && matches!(
                    transition.disposition(),
                    BridgeMixedCauseAsyncResultDisposition::Ordered { ordinal: 0 }
                )
    ));
}

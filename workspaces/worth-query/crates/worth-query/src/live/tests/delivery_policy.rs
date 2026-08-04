use crate::live::*;
#[test]
fn detail_width_overflow_is_rejected() {
    let preflight = crate::harness::fixtures::execution_preflights::direct_runtime_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("detail preflight should promote");

    let width = live.evaluate_delivery_width(33);

    assert_eq!(width.budget_limit(), 32);
    assert_eq!(width.measured_width(), 33);
    assert_eq!(width.resolution(), &PatchWidthResolution::Reject);
    let counters = LivePolicyCounters::from_width_assessment(&width);
    assert_eq!(counters.live_patch_width_overflow_count(), 1);
    assert_eq!(counters.live_refresh_denial_count(), 0);
}

#[test]
fn ordered_collection_width_overflow_requests_coalescing() {
    let preflight = crate::harness::fixtures::execution_preflights::
            ordered_collection_without_traversal_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("collection preflight should promote");

    let width = live.evaluate_delivery_width(65);

    assert_eq!(width.budget_limit(), 64);
    assert_eq!(width.resolution(), &PatchWidthResolution::Coalesce);
}

#[test]
fn bounded_materialization_width_overflow_requests_refresh() {
    let preflight = crate::harness::fixtures::execution_preflights::ordered_collection_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("collection preflight should promote");

    let width = live.evaluate_delivery_width(97);

    assert_eq!(width.budget_limit(), 96);
    match width.resolution() {
        PatchWidthResolution::Refresh(fallback) => {
            assert_eq!(
                fallback.admission_class(),
                &RefreshAdmissionClass::WidthOverflow
            );
            assert_eq!(
                fallback.admission_status(),
                &crate::live_performance::RefreshAdmissionStatus::Debt
            );
        }
        other => panic!("expected refresh resolution, got {other:?}"),
    }
}

#[test]
fn multi_bundle_delivery_requires_admitted_coalescing_class() {
    let preflight = crate::harness::fixtures::execution_preflights::
            ordered_collection_without_traversal_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("collection preflight should promote");

    let decision = live
        .request_coalesced_delivery(3)
        .expect("ordered collection should admit basis-stable coalescing");

    assert_eq!(decision, CoalescingDecision::Admitted { bundle_count: 3 });
}

#[test]
fn explicit_refresh_request_is_denied_for_detail_family() {
    let preflight = crate::harness::fixtures::execution_preflights::direct_runtime_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("detail preflight should promote");

    let error = live
        .request_refresh_fallback(RefreshAdmissionClass::WidthOverflow)
        .expect_err("detail family should forbid refresh admission");

    assert_eq!(
        error,
        LiveRefreshError::ForbiddenAdmissionClass(RefreshAdmissionClass::WidthOverflow)
    );
}

#[test]
fn rejection_helpers_fail_loudly_when_operation_is_actually_admitted() {
    let ordered_preflight = crate::harness::fixtures::execution_preflights::
            ordered_collection_without_traversal_preflight();
    let ordered_live = promote_preflight_bundle_to_live(&ordered_preflight)
        .expect("ordered collection preflight should promote");
    let bounded_preflight =
        crate::harness::fixtures::execution_preflights::ordered_collection_preflight();
    let bounded_live = promote_preflight_bundle_to_live(&bounded_preflight)
        .expect("bounded materialization preflight should promote");
    let detail_preflight =
        crate::harness::fixtures::execution_preflights::direct_runtime_preflight();
    let detail_live = promote_preflight_bundle_to_live(&detail_preflight)
        .expect("detail preflight should promote");

    let refresh_error = MilestoneFiveLiveAdapter::refresh_rejection_lane(
        "refresh-should-fail",
        &bounded_live,
        RefreshAdmissionClass::WidthOverflow,
    )
    .expect_err("admitted refresh should not be encoded as a rejection lane");
    let coalescing_error = MilestoneFiveLiveAdapter::coalescing_rejection_lane(
        "coalescing-should-fail",
        &ordered_live,
        3,
    )
    .expect_err("admitted coalescing should not be encoded as a rejection lane");
    let progress_error = MilestoneFiveLiveAdapter::progress_rejection_lane(
        "progress-should-fail",
        &detail_live,
        LiveChangeOrdinal::from_value(1),
        detail_preflight.basis().clone(),
    )
    .expect_err("monotonic progress should not be encoded as a rejection lane");

    assert_eq!(
        refresh_error,
        LiveExpectedRejectionError::UnexpectedRefreshAdmission {
            admission_class: RefreshAdmissionClass::WidthOverflow,
            admission_status: crate::live_performance::RefreshAdmissionStatus::Debt,
        }
    );
    assert_eq!(
        coalescing_error,
        LiveExpectedRejectionError::UnexpectedCoalescingAdmission {
            decision: CoalescingDecision::Admitted { bundle_count: 3 },
        }
    );
    assert!(matches!(
        progress_error,
        LiveExpectedRejectionError::UnexpectedProgressAdvance { ordinal: 1, .. }
    ));
}

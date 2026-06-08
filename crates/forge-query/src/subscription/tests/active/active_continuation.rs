use super::runtime_harness::{attached_consumer, attached_future_consumer, delivery_budget};
use super::*;
use crate::live::LiveQueryFamily;
use crate::view_shape_live::LiveViewShapeFamily;

#[test]
fn identity_remap_continuation_is_patch_visible_and_changes_window_digest() {
    let mut runtime = ActiveSubscriptionRuntime::new();
    let (_, attachment) = attached_consumer(
        &mut runtime,
        LiveQueryFamily::Detail,
        None,
        "consumer-a",
        1,
        2,
        DeliveryBackpressurePolicy::RetainWithinWindow,
    );
    let window =
        open_query_delivery_window(&mut runtime, &attachment, delivery_budget(2, 2)).unwrap();
    let original_window_digest = window.delivery_window_digest().to_string();
    let evidence = admit_subscription_continuation_evidence(
        attachment.lane_digest().clone(),
        SubscriptionContinuationClass::IdentityRemap,
        "employee:old",
        "employee:new",
        "basis:current",
        "identity-evolution-authority",
        ContinuationRemapWidth::measured(1),
    )
    .unwrap();

    let (window, report) =
        apply_active_subscription_continuation(&mut runtime, window, evidence).unwrap();
    let continued_window_digest = window.delivery_window_digest().to_string();
    let (delta, continuation_counters) = lower_subscription_continuation_report(&report);
    let (delta, lowering_report, _) = lower_query_subscription_maintenance_delta(delta).unwrap();
    let packet = build_active_delivery_work_packet(
        &mut runtime,
        &attachment,
        delta,
        lowering_report,
        ActiveDeliveryDensityPosture::SparseDelta,
        ActiveDeliveryAffectedLaneWidth::measured(1),
        ActiveDeliveryAffectedAttachmentWidth::measured(1),
        PatchGroupWidth::measured(1),
        ActiveDeliveryContinuationWidth::measured(1),
        ActiveDeliveryPreviewResidueWidth::measured(0),
        ActiveAllocationScopeWidth::measured(1),
        ActiveSubscriptionAllocationPosture::PatchScratch,
    )
    .unwrap();
    let batch = emit_query_delivery_batch(&mut runtime, window, packet).unwrap();

    assert_ne!(continued_window_digest, original_window_digest);
    assert_eq!(report.remap_width(), 1);
    assert_eq!(report.performance_receipt().consumed_width(), 1);
    assert_eq!(report.performance_receipt().budgeted_width(), 1);
    assert_eq!(continuation_counters.continuation_remap_count(), 1);
    assert_eq!(continuation_counters.continuation_remap_width(), 1);
    assert_eq!(
        batch.patch_group().kind(),
        QueryPatchGroupKind::ContinuationPatchGroup
    );
    assert_eq!(batch.counters().continuation_remap_count(), 1);
}

#[test]
fn advisory_and_identity_break_continuations_are_distinct_counters() {
    let mut runtime = ActiveSubscriptionRuntime::new();
    let (_, attachment) = attached_consumer(
        &mut runtime,
        LiveQueryFamily::Detail,
        None,
        "consumer-a",
        1,
        2,
        DeliveryBackpressurePolicy::RetainWithinWindow,
    );

    let advisory = admit_subscription_continuation_evidence(
        attachment.lane_digest().clone(),
        SubscriptionContinuationClass::CorrespondenceAdvisory,
        "employee:maybe-old",
        "employee:maybe-new",
        "basis:current",
        "correspondence-authority",
        ContinuationRemapWidth::measured(1),
    )
    .unwrap();
    let identity_break = admit_subscription_continuation_evidence(
        attachment.lane_digest().clone(),
        SubscriptionContinuationClass::IdentityBreak,
        "employee:old",
        "identity-break-terminal",
        "basis:current",
        "identity-evolution-authority",
        ContinuationRemapWidth::measured(1),
    )
    .unwrap();

    let (_, advisory_report) = apply_subscription_continuation(
        open_query_delivery_window(&mut runtime, &attachment, delivery_budget(2, 2)).unwrap(),
        advisory,
    )
    .unwrap();
    let (_, advisory_counters) = lower_subscription_continuation_report(&advisory_report);
    let (_, break_report) = apply_subscription_continuation(
        open_query_delivery_window(&mut runtime, &attachment, delivery_budget(2, 2)).unwrap(),
        identity_break,
    )
    .unwrap();
    let (_, break_counters) = lower_subscription_continuation_report(&break_report);

    assert_eq!(advisory_counters.continuation_advisory_count(), 1);
    assert_eq!(advisory_counters.continuation_identity_break_count(), 0);
    assert_ne!(
        advisory_report
            .performance_receipt()
            .performance_receipt_digest(),
        break_report
            .performance_receipt()
            .performance_receipt_digest()
    );
    assert_eq!(break_counters.continuation_identity_break_count(), 1);
    assert_eq!(break_counters.continuation_advisory_count(), 0);
}

#[test]
fn unsupported_continuation_and_foreign_lane_evidence_deny_typed_and_early() {
    let mut runtime = ActiveSubscriptionRuntime::new();
    let (_, detail) = attached_consumer(
        &mut runtime,
        LiveQueryFamily::Detail,
        None,
        "consumer-a",
        1,
        2,
        DeliveryBackpressurePolicy::RetainWithinWindow,
    );
    let (_, collection) = attached_consumer(
        &mut runtime,
        LiveQueryFamily::OrderedCollection,
        Some(LiveViewShapeFamily::Table),
        "consumer-b",
        1,
        2,
        DeliveryBackpressurePolicy::RetainWithinWindow,
    );

    let unsupported = admit_subscription_continuation_evidence(
        detail.lane_digest().clone(),
        SubscriptionContinuationClass::UnsupportedContinuation,
        "employee:old",
        "employee:new",
        "basis:current",
        "identity-evolution-authority",
        ContinuationRemapWidth::measured(1),
    )
    .unwrap_err();
    assert_eq!(
        unsupported.denial_kind(),
        &SubscriptionContinuationDenialKind::UnsupportedContinuationClass
    );
    assert_eq!(unsupported.counters().continuation_remap_denial_count(), 1);

    let preview_promotion = admit_subscription_continuation_evidence(
        detail.lane_digest().clone(),
        SubscriptionContinuationClass::PreviewPromotionRemap,
        "preview:employee:old",
        "authoritative:employee:new",
        "basis:preview",
        "preview-promotion-authority",
        ContinuationRemapWidth::measured(1),
    )
    .unwrap_err();
    assert_eq!(
        preview_promotion.denial_kind(),
        &SubscriptionContinuationDenialKind::UnsupportedContinuationClass
    );
    assert_eq!(
        preview_promotion
            .counters()
            .continuation_remap_denial_count(),
        1
    );

    let foreign = admit_subscription_continuation_evidence(
        collection.lane_digest().clone(),
        SubscriptionContinuationClass::CollectionMembershipRemap,
        "employee:old",
        "employee:new",
        "basis:current",
        "identity-evolution-authority",
        ContinuationRemapWidth::measured(1),
    )
    .unwrap();
    let window = open_query_delivery_window(&mut runtime, &detail, delivery_budget(2, 2)).unwrap();
    let error = apply_subscription_continuation(window, foreign).unwrap_err();

    assert_eq!(
        error.denial_kind(),
        &SubscriptionContinuationDenialKind::ContinuationEvidenceMismatch
    );
    assert_eq!(error.counters().continuation_remap_denial_count(), 1);
}

#[test]
fn future_bearing_continuation_retains_checkpoint_and_future_identity() {
    let mut runtime = ActiveSubscriptionRuntime::new();
    let attachment = attached_future_consumer(
        &mut runtime,
        LiveQueryFamily::OrderedCollection,
        Some(LiveViewShapeFamily::Table),
        "consumer-a",
        QuerySubscriptionFutureSelection::temporal_async_with_identity(
            true,
            vec![QuerySubscriptionAsyncRequestIdentityPart::new(
                "request",
                "employees",
            )],
        ),
        1,
        2,
        DeliveryBackpressurePolicy::RetainWithinWindow,
    );
    let evidence = admit_subscription_continuation_evidence_with_active_identity(
        attachment.lane_digest().clone(),
        SubscriptionContinuationClass::IdentityRemap,
        "employee:old",
        "employee:new",
        attachment.future_selection().clone(),
        attachment.basis_binding_digest(),
        attachment.checkpoint_identity_digest(),
        "identity-evolution-authority",
        ContinuationRemapWidth::measured(1),
    )
    .unwrap();
    let window =
        open_query_delivery_window(&mut runtime, &attachment, delivery_budget(2, 2)).unwrap();

    let (window, report) =
        apply_active_subscription_continuation(&mut runtime, window, evidence).unwrap();

    assert_eq!(
        report.future_selection().projection_digest(),
        attachment.future_selection().projection_digest()
    );
    assert_eq!(
        report.checkpoint_identity_digest(),
        attachment.checkpoint_identity_digest()
    );
    assert_eq!(window.active_lane_digest(), attachment.lane_digest());
}

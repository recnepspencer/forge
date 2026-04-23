use super::*;
use crate::live::LiveQueryFamily;
use crate::view_shape_live::LiveViewShapeFamily;

fn active_budget() -> ActiveSubscriptionWorkBudget {
    ActiveSubscriptionWorkBudget::admitted(
        ActiveRegistryLookupWidth::measured(1),
        ActiveFanoutWidth::measured(2),
        ActiveAllocationScopeWidth::measured(1),
        ActiveSubscriptionAllocationPolicy::LifecycleArena,
    )
}

fn attachment_budget() -> SubscriptionConsumerAttachmentBudget {
    SubscriptionConsumerAttachmentBudget::admitted(
        ActiveFanoutWidth::measured(2),
        ConsumerDeliveryPacingWidth::measured(1),
        ActiveAllocationScopeWidth::measured(1),
        DeliveryBackpressurePolicy::RetainWithinWindow,
    )
}

fn delivery_budget() -> QueryDeliveryWindowBudget {
    QueryDeliveryWindowBudget::admitted(
        DeliveryWindowWidth::measured(2),
        PatchGroupWidth::measured(2),
        MaintenanceDeltaWidth::measured(2),
        ActiveAllocationScopeWidth::measured(1),
        ActiveSubscriptionAllocationPosture::DeliveryWindowArena,
        DeliveryBackpressurePolicy::RetainWithinWindow,
    )
}

fn activation_for(
    live_family: LiveQueryFamily,
    view_family: Option<LiveViewShapeFamily>,
) -> SubscriptionActivationInput {
    let input = LiveQueryAdmissionArtifact::for_test(
        live_family,
        view_family,
        QuerySubscriptionConstructionSource::FacadeLive,
    );
    let selection = select_query_subscription_family(input, roomy_budget()).unwrap();
    let declaration = declare_query_subscription(selection, roomy_slice_budget()).unwrap();
    let lowering =
        lower_query_subscription_to_bridge(declaration, roomy_lowering_budget()).unwrap();
    let admission = admit_query_subscription(
        lowering,
        QuerySubscriptionAdmissionBudget::admitted(1, 1, 1, 1, 1),
    )
    .unwrap();
    prepare_subscription_activation(admission)
}

fn attached_consumer(
    runtime: &mut ActiveSubscriptionRuntime,
    live_family: LiveQueryFamily,
    view_family: Option<LiveViewShapeFamily>,
    consumer: &str,
) -> SubscriptionConsumerAttachment {
    let activation = activation_for(live_family, view_family);
    let admission = admit_active_subscription_lane(activation, active_budget()).unwrap();
    let handle = open_active_subscription_lane(runtime, admission).unwrap();
    attach_subscription_consumer(
        runtime,
        &handle,
        SubscriptionConsumerAttachmentRequest::admitted(consumer, "cursor"),
        attachment_budget(),
    )
    .unwrap()
}

#[test]
fn identity_remap_continuation_is_patch_visible_and_changes_window_digest() {
    let mut runtime = ActiveSubscriptionRuntime::new();
    let attachment = attached_consumer(&mut runtime, LiveQueryFamily::Detail, None, "consumer-a");
    let window = open_query_delivery_window(&mut runtime, &attachment, delivery_budget()).unwrap();
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
    let attachment = attached_consumer(&mut runtime, LiveQueryFamily::Detail, None, "consumer-a");

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
        open_query_delivery_window(&mut runtime, &attachment, delivery_budget()).unwrap(),
        advisory,
    )
    .unwrap();
    let (_, advisory_counters) = lower_subscription_continuation_report(&advisory_report);
    let (_, break_report) = apply_subscription_continuation(
        open_query_delivery_window(&mut runtime, &attachment, delivery_budget()).unwrap(),
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
    let detail = attached_consumer(&mut runtime, LiveQueryFamily::Detail, None, "consumer-a");
    let collection = attached_consumer(
        &mut runtime,
        LiveQueryFamily::OrderedCollection,
        Some(LiveViewShapeFamily::Table),
        "consumer-b",
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
    let window = open_query_delivery_window(&mut runtime, &detail, delivery_budget()).unwrap();
    let error = apply_subscription_continuation(window, foreign).unwrap_err();

    assert_eq!(
        error.denial_kind(),
        &SubscriptionContinuationDenialKind::ContinuationEvidenceMismatch
    );
    assert_eq!(error.counters().continuation_remap_denial_count(), 1);
}

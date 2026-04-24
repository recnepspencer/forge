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

fn active_lane_for(
    runtime: &mut ActiveSubscriptionRuntime,
    live_family: LiveQueryFamily,
    view_family: Option<LiveViewShapeFamily>,
) -> ActiveSubscriptionLaneHandle {
    let activation = activation_for(live_family, view_family);
    let admission = admit_active_subscription_lane(activation, active_budget()).unwrap();
    open_active_subscription_lane(runtime, admission).unwrap()
}

fn attached_consumer(
    runtime: &mut ActiveSubscriptionRuntime,
    live_family: LiveQueryFamily,
    view_family: Option<LiveViewShapeFamily>,
    consumer: &str,
) -> (ActiveSubscriptionLaneHandle, SubscriptionConsumerAttachment) {
    let handle = active_lane_for(runtime, live_family, view_family);
    let attachment = attach_subscription_consumer(
        runtime,
        &handle,
        SubscriptionConsumerAttachmentRequest::admitted(consumer, "cursor"),
        attachment_budget(),
    )
    .unwrap();
    (handle, attachment)
}

fn zero_authoritative_residue() -> PreviewSubscriptionResidueReport {
    measure_preview_subscription_residue(
        PreviewResidueWidth::measured(0),
        PreviewResidueWidth::measured(0),
        PreviewResidueWidth::measured(0),
        PreviewResidueWidth::measured(0),
        PreviewResidueWidth::measured(0),
        PreviewResidueWidth::measured(1),
        PreviewResidueWidth::measured(1),
    )
}

#[test]
fn preview_discard_zero_authoritative_residue_produces_closeout() {
    let mut runtime = ActiveSubscriptionRuntime::new();
    let (_, attachment) =
        attached_consumer(&mut runtime, LiveQueryFamily::Detail, None, "preview-a");
    let isolation = admit_preview_subscription_isolation(
        &attachment,
        "preview-epoch-a",
        PreviewResidueWidth::measured(2),
    )
    .unwrap();
    let residue = zero_authoritative_residue();
    let residue_digest = residue.report_digest().to_string();

    let closeout = discard_preview_subscription(isolation, residue).unwrap();

    assert_eq!(closeout.active_lane_digest(), attachment.lane_digest());
    assert_eq!(closeout.attachment_digest(), attachment.attachment_digest());
    assert_eq!(closeout.residue_report_digest(), residue_digest);
    assert_eq!(closeout.counters().preview_discard_residue_check_count(), 1);
    assert_eq!(closeout.counters().preview_residue_width(), 2);
    assert_eq!(closeout.counters().preview_authoritative_residue_count(), 0);
    assert_eq!(closeout.performance_receipt().consumed_width(), 2);
    assert_eq!(closeout.performance_receipt().budgeted_width(), 2);
}

#[test]
fn preview_discard_with_authoritative_residue_denies_typed_and_early() {
    let mut runtime = ActiveSubscriptionRuntime::new();
    let (_, attachment) =
        attached_consumer(&mut runtime, LiveQueryFamily::Detail, None, "preview-a");
    let isolation = admit_preview_subscription_isolation(
        &attachment,
        "preview-epoch-a",
        PreviewResidueWidth::measured(4),
    )
    .unwrap();
    let residue = measure_preview_subscription_residue(
        PreviewResidueWidth::measured(1),
        PreviewResidueWidth::measured(0),
        PreviewResidueWidth::measured(1),
        PreviewResidueWidth::measured(0),
        PreviewResidueWidth::measured(0),
        PreviewResidueWidth::measured(1),
        PreviewResidueWidth::measured(1),
    );

    let error = discard_preview_subscription(isolation, residue).unwrap_err();

    assert_eq!(
        error.denial_kind(),
        &PreviewSubscriptionIsolationDenialKind::PreviewDiscardResidueDenied
    );
    assert_eq!(error.counters().preview_discard_residue_check_count(), 1);
    assert_eq!(error.counters().preview_authoritative_residue_count(), 2);
    assert_eq!(error.counters().preview_residue_width(), 4);
}

#[test]
fn preview_discard_denies_when_total_residue_exceeds_admitted_budget() {
    let mut runtime = ActiveSubscriptionRuntime::new();
    let (_, attachment) =
        attached_consumer(&mut runtime, LiveQueryFamily::Detail, None, "preview-a");
    let isolation = admit_preview_subscription_isolation(
        &attachment,
        "preview-epoch-a",
        PreviewResidueWidth::measured(1),
    )
    .unwrap();
    let residue = measure_preview_subscription_residue(
        PreviewResidueWidth::measured(0),
        PreviewResidueWidth::measured(0),
        PreviewResidueWidth::measured(0),
        PreviewResidueWidth::measured(0),
        PreviewResidueWidth::measured(0),
        PreviewResidueWidth::measured(1),
        PreviewResidueWidth::measured(1),
    );

    let error = discard_preview_subscription(isolation, residue).unwrap_err();

    assert_eq!(
        error.denial_kind(),
        &PreviewSubscriptionIsolationDenialKind::PreviewDiscardResidueDenied
    );
    assert_eq!(error.counters().preview_discard_residue_check_count(), 1);
    assert_eq!(error.counters().preview_authoritative_residue_count(), 0);
    assert_eq!(error.counters().preview_residue_width(), 2);
}

#[test]
fn preview_authoritative_sharing_is_denied_even_with_matching_lane_digest() {
    let mut runtime = ActiveSubscriptionRuntime::new();
    let (handle, attachment) =
        attached_consumer(&mut runtime, LiveQueryFamily::Detail, None, "preview-a");
    let isolation = admit_preview_subscription_isolation(
        &attachment,
        "preview-epoch-a",
        PreviewResidueWidth::measured(1),
    )
    .unwrap();

    let error = deny_preview_authoritative_sharing(&isolation, &handle).unwrap_err();

    assert_eq!(
        error.denial_kind(),
        &PreviewSubscriptionIsolationDenialKind::PreviewAuthoritativeSharingDenied
    );
    assert_eq!(
        error
            .counters()
            .preview_authoritative_sharing_denial_count(),
        1
    );
    assert_eq!(isolation.active_lane_digest(), handle.lane_digest());
}

#[test]
fn preview_promotion_emits_handoff_to_authoritative_lane_without_in_place_mutation() {
    let mut runtime = ActiveSubscriptionRuntime::new();
    let (_, attachment) =
        attached_consumer(&mut runtime, LiveQueryFamily::Detail, None, "preview-a");
    let authoritative_handle = active_lane_for(
        &mut runtime,
        LiveQueryFamily::OrderedCollection,
        Some(LiveViewShapeFamily::Table),
    );
    let isolation = admit_preview_subscription_isolation(
        &attachment,
        "preview-epoch-a",
        PreviewResidueWidth::measured(1),
    )
    .unwrap();
    let isolation_digest = isolation.isolation_digest().to_string();
    let residue_report = measure_preview_subscription_residue(
        PreviewResidueWidth::measured(0),
        PreviewResidueWidth::measured(0),
        PreviewResidueWidth::measured(0),
        PreviewResidueWidth::measured(0),
        PreviewResidueWidth::measured(0),
        PreviewResidueWidth::measured(1),
        PreviewResidueWidth::measured(0),
    );

    let handoff = promote_preview_subscription(
        isolation,
        &residue_report,
        &authoritative_handle,
        "promotion-authority",
    )
    .unwrap();

    assert_eq!(
        handoff.authoritative_active_lane_digest(),
        authoritative_handle.lane_digest()
    );
    assert_ne!(
        handoff.preview_lane_digest(),
        handoff.authoritative_active_lane_digest()
    );
    assert_ne!(handoff.handoff_digest(), isolation_digest);
    assert_eq!(
        handoff.residue_report_digest(),
        residue_report.report_digest()
    );
    assert_eq!(handoff.counters().preview_promotion_handoff_count(), 1);
    assert_eq!(handoff.performance_receipt().consumed_width(), 1);
    assert_eq!(handoff.performance_receipt().remaining_width(), 0);
}

#[test]
fn preview_residue_classes_make_authoritative_residue_explicit() {
    let report = measure_preview_subscription_residue(
        PreviewResidueWidth::measured(1),
        PreviewResidueWidth::measured(2),
        PreviewResidueWidth::measured(0),
        PreviewResidueWidth::measured(1),
        PreviewResidueWidth::measured(0),
        PreviewResidueWidth::measured(3),
        PreviewResidueWidth::measured(4),
    );

    assert!(PreviewSubscriptionResidueClass::AuthoritativeRouting.is_authoritative());
    assert!(!PreviewSubscriptionResidueClass::TemporaryPreviewExecution.is_authoritative());
    assert_eq!(
        report.class_width(PreviewSubscriptionResidueClass::AuthoritativeCheckpoint),
        2
    );
    assert_eq!(report.authoritative_residue_width(), 4);
    assert_eq!(report.temporary_residue_width(), 7);
    assert_eq!(report.preview_residue_width(), 11);
}

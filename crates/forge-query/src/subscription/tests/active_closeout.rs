use super::*;
use crate::live::LiveQueryFamily;
use crate::view_shape_live::LiveViewShapeFamily;

fn active_budget() -> ActiveSubscriptionWorkBudget {
    ActiveSubscriptionWorkBudget::admitted(
        ActiveRegistryLookupWidth::measured(2),
        ActiveFanoutWidth::measured(2),
        ActiveAllocationScopeWidth::measured(1),
        ActiveSubscriptionAllocationPosture::LifecycleArena,
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

fn attached_consumer(
    runtime: &mut ActiveSubscriptionRuntime,
    consumer: &str,
) -> (ActiveSubscriptionLaneHandle, SubscriptionConsumerAttachment) {
    let activation = activation_for(
        LiveQueryFamily::OrderedCollection,
        Some(LiveViewShapeFamily::Table),
    );
    let admission = admit_active_subscription_lane(activation, active_budget()).unwrap();
    let handle = open_active_subscription_lane(runtime, admission).unwrap();
    let attachment = attach_subscription_consumer(
        runtime,
        &handle,
        SubscriptionConsumerAttachmentRequest::admitted(consumer, "cursor"),
        attachment_budget(),
    )
    .unwrap();
    (handle, attachment)
}

#[test]
fn closing_one_of_multiple_consumers_keeps_shared_lane_alive() {
    let activation = activation_for(
        LiveQueryFamily::OrderedCollection,
        Some(LiveViewShapeFamily::Table),
    );
    let first = admit_active_subscription_lane(activation.clone(), active_budget()).unwrap();
    let second = admit_active_subscription_lane(activation, active_budget()).unwrap();
    let mut runtime = ActiveSubscriptionRuntime::new();
    let first_handle = open_active_subscription_lane(&mut runtime, first).unwrap();
    let second_handle = open_active_subscription_lane(&mut runtime, second).unwrap();
    let first_attachment = attach_subscription_consumer(
        &mut runtime,
        &first_handle,
        SubscriptionConsumerAttachmentRequest::admitted("consumer-a", "cursor-a"),
        attachment_budget(),
    )
    .unwrap();
    let second_attachment = attach_subscription_consumer(
        &mut runtime,
        &second_handle,
        SubscriptionConsumerAttachmentRequest::admitted("consumer-b", "cursor-b"),
        attachment_budget(),
    )
    .unwrap();

    let closeout = close_subscription_lifecycle(
        &mut runtime,
        &first_handle,
        SubscriptionLifecycleCloseRequest::DetachConsumer(first_attachment),
    )
    .unwrap();

    assert_eq!(runtime.lane_count(), 1);
    assert!(!closeout.lane_terminal());
    assert_eq!(
        closeout.closeout_kind(),
        &SubscriptionLifecycleCloseoutKind::ConsumerDetached
    );
    assert_eq!(closeout.counters().consumer_attachment_close_count(), 1);
    assert_eq!(closeout.counters().active_lane_close_count(), 0);

    let final_closeout = close_subscription_lifecycle(
        &mut runtime,
        &second_handle,
        SubscriptionLifecycleCloseRequest::TerminateConsumer(second_attachment),
    )
    .unwrap();
    assert!(final_closeout.lane_terminal());
    assert_eq!(runtime.lane_count(), 0);
    assert_eq!(final_closeout.counters().active_lane_close_count(), 1);
}

#[test]
fn final_consumer_closeout_carries_support_and_performance_receipt() {
    let mut runtime = ActiveSubscriptionRuntime::new();
    let (handle, attachment) = attached_consumer(&mut runtime, "consumer-a");

    let closeout = close_subscription_lifecycle(
        &mut runtime,
        &handle,
        SubscriptionLifecycleCloseRequest::TerminateConsumer(attachment),
    )
    .unwrap();

    assert!(closeout.lane_terminal());
    assert_eq!(
        closeout.support_profile().runtime_backed_support(),
        &QuerySubscriptionRuntimeBackedSupport::Admitted
    );
    assert_eq!(
        closeout.support_profile().active_lifecycle_support(),
        &QuerySubscriptionActiveLifecycleSupport::Admitted
    );
    assert_eq!(
        closeout.support_profile().lifecycle_closeout_support(),
        &QuerySubscriptionLifecycleCloseoutSupport::Admitted
    );
    assert_eq!(
        closeout.support_profile().durable_support(),
        &QuerySubscriptionDurableSupport::ExplicitDebt
    );
    assert_eq!(closeout.performance_receipt().consumed_width(), 2);
    assert_eq!(closeout.performance_receipt().remaining_width(), 0);
    assert!(!closeout.closeout_digest().is_empty());
    assert_eq!(runtime.lane_count(), 0);
}

#[test]
fn closing_with_foreign_runtime_handle_denies_typed_and_early() {
    let mut owner_runtime = ActiveSubscriptionRuntime::new();
    let (handle, attachment) = attached_consumer(&mut owner_runtime, "consumer-a");
    let mut foreign_runtime = ActiveSubscriptionRuntime::new();

    let error = close_subscription_lifecycle(
        &mut foreign_runtime,
        &handle,
        SubscriptionLifecycleCloseRequest::TerminateConsumer(attachment),
    )
    .unwrap_err();

    assert_eq!(
        error.denial_kind(),
        &SubscriptionLifecycleCloseDenialKind::LaneHandleMismatch
    );
    assert_eq!(
        error
            .counters()
            .subscription_lifecycle_closeout_denial_count(),
        1
    );
}

#[test]
fn preview_discard_closeout_can_close_the_runtime_lane() {
    let mut runtime = ActiveSubscriptionRuntime::new();
    let (handle, attachment) = attached_consumer(&mut runtime, "preview-a");
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
        PreviewResidueWidth::measured(0),
    );
    let preview_closeout = discard_preview_subscription(isolation, residue).unwrap();

    let closeout = close_subscription_lifecycle(
        &mut runtime,
        &handle,
        SubscriptionLifecycleCloseRequest::PreviewDiscard(preview_closeout),
    )
    .unwrap();

    assert!(closeout.lane_terminal());
    assert_eq!(
        closeout.closeout_kind(),
        &SubscriptionLifecycleCloseoutKind::PreviewDiscarded
    );
    assert_eq!(closeout.counters().active_lane_close_count(), 1);
    assert_eq!(closeout.counters().consumer_attachment_close_count(), 1);
    assert_eq!(runtime.lane_count(), 0);
}

use super::*;
use crate::live::LiveQueryFamily;
use crate::view_shape_live::LiveViewShapeFamily;

fn active_budget() -> ActiveSubscriptionWorkBudget {
    ActiveSubscriptionWorkBudget::admitted(
        ActiveRegistryLookupWidth::measured(1),
        ActiveFanoutWidth::measured(1),
        ActiveAllocationScopeWidth::measured(1),
        ActiveSubscriptionAllocationPolicy::LifecycleArena,
    )
}

fn attachment_budget() -> SubscriptionConsumerAttachmentBudget {
    SubscriptionConsumerAttachmentBudget::admitted(
        ActiveFanoutWidth::measured(1),
        ConsumerDeliveryPacingWidth::measured(1),
        ActiveAllocationScopeWidth::measured(1),
        DeliveryBackpressurePolicy::RetainWithinWindow,
    )
}

fn delivery_budget() -> QueryDeliveryWindowBudget {
    QueryDeliveryWindowBudget::admitted(
        DeliveryWindowWidth::measured(2),
        PatchGroupWidth::measured(2),
        MaintenanceDeltaWidth::measured(1),
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
) -> SubscriptionConsumerAttachment {
    let activation = activation_for(live_family, view_family);
    let admission = admit_active_subscription_lane(activation, active_budget()).unwrap();
    let handle = open_active_subscription_lane(runtime, admission).unwrap();
    attach_subscription_consumer(
        runtime,
        &handle,
        SubscriptionConsumerAttachmentRequest::admitted("consumer-a", "cursor-a"),
        attachment_budget(),
    )
    .unwrap()
}

fn emit_for_delta(
    runtime: &mut ActiveSubscriptionRuntime,
    attachment: &SubscriptionConsumerAttachment,
    kind: QuerySubscriptionMaintenanceDeltaKind,
    scope: &str,
    density: ActiveDeliveryDensityPosture,
) -> QueryDeliveryBatch {
    let window = open_query_delivery_window(runtime, attachment, delivery_budget()).unwrap();
    let delta = QuerySubscriptionMaintenanceDelta::admitted(
        kind,
        attachment.lane_digest().clone(),
        scope,
        MaintenanceDeltaWidth::measured(1),
    );
    let (delta, lowering_report, lowering_counters) =
        lower_query_subscription_maintenance_delta(delta).unwrap();
    assert_eq!(lowering_counters.maintenance_delta_lowering_count(), 1);
    let packet = build_active_delivery_work_packet(
        runtime,
        attachment,
        delta,
        lowering_report,
        density,
        ActiveDeliveryAffectedLaneWidth::measured(1),
        ActiveDeliveryAffectedAttachmentWidth::measured(1),
        PatchGroupWidth::measured(1),
        ActiveDeliveryContinuationWidth::measured(0),
        ActiveDeliveryPreviewResidueWidth::measured(0),
        ActiveAllocationScopeWidth::measured(1),
        ActiveSubscriptionAllocationPosture::PatchScratch,
    )
    .unwrap();
    assert_eq!(runtime.counters().active_delivery_work_packet_count(), 1);
    emit_query_delivery_batch(runtime, window, packet).unwrap()
}

#[test]
fn detail_delivery_emits_query_shaped_patch_batch_and_receipt() {
    let mut runtime = ActiveSubscriptionRuntime::new();
    let attachment = attached_consumer(&mut runtime, LiveQueryFamily::Detail, None);

    let batch = emit_for_delta(
        &mut runtime,
        &attachment,
        QuerySubscriptionMaintenanceDeltaKind::DetailFieldDelta,
        "employee.name",
        ActiveDeliveryDensityPosture::SparseDelta,
    );
    let receipt = batch.receipt().clone();
    let attachment =
        advance_subscription_acknowledgement(&mut runtime, attachment, receipt).unwrap();

    assert_eq!(
        batch.patch_group().kind(),
        QueryPatchGroupKind::DetailFieldPatchGroup
    );
    assert_eq!(batch.counters().delivery_batch_count(), 1);
    assert_eq!(batch.counters().detail_field_patch_width(), 1);
    assert_eq!(
        attachment
            .acknowledgement_frontier()
            .acknowledged_sequence()
            .get(),
        batch.sequence().get()
    );
}

#[test]
fn collection_and_grouped_deliveries_have_distinct_patch_digests() {
    let mut runtime = ActiveSubscriptionRuntime::new();
    let collection = attached_consumer(
        &mut runtime,
        LiveQueryFamily::OrderedCollection,
        Some(LiveViewShapeFamily::Table),
    );
    let grouped = attached_consumer(
        &mut runtime,
        LiveQueryFamily::OrderedCollection,
        Some(LiveViewShapeFamily::KanbanGrouped),
    );

    let collection_batch = emit_for_delta(
        &mut runtime,
        &collection,
        QuerySubscriptionMaintenanceDeltaKind::CollectionMembershipDelta,
        "employee.active",
        ActiveDeliveryDensityPosture::SparseDelta,
    );
    let grouped_batch = emit_for_delta(
        &mut runtime,
        &grouped,
        QuerySubscriptionMaintenanceDeltaKind::GroupedMembershipDelta,
        "employee.department",
        ActiveDeliveryDensityPosture::BurstCoalesced,
    );

    assert_eq!(
        collection_batch.patch_group().kind(),
        QueryPatchGroupKind::CollectionMembershipPatchGroup
    );
    assert_eq!(
        grouped_batch.patch_group().kind(),
        QueryPatchGroupKind::GroupedMembershipPatchGroup
    );
    assert_ne!(
        collection_batch.patch_group().patch_group_digest(),
        grouped_batch.patch_group().patch_group_digest()
    );
    assert_eq!(grouped_batch.counters().grouped_membership_patch_width(), 1);
}

#[test]
fn collection_order_delivery_has_its_own_counter_semantics() {
    let mut runtime = ActiveSubscriptionRuntime::new();
    let attachment = attached_consumer(
        &mut runtime,
        LiveQueryFamily::OrderedCollection,
        Some(LiveViewShapeFamily::Table),
    );

    let batch = emit_for_delta(
        &mut runtime,
        &attachment,
        QuerySubscriptionMaintenanceDeltaKind::CollectionOrderDelta,
        "employee.sort_key",
        ActiveDeliveryDensityPosture::SparseDelta,
    );

    assert_eq!(
        batch.patch_group().kind(),
        QueryPatchGroupKind::CollectionOrderPatchGroup
    );
    assert_eq!(batch.counters().collection_order_patch_width(), 1);
    assert_eq!(batch.counters().collection_membership_patch_width(), 0);
}

#[test]
fn raw_cdc_and_raw_bridge_invalidation_deny_before_delivery_batch_exists() {
    let cdc = deny_raw_cdc_delivery_fallback("raw-cdc").unwrap_err();
    let bridge = deny_raw_bridge_invalidation_delivery("raw-bridge").unwrap_err();

    assert_eq!(
        cdc.denial_kind(),
        &QueryDeliveryDenialKind::RawCdcFallbackDenied
    );
    assert_eq!(cdc.counters().raw_cdc_delivery_denial_count(), 1);
    assert_eq!(
        bridge.denial_kind(),
        &QueryDeliveryDenialKind::RawBridgeInvalidationDenied
    );
    assert_eq!(bridge.counters().raw_bridge_invalidation_denial_count(), 1);
}

#[test]
fn delivery_batch_denies_work_packet_that_exceeds_opened_window_budget() {
    let mut runtime = ActiveSubscriptionRuntime::new();
    let attachment = attached_consumer(&mut runtime, LiveQueryFamily::Detail, None);
    let window = open_query_delivery_window(
        &mut runtime,
        &attachment,
        QueryDeliveryWindowBudget::admitted(
            DeliveryWindowWidth::measured(2),
            PatchGroupWidth::measured(1),
            MaintenanceDeltaWidth::measured(1),
            ActiveAllocationScopeWidth::measured(1),
            ActiveSubscriptionAllocationPosture::DeliveryWindowArena,
            DeliveryBackpressurePolicy::RetainWithinWindow,
        ),
    )
    .unwrap();
    let delta = QuerySubscriptionMaintenanceDelta::admitted(
        QuerySubscriptionMaintenanceDeltaKind::DetailFieldDelta,
        attachment.lane_digest().clone(),
        "employee.name",
        MaintenanceDeltaWidth::measured(1),
    );
    let (delta, lowering_report, _) = lower_query_subscription_maintenance_delta(delta).unwrap();
    let packet = build_active_delivery_work_packet(
        &mut runtime,
        &attachment,
        delta,
        lowering_report,
        ActiveDeliveryDensityPosture::SparseDelta,
        ActiveDeliveryAffectedLaneWidth::measured(1),
        ActiveDeliveryAffectedAttachmentWidth::measured(1),
        PatchGroupWidth::measured(2),
        ActiveDeliveryContinuationWidth::measured(0),
        ActiveDeliveryPreviewResidueWidth::measured(0),
        ActiveAllocationScopeWidth::measured(1),
        ActiveSubscriptionAllocationPosture::PatchScratch,
    )
    .unwrap();

    assert_eq!(packet.performance_receipt().consumed_width(), 5);
    assert_eq!(packet.performance_receipt().budgeted_width(), 5);
    assert_eq!(packet.performance_receipt().remaining_width(), 0);

    let error = emit_query_delivery_batch(&mut runtime, window, packet).unwrap_err();

    assert_eq!(
        error.denial_kind(),
        &QueryDeliveryDenialKind::DeliveryWindowBudgetExceeded
    );
    assert_eq!(error.counters().delivery_window_overflow_count(), 1);
}

#[test]
fn work_packet_denies_delta_from_another_active_lane() {
    let mut runtime = ActiveSubscriptionRuntime::new();
    let detail_attachment = attached_consumer(&mut runtime, LiveQueryFamily::Detail, None);
    let collection_attachment = attached_consumer(
        &mut runtime,
        LiveQueryFamily::OrderedCollection,
        Some(LiveViewShapeFamily::Table),
    );
    let foreign_delta = QuerySubscriptionMaintenanceDelta::admitted(
        QuerySubscriptionMaintenanceDeltaKind::CollectionMembershipDelta,
        collection_attachment.lane_digest().clone(),
        "employee.active",
        MaintenanceDeltaWidth::measured(1),
    );
    let (foreign_delta, foreign_lowering_report, _) =
        lower_query_subscription_maintenance_delta(foreign_delta).unwrap();

    let error = build_active_delivery_work_packet(
        &mut runtime,
        &detail_attachment,
        foreign_delta,
        foreign_lowering_report,
        ActiveDeliveryDensityPosture::SparseDelta,
        ActiveDeliveryAffectedLaneWidth::measured(1),
        ActiveDeliveryAffectedAttachmentWidth::measured(1),
        PatchGroupWidth::measured(1),
        ActiveDeliveryContinuationWidth::measured(0),
        ActiveDeliveryPreviewResidueWidth::measured(0),
        ActiveAllocationScopeWidth::measured(1),
        ActiveSubscriptionAllocationPosture::PatchScratch,
    )
    .unwrap_err();

    assert_eq!(
        error.denial_kind(),
        &QueryDeliveryDenialKind::WorkPacketDeltaMismatch
    );
    assert_eq!(error.counters().delivery_window_overflow_count(), 1);
}

#[test]
fn dense_refresh_denial_blocks_work_packet_before_batch_exists() {
    let mut runtime = ActiveSubscriptionRuntime::new();
    let attachment = attached_consumer(&mut runtime, LiveQueryFamily::Detail, None);
    let delta = QuerySubscriptionMaintenanceDelta::admitted(
        QuerySubscriptionMaintenanceDeltaKind::DetailFieldDelta,
        attachment.lane_digest().clone(),
        "employee.name",
        MaintenanceDeltaWidth::measured(1),
    );
    let (delta, lowering_report, _) = lower_query_subscription_maintenance_delta(delta).unwrap();

    let error = build_active_delivery_work_packet(
        &mut runtime,
        &attachment,
        delta,
        lowering_report,
        ActiveDeliveryDensityPosture::DenseRefreshDenied,
        ActiveDeliveryAffectedLaneWidth::measured(1),
        ActiveDeliveryAffectedAttachmentWidth::measured(1),
        PatchGroupWidth::measured(1),
        ActiveDeliveryContinuationWidth::measured(0),
        ActiveDeliveryPreviewResidueWidth::measured(0),
        ActiveAllocationScopeWidth::measured(1),
        ActiveSubscriptionAllocationPosture::PatchScratch,
    )
    .unwrap_err();

    assert_eq!(
        error.denial_kind(),
        &QueryDeliveryDenialKind::DenseRefreshDenied
    );
    assert_eq!(
        error
            .counters()
            .active_delivery_density_dense_denial_count(),
        1
    );
}

#[test]
fn delivery_window_rejects_wrong_phase_or_denied_allocation_posture() {
    let mut runtime = ActiveSubscriptionRuntime::new();
    let attachment = attached_consumer(&mut runtime, LiveQueryFamily::Detail, None);

    let wrong_phase = open_query_delivery_window(
        &mut runtime,
        &attachment,
        QueryDeliveryWindowBudget::admitted(
            DeliveryWindowWidth::measured(2),
            PatchGroupWidth::measured(1),
            MaintenanceDeltaWidth::measured(1),
            ActiveAllocationScopeWidth::measured(1),
            ActiveSubscriptionAllocationPosture::LifecycleArena,
            DeliveryBackpressurePolicy::RetainWithinWindow,
        ),
    )
    .unwrap_err();
    assert_eq!(
        wrong_phase.denial_kind(),
        &QueryDeliveryDenialKind::AllocationPostureForbidden
    );
    assert_eq!(wrong_phase.counters().heap_allocation_denial_count(), 1);

    let denied = open_query_delivery_window(
        &mut runtime,
        &attachment,
        QueryDeliveryWindowBudget::admitted(
            DeliveryWindowWidth::measured(2),
            PatchGroupWidth::measured(1),
            MaintenanceDeltaWidth::measured(1),
            ActiveAllocationScopeWidth::measured(1),
            ActiveSubscriptionAllocationPosture::HeapAllocationDenied,
            DeliveryBackpressurePolicy::RetainWithinWindow,
        ),
    )
    .unwrap_err();
    assert_eq!(
        denied.denial_kind(),
        &QueryDeliveryDenialKind::AllocationPostureForbidden
    );
    assert_eq!(denied.counters().heap_allocation_denial_count(), 1);
}

#[test]
fn delivery_window_heap_allocation_debt_is_explicit() {
    let mut runtime = ActiveSubscriptionRuntime::new();
    let attachment = attached_consumer(&mut runtime, LiveQueryFamily::Detail, None);

    let window = open_query_delivery_window(
        &mut runtime,
        &attachment,
        QueryDeliveryWindowBudget::admitted(
            DeliveryWindowWidth::measured(2),
            PatchGroupWidth::measured(1),
            MaintenanceDeltaWidth::measured(1),
            ActiveAllocationScopeWidth::measured(1),
            ActiveSubscriptionAllocationPosture::HeapAllocationDebtExplicit,
            DeliveryBackpressurePolicy::RetainWithinWindow,
        ),
    )
    .unwrap();

    assert_eq!(
        window.allocation_posture(),
        ActiveSubscriptionAllocationPosture::HeapAllocationDebtExplicit
    );
    assert_eq!(runtime.counters().heap_allocation_debt_count(), 1);
}

#[test]
fn work_packet_rejects_wrong_phase_allocation_posture() {
    let mut runtime = ActiveSubscriptionRuntime::new();
    let attachment = attached_consumer(&mut runtime, LiveQueryFamily::Detail, None);
    let delta = QuerySubscriptionMaintenanceDelta::admitted(
        QuerySubscriptionMaintenanceDeltaKind::DetailFieldDelta,
        attachment.lane_digest().clone(),
        "employee.name",
        MaintenanceDeltaWidth::measured(1),
    );
    let (delta, lowering_report, _) = lower_query_subscription_maintenance_delta(delta).unwrap();

    let error = build_active_delivery_work_packet(
        &mut runtime,
        &attachment,
        delta,
        lowering_report,
        ActiveDeliveryDensityPosture::SparseDelta,
        ActiveDeliveryAffectedLaneWidth::measured(1),
        ActiveDeliveryAffectedAttachmentWidth::measured(1),
        PatchGroupWidth::measured(1),
        ActiveDeliveryContinuationWidth::measured(0),
        ActiveDeliveryPreviewResidueWidth::measured(0),
        ActiveAllocationScopeWidth::measured(1),
        ActiveSubscriptionAllocationPosture::DeliveryWindowArena,
    )
    .unwrap_err();

    assert_eq!(
        error.denial_kind(),
        &QueryDeliveryDenialKind::AllocationPostureForbidden
    );
    assert_eq!(error.counters().heap_allocation_denial_count(), 1);
}

#[test]
fn performance_receipt_digest_binds_allocation_posture() {
    let mut runtime = ActiveSubscriptionRuntime::new();
    let attachment = attached_consumer(&mut runtime, LiveQueryFamily::Detail, None);

    let patch_delta = QuerySubscriptionMaintenanceDelta::admitted(
        QuerySubscriptionMaintenanceDeltaKind::DetailFieldDelta,
        attachment.lane_digest().clone(),
        "employee.name",
        MaintenanceDeltaWidth::measured(1),
    );
    let (patch_delta, patch_lowering, _) =
        lower_query_subscription_maintenance_delta(patch_delta).unwrap();
    let patch_packet = build_active_delivery_work_packet(
        &mut runtime,
        &attachment,
        patch_delta,
        patch_lowering,
        ActiveDeliveryDensityPosture::SparseDelta,
        ActiveDeliveryAffectedLaneWidth::measured(1),
        ActiveDeliveryAffectedAttachmentWidth::measured(1),
        PatchGroupWidth::measured(1),
        ActiveDeliveryContinuationWidth::measured(0),
        ActiveDeliveryPreviewResidueWidth::measured(0),
        ActiveAllocationScopeWidth::measured(1),
        ActiveSubscriptionAllocationPosture::PatchScratch,
    )
    .unwrap();

    let debt_delta = QuerySubscriptionMaintenanceDelta::admitted(
        QuerySubscriptionMaintenanceDeltaKind::DetailFieldDelta,
        attachment.lane_digest().clone(),
        "employee.name",
        MaintenanceDeltaWidth::measured(1),
    );
    let (debt_delta, debt_lowering, _) =
        lower_query_subscription_maintenance_delta(debt_delta).unwrap();
    let debt_packet = build_active_delivery_work_packet(
        &mut runtime,
        &attachment,
        debt_delta,
        debt_lowering,
        ActiveDeliveryDensityPosture::SparseDelta,
        ActiveDeliveryAffectedLaneWidth::measured(1),
        ActiveDeliveryAffectedAttachmentWidth::measured(1),
        PatchGroupWidth::measured(1),
        ActiveDeliveryContinuationWidth::measured(0),
        ActiveDeliveryPreviewResidueWidth::measured(0),
        ActiveAllocationScopeWidth::measured(1),
        ActiveSubscriptionAllocationPosture::HeapAllocationDebtExplicit,
    )
    .unwrap();

    assert_eq!(
        debt_packet.allocation_posture(),
        ActiveSubscriptionAllocationPosture::HeapAllocationDebtExplicit
    );
    assert_eq!(runtime.counters().heap_allocation_debt_count(), 1);
    assert_ne!(
        patch_packet
            .performance_receipt()
            .performance_receipt_digest(),
        debt_packet
            .performance_receipt()
            .performance_receipt_digest()
    );
    assert_ne!(
        patch_packet.work_packet_digest(),
        debt_packet.work_packet_digest()
    );
}

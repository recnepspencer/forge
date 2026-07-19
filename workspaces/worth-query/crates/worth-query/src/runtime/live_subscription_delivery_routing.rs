use std::collections::BTreeMap;

use crate::declarative_live::{DeclarativeLiveQueryRequest, DeclarativeLiveViewShape};
use crate::memory_workspace::{WorthQueryMutationKind, WorthQueryMutationReceipt};
use crate::subscription::{
    advance_subscription_acknowledgement, build_active_delivery_work_packet,
    emit_query_delivery_batch, lower_query_subscription_maintenance_delta,
    open_query_delivery_window, ActiveAllocationScopeWidth, ActiveDeliveryAffectedAttachmentWidth,
    ActiveDeliveryAffectedLaneWidth, ActiveDeliveryContinuationWidth, ActiveDeliveryDensityPosture,
    ActiveDeliveryPreviewResidueWidth, ActiveSubscriptionAllocationPosture,
    ActiveSubscriptionRuntime, DeliveryBackpressurePolicy, DeliveryWindowWidth,
    MaintenanceDeltaWidth, PatchGroupWidth, QueryDeliveryWindowBudget,
    QuerySubscriptionMaintenanceDelta, QuerySubscriptionMaintenanceDeltaKind,
};
use worth_foundational::facade::CanonicalFieldPath;

use super::delivery::{
    WorthQueryLiveSubscriptionIndexEntry, WorthQueryRuntimeDeliveryBatch,
    WorthQueryRuntimeLiveSubscriptionState, WorthQueryRuntimeRetainedDelivery,
};
use super::{
    WorthQueryAspectTouch, WorthQueryLiveArtifactTarget, WorthQueryLiveGraphReadAccessPlan,
    WorthQueryLiveGraphReadMaintenanceBudget, WorthQueryLiveGraphReadMaintenanceReceipt,
    WorthQueryRuntimeError,
};

pub(super) fn route_live_subscription_delivery(
    active_subscriptions: &mut ActiveSubscriptionRuntime,
    live_subscriptions: &mut BTreeMap<
        WorthQueryLiveArtifactTarget,
        WorthQueryRuntimeLiveSubscriptionState,
    >,
    live_subscription_index: &[WorthQueryLiveSubscriptionIndexEntry],
    receipt: &WorthQueryMutationReceipt,
) -> Result<Vec<WorthQueryLiveArtifactTarget>, WorthQueryRuntimeError> {
    let mut affected = Vec::new();
    for delta in &receipt.deltas {
        let Some(candidate_entry) = live_subscription_index.iter().find(|entry| {
            delta
                .target_collection_identity()
                .same_target_collection_as(entry.target_collection())
        }) else {
            continue;
        };
        for target in candidate_entry.targets() {
            let Some(state) = live_subscriptions.get_mut(target) else {
                continue;
            };
            let Some(delta_kind) = maintenance_delta_kind_for_live_change(
                &state.request,
                &delta.kind,
                delta.admitted_touched_aspects(),
            ) else {
                continue;
            };
            route_relevant_live_subscription_delta(
                active_subscriptions,
                state,
                target,
                receipt,
                delta,
                delta_kind,
                &mut affected,
            )?;
        }
    }
    affected.sort();
    affected.dedup();
    Ok(affected)
}

fn route_relevant_live_subscription_delta(
    active_subscriptions: &mut ActiveSubscriptionRuntime,
    state: &mut WorthQueryRuntimeLiveSubscriptionState,
    target: &WorthQueryLiveArtifactTarget,
    receipt: &WorthQueryMutationReceipt,
    delta: &crate::memory_workspace::WorthQueryMutationDelta,
    delta_kind: QuerySubscriptionMaintenanceDeltaKind,
    affected: &mut Vec<WorthQueryLiveArtifactTarget>,
) -> Result<(), WorthQueryRuntimeError> {
    let view_name = target.view_name();
    let patch_width = delta.admitted_touched_aspects().len().max(1) as u64;
    let maintenance_delta =
        admitted_subscription_maintenance_delta(receipt, delta, delta_kind, state, patch_width);
    let live_graph_read_maintenance =
        live_graph_read_maintenance_receipt(view_name, state, &maintenance_delta, patch_width)?;
    let (maintenance_delta, lowering_report, _) =
        lower_query_subscription_maintenance_delta(maintenance_delta).map_err(|error| {
            WorthQueryRuntimeError::LiveSubscriptionInstallation {
                view_name: view_name.to_string(),
                stage: "delivery-delta-lowering",
                message: error.message().to_string(),
            }
        })?;
    let window = open_query_delivery_window(
        active_subscriptions,
        &state.consumer_attachment,
        runtime_delivery_window_budget(patch_width),
    )
    .map_err(
        |error| WorthQueryRuntimeError::LiveSubscriptionInstallation {
            view_name: view_name.to_string(),
            stage: "delivery-window",
            message: error.message().to_string(),
        },
    )?;
    let work_packet = build_active_delivery_work_packet(
        active_subscriptions,
        &state.consumer_attachment,
        maintenance_delta,
        lowering_report,
        ActiveDeliveryDensityPosture::SparseDelta,
        ActiveDeliveryAffectedLaneWidth::measured(1),
        ActiveDeliveryAffectedAttachmentWidth::measured(1),
        PatchGroupWidth::measured(patch_width),
        ActiveDeliveryContinuationWidth::measured(0),
        ActiveDeliveryPreviewResidueWidth::measured(0),
        ActiveAllocationScopeWidth::measured(1),
        ActiveSubscriptionAllocationPosture::PatchScratch,
    )
    .map_err(
        |error| WorthQueryRuntimeError::LiveSubscriptionInstallation {
            view_name: view_name.to_string(),
            stage: "delivery-work-packet",
            message: error.message().to_string(),
        },
    )?;
    let batch =
        emit_query_delivery_batch(active_subscriptions, window, work_packet).map_err(|error| {
            WorthQueryRuntimeError::LiveSubscriptionInstallation {
                view_name: view_name.to_string(),
                stage: "delivery-batch",
                message: error.message().to_string(),
            }
        })?;
    let delivery_receipt = batch.receipt().clone();
    let runtime_batch = WorthQueryRuntimeDeliveryBatch::from_query_delivery(
        view_name,
        &batch,
        Some(live_graph_read_maintenance),
    );
    state.last_delivery = Some(WorthQueryRuntimeRetainedDelivery::from_batch(
        &runtime_batch,
    ));
    state.delivery_batches.push(runtime_batch);
    state.consumer_attachment = advance_subscription_acknowledgement(
        active_subscriptions,
        state.consumer_attachment.clone(),
        delivery_receipt,
    )
    .map_err(
        |error| WorthQueryRuntimeError::LiveSubscriptionInstallation {
            view_name: view_name.to_string(),
            stage: "delivery-acknowledgement",
            message: error.message().to_string(),
        },
    )?;
    affected.push(target.clone());
    Ok(())
}

fn admitted_subscription_maintenance_delta(
    receipt: &WorthQueryMutationReceipt,
    delta: &crate::memory_workspace::WorthQueryMutationDelta,
    delta_kind: QuerySubscriptionMaintenanceDeltaKind,
    state: &WorthQueryRuntimeLiveSubscriptionState,
    patch_width: u64,
) -> QuerySubscriptionMaintenanceDelta {
    let commit_evidence = receipt.commit_identity.evidence_identity();
    let entity_evidence = delta.entity_identity.evidence_identity();
    QuerySubscriptionMaintenanceDelta::admitted_with_typed_scope(
        delta_kind,
        state.active_lane_handle.lane_digest().clone(),
        &commit_evidence,
        delta.target_collection_identity().evidence_identity(),
        &entity_evidence,
        MaintenanceDeltaWidth::measured(patch_width),
    )
}

fn live_graph_read_maintenance_receipt(
    view_name: &str,
    state: &WorthQueryRuntimeLiveSubscriptionState,
    maintenance_delta: &QuerySubscriptionMaintenanceDelta,
    patch_width: u64,
) -> Result<WorthQueryLiveGraphReadMaintenanceReceipt, WorthQueryRuntimeError> {
    let live_graph_access_plan = WorthQueryLiveGraphReadAccessPlan::from_live_installation(
        &state.installation,
        WorthQueryLiveGraphReadMaintenanceBudget::bounded_with_snapshot_refresh(),
    )
    .map_err(
        |error| WorthQueryRuntimeError::LiveSubscriptionInstallation {
            view_name: view_name.to_string(),
            stage: "live-graph-access-maintenance-plan",
            message: error.message().to_string(),
        },
    )?;
    Ok(
        WorthQueryLiveGraphReadMaintenanceReceipt::from_maintenance_delta(
            &live_graph_access_plan,
            maintenance_delta,
            patch_width as usize,
        ),
    )
}

fn runtime_delivery_window_budget(patch_width: u64) -> QueryDeliveryWindowBudget {
    let bounded_patch_width = patch_width.max(1);
    QueryDeliveryWindowBudget::admitted(
        DeliveryWindowWidth::measured(1),
        PatchGroupWidth::measured(bounded_patch_width),
        MaintenanceDeltaWidth::measured(bounded_patch_width),
        ActiveAllocationScopeWidth::measured(1),
        ActiveSubscriptionAllocationPosture::DeliveryWindowArena,
        DeliveryBackpressurePolicy::RetainWithinWindow,
    )
}

fn maintenance_delta_kind_for_live_change(
    request: &DeclarativeLiveQueryRequest,
    mutation_kind: &WorthQueryMutationKind,
    aspect_touches: &[WorthQueryAspectTouch],
) -> Option<QuerySubscriptionMaintenanceDeltaKind> {
    if !live_change_is_relevant(request, mutation_kind, aspect_touches) {
        return None;
    }
    match request.view_shape() {
        DeclarativeLiveViewShape::InspectorObserved
        | DeclarativeLiveViewShape::InspectorFocused { .. }
        | DeclarativeLiveViewShape::IdentityAwareInspectorFocused { .. } => {
            Some(QuerySubscriptionMaintenanceDeltaKind::InspectorFocusDelta)
        }
        DeclarativeLiveViewShape::KanbanGrouped { grouping_aspect } => {
            if is_membership_change(mutation_kind)
                || aspect_touches
                    .iter()
                    .any(|touch| touch.native_aspect_key() == grouping_aspect)
            {
                Some(QuerySubscriptionMaintenanceDeltaKind::GroupedMembershipDelta)
            } else {
                Some(QuerySubscriptionMaintenanceDeltaKind::DetailFieldDelta)
            }
        }
        DeclarativeLiveViewShape::ListSplice | DeclarativeLiveViewShape::Table => {
            if is_membership_change(mutation_kind) {
                Some(QuerySubscriptionMaintenanceDeltaKind::CollectionMembershipDelta)
            } else {
                Some(QuerySubscriptionMaintenanceDeltaKind::DetailFieldDelta)
            }
        }
        DeclarativeLiveViewShape::Detail => {
            if matches!(mutation_kind, WorthQueryMutationKind::Deleted) {
                Some(QuerySubscriptionMaintenanceDeltaKind::CollectionMembershipDelta)
            } else {
                Some(QuerySubscriptionMaintenanceDeltaKind::DetailFieldDelta)
            }
        }
    }
}

fn live_change_is_relevant(
    request: &DeclarativeLiveQueryRequest,
    mutation_kind: &WorthQueryMutationKind,
    aspect_touches: &[WorthQueryAspectTouch],
) -> bool {
    if is_membership_change(mutation_kind) || aspect_touches.is_empty() {
        return true;
    }
    aspect_touches.iter().any(|changed| {
        request.projection().iter().any(|field| {
            changed.matches_or_contains(&live_request_field_touch(field.source_field_key()))
        }) || match request.view_shape() {
            DeclarativeLiveViewShape::InspectorFocused { focused_aspect }
            | DeclarativeLiveViewShape::IdentityAwareInspectorFocused { focused_aspect, .. } => {
                changed.native_aspect_key() == focused_aspect
            }
            DeclarativeLiveViewShape::KanbanGrouped { grouping_aspect } => {
                changed.native_aspect_key() == grouping_aspect
            }
            _ => false,
        }
    })
}

fn is_membership_change(mutation_kind: &WorthQueryMutationKind) -> bool {
    matches!(
        mutation_kind,
        WorthQueryMutationKind::Created | WorthQueryMutationKind::Deleted
    )
}

fn live_request_field_touch(field: &crate::authoring::AspectFieldKey) -> WorthQueryAspectTouch {
    WorthQueryAspectTouch::from_native_parts(
        field.native_aspect_key(),
        Some(CanonicalFieldPath::single(field.native_field_key())),
    )
}

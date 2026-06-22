use std::collections::{BTreeMap, BTreeSet};

use crate::declarative_live::{DeclarativeLiveQueryRequest, DeclarativeLiveViewShape};
use crate::memory_workspace::{ForgeQueryMutationKind, ForgeQueryMutationReceipt};
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

use super::delivery::{
    ForgeQueryRuntimeDeliveryBatch, ForgeQueryRuntimeLiveSubscriptionState,
    ForgeQueryRuntimeRetainedDelivery,
};
use super::{
    ForgeQueryLiveGraphReadAccessPlan, ForgeQueryLiveGraphReadMaintenanceBudget,
    ForgeQueryLiveGraphReadMaintenanceReceipt, ForgeQueryMutationTargetCollectionIdentity,
    ForgeQueryRuntimeError,
};

pub(super) fn route_live_subscription_delivery(
    active_subscriptions: &mut ActiveSubscriptionRuntime,
    live_subscriptions: &mut BTreeMap<String, ForgeQueryRuntimeLiveSubscriptionState>,
    live_subscription_index: &BTreeMap<String, BTreeSet<String>>,
    receipt: &ForgeQueryMutationReceipt,
) -> Result<Vec<String>, ForgeQueryRuntimeError> {
    let mut affected = Vec::new();
    for delta in &receipt.deltas {
        let Some(candidate_view_names) = live_subscription_index.get(&delta.collection) else {
            continue;
        };
        for view_name in candidate_view_names {
            let Some(state) = live_subscriptions.get_mut(view_name) else {
                continue;
            };
            let Some(delta_kind) = maintenance_delta_kind_for_live_change(
                &state.request,
                &delta.kind,
                &delta.aspect_paths,
            ) else {
                continue;
            };
            route_relevant_live_subscription_delta(
                active_subscriptions,
                state,
                view_name,
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
    state: &mut ForgeQueryRuntimeLiveSubscriptionState,
    view_name: &str,
    receipt: &ForgeQueryMutationReceipt,
    delta: &crate::memory_workspace::ForgeQueryMutationDelta,
    delta_kind: QuerySubscriptionMaintenanceDeltaKind,
    affected: &mut Vec<String>,
) -> Result<(), ForgeQueryRuntimeError> {
    let patch_width = delta.aspect_paths.len().max(1) as u64;
    let maintenance_delta =
        admitted_subscription_maintenance_delta(receipt, delta, delta_kind, state, patch_width);
    let live_graph_read_maintenance =
        live_graph_read_maintenance_receipt(view_name, state, &maintenance_delta, patch_width)?;
    let (maintenance_delta, lowering_report, _) =
        lower_query_subscription_maintenance_delta(maintenance_delta).map_err(|error| {
            ForgeQueryRuntimeError::LiveSubscriptionInstallation {
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
        |error| ForgeQueryRuntimeError::LiveSubscriptionInstallation {
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
        |error| ForgeQueryRuntimeError::LiveSubscriptionInstallation {
            view_name: view_name.to_string(),
            stage: "delivery-work-packet",
            message: error.message().to_string(),
        },
    )?;
    let batch =
        emit_query_delivery_batch(active_subscriptions, window, work_packet).map_err(|error| {
            ForgeQueryRuntimeError::LiveSubscriptionInstallation {
                view_name: view_name.to_string(),
                stage: "delivery-batch",
                message: error.message().to_string(),
            }
        })?;
    let delivery_receipt = batch.receipt().clone();
    let runtime_batch = ForgeQueryRuntimeDeliveryBatch::from_query_delivery(
        view_name,
        &batch,
        Some(live_graph_read_maintenance),
    );
    state.last_delivery = Some(ForgeQueryRuntimeRetainedDelivery::from_batch(
        &runtime_batch,
    ));
    state.delivery_batches.push(runtime_batch);
    state.consumer_attachment = advance_subscription_acknowledgement(
        active_subscriptions,
        state.consumer_attachment.clone(),
        delivery_receipt,
    )
    .map_err(
        |error| ForgeQueryRuntimeError::LiveSubscriptionInstallation {
            view_name: view_name.to_string(),
            stage: "delivery-acknowledgement",
            message: error.message().to_string(),
        },
    )?;
    affected.push(view_name.to_string());
    Ok(())
}

fn admitted_subscription_maintenance_delta(
    receipt: &ForgeQueryMutationReceipt,
    delta: &crate::memory_workspace::ForgeQueryMutationDelta,
    delta_kind: QuerySubscriptionMaintenanceDeltaKind,
    state: &ForgeQueryRuntimeLiveSubscriptionState,
    patch_width: u64,
) -> QuerySubscriptionMaintenanceDelta {
    let commit_evidence = receipt.commit_identity.evidence_identity();
    let entity_evidence = delta.entity_identity.evidence_identity();
    let collection_identity = ForgeQueryMutationTargetCollectionIdentity::new(
        "live-subscription-maintenance-delta",
        &delta.collection,
    );
    QuerySubscriptionMaintenanceDelta::admitted_with_typed_scope(
        delta_kind,
        state.active_lane_handle.lane_digest().clone(),
        &commit_evidence,
        collection_identity.evidence_identity(),
        &entity_evidence,
        MaintenanceDeltaWidth::measured(patch_width),
    )
}

fn live_graph_read_maintenance_receipt(
    view_name: &str,
    state: &ForgeQueryRuntimeLiveSubscriptionState,
    maintenance_delta: &QuerySubscriptionMaintenanceDelta,
    patch_width: u64,
) -> Result<ForgeQueryLiveGraphReadMaintenanceReceipt, ForgeQueryRuntimeError> {
    let live_graph_access_plan = ForgeQueryLiveGraphReadAccessPlan::from_live_installation(
        &state.installation,
        ForgeQueryLiveGraphReadMaintenanceBudget::bounded_with_snapshot_refresh(),
    )
    .map_err(
        |error| ForgeQueryRuntimeError::LiveSubscriptionInstallation {
            view_name: view_name.to_string(),
            stage: "live-graph-access-maintenance-plan",
            message: error.message().to_string(),
        },
    )?;
    Ok(
        ForgeQueryLiveGraphReadMaintenanceReceipt::from_maintenance_delta(
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
    mutation_kind: &ForgeQueryMutationKind,
    aspect_paths: &[String],
) -> Option<QuerySubscriptionMaintenanceDeltaKind> {
    if !live_change_is_relevant(request, mutation_kind, aspect_paths) {
        return None;
    }
    match request.view_shape() {
        DeclarativeLiveViewShape::InspectorObserved
        | DeclarativeLiveViewShape::InspectorFocused { .. }
        | DeclarativeLiveViewShape::IdentityAwareInspectorFocused { .. } => {
            Some(QuerySubscriptionMaintenanceDeltaKind::InspectorFocusDelta)
        }
        DeclarativeLiveViewShape::KanbanGrouped { grouping_aspect } => {
            let grouping_aspect_text = grouping_aspect.as_str();
            if is_membership_change(mutation_kind)
                || aspect_paths
                    .iter()
                    .any(|path| path.starts_with(grouping_aspect_text))
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
            if matches!(mutation_kind, ForgeQueryMutationKind::Deleted) {
                Some(QuerySubscriptionMaintenanceDeltaKind::CollectionMembershipDelta)
            } else {
                Some(QuerySubscriptionMaintenanceDeltaKind::DetailFieldDelta)
            }
        }
    }
}

fn live_change_is_relevant(
    request: &DeclarativeLiveQueryRequest,
    mutation_kind: &ForgeQueryMutationKind,
    aspect_paths: &[String],
) -> bool {
    if is_membership_change(mutation_kind) || aspect_paths.is_empty() {
        return true;
    }
    aspect_paths.iter().any(|changed| {
        request.projection().iter().any(|field| {
            changed == &format!("{}.{}", field.aspect(), field.field())
                || changed.starts_with(&format!("{}.", field.aspect()))
        }) || match request.view_shape() {
            DeclarativeLiveViewShape::InspectorFocused { focused_aspect }
            | DeclarativeLiveViewShape::IdentityAwareInspectorFocused { focused_aspect, .. } => {
                changed.starts_with(focused_aspect)
            }
            DeclarativeLiveViewShape::KanbanGrouped { grouping_aspect } => {
                changed.starts_with(grouping_aspect.as_str())
            }
            _ => false,
        }
    })
}

fn is_membership_change(mutation_kind: &ForgeQueryMutationKind) -> bool {
    matches!(
        mutation_kind,
        ForgeQueryMutationKind::Created | ForgeQueryMutationKind::Deleted
    )
}

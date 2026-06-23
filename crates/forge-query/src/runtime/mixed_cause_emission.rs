#![allow(dead_code)]

use std::collections::BTreeMap;

use forge_runtime_bridge::facade::{BridgeMixedCauseDeliveryWindowPlan, BridgeMixedCauseOrdering};

use crate::subscription::{
    advance_subscription_acknowledgement, emit_query_mixed_cause_delivery_batch,
    open_query_delivery_window, ActiveAllocationScopeWidth, ActiveSubscriptionAllocationPosture,
    ActiveSubscriptionRuntime, DeliveryBackpressurePolicy, DeliveryWindowWidth,
    MaintenanceDeltaWidth, PatchGroupWidth, QueryDeliveryWindowBudget, QueryPatchGroup,
    QueryPatchGroupKind, QuerySubscriptionDeliveryCause,
};

use super::delivery::{
    ForgeQueryRuntimeDeliveryBatch, ForgeQueryRuntimeLiveSubscriptionState,
    ForgeQueryRuntimeRetainedDelivery,
};
use super::{
    ForgeQueryLiveArtifactTarget, ForgeQueryRuntime, ForgeQueryRuntimeError,
    ForgeQueryRuntimeMixedCauseDelivery,
};

pub(crate) fn emit_mixed_cause_live_subscription_delivery(
    active_subscriptions: &mut ActiveSubscriptionRuntime,
    live_subscriptions: &mut BTreeMap<
        ForgeQueryLiveArtifactTarget,
        ForgeQueryRuntimeLiveSubscriptionState,
    >,
    view_name: &str,
    ordering: &BridgeMixedCauseOrdering,
    delivery_window: &BridgeMixedCauseDeliveryWindowPlan,
) -> Result<ForgeQueryRuntimeDeliveryBatch, ForgeQueryRuntimeError> {
    let target = ForgeQueryLiveArtifactTarget::from_view_name(view_name);
    let state = live_subscriptions
        .get_mut(&target)
        .ok_or_else(|| ForgeQueryRuntimeError::MissingLiveSubscription(view_name.to_string()))?;
    let mixed_cause_delivery =
        ForgeQueryRuntimeMixedCauseDelivery::from_bridge(ordering, delivery_window);
    let delivery_cause = QuerySubscriptionDeliveryCause::classified(
        mixed_cause_delivery.primary_delivery_cause_kind(),
        mixed_cause_delivery.ordering_identity(),
    );
    let patch_group = QueryPatchGroup::new(
        QueryPatchGroupKind::MixedCauseDeliveryGroup,
        mixed_cause_delivery.mixed_cause_identity(),
        delivery_window.ordered_causes().len().max(1) as u64,
    );
    let window = open_query_delivery_window(
        active_subscriptions,
        &state.consumer_attachment,
        runtime_mixed_cause_delivery_window_budget(delivery_window.ordered_causes().len() as u64),
    )
    .map_err(|error| mixed_cause_delivery_error(view_name, error))?;
    let batch = emit_query_mixed_cause_delivery_batch(
        active_subscriptions,
        window,
        delivery_cause,
        mixed_cause_delivery.has_relational_patch(),
        patch_group,
    )
    .map_err(|error| mixed_cause_delivery_error(view_name, error))?;
    let delivery_receipt = batch.receipt().clone();
    let mut runtime_batch =
        ForgeQueryRuntimeDeliveryBatch::from_query_delivery(view_name, &batch, None);
    runtime_batch.mixed_cause_delivery = mixed_cause_delivery;
    state.last_delivery = Some(ForgeQueryRuntimeRetainedDelivery::from_batch(
        &runtime_batch,
    ));
    state.delivery_batches.push(runtime_batch.clone());
    state.consumer_attachment = advance_subscription_acknowledgement(
        active_subscriptions,
        state.consumer_attachment.clone(),
        delivery_receipt,
    )
    .map_err(
        |error| ForgeQueryRuntimeError::LiveSubscriptionInstallation {
            view_name: view_name.to_string(),
            stage: "mixed-cause-delivery-acknowledgement",
            message: format!("{error:?}"),
        },
    )?;
    Ok(runtime_batch)
}

fn runtime_mixed_cause_delivery_window_budget(
    ordered_cause_count: u64,
) -> QueryDeliveryWindowBudget {
    let width = ordered_cause_count.max(1);
    QueryDeliveryWindowBudget::admitted(
        DeliveryWindowWidth::measured(1),
        PatchGroupWidth::measured(width),
        MaintenanceDeltaWidth::measured(width),
        ActiveAllocationScopeWidth::measured(1),
        ActiveSubscriptionAllocationPosture::DeliveryWindowArena,
        DeliveryBackpressurePolicy::RetainWithinWindow,
    )
}

fn mixed_cause_delivery_error(
    view_name: &str,
    error: crate::subscription::QueryDeliveryError,
) -> ForgeQueryRuntimeError {
    ForgeQueryRuntimeError::LiveSubscriptionInstallation {
        view_name: view_name.to_string(),
        stage: "mixed-cause-delivery",
        message: format!("{error:?}"),
    }
}

impl ForgeQueryRuntime {
    pub(crate) fn emit_mixed_cause_delivery(
        &mut self,
        view_name: &str,
        ordering: &BridgeMixedCauseOrdering,
        delivery_window: &BridgeMixedCauseDeliveryWindowPlan,
    ) -> Result<ForgeQueryRuntimeDeliveryBatch, ForgeQueryRuntimeError> {
        emit_mixed_cause_live_subscription_delivery(
            &mut self.active_subscriptions,
            &mut self.live_subscriptions,
            view_name,
            ordering,
            delivery_window,
        )
    }
}

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
    MaintenanceDeltaWidth, PatchGroupWidth, QueryDeliveryBatch, QueryDeliveryWindowBudget,
    QueryPatchGroupKind, QuerySubscriptionMaintenanceDelta, QuerySubscriptionMaintenanceDeltaKind,
    SubscriptionConsumerAttachment,
};

use super::{
    ForgeQueryAuthorityLane, ForgeQueryRuntimeError, ForgeQueryRuntimeLiveSubscriptionInstallation,
};

pub(super) struct ForgeQueryRuntimeLiveSubscriptionActivation {
    pub(super) installation: ForgeQueryRuntimeLiveSubscriptionInstallation,
    pub(super) active_lane_handle: crate::subscription::ActiveSubscriptionLaneHandle,
    pub(super) consumer_attachment: SubscriptionConsumerAttachment,
    pub(super) request: DeclarativeLiveQueryRequest,
}

pub(super) struct ForgeQueryRuntimeLiveSubscriptionState {
    pub(super) installation: ForgeQueryRuntimeLiveSubscriptionInstallation,
    pub(super) active_lane_handle: crate::subscription::ActiveSubscriptionLaneHandle,
    pub(super) consumer_attachment: SubscriptionConsumerAttachment,
    pub(super) request: DeclarativeLiveQueryRequest,
    pub(super) delivery_batches: Vec<ForgeQueryRuntimeDeliveryBatch>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryRuntimeDeliveryBatch {
    view_name: String,
    authority_lane: ForgeQueryAuthorityLane,
    delivery_batch_digest: String,
    delivery_window_digest: String,
    consumer_attachment_digest: String,
    sequence: u64,
    patch_group_kind: QueryPatchGroupKind,
    patch_group_digest: String,
    patch_group_width: u64,
}

impl ForgeQueryRuntimeDeliveryBatch {
    fn from_query_delivery(view_name: &str, batch: &QueryDeliveryBatch) -> Self {
        Self {
            view_name: view_name.to_string(),
            authority_lane: ForgeQueryAuthorityLane::AuthoritativeTruth,
            delivery_batch_digest: batch.delivery_batch_digest().to_string(),
            delivery_window_digest: batch.delivery_window_digest().to_string(),
            consumer_attachment_digest: batch.attachment_digest().as_str().to_string(),
            sequence: batch.sequence().get(),
            patch_group_kind: batch.patch_group().kind(),
            patch_group_digest: batch.patch_group().patch_group_digest().to_string(),
            patch_group_width: batch.patch_group().width(),
        }
    }

    pub fn view_name(&self) -> &str {
        &self.view_name
    }

    pub fn authority_lane(&self) -> ForgeQueryAuthorityLane {
        self.authority_lane
    }

    pub fn delivery_batch_digest(&self) -> &str {
        &self.delivery_batch_digest
    }

    pub fn delivery_window_digest(&self) -> &str {
        &self.delivery_window_digest
    }

    pub fn consumer_attachment_digest(&self) -> &str {
        &self.consumer_attachment_digest
    }

    pub fn sequence(&self) -> u64 {
        self.sequence
    }

    pub fn patch_group_kind(&self) -> QueryPatchGroupKind {
        self.patch_group_kind
    }

    pub fn patch_group_digest(&self) -> &str {
        &self.patch_group_digest
    }

    pub fn patch_group_width(&self) -> u64 {
        self.patch_group_width
    }
}

pub(super) fn register_live_subscription_index(
    index: &mut BTreeMap<String, BTreeSet<String>>,
    view_name: &str,
    request: &DeclarativeLiveQueryRequest,
) {
    unregister_live_subscription_index(index, view_name);
    index
        .entry(request.target().to_string())
        .or_default()
        .insert(view_name.to_string());
}

fn unregister_live_subscription_index(
    index: &mut BTreeMap<String, BTreeSet<String>>,
    view_name: &str,
) {
    let empty_collections = index
        .iter_mut()
        .filter_map(|(collection, view_names)| {
            view_names.remove(view_name);
            view_names.is_empty().then(|| collection.clone())
        })
        .collect::<Vec<_>>();
    for collection in empty_collections {
        index.remove(&collection);
    }
}

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
            let patch_width = delta.aspect_paths.len().max(1) as u64;
            let maintenance_delta = QuerySubscriptionMaintenanceDelta::admitted(
                delta_kind,
                state.active_lane_handle.lane_digest().clone(),
                format!(
                    "{}:{}:{}",
                    receipt.commit_identity, delta.collection, delta.entity_identity
                ),
                MaintenanceDeltaWidth::measured(patch_width),
            );
            let (maintenance_delta, lowering_report, _) =
                lower_query_subscription_maintenance_delta(maintenance_delta).map_err(|error| {
                    ForgeQueryRuntimeError::LiveSubscriptionInstallation {
                        view_name: view_name.clone(),
                        stage: "delivery-delta-lowering",
                        message: format!("{error:?}"),
                    }
                })?;
            let window = open_query_delivery_window(
                active_subscriptions,
                &state.consumer_attachment,
                runtime_delivery_window_budget(patch_width),
            )
            .map_err(
                |error| ForgeQueryRuntimeError::LiveSubscriptionInstallation {
                    view_name: view_name.clone(),
                    stage: "delivery-window",
                    message: format!("{error:?}"),
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
                    view_name: view_name.clone(),
                    stage: "delivery-work-packet",
                    message: format!("{error:?}"),
                },
            )?;
            let batch = emit_query_delivery_batch(active_subscriptions, window, work_packet)
                .map_err(
                    |error| ForgeQueryRuntimeError::LiveSubscriptionInstallation {
                        view_name: view_name.clone(),
                        stage: "delivery-batch",
                        message: format!("{error:?}"),
                    },
                )?;
            let delivery_receipt = batch.receipt().clone();
            state
                .delivery_batches
                .push(ForgeQueryRuntimeDeliveryBatch::from_query_delivery(
                    view_name, &batch,
                ));
            state.consumer_attachment = advance_subscription_acknowledgement(
                active_subscriptions,
                state.consumer_attachment.clone(),
                delivery_receipt,
            )
            .map_err(
                |error| ForgeQueryRuntimeError::LiveSubscriptionInstallation {
                    view_name: view_name.clone(),
                    stage: "delivery-acknowledgement",
                    message: format!("{error:?}"),
                },
            )?;
            affected.push(view_name.clone());
        }
    }
    affected.sort();
    affected.dedup();
    Ok(affected)
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
            if is_membership_change(mutation_kind)
                || aspect_paths
                    .iter()
                    .any(|path| path.starts_with(grouping_aspect))
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
                changed.starts_with(grouping_aspect)
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

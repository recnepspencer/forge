use std::collections::{BTreeMap, BTreeSet};

use crate::declarative_live::{DeclarativeLiveQueryRequest, DeclarativeLiveViewShape};
use crate::evidence_identity::ForgeQueryEvidenceIdentity;
use crate::memory_workspace::{ForgeQueryMutationKind, ForgeQueryMutationReceipt};
use crate::subscription::{
    advance_subscription_acknowledgement, build_active_delivery_work_packet,
    emit_query_delivery_batch, lower_query_subscription_maintenance_delta,
    open_query_delivery_window, ActiveAllocationScopeWidth, ActiveDeliveryAffectedAttachmentWidth,
    ActiveDeliveryAffectedLaneWidth, ActiveDeliveryContinuationWidth, ActiveDeliveryDensityPosture,
    ActiveDeliveryPreviewResidueWidth, ActiveSubscriptionAllocationPosture,
    ActiveSubscriptionRuntime, DeliveryBackpressurePolicy, DeliveryWindowWidth,
    MaintenanceDeltaWidth, PatchGroupWidth, QueryDeliveryBatch, QueryDeliveryWindowBudget,
    QueryPatchGroupKind, QuerySubscriptionDeliveryCauseKind, QuerySubscriptionMaintenanceDelta,
    QuerySubscriptionMaintenanceDeltaKind, SubscriptionConsumerAttachment,
};

use super::{
    ForgeQueryAuthorityLane, ForgeQueryRuntimeAsyncResultState, ForgeQueryRuntimeError,
    ForgeQueryRuntimeLiveSubscriptionInstallation, ForgeQueryRuntimeMixedCauseDelivery,
    ForgeQueryRuntimeRemaskPosture,
};

pub(super) struct ForgeQueryRuntimeLiveSubscriptionActivation {
    pub(super) installation: ForgeQueryRuntimeLiveSubscriptionInstallation,
    pub(super) active_lane_handle: crate::subscription::ActiveSubscriptionLaneHandle,
    pub(super) consumer_attachment: SubscriptionConsumerAttachment,
    pub(super) request: DeclarativeLiveQueryRequest,
    pub(super) remask_posture: Option<ForgeQueryRuntimeRemaskPosture>,
}

pub(super) struct ForgeQueryRuntimeLiveSubscriptionState {
    pub(super) installation: ForgeQueryRuntimeLiveSubscriptionInstallation,
    pub(super) active_lane_handle: crate::subscription::ActiveSubscriptionLaneHandle,
    pub(super) consumer_attachment: SubscriptionConsumerAttachment,
    pub(super) request: DeclarativeLiveQueryRequest,
    pub(super) delivery_batches: Vec<ForgeQueryRuntimeDeliveryBatch>,
    pub(super) last_delivery: Option<ForgeQueryRuntimeRetainedDelivery>,
    pub(super) async_result_state: Option<ForgeQueryRuntimeAsyncResultState>,
    pub(super) remask_posture: Option<ForgeQueryRuntimeRemaskPosture>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryRuntimeDeliveryBatch {
    pub(super) view_name: String,
    pub(super) authority_lane: ForgeQueryAuthorityLane,
    pub(super) delivery_batch_identity: ForgeQueryEvidenceIdentity,
    pub(super) delivery_window_identity: ForgeQueryEvidenceIdentity,
    pub(super) consumer_attachment_identity: ForgeQueryEvidenceIdentity,
    pub(super) sequence: u64,
    pub(super) delivery_cause_kind: QuerySubscriptionDeliveryCauseKind,
    pub(super) delivery_cause_identity: ForgeQueryEvidenceIdentity,
    pub(super) has_relational_patch: bool,
    pub(super) patch_group_kind: QueryPatchGroupKind,
    pub(super) patch_group_identity: ForgeQueryEvidenceIdentity,
    pub(super) patch_group_width: u64,
    pub(super) mixed_cause_delivery: ForgeQueryRuntimeMixedCauseDelivery,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct ForgeQueryRuntimeRetainedDelivery {
    delivery_batch_identity: ForgeQueryEvidenceIdentity,
    delivery_cause_kind: QuerySubscriptionDeliveryCauseKind,
    delivery_cause_identity: ForgeQueryEvidenceIdentity,
    has_relational_patch: bool,
    sequence: u64,
    mixed_cause_delivery: ForgeQueryRuntimeMixedCauseDelivery,
}

impl ForgeQueryRuntimeDeliveryBatch {
    pub(super) fn from_query_delivery(view_name: &str, batch: &QueryDeliveryBatch) -> Self {
        Self {
            view_name: view_name.to_string(),
            authority_lane: ForgeQueryAuthorityLane::AuthoritativeTruth,
            delivery_batch_identity: batch.evidence_identity().clone(),
            delivery_window_identity: batch.delivery_window_identity().clone(),
            consumer_attachment_identity: batch.attachment_digest().evidence_identity().clone(),
            sequence: batch.sequence().get(),
            delivery_cause_kind: batch.delivery_cause_kind(),
            delivery_cause_identity: batch.delivery_cause().delivery_cause_identity().clone(),
            has_relational_patch: batch.has_relational_patch(),
            patch_group_kind: batch.patch_group().kind(),
            patch_group_identity: batch.patch_group().patch_group_identity().clone(),
            patch_group_width: batch.patch_group().width(),
            mixed_cause_delivery: if batch.has_relational_patch() {
                ForgeQueryRuntimeMixedCauseDelivery::atomic_relational_patch(
                    batch.delivery_cause().delivery_cause_identity(),
                )
            } else {
                ForgeQueryRuntimeMixedCauseDelivery::atomic_time_only(
                    batch.delivery_cause_kind(),
                    batch.delivery_cause().delivery_cause_identity(),
                )
            },
        }
    }

    pub fn view_name(&self) -> &str {
        &self.view_name
    }

    pub fn authority_lane(&self) -> ForgeQueryAuthorityLane {
        self.authority_lane
    }

    pub fn delivery_batch_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.delivery_batch_identity
    }

    pub fn delivery_batch_for_reporting(&self) -> &str {
        self.delivery_batch_identity.as_str()
    }

    pub fn delivery_window_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.delivery_window_identity
    }

    pub fn delivery_window_for_reporting(&self) -> &str {
        self.delivery_window_identity.as_str()
    }

    pub fn consumer_attachment_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.consumer_attachment_identity
    }

    pub fn consumer_attachment_for_reporting(&self) -> &str {
        self.consumer_attachment_identity.as_str()
    }

    pub fn sequence(&self) -> u64 {
        self.sequence
    }

    pub fn delivery_cause_kind(&self) -> QuerySubscriptionDeliveryCauseKind {
        self.delivery_cause_kind
    }

    pub fn delivery_cause_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.delivery_cause_identity
    }

    pub fn delivery_cause_for_reporting(&self) -> &str {
        self.delivery_cause_identity.as_str()
    }

    pub fn has_relational_patch(&self) -> bool {
        self.has_relational_patch
    }

    pub fn patch_group_kind(&self) -> QueryPatchGroupKind {
        self.patch_group_kind
    }

    pub fn patch_group_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.patch_group_identity
    }

    pub fn patch_group_for_reporting(&self) -> &str {
        self.patch_group_identity.as_str()
    }

    pub fn patch_group_width(&self) -> u64 {
        self.patch_group_width
    }

    pub fn mixed_cause_delivery(&self) -> &ForgeQueryRuntimeMixedCauseDelivery {
        &self.mixed_cause_delivery
    }
}

impl ForgeQueryRuntimeRetainedDelivery {
    pub(super) fn from_batch(batch: &ForgeQueryRuntimeDeliveryBatch) -> Self {
        Self {
            delivery_batch_identity: batch.delivery_batch_identity().clone(),
            delivery_cause_kind: batch.delivery_cause_kind(),
            delivery_cause_identity: batch.delivery_cause_identity().clone(),
            has_relational_patch: batch.has_relational_patch(),
            sequence: batch.sequence(),
            mixed_cause_delivery: batch.mixed_cause_delivery().clone(),
        }
    }

    pub(super) fn delivery_batch_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.delivery_batch_identity
    }

    pub(super) fn delivery_batch_for_reporting(&self) -> &str {
        self.delivery_batch_identity.as_str()
    }

    pub(super) fn delivery_cause_kind(&self) -> QuerySubscriptionDeliveryCauseKind {
        self.delivery_cause_kind
    }

    pub(super) fn delivery_cause_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.delivery_cause_identity
    }

    pub(super) fn delivery_cause_for_reporting(&self) -> &str {
        self.delivery_cause_identity.as_str()
    }

    pub(super) fn has_relational_patch(&self) -> bool {
        self.has_relational_patch
    }

    pub(super) fn sequence(&self) -> u64 {
        self.sequence
    }

    pub(super) fn mixed_cause_delivery(&self) -> &ForgeQueryRuntimeMixedCauseDelivery {
        &self.mixed_cause_delivery
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
                    receipt.commit_identity.evidence_identity(),
                    delta.collection,
                    delta.entity_identity.evidence_identity()
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
            let runtime_batch =
                ForgeQueryRuntimeDeliveryBatch::from_query_delivery(view_name, &batch);
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

use std::collections::BTreeSet;

use crate::declarative_live::DeclarativeLiveQueryRequest;
use crate::evidence_identity::WorthQueryEvidenceIdentity;
use crate::subscription::{
    QueryDeliveryBatch, QueryPatchGroupKind, QuerySubscriptionDeliveryCauseKind,
    SubscriptionConsumerAttachment,
};

use super::{
    WorthQueryAuthorityLane, WorthQueryLiveArtifactTarget,
    WorthQueryLiveGraphReadMaintenanceReceipt, WorthQueryRuntimeAsyncResultState,
    WorthQueryRuntimeLiveSubscriptionInstallation, WorthQueryRuntimeMixedCauseDelivery,
    WorthQueryRuntimeRemaskPosture,
};
use crate::runtime::WorthQueryMutationTargetCollectionIdentity;

pub(super) struct WorthQueryRuntimeLiveSubscriptionActivation {
    pub(super) installation: WorthQueryRuntimeLiveSubscriptionInstallation,
    pub(super) active_lane_handle: crate::subscription::ActiveSubscriptionLaneHandle,
    pub(super) consumer_attachment: SubscriptionConsumerAttachment,
    pub(super) request: DeclarativeLiveQueryRequest,
    pub(super) remask_posture: Option<WorthQueryRuntimeRemaskPosture>,
}

pub(super) struct WorthQueryRuntimeLiveSubscriptionState {
    pub(super) installation: WorthQueryRuntimeLiveSubscriptionInstallation,
    pub(super) active_lane_handle: crate::subscription::ActiveSubscriptionLaneHandle,
    pub(super) consumer_attachment: SubscriptionConsumerAttachment,
    pub(super) request: DeclarativeLiveQueryRequest,
    pub(super) delivery_batches: Vec<WorthQueryRuntimeDeliveryBatch>,
    pub(super) last_delivery: Option<WorthQueryRuntimeRetainedDelivery>,
    pub(super) async_result_state: Option<WorthQueryRuntimeAsyncResultState>,
    pub(super) remask_posture: Option<WorthQueryRuntimeRemaskPosture>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct WorthQueryLiveSubscriptionIndexEntry {
    target_collection: WorthQueryMutationTargetCollectionIdentity,
    targets: BTreeSet<WorthQueryLiveArtifactTarget>,
}

impl WorthQueryLiveSubscriptionIndexEntry {
    fn new(target_collection: WorthQueryMutationTargetCollectionIdentity) -> Self {
        Self {
            target_collection,
            targets: BTreeSet::new(),
        }
    }

    pub(super) fn target_collection(&self) -> &WorthQueryMutationTargetCollectionIdentity {
        &self.target_collection
    }

    pub(super) fn targets(&self) -> &BTreeSet<WorthQueryLiveArtifactTarget> {
        &self.targets
    }

    fn targets_mut(&mut self) -> &mut BTreeSet<WorthQueryLiveArtifactTarget> {
        &mut self.targets
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryRuntimeDeliveryBatch {
    pub(super) view_name: String,
    pub(super) authority_lane: WorthQueryAuthorityLane,
    pub(super) delivery_batch_identity: WorthQueryEvidenceIdentity,
    pub(super) delivery_window_identity: WorthQueryEvidenceIdentity,
    pub(super) consumer_attachment_identity: WorthQueryEvidenceIdentity,
    pub(super) sequence: u64,
    pub(super) delivery_cause_kind: QuerySubscriptionDeliveryCauseKind,
    pub(super) delivery_cause_identity: WorthQueryEvidenceIdentity,
    pub(super) has_relational_patch: bool,
    pub(super) patch_group_kind: QueryPatchGroupKind,
    pub(super) patch_group_identity: WorthQueryEvidenceIdentity,
    pub(super) patch_group_width: u64,
    pub(super) live_graph_read_maintenance: Option<WorthQueryLiveGraphReadMaintenanceReceipt>,
    pub(super) mixed_cause_delivery: WorthQueryRuntimeMixedCauseDelivery,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct WorthQueryRuntimeRetainedDelivery {
    delivery_batch_identity: WorthQueryEvidenceIdentity,
    delivery_cause_kind: QuerySubscriptionDeliveryCauseKind,
    delivery_cause_identity: WorthQueryEvidenceIdentity,
    has_relational_patch: bool,
    sequence: u64,
    mixed_cause_delivery: WorthQueryRuntimeMixedCauseDelivery,
}

impl WorthQueryRuntimeDeliveryBatch {
    pub(super) fn from_query_delivery(
        view_name: &str,
        batch: &QueryDeliveryBatch,
        live_graph_read_maintenance: Option<WorthQueryLiveGraphReadMaintenanceReceipt>,
    ) -> Self {
        Self {
            view_name: view_name.to_string(),
            authority_lane: WorthQueryAuthorityLane::AuthoritativeTruth,
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
            live_graph_read_maintenance,
            mixed_cause_delivery: if batch.has_relational_patch() {
                WorthQueryRuntimeMixedCauseDelivery::atomic_relational_patch(
                    batch.delivery_cause().delivery_cause_identity(),
                )
            } else {
                WorthQueryRuntimeMixedCauseDelivery::atomic_time_only(
                    batch.delivery_cause_kind(),
                    batch.delivery_cause().delivery_cause_identity(),
                )
            },
        }
    }

    pub fn view_name(&self) -> &str {
        &self.view_name
    }

    pub fn authority_lane(&self) -> WorthQueryAuthorityLane {
        self.authority_lane
    }

    pub fn delivery_batch_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.delivery_batch_identity
    }

    pub fn delivery_batch_for_reporting(&self) -> &str {
        self.delivery_batch_identity.as_str()
    }

    pub fn delivery_window_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.delivery_window_identity
    }

    pub fn delivery_window_for_reporting(&self) -> &str {
        self.delivery_window_identity.as_str()
    }

    pub fn consumer_attachment_identity(&self) -> &WorthQueryEvidenceIdentity {
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

    pub fn delivery_cause_identity(&self) -> &WorthQueryEvidenceIdentity {
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

    pub fn patch_group_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.patch_group_identity
    }

    pub fn patch_group_for_reporting(&self) -> &str {
        self.patch_group_identity.as_str()
    }

    pub fn patch_group_width(&self) -> u64 {
        self.patch_group_width
    }

    pub fn live_graph_read_maintenance(
        &self,
    ) -> Option<&WorthQueryLiveGraphReadMaintenanceReceipt> {
        self.live_graph_read_maintenance.as_ref()
    }

    pub fn mixed_cause_delivery(&self) -> &WorthQueryRuntimeMixedCauseDelivery {
        &self.mixed_cause_delivery
    }
}

impl WorthQueryRuntimeRetainedDelivery {
    pub(super) fn from_batch(batch: &WorthQueryRuntimeDeliveryBatch) -> Self {
        Self {
            delivery_batch_identity: batch.delivery_batch_identity().clone(),
            delivery_cause_kind: batch.delivery_cause_kind(),
            delivery_cause_identity: batch.delivery_cause_identity().clone(),
            has_relational_patch: batch.has_relational_patch(),
            sequence: batch.sequence(),
            mixed_cause_delivery: batch.mixed_cause_delivery().clone(),
        }
    }

    pub(super) fn delivery_batch_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.delivery_batch_identity
    }

    #[allow(dead_code)]
    pub(super) fn delivery_batch_for_reporting(&self) -> &str {
        self.delivery_batch_identity.as_str()
    }

    pub(super) fn delivery_cause_kind(&self) -> QuerySubscriptionDeliveryCauseKind {
        self.delivery_cause_kind
    }

    pub(super) fn delivery_cause_identity(&self) -> &WorthQueryEvidenceIdentity {
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

    pub(super) fn mixed_cause_delivery(&self) -> &WorthQueryRuntimeMixedCauseDelivery {
        &self.mixed_cause_delivery
    }
}

pub(super) fn register_live_subscription_index(
    index: &mut Vec<WorthQueryLiveSubscriptionIndexEntry>,
    view_name: &str,
    target: WorthQueryLiveArtifactTarget,
    request: &DeclarativeLiveQueryRequest,
) {
    unregister_live_subscription_index(index, view_name);
    let target_collection = request.target_collection_identity();
    let entry = match index.iter_mut().find(|entry| {
        entry
            .target_collection
            .same_target_collection_as(&target_collection)
    }) {
        Some(entry) => entry,
        None => {
            index.push(WorthQueryLiveSubscriptionIndexEntry::new(target_collection));
            index.last_mut().expect("inserted subscription index entry")
        }
    };
    entry.targets_mut().insert(target);
}

fn unregister_live_subscription_index(
    index: &mut Vec<WorthQueryLiveSubscriptionIndexEntry>,
    view_name: &str,
) {
    index.retain_mut(|entry| {
        entry
            .targets_mut()
            .retain(|target| target.view_name() != view_name);
        !entry.targets().is_empty()
    });
}

use std::collections::BTreeSet;

use crate::declarative_live::DeclarativeLiveQueryRequest;
use crate::evidence_identity::ForgeQueryEvidenceIdentity;
use crate::subscription::{
    QueryDeliveryBatch, QueryPatchGroupKind, QuerySubscriptionDeliveryCauseKind,
    SubscriptionConsumerAttachment,
};

use super::{
    ForgeQueryAuthorityLane, ForgeQueryLiveArtifactTarget,
    ForgeQueryLiveGraphReadMaintenanceReceipt, ForgeQueryRuntimeAsyncResultState,
    ForgeQueryRuntimeLiveSubscriptionInstallation, ForgeQueryRuntimeMixedCauseDelivery,
    ForgeQueryRuntimeRemaskPosture,
};
use crate::runtime::ForgeQueryMutationTargetCollectionIdentity;

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
pub(super) struct ForgeQueryLiveSubscriptionIndexEntry {
    target_collection: ForgeQueryMutationTargetCollectionIdentity,
    targets: BTreeSet<ForgeQueryLiveArtifactTarget>,
}

impl ForgeQueryLiveSubscriptionIndexEntry {
    fn new(target_collection: ForgeQueryMutationTargetCollectionIdentity) -> Self {
        Self {
            target_collection,
            targets: BTreeSet::new(),
        }
    }

    pub(super) fn target_collection(&self) -> &ForgeQueryMutationTargetCollectionIdentity {
        &self.target_collection
    }

    pub(super) fn targets(&self) -> &BTreeSet<ForgeQueryLiveArtifactTarget> {
        &self.targets
    }

    fn targets_mut(&mut self) -> &mut BTreeSet<ForgeQueryLiveArtifactTarget> {
        &mut self.targets
    }
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
    pub(super) live_graph_read_maintenance: Option<ForgeQueryLiveGraphReadMaintenanceReceipt>,
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
    pub(super) fn from_query_delivery(
        view_name: &str,
        batch: &QueryDeliveryBatch,
        live_graph_read_maintenance: Option<ForgeQueryLiveGraphReadMaintenanceReceipt>,
    ) -> Self {
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
            live_graph_read_maintenance,
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

    pub fn live_graph_read_maintenance(
        &self,
    ) -> Option<&ForgeQueryLiveGraphReadMaintenanceReceipt> {
        self.live_graph_read_maintenance.as_ref()
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

    #[allow(dead_code)]
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
    index: &mut Vec<ForgeQueryLiveSubscriptionIndexEntry>,
    view_name: &str,
    target: ForgeQueryLiveArtifactTarget,
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
            index.push(ForgeQueryLiveSubscriptionIndexEntry::new(target_collection));
            index.last_mut().expect("inserted subscription index entry")
        }
    };
    entry.targets_mut().insert(target);
}

fn unregister_live_subscription_index(
    index: &mut Vec<ForgeQueryLiveSubscriptionIndexEntry>,
    view_name: &str,
) {
    index.retain_mut(|entry| {
        entry
            .targets_mut()
            .retain(|target| target.view_name() != view_name);
        !entry.targets().is_empty()
    });
}

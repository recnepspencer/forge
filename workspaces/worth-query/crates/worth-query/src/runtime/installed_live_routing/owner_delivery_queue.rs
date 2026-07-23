use std::collections::{HashMap, VecDeque};
use std::sync::Arc;

use super::super::conditional_owner_delivery_admission::{
    compare_owner_delivery, owner_only_work, WorthQueryStagedOwnerDeliveryAdmission,
    WorthQueryStagedOwnerDeliveryAdmissionError,
};
use super::super::conditional_owner_delivery_continuation::{
    WorthQueryOwnerDeliveryContinuation, WorthQueryOwnerDeliveryTargetContinuation,
};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct OwnerDeliveryCausalKey {
    commit: worth_runtime_bridge::facade::TruthCommitIdentity,
    patch: worth_runtime_bridge::facade::TruthPatchIdentity,
    snapshot: worth_runtime_bridge::facade::TruthSnapshotIdentity,
    branch: worth_runtime_bridge::facade::TruthBranchIdentity,
}

struct StagedOwnerDelivery {
    staging_id: u64,
    receipt: worth_runtime_bridge::facade::BridgeCorrespondenceDeliveryReceipt,
    continuation: Arc<WorthQueryOwnerDeliveryContinuation>,
    target_continuation: Arc<WorthQueryOwnerDeliveryTargetContinuation>,
    routing_work: crate::runtime::WorthQueryLiveMutationRoutingWork,
}

#[derive(Default)]
pub(super) struct WorthQueryInstalledOwnerDeliveryQueue {
    next_staging_id: u64,
    order: VecDeque<(OwnerDeliveryCausalKey, u64)>,
    pending: HashMap<OwnerDeliveryCausalKey, VecDeque<StagedOwnerDelivery>>,
}

pub(super) struct WorthQueryQueuedOwnerDeliveryAdmission {
    causal_key: OwnerDeliveryCausalKey,
    staging_id: u64,
    receipt: worth_runtime_bridge::facade::BridgeCorrespondenceDeliveryReceipt,
    continuation: Arc<WorthQueryOwnerDeliveryContinuation>,
    target_continuation: Arc<WorthQueryOwnerDeliveryTargetContinuation>,
    work: WorthQueryStagedOwnerDeliveryAdmission,
    routing_work: crate::runtime::WorthQueryLiveMutationRoutingWork,
}

impl WorthQueryQueuedOwnerDeliveryAdmission {
    pub(super) fn receipt(
        &self,
    ) -> &worth_runtime_bridge::facade::BridgeCorrespondenceDeliveryReceipt {
        &self.receipt
    }

    pub(super) fn continuation(&self) -> &Arc<WorthQueryOwnerDeliveryContinuation> {
        &self.continuation
    }

    pub(super) fn target_continuation(&self) -> &Arc<WorthQueryOwnerDeliveryTargetContinuation> {
        &self.target_continuation
    }

    pub(super) const fn work(&self) -> WorthQueryStagedOwnerDeliveryAdmission {
        self.work
    }

    pub(super) const fn routing_work(&self) -> crate::runtime::WorthQueryLiveMutationRoutingWork {
        self.routing_work
    }
}

impl WorthQueryInstalledOwnerDeliveryQueue {
    pub(super) fn stage(
        &mut self,
        receipt: &worth_runtime_bridge::facade::BridgeCorrespondenceDeliveryReceipt,
        continuation: Arc<WorthQueryOwnerDeliveryContinuation>,
        routing_work: crate::runtime::WorthQueryLiveMutationRoutingWork,
    ) {
        let causal_key = causal_key(receipt);
        self.next_staging_id = self.next_staging_id.saturating_add(1);
        let staging_id = self.next_staging_id;
        self.order.push_back((causal_key.clone(), staging_id));
        self.pending
            .entry(causal_key)
            .or_default()
            .push_back(StagedOwnerDelivery {
                staging_id,
                receipt: receipt.clone(),
                continuation,
                target_continuation: Arc::new(WorthQueryOwnerDeliveryTargetContinuation::default()),
                routing_work,
            });
    }

    pub(super) fn admit(
        &self,
        owner: &worth_runtime_bridge::facade::BridgeCorrespondenceDeliveryReceipt,
    ) -> Result<WorthQueryQueuedOwnerDeliveryAdmission, WorthQueryStagedOwnerDeliveryAdmissionError>
    {
        let owner_key = causal_key(owner);
        let Some((oldest_key, oldest_id)) = self.order.front() else {
            return Err(causal_mismatch(owner_only_work(owner)));
        };
        if oldest_key != &owner_key {
            return Err(WorthQueryStagedOwnerDeliveryAdmissionError::out_of_order(
                owner_only_work(owner),
            ));
        }
        let bucket = self
            .pending
            .get(&owner_key)
            .expect("the oldest owner causal key must retain its bucket");
        let staged = bucket
            .front()
            .expect("the oldest owner causal bucket must retain its receipt");
        debug_assert_eq!(staged.staging_id, *oldest_id);
        let (matches, work) = compare_owner_delivery(&staged.receipt, owner);
        if !matches {
            return Err(causal_mismatch(work));
        }
        Ok(WorthQueryQueuedOwnerDeliveryAdmission {
            causal_key: owner_key,
            staging_id: staged.staging_id,
            receipt: staged.receipt.clone(),
            continuation: Arc::clone(&staged.continuation),
            target_continuation: Arc::clone(&staged.target_continuation),
            work,
            routing_work: staged.routing_work,
        })
    }

    pub(super) fn consume(&mut self, admitted: &WorthQueryQueuedOwnerDeliveryAdmission) {
        let bucket = self
            .pending
            .get_mut(&admitted.causal_key)
            .expect("admitted owner causal bucket must remain staged");
        let staged = bucket
            .pop_front()
            .expect("admitted owner causal bucket must retain its oldest receipt");
        debug_assert_eq!(staged.staging_id, admitted.staging_id);
        let (causal_key, staging_id) = self
            .order
            .pop_front()
            .expect("admitted owner queue must retain its oldest identity");
        debug_assert_eq!(causal_key, admitted.causal_key);
        debug_assert_eq!(staging_id, admitted.staging_id);
        if bucket.is_empty() {
            self.pending.remove(&admitted.causal_key);
        }
    }
}

fn causal_key(
    receipt: &worth_runtime_bridge::facade::BridgeCorrespondenceDeliveryReceipt,
) -> OwnerDeliveryCausalKey {
    let change_set = receipt.change_set();
    OwnerDeliveryCausalKey {
        commit: change_set.commit_identity().clone(),
        patch: change_set.patch_identity().clone(),
        snapshot: change_set.snapshot_identity().clone(),
        branch: change_set.branch_identity().clone(),
    }
}

fn causal_mismatch(
    work: WorthQueryStagedOwnerDeliveryAdmission,
) -> WorthQueryStagedOwnerDeliveryAdmissionError {
    WorthQueryStagedOwnerDeliveryAdmissionError::causal_mismatch(work)
}

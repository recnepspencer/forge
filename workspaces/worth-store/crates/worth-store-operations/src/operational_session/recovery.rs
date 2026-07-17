use crate::{ReplicaBootstrapRecoveryHandle, ReplicaPromotionRecoveryHandle};

use super::{OperationalSessionIdentity, OperationalSessionKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationalSafeNextAction {
    ResumeExecution,
    PersistOwnerReceipt,
    PersistExternalFence,
    ContinueAfterDurableFence,
    PostVerifyRecordedResult,
    AcquireServeLease,
    ReacquireServeLease,
    Finalized,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OperationalSessionRecoveryHandle {
    session: OperationalSessionIdentity,
    kind: OperationalSessionKind,
    next_action: OperationalSafeNextAction,
    durable_transition_count: u64,
    retained_source_lease: bool,
    external_fence_known: bool,
}

impl OperationalSessionRecoveryHandle {
    pub fn from_replica_bootstrap(handle: &ReplicaBootstrapRecoveryHandle) -> Self {
        Self {
            session: OperationalSessionIdentity::from_operation(handle.operation_id()),
            kind: OperationalSessionKind::ReplicaBootstrap,
            next_action: match (handle.transfer(), handle.disposition()) {
                (_, Some(_)) => OperationalSafeNextAction::Finalized,
                (Some(_), None) => OperationalSafeNextAction::PostVerifyRecordedResult,
                (None, None) => OperationalSafeNextAction::ResumeExecution,
            },
            durable_transition_count: 1
                + u64::from(handle.transfer().is_some())
                + u64::from(handle.disposition().is_some()),
            retained_source_lease: handle.disposition().is_none(),
            external_fence_known: false,
        }
    }

    pub fn from_replica_promotion(handle: &ReplicaPromotionRecoveryHandle) -> Self {
        let next_action = match (
            handle.fence(),
            handle.receipt(),
            handle.publication(),
            handle.readmission(),
        ) {
            (_, _, _, Some(_)) => OperationalSafeNextAction::ReacquireServeLease,
            (_, _, Some(_), None) => OperationalSafeNextAction::AcquireServeLease,
            (_, Some(_), None, None) => OperationalSafeNextAction::PostVerifyRecordedResult,
            (Some(_), None, None, None) => OperationalSafeNextAction::ContinueAfterDurableFence,
            (None, None, None, None) => OperationalSafeNextAction::PersistExternalFence,
        };
        Self {
            session: OperationalSessionIdentity::from_operation(handle.operation_id()),
            kind: OperationalSessionKind::ReplicaPromotion,
            next_action,
            durable_transition_count: 1
                + u64::from(handle.fence().is_some())
                + u64::from(handle.receipt().is_some())
                + u64::from(handle.publication().is_some())
                + u64::from(handle.readmission().is_some()),
            retained_source_lease: false,
            external_fence_known: handle.fence().is_some(),
        }
    }

    pub const fn session(self) -> OperationalSessionIdentity {
        self.session
    }
    pub const fn kind(self) -> OperationalSessionKind {
        self.kind
    }
    pub const fn next_action(self) -> OperationalSafeNextAction {
        self.next_action
    }
    pub const fn durable_transition_count(self) -> u64 {
        self.durable_transition_count
    }
    pub const fn retained_source_lease(self) -> bool {
        self.retained_source_lease
    }
    pub const fn external_fence_known(self) -> bool {
        self.external_fence_known
    }
}

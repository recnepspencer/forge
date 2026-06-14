use crate::evidence_identity::ForgeQueryEvidenceIdentity;

use super::active_budget::{ActiveSubscriptionAllocationPosture, ActiveSubscriptionWorkBudget};
use super::active_counters::ActiveSubscriptionCounters;
use super::active_digest::ActiveSubscriptionLaneDigest;
use super::active_posture::{
    ActiveLaneLookupClass, ActiveSubscriptionDeliveryPosture, ActiveSubscriptionLifecyclePosture,
};
use super::future_selection::QuerySubscriptionFutureSelection;
use super::performance_receipt::SubscriptionPerformanceReceipt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActiveSubscriptionLaneAdmission {
    pub(super) lane_digest: ActiveSubscriptionLaneDigest,
    pub(super) activation_digest: String,
    pub(super) admission_digest: String,
    pub(super) query_declaration_digest: String,
    pub(super) bridge_declaration_digest: String,
    pub(super) future_selection: QuerySubscriptionFutureSelection,
    pub(super) basis_binding_identity: ForgeQueryEvidenceIdentity,
    pub(super) checkpoint_identity: ForgeQueryEvidenceIdentity,
    pub(super) signal_strategy_digest: String,
    pub(super) lifecycle_posture: ActiveSubscriptionLifecyclePosture,
    pub(super) delivery_posture: ActiveSubscriptionDeliveryPosture,
    pub(super) lookup_class: ActiveLaneLookupClass,
    pub(super) allocation_policy: ActiveSubscriptionAllocationPosture,
    pub(super) budget: ActiveSubscriptionWorkBudget,
    pub(super) performance_receipt: SubscriptionPerformanceReceipt,
    pub(super) counters: ActiveSubscriptionCounters,
}

impl ActiveSubscriptionLaneAdmission {
    pub fn lane_digest(&self) -> &ActiveSubscriptionLaneDigest {
        &self.lane_digest
    }

    pub fn activation_digest(&self) -> &str {
        &self.activation_digest
    }

    pub fn admission_digest(&self) -> &str {
        &self.admission_digest
    }

    pub fn query_declaration_digest(&self) -> &str {
        &self.query_declaration_digest
    }

    pub fn bridge_declaration_digest(&self) -> &str {
        &self.bridge_declaration_digest
    }

    pub fn future_selection(&self) -> &QuerySubscriptionFutureSelection {
        &self.future_selection
    }

    pub fn basis_binding_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.basis_binding_identity
    }

    pub fn basis_binding_for_reporting(&self) -> &str {
        self.basis_binding_identity.as_str()
    }

    pub fn checkpoint_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.checkpoint_identity
    }

    pub fn checkpoint_for_reporting(&self) -> &str {
        self.checkpoint_identity.as_str()
    }

    pub fn signal_strategy_digest(&self) -> &str {
        &self.signal_strategy_digest
    }

    pub fn lifecycle_posture(&self) -> &ActiveSubscriptionLifecyclePosture {
        &self.lifecycle_posture
    }

    pub fn delivery_posture(&self) -> &ActiveSubscriptionDeliveryPosture {
        &self.delivery_posture
    }

    pub fn lookup_class(&self) -> &ActiveLaneLookupClass {
        &self.lookup_class
    }

    pub fn allocation_policy(&self) -> &ActiveSubscriptionAllocationPosture {
        &self.allocation_policy
    }

    pub fn allocation_posture(&self) -> ActiveSubscriptionAllocationPosture {
        self.allocation_policy
    }

    pub fn budget(&self) -> &ActiveSubscriptionWorkBudget {
        &self.budget
    }

    pub fn performance_receipt(&self) -> &SubscriptionPerformanceReceipt {
        &self.performance_receipt
    }

    pub fn counters(&self) -> &ActiveSubscriptionCounters {
        &self.counters
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActiveSubscriptionLane {
    pub(super) lane_digest: ActiveSubscriptionLaneDigest,
    pub(super) activation_digest: String,
    pub(super) admission_digest: String,
    pub(super) query_declaration_digest: String,
    pub(super) bridge_declaration_digest: String,
    pub(super) future_selection: QuerySubscriptionFutureSelection,
    pub(super) basis_binding_identity: ForgeQueryEvidenceIdentity,
    pub(super) checkpoint_identity: ForgeQueryEvidenceIdentity,
    pub(super) signal_strategy_digest: String,
    pub(super) lifecycle_posture: ActiveSubscriptionLifecyclePosture,
    pub(super) delivery_posture: ActiveSubscriptionDeliveryPosture,
    pub(super) lookup_class: ActiveLaneLookupClass,
    pub(super) allocation_policy: ActiveSubscriptionAllocationPosture,
    pub(super) attachment_count: u64,
}

impl ActiveSubscriptionLane {
    pub fn lane_digest(&self) -> &ActiveSubscriptionLaneDigest {
        &self.lane_digest
    }

    pub fn activation_digest(&self) -> &str {
        &self.activation_digest
    }

    pub fn admission_digest(&self) -> &str {
        &self.admission_digest
    }

    pub fn query_declaration_digest(&self) -> &str {
        &self.query_declaration_digest
    }

    pub fn bridge_declaration_digest(&self) -> &str {
        &self.bridge_declaration_digest
    }

    pub fn future_selection(&self) -> &QuerySubscriptionFutureSelection {
        &self.future_selection
    }

    pub fn basis_binding_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.basis_binding_identity
    }

    pub fn basis_binding_for_reporting(&self) -> &str {
        self.basis_binding_identity.as_str()
    }

    pub fn checkpoint_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.checkpoint_identity
    }

    pub fn checkpoint_for_reporting(&self) -> &str {
        self.checkpoint_identity.as_str()
    }

    pub fn signal_strategy_digest(&self) -> &str {
        &self.signal_strategy_digest
    }

    pub fn lifecycle_posture(&self) -> &ActiveSubscriptionLifecyclePosture {
        &self.lifecycle_posture
    }

    pub fn delivery_posture(&self) -> &ActiveSubscriptionDeliveryPosture {
        &self.delivery_posture
    }

    pub fn lookup_class(&self) -> &ActiveLaneLookupClass {
        &self.lookup_class
    }

    pub fn allocation_policy(&self) -> &ActiveSubscriptionAllocationPosture {
        &self.allocation_policy
    }

    pub fn allocation_posture(&self) -> ActiveSubscriptionAllocationPosture {
        self.allocation_policy
    }
}

use crate::evidence_identity::ForgeQueryEvidenceIdentity;
use crate::identity_authority::{QueryProjectionIdentity, QuerySubscriptionIdentityKind};

use super::active_budget::{ActiveSubscriptionAllocationPosture, ActiveSubscriptionWorkBudget};
use super::active_counters::ActiveSubscriptionCounters;
use super::active_digest::ActiveSubscriptionLaneDigest;
use super::active_posture::{
    ActiveLaneLookupClass, ActiveSubscriptionDeliveryPosture, ActiveSubscriptionLifecyclePosture,
};
use super::evidence_projection::subscription_evidence_projection;
use super::future_selection::QuerySubscriptionFutureSelection;
use super::performance_receipt::SubscriptionPerformanceReceipt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActiveSubscriptionLaneAdmission {
    pub(super) lane_digest: ActiveSubscriptionLaneDigest,
    pub(super) activation_identity: ForgeQueryEvidenceIdentity,
    pub(super) admission_identity: ForgeQueryEvidenceIdentity,
    pub(super) query_declaration_identity: ForgeQueryEvidenceIdentity,
    pub(super) bridge_declaration_identity: ForgeQueryEvidenceIdentity,
    pub(super) future_selection: QuerySubscriptionFutureSelection,
    pub(super) basis_binding_identity: ForgeQueryEvidenceIdentity,
    pub(super) checkpoint_identity: ForgeQueryEvidenceIdentity,
    pub(super) signal_strategy_identity: ForgeQueryEvidenceIdentity,
    pub(super) lifecycle_posture: ActiveSubscriptionLifecyclePosture,
    pub(super) delivery_posture: ActiveSubscriptionDeliveryPosture,
    pub(super) lookup_class: ActiveLaneLookupClass,
    pub(super) allocation_policy: ActiveSubscriptionAllocationPosture,
    pub(super) budget: ActiveSubscriptionWorkBudget,
    pub(super) performance_receipt: SubscriptionPerformanceReceipt,
    pub(super) counters: ActiveSubscriptionCounters,
}

impl ActiveSubscriptionLaneAdmission {
    pub(crate) fn lane_digest(&self) -> &ActiveSubscriptionLaneDigest {
        &self.lane_digest
    }

    pub fn activation_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        subscription_evidence_projection(&self.activation_identity)
    }

    pub fn activation_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.activation_identity
    }

    pub fn admission_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        subscription_evidence_projection(&self.admission_identity)
    }

    pub fn admission_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.admission_identity
    }

    pub fn query_declaration_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        subscription_evidence_projection(&self.query_declaration_identity)
    }

    pub fn query_declaration_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.query_declaration_identity
    }

    pub fn bridge_declaration_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        subscription_evidence_projection(&self.bridge_declaration_identity)
    }

    pub fn bridge_declaration_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.bridge_declaration_identity
    }

    pub fn future_selection(&self) -> &QuerySubscriptionFutureSelection {
        &self.future_selection
    }

    pub fn basis_binding_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        subscription_evidence_projection(&self.basis_binding_identity)
    }

    pub fn basis_binding_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.basis_binding_identity
    }

    pub fn checkpoint_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        subscription_evidence_projection(&self.checkpoint_identity)
    }

    pub fn checkpoint_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.checkpoint_identity
    }

    pub fn signal_strategy_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        subscription_evidence_projection(&self.signal_strategy_identity)
    }

    pub fn signal_strategy_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.signal_strategy_identity
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
    pub(super) activation_identity: ForgeQueryEvidenceIdentity,
    pub(super) admission_identity: ForgeQueryEvidenceIdentity,
    pub(super) query_declaration_identity: ForgeQueryEvidenceIdentity,
    pub(super) bridge_declaration_identity: ForgeQueryEvidenceIdentity,
    pub(super) future_selection: QuerySubscriptionFutureSelection,
    pub(super) basis_binding_identity: ForgeQueryEvidenceIdentity,
    pub(super) checkpoint_identity: ForgeQueryEvidenceIdentity,
    pub(super) signal_strategy_identity: ForgeQueryEvidenceIdentity,
    pub(super) lifecycle_posture: ActiveSubscriptionLifecyclePosture,
    pub(super) delivery_posture: ActiveSubscriptionDeliveryPosture,
    pub(super) lookup_class: ActiveLaneLookupClass,
    pub(super) allocation_policy: ActiveSubscriptionAllocationPosture,
    pub(super) attachment_count: u64,
}

impl ActiveSubscriptionLane {
    pub(crate) fn lane_digest(&self) -> &ActiveSubscriptionLaneDigest {
        &self.lane_digest
    }

    pub fn activation_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        subscription_evidence_projection(&self.activation_identity)
    }

    pub fn activation_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.activation_identity
    }

    pub fn admission_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        subscription_evidence_projection(&self.admission_identity)
    }

    pub fn admission_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.admission_identity
    }

    pub fn query_declaration_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        subscription_evidence_projection(&self.query_declaration_identity)
    }

    pub fn query_declaration_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.query_declaration_identity
    }

    pub fn bridge_declaration_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        subscription_evidence_projection(&self.bridge_declaration_identity)
    }

    pub fn bridge_declaration_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.bridge_declaration_identity
    }

    pub fn future_selection(&self) -> &QuerySubscriptionFutureSelection {
        &self.future_selection
    }

    pub fn basis_binding_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        subscription_evidence_projection(&self.basis_binding_identity)
    }

    pub fn basis_binding_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.basis_binding_identity
    }

    pub fn checkpoint_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        subscription_evidence_projection(&self.checkpoint_identity)
    }

    pub fn checkpoint_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.checkpoint_identity
    }

    pub fn signal_strategy_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        subscription_evidence_projection(&self.signal_strategy_identity)
    }

    pub fn signal_strategy_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.signal_strategy_identity
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

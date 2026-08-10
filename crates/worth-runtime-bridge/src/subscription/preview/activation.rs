use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::speculation::{
    BridgePreviewLifecycleStateKind, BridgePreviewSessionIdentity, PreviewExecutionRecordIdentity,
};

use super::super::{
    BridgeAdmittedSubscriptionIdentity, BridgePreviewActiveSubscriptionIdentity,
    BridgeSubscriptionActivationReady, BridgeSubscriptionConsumerContract,
    BridgeSubscriptionConsumerContractIdentity, BridgeSubscriptionCounters,
    BridgeSubscriptionDeliveryBufferLifecycleIdentity, BridgeSubscriptionDeliveryBufferPlan,
    BridgeSubscriptionDeliveryCostProfile, BridgeSubscriptionDeliveryCostProfileIdentity,
    BridgeSubscriptionLifecycleIdentity, BridgeSubscriptionPreviewBasisIdentity,
    BridgeSubscriptionPreviewLifecycleIdentity, BridgeSubscriptionPreviewParentBasisIdentity,
    BridgeSubscriptionPreviewResidueScopeIdentity, BridgeSubscriptionPreviewScopeIdentity,
};
use super::basis::BridgeSubscriptionPreviewBasisBinding;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgePreviewActiveSubscription {
    preview_active_subscription_identity: BridgePreviewActiveSubscriptionIdentity,
    preview_basis_identity: BridgeSubscriptionPreviewBasisIdentity,
    preview_scope_identity: BridgeSubscriptionPreviewScopeIdentity,
    preview_parent_basis_identity: BridgeSubscriptionPreviewParentBasisIdentity,
    preview_lifecycle_identity: BridgeSubscriptionPreviewLifecycleIdentity,
    preview_residue_scope_identity: BridgeSubscriptionPreviewResidueScopeIdentity,
    preview_session_identity: BridgePreviewSessionIdentity,
    preview_execution_record_identity: PreviewExecutionRecordIdentity,
    preview_basis_digest: Arc<str>,
    branch_binding_digest: Arc<str>,
    parent_truth_view_basis_digest: Arc<str>,
    preview_lifecycle_state_kind: BridgePreviewLifecycleStateKind,
    active_preview_session_digest: Arc<str>,
    admitted_subscription_identity: BridgeAdmittedSubscriptionIdentity,
    activation_lifecycle_identity: BridgeSubscriptionLifecycleIdentity,
    cost_profile_identity: BridgeSubscriptionDeliveryCostProfileIdentity,
    consumer_contract_identity: BridgeSubscriptionConsumerContractIdentity,
    buffer_lifecycle_identity: BridgeSubscriptionDeliveryBufferLifecycleIdentity,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgePreviewActiveSubscription {
    pub(crate) fn activate(
        activation_ready: BridgeSubscriptionActivationReady,
        preview_basis: BridgeSubscriptionPreviewBasisBinding,
        cost_profile: BridgeSubscriptionDeliveryCostProfile,
        consumer_contract: BridgeSubscriptionConsumerContract,
    ) -> Self {
        let buffer_plan = BridgeSubscriptionDeliveryBufferPlan::from_cost_profile(&cost_profile);
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-preview-active-subscription|ready={}|preview-basis={}|preview-scope={}|preview-parent-basis={}|preview-lifecycle={}|preview-residue-scope={}|preview-session={}|preview-execution={}|preview-basis-digest={}|branch-binding={}|parent-truth-view={}|active-preview={}|admitted={}|lifecycle={}|cost-profile={}|consumer={}|buffer={}",
            activation_ready.digest(),
            preview_basis.preview_basis_identity().as_str(),
            preview_basis.preview_scope_identity().as_str(),
            preview_basis.preview_parent_basis_identity().as_str(),
            preview_basis.preview_lifecycle_identity().as_str(),
            preview_basis.preview_residue_scope_identity().as_str(),
            preview_basis.preview_session_identity().as_str(),
            preview_basis.preview_execution_record_identity().as_str(),
            preview_basis.digest(),
            preview_basis.branch_binding_digest(),
            preview_basis.parent_truth_view_basis_digest(),
            preview_basis.active_preview_session_digest(),
            activation_ready.admitted().admitted_subscription_identity().as_str(),
            activation_ready.lifecycle_record().lifecycle_identity().as_str(),
            cost_profile.cost_profile_identity().as_str(),
            consumer_contract.consumer_contract_identity().as_str(),
            buffer_plan.buffer_lifecycle_identity().as_str(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            preview_active_subscription_identity:
                BridgePreviewActiveSubscriptionIdentity::admit_bridge_owned(format!(
                    "bridge-preview-active-subscription-id:sha256:{digest:x}"
                )),
            preview_basis_identity: preview_basis.preview_basis_identity,
            preview_scope_identity: preview_basis.preview_scope_identity,
            preview_parent_basis_identity: preview_basis.preview_parent_basis_identity,
            preview_lifecycle_identity: preview_basis.preview_lifecycle_identity,
            preview_residue_scope_identity: preview_basis.preview_residue_scope_identity,
            preview_session_identity: preview_basis.preview_session_identity,
            preview_execution_record_identity: preview_basis.preview_execution_record_identity,
            preview_basis_digest: preview_basis.digest,
            branch_binding_digest: preview_basis.branch_binding_digest,
            parent_truth_view_basis_digest: preview_basis.parent_truth_view_basis_digest,
            preview_lifecycle_state_kind: preview_basis.preview_lifecycle_state_kind,
            active_preview_session_digest: preview_basis.active_preview_session_digest,
            admitted_subscription_identity: activation_ready
                .admitted()
                .admitted_subscription_identity()
                .clone(),
            activation_lifecycle_identity: activation_ready
                .lifecycle_record()
                .lifecycle_identity()
                .clone(),
            cost_profile_identity: cost_profile.cost_profile_identity().clone(),
            consumer_contract_identity: consumer_contract.consumer_contract_identity().clone(),
            buffer_lifecycle_identity: buffer_plan.buffer_lifecycle_identity().clone(),
            counters: BridgeSubscriptionCounters::from_subscription_preview_activation(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-preview-active-subscription:sha256:{digest:x}"
            )),
        }
    }

    pub fn preview_active_subscription_identity(&self) -> &BridgePreviewActiveSubscriptionIdentity {
        &self.preview_active_subscription_identity
    }
    pub fn preview_basis_identity(&self) -> &BridgeSubscriptionPreviewBasisIdentity {
        &self.preview_basis_identity
    }
    pub fn preview_scope_identity(&self) -> &BridgeSubscriptionPreviewScopeIdentity {
        &self.preview_scope_identity
    }
    pub fn preview_parent_basis_identity(&self) -> &BridgeSubscriptionPreviewParentBasisIdentity {
        &self.preview_parent_basis_identity
    }
    pub fn preview_lifecycle_identity(&self) -> &BridgeSubscriptionPreviewLifecycleIdentity {
        &self.preview_lifecycle_identity
    }
    pub fn preview_residue_scope_identity(&self) -> &BridgeSubscriptionPreviewResidueScopeIdentity {
        &self.preview_residue_scope_identity
    }
    pub fn preview_session_identity(&self) -> &BridgePreviewSessionIdentity {
        &self.preview_session_identity
    }
    pub fn preview_execution_record_identity(&self) -> &PreviewExecutionRecordIdentity {
        &self.preview_execution_record_identity
    }
    pub fn preview_basis_digest(&self) -> &str {
        self.preview_basis_digest.as_ref()
    }
    pub fn branch_binding_digest(&self) -> &str {
        self.branch_binding_digest.as_ref()
    }
    pub fn parent_truth_view_basis_digest(&self) -> &str {
        self.parent_truth_view_basis_digest.as_ref()
    }
    pub fn preview_lifecycle_state_kind(&self) -> BridgePreviewLifecycleStateKind {
        self.preview_lifecycle_state_kind
    }
    pub fn active_preview_session_digest(&self) -> &str {
        self.active_preview_session_digest.as_ref()
    }
    pub fn admitted_subscription_identity(&self) -> &BridgeAdmittedSubscriptionIdentity {
        &self.admitted_subscription_identity
    }
    pub fn activation_lifecycle_identity(&self) -> &BridgeSubscriptionLifecycleIdentity {
        &self.activation_lifecycle_identity
    }
    pub fn cost_profile_identity(&self) -> &BridgeSubscriptionDeliveryCostProfileIdentity {
        &self.cost_profile_identity
    }
    pub fn consumer_contract_identity(&self) -> &BridgeSubscriptionConsumerContractIdentity {
        &self.consumer_contract_identity
    }
    pub fn buffer_lifecycle_identity(&self) -> &BridgeSubscriptionDeliveryBufferLifecycleIdentity {
        &self.buffer_lifecycle_identity
    }
    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }
    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

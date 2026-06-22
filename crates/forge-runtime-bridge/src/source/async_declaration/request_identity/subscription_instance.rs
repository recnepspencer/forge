use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::identity::{AsyncRequestSubscriptionInstanceIdentityTag, BridgeIdentity};
use crate::subscription::{BridgePreviewActiveSubscription, BridgeSubscriptionActivationReady};

pub type BridgeAsyncRequestSubscriptionInstanceIdentity =
    BridgeIdentity<AsyncRequestSubscriptionInstanceIdentityTag>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeAsyncRequestSubscriptionInstanceKind {
    Authoritative,
    Preview,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeAsyncRequestSubscriptionInstance {
    subscription_instance_identity: BridgeAsyncRequestSubscriptionInstanceIdentity,
    kind: BridgeAsyncRequestSubscriptionInstanceKind,
    admitted_subscription_identity: crate::subscription::BridgeAdmittedSubscriptionIdentity,
    activation_lifecycle_identity: crate::subscription::BridgeSubscriptionLifecycleIdentity,
    preview_active_subscription_identity:
        Option<crate::subscription::BridgePreviewActiveSubscriptionIdentity>,
    parent_truth_view_basis_digest: Option<Arc<str>>,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeAsyncRequestSubscriptionInstance {
    pub fn authoritative(activation_ready: &BridgeSubscriptionActivationReady) -> Self {
        Self::new(
            BridgeAsyncRequestSubscriptionInstanceKind::Authoritative,
            activation_ready
                .admitted()
                .admitted_subscription_identity()
                .clone(),
            activation_ready
                .lifecycle_record()
                .lifecycle_identity()
                .clone(),
            None,
            None,
        )
    }

    pub fn preview(preview_active: &BridgePreviewActiveSubscription) -> Self {
        Self::new(
            BridgeAsyncRequestSubscriptionInstanceKind::Preview,
            preview_active.admitted_subscription_identity().clone(),
            preview_active.activation_lifecycle_identity().clone(),
            Some(
                preview_active
                    .preview_active_subscription_identity()
                    .clone(),
            ),
            Some(Arc::from(
                preview_active.parent_truth_view_basis_digest().to_owned(),
            )),
        )
    }

    fn new(
        kind: BridgeAsyncRequestSubscriptionInstanceKind,
        admitted_subscription_identity: crate::subscription::BridgeAdmittedSubscriptionIdentity,
        activation_lifecycle_identity: crate::subscription::BridgeSubscriptionLifecycleIdentity,
        preview_active_subscription_identity: Option<
            crate::subscription::BridgePreviewActiveSubscriptionIdentity,
        >,
        parent_truth_view_basis_digest: Option<Arc<str>>,
    ) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-async-request-subscription-instance|kind={kind:?}|admitted={}|lifecycle={}|preview-active={}|parent-truth-view={}",
            admitted_subscription_identity.as_str(),
            activation_lifecycle_identity.as_str(),
            preview_active_subscription_identity
                .as_ref()
                .map(BridgeIdentity::as_str)
                .unwrap_or("-"),
            parent_truth_view_basis_digest.as_deref().unwrap_or("-"),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            subscription_instance_identity:
                BridgeAsyncRequestSubscriptionInstanceIdentity::admit_bridge_owned(format!(
                    "bridge-async-request-subscription-instance-id:sha256:{digest:x}"
                )),
            kind,
            admitted_subscription_identity,
            activation_lifecycle_identity,
            preview_active_subscription_identity,
            parent_truth_view_basis_digest,
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-async-request-subscription-instance:sha256:{digest:x}"
            )),
        }
    }

    pub fn subscription_instance_identity(
        &self,
    ) -> &BridgeAsyncRequestSubscriptionInstanceIdentity {
        &self.subscription_instance_identity
    }

    pub fn kind(&self) -> BridgeAsyncRequestSubscriptionInstanceKind {
        self.kind
    }

    pub fn admitted_subscription_identity(
        &self,
    ) -> &crate::subscription::BridgeAdmittedSubscriptionIdentity {
        &self.admitted_subscription_identity
    }

    pub fn activation_lifecycle_identity(
        &self,
    ) -> &crate::subscription::BridgeSubscriptionLifecycleIdentity {
        &self.activation_lifecycle_identity
    }

    pub fn preview_active_subscription_identity(
        &self,
    ) -> Option<&crate::subscription::BridgePreviewActiveSubscriptionIdentity> {
        self.preview_active_subscription_identity.as_ref()
    }

    pub fn parent_truth_view_basis_digest(&self) -> Option<&str> {
        self.parent_truth_view_basis_digest.as_deref()
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

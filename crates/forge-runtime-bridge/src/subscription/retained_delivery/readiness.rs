use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::super::{
    BridgeSubscriptionCounters, BridgeSubscriptionDeliveryFamilyKind,
    BridgeSubscriptionDeliveryReplayReadinessIdentity, BridgeSubscriptionDeliveryWindowIdentity,
    BridgeSubscriptionDeliveryWindowSealed,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeSubscriptionDeliveryReplayReadinessClass {
    DescriptorOnlyReplayReady,
    CanonicalMemberReplayReady,
    ReplayBlockedByOmittedPayload,
    ReplayBlockedByDiagnosticsPolicy,
    ReplayBlockedByUnsupportedFamily,
}

impl BridgeSubscriptionDeliveryReplayReadinessClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DescriptorOnlyReplayReady => "descriptor_only_replay_ready",
            Self::CanonicalMemberReplayReady => "canonical_member_replay_ready",
            Self::ReplayBlockedByOmittedPayload => "replay_blocked_by_omitted_payload",
            Self::ReplayBlockedByDiagnosticsPolicy => "replay_blocked_by_diagnostics_policy",
            Self::ReplayBlockedByUnsupportedFamily => "replay_blocked_by_unsupported_family",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionDeliveryWindowReplayReadiness {
    delivery_replay_readiness_identity: BridgeSubscriptionDeliveryReplayReadinessIdentity,
    readiness_class: BridgeSubscriptionDeliveryReplayReadinessClass,
    delivery_window_identity: BridgeSubscriptionDeliveryWindowIdentity,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionDeliveryWindowReplayReadiness {
    pub(super) fn classify(
        sealed_window: &BridgeSubscriptionDeliveryWindowSealed,
    ) -> BridgeSubscriptionDeliveryReplayReadinessClass {
        match sealed_window.delivery_family().family_kind() {
            BridgeSubscriptionDeliveryFamilyKind::ReplayAuditDescriptor
            | BridgeSubscriptionDeliveryFamilyKind::RouteFocusedDescriptor => {
                BridgeSubscriptionDeliveryReplayReadinessClass::DescriptorOnlyReplayReady
            }
            BridgeSubscriptionDeliveryFamilyKind::CanonicalMember
            | BridgeSubscriptionDeliveryFamilyKind::AdmittedCoalesced => {
                if sealed_window
                    .members()
                    .iter()
                    .any(|member| member.payload_omitted_reason().is_some())
                {
                    BridgeSubscriptionDeliveryReplayReadinessClass::ReplayBlockedByOmittedPayload
                } else {
                    BridgeSubscriptionDeliveryReplayReadinessClass::CanonicalMemberReplayReady
                }
            }
        }
    }

    pub(crate) fn inspect(sealed_window: &BridgeSubscriptionDeliveryWindowSealed) -> Self {
        let readiness_class = Self::classify(sealed_window);
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-delivery-window-replay-readiness|window={}|family={}|class={}",
            sealed_window.delivery_window_identity().as_str(),
            sealed_window
                .delivery_family()
                .delivery_family_identity()
                .as_str(),
            readiness_class.as_str(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            delivery_replay_readiness_identity:
                BridgeSubscriptionDeliveryReplayReadinessIdentity::new(format!(
                    "bridge-subscription-delivery-replay-readiness-id:sha256:{digest:x}"
                )),
            readiness_class,
            delivery_window_identity: sealed_window.delivery_window_identity().clone(),
            counters: BridgeSubscriptionCounters::from_delivery_replay_readiness_inspection(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-delivery-window-replay-readiness:sha256:{digest:x}"
            )),
        }
    }

    pub fn delivery_replay_readiness_identity(
        &self,
    ) -> &BridgeSubscriptionDeliveryReplayReadinessIdentity {
        &self.delivery_replay_readiness_identity
    }

    pub fn readiness_class(&self) -> BridgeSubscriptionDeliveryReplayReadinessClass {
        self.readiness_class
    }

    pub fn delivery_window_identity(&self) -> &BridgeSubscriptionDeliveryWindowIdentity {
        &self.delivery_window_identity
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

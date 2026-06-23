use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::BridgeSubscriptionDeliveryFamilyIdentity;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BridgeSubscriptionDeliveryFamilyKind {
    CanonicalMember,
    AdmittedCoalesced,
    ReplayAuditDescriptor,
    RouteFocusedDescriptor,
}

impl BridgeSubscriptionDeliveryFamilyKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CanonicalMember => "canonical_member",
            Self::AdmittedCoalesced => "admitted_coalesced",
            Self::ReplayAuditDescriptor => "replay_audit_descriptor",
            Self::RouteFocusedDescriptor => "route_focused_descriptor",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionDeliveryFamily {
    delivery_family_identity: BridgeSubscriptionDeliveryFamilyIdentity,
    family_kind: BridgeSubscriptionDeliveryFamilyKind,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionDeliveryFamily {
    pub(crate) fn select(family_kind: BridgeSubscriptionDeliveryFamilyKind) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-delivery-family|kind={}",
            family_kind.as_str()
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            delivery_family_identity: BridgeSubscriptionDeliveryFamilyIdentity::admit_bridge_owned(
                format!("bridge-subscription-delivery-family-id:sha256:{digest:x}"),
            ),
            family_kind,
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-delivery-family:sha256:{digest:x}"
            )),
        }
    }

    pub fn delivery_family_identity(&self) -> &BridgeSubscriptionDeliveryFamilyIdentity {
        &self.delivery_family_identity
    }

    pub fn family_kind(&self) -> BridgeSubscriptionDeliveryFamilyKind {
        self.family_kind
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

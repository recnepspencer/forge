use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::subscription::{
    BridgePreviewActiveSubscription, BridgePreviewActiveSubscriptionIdentity,
    BridgeSubscriptionPreviewBasisIdentity, BridgeSubscriptionPreviewLifecycleIdentity,
    BridgeSubscriptionPreviewScopeIdentity,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BridgeSubscriptionPreviewWorkKind {
    Routing,
    Delivery,
    Diagnostics,
    Continuation,
}

impl BridgeSubscriptionPreviewWorkKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Routing => "routing",
            Self::Delivery => "delivery",
            Self::Diagnostics => "diagnostics",
            Self::Continuation => "continuation",
        }
    }

    pub(super) const fn all() -> [Self; 4] {
        [
            Self::Routing,
            Self::Delivery,
            Self::Diagnostics,
            Self::Continuation,
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionPreviewWorkEvidence {
    preview_active_subscription_identity: BridgePreviewActiveSubscriptionIdentity,
    preview_basis_identity: BridgeSubscriptionPreviewBasisIdentity,
    preview_scope_identity: BridgeSubscriptionPreviewScopeIdentity,
    preview_lifecycle_identity: BridgeSubscriptionPreviewLifecycleIdentity,
    source_preview_digest: Arc<str>,
    kind: BridgeSubscriptionPreviewWorkKind,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionPreviewWorkEvidence {
    pub(super) fn from_preview_active(
        preview_active: &BridgePreviewActiveSubscription,
        kind: BridgeSubscriptionPreviewWorkKind,
    ) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-preview-work-evidence|preview-active={}|preview-digest={}|preview-basis={}|preview-scope={}|preview-lifecycle={}|kind={}",
            preview_active.preview_active_subscription_identity().as_str(),
            preview_active.digest(),
            preview_active.preview_basis_identity().as_str(),
            preview_active.preview_scope_identity().as_str(),
            preview_active.preview_lifecycle_identity().as_str(),
            kind.as_str(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            preview_active_subscription_identity: preview_active
                .preview_active_subscription_identity()
                .clone(),
            preview_basis_identity: preview_active.preview_basis_identity().clone(),
            preview_scope_identity: preview_active.preview_scope_identity().clone(),
            preview_lifecycle_identity: preview_active.preview_lifecycle_identity().clone(),
            source_preview_digest: Arc::from(preview_active.digest()),
            kind,
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-preview-work-evidence:sha256:{digest:x}"
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

    pub fn preview_lifecycle_identity(&self) -> &BridgeSubscriptionPreviewLifecycleIdentity {
        &self.preview_lifecycle_identity
    }

    pub fn source_preview_digest(&self) -> &str {
        self.source_preview_digest.as_ref()
    }

    pub fn kind(&self) -> BridgeSubscriptionPreviewWorkKind {
        self.kind
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionPreviewWorkInput {
    evidence: BridgeSubscriptionPreviewWorkEvidence,
}

impl BridgeSubscriptionPreviewWorkInput {
    pub fn routing(preview_active: &BridgePreviewActiveSubscription) -> Self {
        Self::new(preview_active, BridgeSubscriptionPreviewWorkKind::Routing)
    }

    pub fn delivery(preview_active: &BridgePreviewActiveSubscription) -> Self {
        Self::new(preview_active, BridgeSubscriptionPreviewWorkKind::Delivery)
    }

    pub fn diagnostics(preview_active: &BridgePreviewActiveSubscription) -> Self {
        Self::new(
            preview_active,
            BridgeSubscriptionPreviewWorkKind::Diagnostics,
        )
    }

    pub fn continuation(preview_active: &BridgePreviewActiveSubscription) -> Self {
        Self::new(
            preview_active,
            BridgeSubscriptionPreviewWorkKind::Continuation,
        )
    }

    fn new(
        preview_active: &BridgePreviewActiveSubscription,
        kind: BridgeSubscriptionPreviewWorkKind,
    ) -> Self {
        Self {
            evidence: BridgeSubscriptionPreviewWorkEvidence::from_preview_active(
                preview_active,
                kind,
            ),
        }
    }

    pub fn kind(&self) -> BridgeSubscriptionPreviewWorkKind {
        self.evidence.kind()
    }

    pub fn evidence(&self) -> &BridgeSubscriptionPreviewWorkEvidence {
        &self.evidence
    }

    pub fn evidence_digest(&self) -> &str {
        self.evidence.digest()
    }
}

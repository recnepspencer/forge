use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::subscription::BridgePreviewActiveSubscription;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeSubscriptionPreviewWorkTraceRejectionKind {
    DuplicateWorkKind,
    MissingWorkKind,
    PreviewWorkEvidenceMismatch,
}

impl BridgeSubscriptionPreviewWorkTraceRejectionKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DuplicateWorkKind => "duplicate_work_kind",
            Self::MissingWorkKind => "missing_work_kind",
            Self::PreviewWorkEvidenceMismatch => "preview_work_evidence_mismatch",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionPreviewWorkTraceRejection {
    rejection_kind: BridgeSubscriptionPreviewWorkTraceRejectionKind,
    rejection_context: Arc<str>,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionPreviewWorkTraceRejection {
    pub(super) fn new(
        preview_active: &BridgePreviewActiveSubscription,
        rejection_kind: BridgeSubscriptionPreviewWorkTraceRejectionKind,
        rejection_context: impl Into<Arc<str>>,
    ) -> Self {
        let rejection_context = rejection_context.into();
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-preview-work-trace-rejection|preview-active={}|scope={}|kind={}|context={}",
            preview_active.preview_active_subscription_identity().as_str(),
            preview_active.preview_scope_identity().as_str(),
            rejection_kind.as_str(),
            rejection_context.as_ref(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            rejection_kind,
            rejection_context,
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-preview-work-trace-rejection:sha256:{digest:x}"
            )),
        }
    }

    pub fn rejection_kind(&self) -> BridgeSubscriptionPreviewWorkTraceRejectionKind {
        self.rejection_kind
    }

    pub fn rejection_context(&self) -> &str {
        self.rejection_context.as_ref()
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

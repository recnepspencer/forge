use std::sync::Arc;

use sha2::{Digest, Sha256};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeFailureEvidenceAttachment {
    family: Arc<str>,
    identity: Arc<str>,
    digest: Arc<str>,
    detail: Option<Arc<str>>,
}

impl BridgeFailureEvidenceAttachment {
    pub fn reference(
        family: impl Into<Arc<str>>,
        identity: impl Into<Arc<str>>,
        digest: impl Into<Arc<str>>,
    ) -> Self {
        Self {
            family: family.into(),
            identity: identity.into(),
            digest: digest.into(),
            detail: None,
        }
    }

    pub fn with_detail(mut self, detail: impl Into<Arc<str>>) -> Self {
        self.detail = Some(detail.into());
        self
    }

    pub(crate) fn synthetic(
        family: impl Into<Arc<str>>,
        identity: impl Into<Arc<str>>,
        detail: impl Into<Arc<str>>,
    ) -> Self {
        let family = family.into();
        let identity = identity.into();
        let detail = detail.into();
        let canonical_basis = format!(
            "{}|{}|{}",
            family.as_ref(),
            identity.as_ref(),
            detail.as_ref()
        );
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self::reference(
            family,
            identity,
            Arc::<str>::from(format!(
                "bridge-failure-evidence-synthetic:sha256:{digest:x}"
            )),
        )
        .with_detail(detail)
    }

    pub fn family(&self) -> &str {
        self.family.as_ref()
    }

    pub fn identity(&self) -> &str {
        self.identity.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }

    pub fn detail(&self) -> Option<&str> {
        self.detail.as_deref()
    }

    pub(crate) fn canonical_fragment(&self) -> String {
        format!(
            "{}|{}|{}",
            self.family.as_ref(),
            self.identity.as_ref(),
            self.digest.as_ref()
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeFailureEvidenceAttachmentSet {
    attachments: Arc<[BridgeFailureEvidenceAttachment]>,
    digest: Arc<str>,
}

impl BridgeFailureEvidenceAttachmentSet {
    pub fn new(attachments: Vec<BridgeFailureEvidenceAttachment>) -> Self {
        let canonical_basis = attachments
            .iter()
            .map(BridgeFailureEvidenceAttachment::canonical_fragment)
            .collect::<Vec<_>>()
            .join(",");
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            attachments: attachments.into(),
            digest: Arc::from(format!(
                "bridge-failure-evidence-attachments:sha256:{digest:x}"
            )),
        }
    }

    pub fn attachments(&self) -> &[BridgeFailureEvidenceAttachment] {
        self.attachments.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

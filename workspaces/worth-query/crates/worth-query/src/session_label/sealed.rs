use super::namespace::WorthQuerySessionNamespace;
use super::segment::WorthQuerySessionLabelSegment;
use crate::evidence_identity::WorthQueryEvidenceIdentity;

pub(crate) struct SealedWorthQuerySessionLabel {
    pub(crate) namespace: WorthQuerySessionNamespace,
    pub(crate) name_segments: Vec<WorthQuerySessionLabelSegment>,
    pub(crate) display: String,
    pub(crate) identity_digest: WorthQueryEvidenceIdentity,
}

impl SealedWorthQuerySessionLabel {
    pub(crate) fn new(
        namespace: WorthQuerySessionNamespace,
        name_segments: Vec<WorthQuerySessionLabelSegment>,
        display: String,
        identity_digest: WorthQueryEvidenceIdentity,
    ) -> Self {
        Self {
            namespace,
            name_segments,
            display,
            identity_digest,
        }
    }
}

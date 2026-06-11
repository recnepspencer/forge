use super::namespace::ForgeQuerySessionNamespace;
use super::segment::ForgeQuerySessionLabelSegment;
use crate::evidence_identity::ForgeQueryEvidenceIdentity;

pub(crate) struct SealedForgeQuerySessionLabel {
    pub(crate) namespace: ForgeQuerySessionNamespace,
    pub(crate) name_segments: Vec<ForgeQuerySessionLabelSegment>,
    pub(crate) display: String,
    pub(crate) identity_digest: ForgeQueryEvidenceIdentity,
}

impl SealedForgeQuerySessionLabel {
    pub(crate) fn new(
        namespace: ForgeQuerySessionNamespace,
        name_segments: Vec<ForgeQuerySessionLabelSegment>,
        display: String,
        identity_digest: ForgeQueryEvidenceIdentity,
    ) -> Self {
        Self {
            namespace,
            name_segments,
            display,
            identity_digest,
        }
    }
}

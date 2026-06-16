use super::namespace::ForgeQuerySessionNamespace;
use super::segment::ForgeQuerySessionLabelSegment;
use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};

pub(crate) fn derive_session_label_identity(
    namespace: &ForgeQuerySessionNamespace,
    name_segments: &[ForgeQuerySessionLabelSegment],
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::SessionLabelIdentity)
        .field_value(
            ForgeQueryEvidenceTag::new("session_label_namespace"),
            namespace.as_str(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("session_label_name_segment_count"),
            name_segments.len(),
        )
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("session_label_name_segments"),
            name_segments
                .iter()
                .map(ForgeQuerySessionLabelSegment::as_str),
        )
        .seal()
}

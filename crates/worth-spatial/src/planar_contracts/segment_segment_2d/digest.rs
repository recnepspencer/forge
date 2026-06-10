use worth_primitives::{truth_digest_parts, TruthDigestScope};

pub(crate) fn certified_segment_segment_2d_digest(parts: &[String]) -> String {
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, parts)
}

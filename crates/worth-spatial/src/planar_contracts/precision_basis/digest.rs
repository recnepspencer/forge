use worth_primitives::{truth_digest_parts, TruthDigestScope};

pub(crate) fn planar_precision_digest(parts: &[String]) -> String {
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, parts)
}

use worth_primitives::{truth_digest_parts, TruthDigestScope};

pub(crate) fn project_point_to_certified_plane_2d_digest(parts: &[String]) -> String {
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, parts)
}

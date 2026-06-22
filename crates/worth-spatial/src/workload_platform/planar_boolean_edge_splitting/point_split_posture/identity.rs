use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::posture::PosturedPointSplitCandidate;

pub(crate) fn postured_candidate_identity(
    admitted_point_candidate_identity: &str,
    posture_name: &str,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "planar-boolean-postured-point-split-candidate".to_string(),
            format!("candidate:{admitted_point_candidate_identity}"),
            format!("posture:{posture_name}"),
        ],
    )
}

pub(crate) fn posture_set_identity(
    point_candidate_set_identity: &str,
    candidates: &[PosturedPointSplitCandidate],
) -> String {
    let mut parts = vec![
        "planar-boolean-point-split-posture-set".to_string(),
        format!("point-candidate-set:{point_candidate_set_identity}"),
    ];
    parts.extend(
        candidates
            .iter()
            .map(|candidate| format!("postured:{}", candidate.postured_candidate_identity())),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

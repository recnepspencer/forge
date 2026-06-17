use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::candidate::PlanarBooleanPointSplitCandidate;

pub(crate) fn point_candidate_identity(
    participation_index_identity: &str,
    point_event_identity: &str,
    carrier_identity: &str,
    parameter_fact_identity: &str,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "planar-boolean-point-split-candidate".to_string(),
            format!("participation-index:{participation_index_identity}"),
            format!("point-event:{point_event_identity}"),
            format!("carrier:{carrier_identity}"),
            format!("parameter-fact:{parameter_fact_identity}"),
        ],
    )
}

pub(crate) fn point_candidate_set_identity(
    participation_index_identity: &str,
    candidates: &[PlanarBooleanPointSplitCandidate],
) -> String {
    let mut parts = vec![
        "planar-boolean-point-split-candidate-set".to_string(),
        format!("participation-index:{participation_index_identity}"),
    ];
    parts.extend(
        candidates
            .iter()
            .map(|candidate| format!("candidate:{}", candidate.candidate_identity())),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::candidate::PlanarBooleanIntervalSplitCandidate;

pub(crate) fn interval_candidate_identity(
    participation_index_identity: &str,
    interval_event_identity: &str,
    carrier_identity: &str,
    source_interval_identity: &str,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "planar-boolean-interval-split-candidate".to_string(),
            format!("participation-index:{participation_index_identity}"),
            format!("interval-event:{interval_event_identity}"),
            format!("carrier:{carrier_identity}"),
            format!("source-interval:{source_interval_identity}"),
        ],
    )
}

pub(crate) fn interval_candidate_set_identity(
    participation_index_identity: &str,
    candidates: &[PlanarBooleanIntervalSplitCandidate],
) -> String {
    let mut parts = vec![
        "planar-boolean-interval-split-candidate-set".to_string(),
        format!("participation-index:{participation_index_identity}"),
    ];
    parts.extend(
        candidates
            .iter()
            .map(|candidate| format!("candidate:{}", candidate.candidate_identity())),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

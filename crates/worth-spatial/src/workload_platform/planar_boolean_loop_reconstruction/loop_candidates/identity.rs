use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::row::{PlanarBooleanDeniedLoopCandidate, PlanarBooleanLoopCandidate};

pub(crate) fn loop_candidate_identity(
    request_identity: &str,
    walk_outcome_identity: &str,
    source_loop_identity: &str,
    fragment_identities: &[String],
) -> String {
    let mut parts = vec![
        "planar-boolean-loop-candidate".to_string(),
        format!("request:{request_identity}"),
        format!("walk-outcome:{walk_outcome_identity}"),
        format!("source-loop:{source_loop_identity}"),
    ];
    parts.extend(
        fragment_identities
            .iter()
            .map(|identity| format!("fragment:{identity}")),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

pub(crate) fn denied_loop_candidate_identity(
    request_identity: &str,
    walk_outcome_identity: &str,
    source_loop_identity: &str,
    kind_name: &str,
    fragment_identities: &[String],
) -> String {
    let mut parts = vec![
        "planar-boolean-denied-loop-candidate".to_string(),
        format!("request:{request_identity}"),
        format!("walk-outcome:{walk_outcome_identity}"),
        format!("source-loop:{source_loop_identity}"),
        format!("kind:{kind_name}"),
    ];
    parts.extend(
        fragment_identities
            .iter()
            .map(|identity| format!("fragment:{identity}")),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

pub(crate) fn loop_candidate_set_identity(
    request_identity: &str,
    walk_outcome_set_identity: &str,
    rows: &[PlanarBooleanLoopCandidate],
) -> String {
    let mut parts = vec![
        "planar-boolean-loop-candidate-set".to_string(),
        format!("request:{request_identity}"),
        format!("walk-outcome-set:{walk_outcome_set_identity}"),
    ];
    parts.extend(
        rows.iter()
            .map(|row| format!("loop-candidate:{}", row.loop_candidate_identity())),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

pub(crate) fn denied_loop_candidate_set_identity(
    request_identity: &str,
    walk_outcome_set_identity: &str,
    rows: &[PlanarBooleanDeniedLoopCandidate],
) -> String {
    let mut parts = vec![
        "planar-boolean-denied-loop-candidate-set".to_string(),
        format!("request:{request_identity}"),
        format!("walk-outcome-set:{walk_outcome_set_identity}"),
    ];
    parts.extend(rows.iter().map(|row| {
        format!(
            "denied-loop-candidate:{}",
            row.denied_loop_candidate_identity()
        )
    }));
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

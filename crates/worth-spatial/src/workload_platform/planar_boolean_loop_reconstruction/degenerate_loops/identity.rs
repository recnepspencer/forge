use crate::workload_platform::planar_boolean_loop_reconstruction::PlanarBooleanDegenerateLoopOutcome;
use worth_primitives::{truth_digest_parts, TruthDigestScope};

pub(super) fn degenerate_loop_outcome_identity(
    request_identity: &str,
    loop_identity: &str,
    outcome_kind: &str,
    fragment_identities: &[String],
    split_vertex_identities: &[String],
) -> String {
    let mut parts = vec![
        "planar-boolean-degenerate-loop-outcome".to_string(),
        format!("request:{request_identity}"),
        format!("loop:{loop_identity}"),
        format!("kind:{outcome_kind}"),
    ];
    parts.extend(
        fragment_identities
            .iter()
            .map(|identity| format!("fragment:{identity}")),
    );
    parts.extend(
        split_vertex_identities
            .iter()
            .map(|identity| format!("split-vertex:{identity}")),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

pub(super) fn degenerate_loop_outcome_set_identity(
    request_identity: &str,
    rows: &[PlanarBooleanDegenerateLoopOutcome],
) -> String {
    let mut parts = vec![
        "planar-boolean-degenerate-loop-outcome-set".to_string(),
        format!("request:{request_identity}"),
    ];
    parts.extend(
        rows.iter()
            .map(|row| format!("outcome:{}", row.degenerate_loop_outcome_identity())),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

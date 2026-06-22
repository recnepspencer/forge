use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::row::PlanarBooleanWalkOutcomeRow;

pub(crate) fn walk_outcome_identity(
    request_identity: &str,
    continuation_index_identity: &str,
    source_loop_identity: &str,
    kind_name: &str,
    fragment_identities: &[String],
    split_vertex_identities: &[String],
    continuation_identities: &[String],
) -> String {
    let mut parts = vec![
        "planar-boolean-walk-outcome".to_string(),
        format!("request:{request_identity}"),
        format!("continuation-index:{continuation_index_identity}"),
        format!("source-loop:{source_loop_identity}"),
        format!("kind:{kind_name}"),
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
    parts.extend(
        continuation_identities
            .iter()
            .map(|identity| format!("continuation:{identity}")),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

pub(crate) fn walk_outcome_set_identity(
    request_identity: &str,
    continuation_index_identity: &str,
    rows: &[PlanarBooleanWalkOutcomeRow],
) -> String {
    let mut parts = vec![
        "planar-boolean-walk-outcome-set".to_string(),
        format!("request:{request_identity}"),
        format!("continuation-index:{continuation_index_identity}"),
    ];
    parts.extend(
        rows.iter()
            .map(|row| format!("walk-outcome:{}", row.walk_outcome_identity())),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

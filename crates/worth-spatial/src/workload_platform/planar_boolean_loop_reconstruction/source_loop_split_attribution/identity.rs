use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::row::PlanarBooleanSourceLoopSplitAttributionRow;

pub(crate) fn split_attribution_row_identity(
    request_identity: &str,
    source_loop_identity: &str,
    island_identities: &[String],
) -> String {
    let mut parts = vec![
        "planar-boolean-source-loop-split-attribution-row".to_string(),
        format!("request:{request_identity}"),
        format!("source-loop:{source_loop_identity}"),
    ];
    parts.extend(
        island_identities
            .iter()
            .map(|identity| format!("island:{identity}")),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

pub(crate) fn split_attribution_identity(
    request_identity: &str,
    rows: &[PlanarBooleanSourceLoopSplitAttributionRow],
) -> String {
    let mut parts = vec![
        "planar-boolean-source-loop-split-attribution".to_string(),
        format!("request:{request_identity}"),
    ];
    parts.extend(
        rows.iter()
            .map(|row| format!("attribution-row:{}", row.attribution_identity())),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

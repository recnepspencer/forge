use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::row::PlanarBooleanLoopIslandPartitionRow;

pub(crate) fn island_partition_row_identity(
    request_identity: &str,
    source_loop_identity: &str,
    member_loop_identities: &[String],
) -> String {
    let mut parts = vec![
        "planar-boolean-loop-island-partition-row".to_string(),
        format!("request:{request_identity}"),
        format!("source-loop:{source_loop_identity}"),
    ];
    parts.extend(
        member_loop_identities
            .iter()
            .map(|identity| format!("member-loop:{identity}")),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

pub(crate) fn island_partition_identity(
    request_identity: &str,
    rows: &[PlanarBooleanLoopIslandPartitionRow],
) -> String {
    let mut parts = vec![
        "planar-boolean-loop-island-partition".to_string(),
        format!("request:{request_identity}"),
    ];
    parts.extend(
        rows.iter()
            .map(|row| format!("island-row:{}", row.island_identity())),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

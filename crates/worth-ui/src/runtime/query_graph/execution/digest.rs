use super::WorthUiQueryGraphExecutionRow;

pub(super) fn execution_digest(
    touch_digest: &str,
    world_digest: &str,
    proof_digest: &str,
    rows: &[WorthUiQueryGraphExecutionRow],
) -> u64 {
    let mut digest = fold(0xcbf2_9ce4_8422_2325, b"worth-ui-query-graph-execution");
    digest = fold(digest, touch_digest.as_bytes());
    digest = fold(digest, world_digest.as_bytes());
    digest = fold(digest, proof_digest.as_bytes());
    for row in rows {
        digest = fold(digest, row.semantic().as_str().as_bytes());
        digest = fold(digest, row.canonical_kind().as_str().as_bytes());
        digest = fold(digest, row.support_lane().as_bytes());
        digest = fold(digest, row.support_status().as_bytes());
        digest = fold(digest, row.execution_status().as_bytes());
        digest = fold(digest, row.row_digest().as_bytes());
    }
    digest
}

fn fold(mut digest: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        digest ^= u64::from(*byte);
        digest = digest.wrapping_mul(0x0000_0100_0000_01b3);
    }
    digest
}

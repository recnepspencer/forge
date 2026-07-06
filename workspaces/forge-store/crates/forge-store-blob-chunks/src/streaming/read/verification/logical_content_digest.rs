use forge_store_contracts::StableDigest;

use crate::{BlobChunkByteRange, BlobChunkOrdinal, LogicalContentDigest};

pub(crate) fn accumulator_seed(lane: &str) -> u64 {
    accumulate_bytes(0xcbf2_9ce4_8422_2325, lane.as_bytes())
}

pub(crate) fn accumulate_bytes(mut hash: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

pub(crate) fn accumulate_chunk_bytes(basis: u64, payload_bytes: &[u8]) -> u64 {
    accumulate_bytes(basis, payload_bytes)
}

pub(crate) fn finalize_logical_content_digest(
    basis: u64,
    total_bytes: u64,
    chunk_count: u64,
) -> LogicalContentDigest {
    LogicalContentDigest::from_declared_digest(accumulated_digest(
        "logical-content",
        basis,
        total_bytes,
        chunk_count,
    ))
}

fn accumulated_digest(
    lane: &str,
    accumulator: u64,
    total_bytes: u64,
    chunk_count: u64,
) -> StableDigest {
    let evidence = format!("{accumulator:016x}:{total_bytes}:{chunk_count}");
    stable_digest_for_read(
        lane,
        "s7.sequence.v1",
        BlobChunkOrdinal::first(),
        BlobChunkByteRange::new(chunk_count, evidence.len() as u64)
            .expect("finalized read sequence has nonempty evidence"),
        &evidence,
    )
}

fn stable_digest_for_read(
    domain: &str,
    rule: &str,
    ordinal: BlobChunkOrdinal,
    range: BlobChunkByteRange,
    bytes: &str,
) -> StableDigest {
    let mut hash = accumulator_seed(domain);
    hash = accumulate_bytes(hash, rule.as_bytes());
    hash = accumulate_u64(hash, ordinal.get());
    hash = accumulate_u64(hash, range.start());
    hash = accumulate_u64(hash, range.len());
    hash = accumulate_bytes(hash, bytes.as_bytes());
    StableDigest::new(format!("s7:{domain}:{hash:016x}",))
        .expect("blob read stable digest should be nonempty")
}

fn accumulate_u64(hash: u64, value: u64) -> u64 {
    accumulate_bytes(hash, &value.to_le_bytes())
}
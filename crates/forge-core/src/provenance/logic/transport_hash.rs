//! Deterministic transport hash functions for snapshot segment identifiers.

use crate::provenance::data::SnapshotHandleRef;

/// Deterministic transport hash for a directed segment between two snapshot handles.
///
/// Intended as a compact join key / transport identifier for certifier and audit
/// records. This is not a replacement for rich provenance fields.
pub fn hash_directed_snapshot_segment_transport(
    start: SnapshotHandleRef,
    end: SnapshotHandleRef,
) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for word in [
        start.kind as u64,
        start.index as u64,
        start.generation as u64,
        end.kind as u64,
        end.index as u64,
        end.generation as u64,
    ] {
        h ^= word;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

/// Deterministic transport hash for an undirected segment between two snapshot handles.
///
/// Endpoint order is canonicalized so `(a,b)` and `(b,a)` produce the same hash.
pub fn hash_undirected_snapshot_segment_transport(
    a: SnapshotHandleRef,
    b: SnapshotHandleRef,
) -> u64 {
    let (start, end) =
        if (a.kind as u8, a.index, a.generation) <= (b.kind as u8, b.index, b.generation) {
            (a, b)
        } else {
            (b, a)
        };
    hash_directed_snapshot_segment_transport(start, end)
}

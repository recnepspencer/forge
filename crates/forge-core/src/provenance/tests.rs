use super::*;
use crate::tracing::EntityKind;

#[test]
fn boundary_segment_provenance_json_round_trip() {
    let start = SnapshotHandleRef::new(EntityKind::Vertex, 12, 1);
    let end = SnapshotHandleRef::new(EntityKind::Vertex, 18, 2);
    let mut p = BoundarySegmentProvenance::new(start, end);
    p.source_halfedge_snapshot = Some(SnapshotHandleRef::new(EntityKind::HalfEdge, 7, 0));
    p.source_edge_snapshot = Some(SnapshotHandleRef::new(EntityKind::Edge, 4, 0));
    p.source_face_snapshot = Some(SnapshotHandleRef::new(EntityKind::Face, 2, 3));

    let json = serde_json::to_string(&p).expect("serialize provenance");
    let restored: BoundarySegmentProvenance =
        serde_json::from_str(&json).expect("deserialize provenance");

    assert_eq!(restored, p);
    assert_eq!(restored.validate(), Ok(()));
}

#[test]
fn transport_hash_changes_on_generation_change() {
    let a0 = SnapshotHandleRef::new(EntityKind::Vertex, 12, 0);
    let a1 = SnapshotHandleRef::new(EntityKind::Vertex, 12, 1);
    let b0 = SnapshotHandleRef::new(EntityKind::Vertex, 34, 0);

    let h0 = hash_directed_snapshot_segment_transport(a0, b0);
    let h1 = hash_directed_snapshot_segment_transport(a1, b0);

    assert_ne!(h0, h1, "generation reuse must change transport hash");
}

#[test]
fn transport_hash_changes_when_only_high_generation_bits_change() {
    let a_low = SnapshotHandleRef::new(EntityKind::Vertex, 12, 0x0000_00aa);
    let a_high = SnapshotHandleRef::new(EntityKind::Vertex, 12, 0x0100_00aa);
    let b = SnapshotHandleRef::new(EntityKind::Vertex, 34, 0);

    let h_low = hash_directed_snapshot_segment_transport(a_low, b);
    let h_high = hash_directed_snapshot_segment_transport(a_high, b);

    assert_ne!(
        h_low, h_high,
        "high generation bits must affect transport hash (no truncation)"
    );
}

#[test]
fn transport_hash_depends_on_both_endpoints_and_direction() {
    let a = SnapshotHandleRef::new(EntityKind::Vertex, 10, 2);
    let b = SnapshotHandleRef::new(EntityKind::Vertex, 20, 3);
    let c = SnapshotHandleRef::new(EntityKind::Vertex, 21, 3);

    let ab = hash_directed_snapshot_segment_transport(a, b);
    let ac = hash_directed_snapshot_segment_transport(a, c);
    let ba = hash_directed_snapshot_segment_transport(b, a);

    assert_ne!(ab, ac);
    assert_ne!(ab, ba);
}

#[test]
fn undirected_transport_hash_is_order_invariant() {
    let a = SnapshotHandleRef::new(EntityKind::Vertex, 10, 2);
    let b = SnapshotHandleRef::new(EntityKind::Vertex, 20, 3);

    assert_eq!(
        hash_undirected_snapshot_segment_transport(a, b),
        hash_undirected_snapshot_segment_transport(b, a)
    );
}

#[test]
fn boundary_segment_provenance_validate_detects_tampered_hash_mode_mismatch() {
    // Use reverse-ordered endpoints so directed and undirected hashes differ.
    let start = SnapshotHandleRef::new(EntityKind::Vertex, 9, 0);
    let end = SnapshotHandleRef::new(EntityKind::Vertex, 2, 0);
    let mut p = BoundarySegmentProvenance::new(start, end);
    p.directed = false; // tamper without recomputing transport hash

    assert!(matches!(
        p.validate(),
        Err(ProvenanceValidationError::TransportHashMismatch {
            directed: false,
            ..
        })
    ));
}

#[test]
fn boundary_segment_provenance_validate_rejects_wrong_source_handle_kind() {
    let start = SnapshotHandleRef::new(EntityKind::Vertex, 12, 1);
    let end = SnapshotHandleRef::new(EntityKind::Vertex, 18, 2);
    let mut p = BoundarySegmentProvenance::new(start, end);
    p.source_edge_snapshot = Some(SnapshotHandleRef::new(EntityKind::Face, 4, 0));

    assert!(matches!(
        p.validate(),
        Err(ProvenanceValidationError::InvalidSourceKind {
            field: "source_edge_snapshot",
            expected: EntityKind::Edge,
            actual: EntityKind::Face,
        })
    ));
}

#[test]
fn snapshot_handle_ref_packs_generational_identity() {
    let h = SnapshotHandleRef::new(EntityKind::Face, 42, 7);
    assert_eq!(h.packed_generational(), ((7u64) << 32) | 42u64);
}

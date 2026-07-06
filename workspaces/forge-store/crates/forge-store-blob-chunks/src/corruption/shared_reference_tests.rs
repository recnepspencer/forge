use crate::corruption::test_support::{
    publish_shared_scope_generation, registered_share_reference_for_first_chunk,
};
use crate::test_support::frontier_for;
use crate::{
    BlobChunkOrdinal, BlobCorruptedChunkLocalization, BlobCorruptionDenial,
    BlobCorruptionPlacementClass, BlobCorruptionReferenceEdge, BlobCorruptionReferenceEdges,
    BlobCorruptionReferenceSharingScope,
};

#[test]
fn admitted_shared_dedupe_reference_edges_localize_all_affected_edges() {
    let scope_case = "phase11.shared.scope";
    let bytes = b"aaaabbbb";
    let (published, visible) =
        publish_shared_scope_generation(scope_case, "phase11.shared.source", 1, bytes, 4);
    let (affected_published, _) =
        publish_shared_scope_generation(scope_case, "phase11.shared.affected", 2, bytes, 4);
    let source_frontier = frontier_for(scope_case, bytes, 4);
    let affected_frontier = frontier_for(scope_case, bytes, 4);
    let registered_reference = registered_share_reference_for_first_chunk(scope_case, b"aaaa");

    let localized_edge =
        BlobCorruptionReferenceEdge::from_reachability_staging_identity(published.staging_identity());
    let affected_edge = BlobCorruptionReferenceEdge::from_admitted_shared_dedupe_reference(
        published.staging_identity(),
        affected_published.staging_identity(),
        &affected_frontier,
        BlobChunkOrdinal::first(),
        &registered_reference,
    )
    .expect("admitted dedupe sharing should mint affected edge");
    let edges = BlobCorruptionReferenceEdges::from_admitted_edges(&[localized_edge, affected_edge])
        .expect("localized and affected shared edges should bind");

    let localized = BlobCorruptedChunkLocalization::from_read_corruption(
        visible,
        source_frontier,
        BlobChunkOrdinal::first(),
        BlobCorruptionPlacementClass::LocalPhysical,
        edges,
    )
    .expect("shared dedupe corruption should localize all affected edges");

    assert_eq!(
        localized.sharing_scope(),
        BlobCorruptionReferenceSharingScope::SharedReferenceEdges
    );
    assert_eq!(localized.reference_edges().edge_count(), 2);
    assert_eq!(localized.counters().affected_reference_edges(), 2);
}

#[test]
fn shared_dedupe_edge_requires_affected_frontier_to_contain_the_shared_chunk() {
    let scope_case = "phase11.shared.mismatch";
    let (published, _) =
        publish_shared_scope_generation(scope_case, "phase11.shared.mismatch.source", 1, b"aaaabbbb", 4);
    let (affected_published, _) =
        publish_shared_scope_generation(scope_case, "phase11.shared.mismatch.affected", 2, b"ccccdddd", 4);
    let affected_frontier = frontier_for(scope_case, b"ccccdddd", 4);
    let registered_reference = registered_share_reference_for_first_chunk(scope_case, b"aaaa");

    let denied = BlobCorruptionReferenceEdge::from_admitted_shared_dedupe_reference(
        published.staging_identity(),
        affected_published.staging_identity(),
        &affected_frontier,
        BlobChunkOrdinal::first(),
        &registered_reference,
    )
    .expect_err("copied dedupe claim must not bind an unrelated affected frontier");

    assert!(matches!(
        denied,
        BlobCorruptionDenial::AffectedReferenceEdgeMismatch { .. }
    ));
}
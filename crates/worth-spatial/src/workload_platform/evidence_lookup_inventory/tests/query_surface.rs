use super::super::{
    classify_evidence_lookup_query_surface, EvidenceLookupAuthorityKind,
    EvidenceLookupInventoryErrorKind, EvidenceLookupQuerySurface,
    EvidenceLookupQuerySurfaceContext,
};

#[test]
fn query_surface_classifier_rejects_category_swaps() {
    let non_query = classify_evidence_lookup_query_surface(
        EvidenceLookupAuthorityKind::StageLocalNearbyLookup,
        EvidenceLookupQuerySurfaceContext::ConsumerKitProof,
    )
    .expect_err("ordinary lookup authority cannot claim a Query surface");
    assert_eq!(
        non_query.kind(),
        EvidenceLookupInventoryErrorKind::QuerySurfaceCannotMintLookupAuthority
    );

    let missing_query = classify_evidence_lookup_query_surface(
        EvidenceLookupAuthorityKind::QueryLookingLocalProof,
        EvidenceLookupQuerySurfaceContext::NotQuery,
    )
    .expect_err("query-looking proof must name exact Query surface");
    assert_eq!(
        missing_query.kind(),
        EvidenceLookupInventoryErrorKind::QuerySurfaceRequired
    );

    let surface = classify_evidence_lookup_query_surface(
        EvidenceLookupAuthorityKind::QueryLookingLocalProof,
        EvidenceLookupQuerySurfaceContext::TypedArtifactIdentity,
    )
    .expect("query-looking proof can classify a specific adjacent Query surface");
    assert_eq!(surface, EvidenceLookupQuerySurface::TypedArtifactIdentity);
}

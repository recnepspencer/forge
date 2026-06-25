use forge_query::facade::ForgeQueryDeclarationFamilyMarker;
use worth_spatial::facade::planar_segment_segment::{
    CertifiedProjectedSegment2D, CertifiedSegmentSegment2D,
    CertifiedSegmentSegment2DDeclarationFamily, CertifiedSegmentSegment2DQueryDomain,
    CertifiedSegmentSegment2DQueryWorld, SegmentContactPolicy,
};

#[test]
fn spatial_public_facade_exports_readable_segment_classification_surface() {
    let _: Option<CertifiedProjectedSegment2D> = None;
    let _: Option<CertifiedSegmentSegment2D> = None;
    let _: CertifiedSegmentSegment2DDeclarationFamily = CertifiedSegmentSegment2DDeclarationFamily;
    let _: CertifiedSegmentSegment2DQueryDomain = CertifiedSegmentSegment2DQueryDomain;
    let _: CertifiedSegmentSegment2DQueryWorld = CertifiedSegmentSegment2DQueryWorld::new("public");
    let _: SegmentContactPolicy = SegmentContactPolicy::CertifyContactsDenyImprintRequired;
    let _: SegmentContactPolicy = SegmentContactPolicy::RequireImprintForCollinearOverlap;
}

#[test]
fn certified_segment_segment_family_is_query_native_and_retained() {
    let aspect_contract = CertifiedSegmentSegment2DDeclarationFamily::aspect_contract();

    assert_eq!(
        CertifiedSegmentSegment2DDeclarationFamily::semantic_family_key(),
        "CertifiedSegmentSegment2D"
    );
    assert_eq!(
        CertifiedSegmentSegment2DDeclarationFamily::route_contract().reason(),
        "the declaration lowers through one relational route"
    );
    assert!(aspect_contract
        .required()
        .contains(&crate::query_contract_helpers::aspect_field_key(
            "geometry.segment_segment_2d.endpoint.0.projection_fact"
        )));
    assert!(aspect_contract.preserved().contains(
        &crate::query_contract_helpers::aspect_field_key(
            "geometry.segment_segment_2d.classification"
        )
    ));
}

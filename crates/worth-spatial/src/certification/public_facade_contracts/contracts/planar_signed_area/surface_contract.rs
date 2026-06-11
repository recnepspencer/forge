use forge_query::facade::ForgeQueryDeclarationFamilyMarker;
use worth_spatial::facade::planar_signed_area::{
    AreaDegeneracyClass, AreaDegeneracyPolicy, CertifiedSignedArea2D,
    CertifiedSignedArea2DDeclarationFamily, CertifiedSignedArea2DQueryDomain,
    CertifiedSignedArea2DQueryWorld, SignedAreaOrientation,
};
use worth_spatial::facade::planar_winding::{CertifiedPolygonWinding2D, CertifiedProjectedLoop2D};

use super::proof_fixture::{
    loop_points, precision_and_frame, signed_area_contracts, topology_basis, winding_contracts,
};

#[test]
fn spatial_public_facade_exports_readable_signed_area_surface() {
    let _: Option<CertifiedSignedArea2D> = None;
    let _: CertifiedSignedArea2DDeclarationFamily = CertifiedSignedArea2DDeclarationFamily;
    let _: CertifiedSignedArea2DQueryDomain = CertifiedSignedArea2DQueryDomain;
    let _: CertifiedSignedArea2DQueryWorld = CertifiedSignedArea2DQueryWorld::new("public");
    let _: AreaDegeneracyPolicy = AreaDegeneracyPolicy::ClassifyWithoutRepair;
    let _: AreaDegeneracyClass = AreaDegeneracyClass::WellFormed;
    let _: SignedAreaOrientation = SignedAreaOrientation::CounterClockwise;
}

#[test]
fn certified_signed_area_family_is_query_native_and_retained() {
    let aspect_contract = CertifiedSignedArea2DDeclarationFamily::aspect_contract();

    assert_eq!(
        CertifiedSignedArea2DDeclarationFamily::semantic_family_key(),
        "CertifiedSignedArea2D"
    );
    assert!(aspect_contract
        .required()
        .contains(&"geometry.signed_area_2d.winding_fact".to_string()));
    assert!(aspect_contract
        .required()
        .contains(&"geometry.signed_area_2d.precision_fact".to_string()));
    assert!(aspect_contract
        .preserved()
        .contains(&"geometry.signed_area_2d.degeneracy".to_string()));
}

#[test]
fn signed_area_dx_reads_as_receipt_consuming_plan() {
    let world = "signed-area-dx";
    let (precision, frame) = precision_and_frame(world, "movement:stable");
    let outer = CertifiedProjectedLoop2D::from_projected_vertices(
        "loop:area-outer",
        topology_basis("loop:area-outer"),
        loop_points(
            world,
            &frame,
            "outer",
            &[[0.0, 0.0], [4.0e-9, 0.0], [4.0e-9, 4.0e-9], [0.0, 4.0e-9]],
        ),
    )
    .expect("outer loop");
    let winding = CertifiedPolygonWinding2D::certify(outer)
        .within_planar_neighborhood("topology:signed-area-face")
        .compile(&winding_contracts(world))
        .expect("winding plan")
        .certify()
        .expect("winding receipt");

    let area_contracts = signed_area_contracts(world);
    let plan = CertifiedSignedArea2D::measure_face(winding)
        .using_precision_basis(precision)
        .classifying_degeneracy(AreaDegeneracyPolicy::ClassifyWithoutRepair)
        .compile(&area_contracts)
        .expect("signed-area plan");

    assert_eq!(plan.loop_edges_walked(), 4);
    assert_eq!(plan.local_scale_comparisons_required(), 3);

    let receipt = plan.certify().expect("signed-area receipt");
    assert_eq!(
        receipt.orientation(),
        SignedAreaOrientation::CounterClockwise
    );
    assert_eq!(receipt.degeneracy(), AreaDegeneracyClass::WellFormed);
    assert!(receipt.used_local_frame_scale());
    assert_eq!(receipt.counters().area_terms_evaluated(), 4);
}

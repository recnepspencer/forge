use worth_spatial::facade::planar_signed_area::{
    AreaDegeneracyClass, CertifiedSignedArea2D, CertifiedSignedArea2DDenialKind,
    SignedAreaDegeneracyCause,
};
use worth_spatial::facade::planar_winding::{
    CertifiedPolygonWinding2D, CertifiedPolygonWinding2DDenialKind,
    CertifiedPolygonWinding2DFactError, CertifiedProjectedLoop2D,
};

use super::proof_fixture::{
    loop_points, precision_and_frame, signed_area_contracts, topology_basis, winding_contracts,
};

#[test]
fn certified_signed_area_uses_local_frame_scale_for_1e12_world_1e_minus_9_feature() {
    let world = "signed-area-scale";
    let (precision, frame) = precision_and_frame(world, "movement:stable");
    let winding = loop_receipt(
        world,
        &frame,
        "loop:scale-outer",
        &[[0.0, 0.0], [8.0e-9, 0.0], [8.0e-9, 8.0e-9], [0.0, 8.0e-9]],
    );
    let receipt = CertifiedSignedArea2D::measure_face(winding)
        .using_precision_basis(precision)
        .compile(&signed_area_contracts(world))
        .expect("plan")
        .certify()
        .expect("signed area");

    assert_eq!(receipt.degeneracy(), AreaDegeneracyClass::WellFormed);
    assert!(receipt.used_local_frame_scale());
    assert_eq!(receipt.counters().local_scale_comparisons(), 3);
    assert_eq!(receipt.counters().loop_edges_walked(), 4);
}

#[test]
fn certified_signed_area_classifies_sliver_without_repair() {
    let world = "signed-area-degeneracy";
    let (precision, frame) = precision_and_frame(world, "movement:stable");
    let winding = loop_receipt(
        world,
        &frame,
        "loop:thin-outer",
        &[
            [0.0, 0.0],
            [10.0e-9, 0.0],
            [10.0e-9, 1.0e-20],
            [0.0, 1.0e-20],
        ],
    );
    let sliver = CertifiedSignedArea2D::measure_face(winding)
        .using_precision_basis(precision)
        .compile(&signed_area_contracts(world))
        .expect("area plan")
        .certify()
        .expect("area receipt");

    assert_eq!(sliver.degeneracy(), AreaDegeneracyClass::Sliver);
    assert_eq!(sliver.repair_action(), None);
    assert_area_sum_cause(sliver.basis().localized_cause(), "loop:thin-outer");
}

#[test]
fn certified_signed_area_denies_zero_sliver_and_needle_cases_with_localized_cause() {
    let world = "signed-area-zero-sliver-needle";
    let (precision, frame) = precision_and_frame(world, "movement:stable");

    let zero = CertifiedProjectedLoop2D::from_projected_vertices(
        "loop:zero-area",
        topology_basis("loop:zero-area"),
        loop_points(
            world,
            &frame,
            "zero-area",
            &[[0.0, 0.0], [1.0e-9, 0.0], [2.0e-9, 0.0], [3.0e-9, 0.0]],
        ),
    )
    .expect("zero area loop");
    let zero_error = CertifiedPolygonWinding2D::certify(zero)
        .within_planar_neighborhood("topology:zero-area")
        .compile(&winding_contracts(world))
        .expect("zero winding plan")
        .certify()
        .expect_err("zero area must not mint an area-ready winding receipt");
    assert_winding_basis_denial(
        zero_error,
        CertifiedPolygonWinding2DDenialKind::SelfIntersectionOrAmbiguousTouch,
    );

    let needle = CertifiedProjectedLoop2D::from_projected_vertices(
        "loop:needle",
        topology_basis("loop:needle"),
        loop_points(
            world,
            &frame,
            "needle",
            &[[0.0, 0.0], [2.0e-9, 0.0], [2.0e-9, 0.0], [0.0, 2.0e-9]],
        ),
    )
    .expect("needle loop");
    let needle_denial = match CertifiedPolygonWinding2D::certify(needle)
        .within_planar_neighborhood("topology:needle")
        .compile(&winding_contracts(world))
    {
        Ok(_) => panic!("needle duplicate edge must deny before signed area"),
        Err(denial) => denial,
    };
    assert_eq!(
        needle_denial.kind(),
        CertifiedPolygonWinding2DDenialKind::DuplicateVertex
    );

    let sliver_winding = loop_receipt(
        world,
        &frame,
        "loop:sliver-retained",
        &[
            [0.0, 0.0],
            [10.0e-9, 0.0],
            [10.0e-9, 1.0e-20],
            [0.0, 1.0e-20],
        ],
    );
    let sliver = CertifiedSignedArea2D::measure_face(sliver_winding)
        .using_precision_basis(precision)
        .compile(&signed_area_contracts(world))
        .expect("sliver area plan")
        .certify()
        .expect("sliver area receipt");
    assert_eq!(sliver.degeneracy(), AreaDegeneracyClass::Sliver);
    assert_area_sum_cause(sliver.basis().localized_cause(), "loop:sliver-retained");
}

#[test]
fn certified_signed_area_classifies_tiny_hole_without_boolean_keep_discard() {
    let world = "signed-area-tiny-hole";
    let (precision, frame) = precision_and_frame(world, "movement:stable");
    let outer = CertifiedProjectedLoop2D::from_projected_vertices(
        "loop:tiny-hole-outer",
        topology_basis("loop:tiny-hole-outer"),
        loop_points(
            world,
            &frame,
            "tiny-hole-outer",
            &[
                [0.0, 0.0],
                [12.0e-9, 0.0],
                [12.0e-9, 12.0e-9],
                [0.0, 12.0e-9],
            ],
        ),
    )
    .expect("outer");
    let hole = CertifiedProjectedLoop2D::from_projected_vertices(
        "loop:tiny-hole",
        topology_basis("loop:tiny-hole"),
        loop_points(
            world,
            &frame,
            "tiny-hole",
            &[
                [1.0e-9, 1.0e-9],
                [2.0e-9, 1.0e-9],
                [2.0e-9, 2.0e-9],
                [1.0e-9, 2.0e-9],
            ],
        ),
    )
    .expect("hole");
    let winding = CertifiedPolygonWinding2D::certify(outer)
        .with_containment_candidate(hole)
        .within_planar_neighborhood("topology:tiny-hole-face")
        .compile(&winding_contracts(world))
        .expect("winding plan")
        .certify()
        .expect("winding receipt");

    let receipt = CertifiedSignedArea2D::measure_face(winding)
        .using_precision_basis(precision)
        .compile(&signed_area_contracts(world))
        .expect("area plan")
        .certify()
        .expect("area receipt");

    assert_eq!(receipt.degeneracy(), AreaDegeneracyClass::TinyHole);
    assert_eq!(receipt.repair_action(), None);
    assert_eq!(receipt.counters().loop_edges_walked(), 8);
}

#[test]
fn certified_signed_area_marks_outside_candidate_policy_required_without_area_guess() {
    let world = "signed-area-outside";
    let (precision, frame) = precision_and_frame(world, "movement:stable");
    let outer = CertifiedProjectedLoop2D::from_projected_vertices(
        "loop:outside-outer",
        topology_basis("loop:outside-outer"),
        loop_points(
            world,
            &frame,
            "outside-outer",
            &[[0.0, 0.0], [8.0e-9, 0.0], [8.0e-9, 8.0e-9], [0.0, 8.0e-9]],
        ),
    )
    .expect("outer");
    let outside = CertifiedProjectedLoop2D::from_projected_vertices(
        "loop:outside-candidate",
        topology_basis("loop:outside-candidate"),
        loop_points(
            world,
            &frame,
            "outside-candidate",
            &[
                [20.0e-9, 0.0],
                [22.0e-9, 0.0],
                [22.0e-9, 2.0e-9],
                [20.0e-9, 2.0e-9],
            ],
        ),
    )
    .expect("outside candidate");
    let winding = CertifiedPolygonWinding2D::certify(outer)
        .with_containment_candidate(outside)
        .within_planar_neighborhood("topology:outside-face")
        .compile(&winding_contracts(world))
        .expect("winding plan")
        .certify()
        .expect("winding receipt");

    let receipt = CertifiedSignedArea2D::measure_face(winding)
        .using_precision_basis(precision)
        .compile(&signed_area_contracts(world))
        .expect("area plan")
        .certify()
        .expect("area receipt");

    assert_eq!(receipt.degeneracy(), AreaDegeneracyClass::PolicyRequired);
    assert_eq!(
        receipt.basis().localized_cause(),
        Some(&SignedAreaDegeneracyCause::ContainmentPolicyRequired {
            loop_identity: "loop:outside-candidate".to_string(),
            containment: "outside".to_string(),
            policy: "classify-without-repair".to_string(),
        })
    );
    assert_eq!(receipt.counters().loop_edges_walked(), 8);
    assert_eq!(receipt.counters().area_terms_evaluated(), 4);
}

#[test]
fn certified_signed_area_denies_mismatched_precision_movement_before_facts() {
    let world = "signed-area-mismatch";
    let (stable_precision, stable_frame) = precision_and_frame(world, "movement:stable");
    let (moved_precision, _) = precision_and_frame(world, "movement:moved");
    let winding = loop_receipt(
        world,
        &stable_frame,
        "loop:mismatch",
        &[[0.0, 0.0], [4.0e-9, 0.0], [4.0e-9, 4.0e-9], [0.0, 4.0e-9]],
    );
    let area_contracts = signed_area_contracts(world);
    let denial = match CertifiedSignedArea2D::measure_face(winding)
        .using_precision_basis(moved_precision.clone())
        .compile(&area_contracts)
    {
        Ok(_) => panic!("mismatched movement must deny before retained facts"),
        Err(denial) => denial,
    };

    assert_eq!(
        denial.kind(),
        CertifiedSignedArea2DDenialKind::MovementRotationMismatch
    );
    assert_ne!(
        stable_precision.fact_digest(),
        moved_precision.fact_digest()
    );
}

#[test]
fn mb_m6_3_signed_area_and_degeneracy_survive_thin_feature_pressure() {
    let world = "mb-m6-3-signed-area";
    let (precision, frame) = precision_and_frame(world, "movement:rotate-cancelled");
    let winding = loop_receipt(
        world,
        &frame,
        "loop:micro-feature-371",
        &[[0.0, 0.0], [3.0e-9, 0.0], [3.0e-9, 3.0e-9], [0.0, 3.0e-9]],
    );
    let receipt = CertifiedSignedArea2D::measure_face(winding)
        .using_precision_basis(precision)
        .compile(&signed_area_contracts(world))
        .expect("area plan")
        .certify()
        .expect("area receipt");

    assert_eq!(receipt.degeneracy(), AreaDegeneracyClass::WellFormed);
    assert!(receipt.basis().localized_cause().is_none());
    assert_eq!(receipt.counters().degeneracy_localization_breadth(), 1);
}

fn loop_receipt(
    world: &'static str,
    frame: &worth_spatial::facade::planar_local_frame::PlanarLocalFrameCertificateReceipt,
    loop_identity: &'static str,
    points: &[[f64; 2]],
) -> worth_spatial::facade::planar_winding::CertifiedPolygonWinding2DReceipt {
    let loop_ = CertifiedProjectedLoop2D::from_projected_vertices(
        loop_identity,
        topology_basis(loop_identity),
        loop_points(world, frame, loop_identity, points),
    )
    .expect("projected loop");
    CertifiedPolygonWinding2D::certify(loop_)
        .within_planar_neighborhood("topology:signed-area-scale")
        .compile(&winding_contracts(world))
        .expect("winding plan")
        .certify()
        .expect("winding receipt")
}

fn assert_winding_basis_denial(
    error: CertifiedPolygonWinding2DFactError,
    expected: CertifiedPolygonWinding2DDenialKind,
) {
    match error {
        CertifiedPolygonWinding2DFactError::WindingBasis { denial } => {
            assert_eq!(denial.kind(), expected);
        }
        other => panic!("expected winding basis denial, got {other:?}"),
    }
}

fn assert_area_sum_cause(cause: Option<&SignedAreaDegeneracyCause>, expected_loop: &str) {
    match cause {
        Some(SignedAreaDegeneracyCause::AreaSum { loop_identity, .. }) => {
            assert_eq!(loop_identity, expected_loop);
        }
        other => panic!("expected area-sum cause for {expected_loop}, got {other:?}"),
    }
}

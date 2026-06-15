use topology::facade::TopologySeed;
use worth_spatial::facade::planar_boolean_common_plane::{
    PlanarBooleanCommonPlaneAgreementDenial, PlanarBooleanCommonPlaneAgreementDenialKind,
    PlanarBooleanCommonPlaneAgreementWorkload, PlanarBooleanCommonPlaneOperandSide,
    PlanarBooleanCommonPlanePostureAgreementDenial,
    PlanarBooleanCommonPlanePostureAgreementDenialKind,
    PlanarBooleanCommonPlanePostureAgreementWorkload,
    PlanarBooleanCommonPlaneSharedPlaneIdentityReceipt,
};
use worth_spatial::facade::projection_workload::{LocalFrameBasis, ProjectionWorkload};
use worth_spatial::facade::surface_support::{
    CertifiedSurfaceSupport, SurfaceFamily, SurfaceSupportWorkload,
};
use worth_spatial::facade::transform_workload::{
    RotationTurn, TransformReceiptSet, TransformReorientation, TransformSequence,
    TransformWorkload, VectorDelta,
};
use worth_spatial::facade::workload_binding::{
    BoundGeometryWorkload, GeometryBindingWorkload, PlanarEdgeCarrierSet, PlanarFaceCarrierSet,
    PlanarLoopCarrierSet,
};

#[test]
fn planar_boolean_common_plane_agreement_replays_to_one_shared_plane_identity() {
    let left = certified_surface_support("phase7.1 plane agreement parity");
    let right = certified_surface_support("phase7.1 plane agreement parity");
    let first = PlanarBooleanCommonPlaneAgreementWorkload::for_surface_support_pair(
        left.clone(),
        right.clone(),
    )
    .declared("phase7.1 plane agreement parity")
    .certify()
    .expect("equivalent planar supports should agree on one plane");
    let replayed = PlanarBooleanCommonPlaneAgreementWorkload::for_surface_support_pair(left, right)
        .declared("phase7.1 plane agreement parity")
        .certify()
        .expect("replayed planar supports should preserve the same plane agreement");

    assert_eq!(first, replayed);
    assert_eq!(
        first.left_witness().plane_identity_digest(),
        first.right_witness().plane_identity_digest()
    );
}

#[test]
fn planar_boolean_common_plane_agreement_denies_multi_plane_operand_before_shared_identity() {
    let left = certified_surface_support("phase7.1 plane agreement left");
    let right = certified_cube_surface_support("phase7.1 plane agreement distinct");
    let denial = PlanarBooleanCommonPlaneAgreementWorkload::for_surface_support_pair(left, right)
        .declared("phase7.1 plane agreement denial")
        .certify()
        .expect_err("multi-plane support must deny before minting a shared plane identity");

    assert_eq!(
        denial.kind(),
        PlanarBooleanCommonPlaneAgreementDenialKind::AmbiguousCertifiedFacePlaneWitness
    );
    assert!(denial
        .human_reason()
        .contains("exactly one certified face-plane witness"));
}

#[test]
fn planar_boolean_common_plane_agreement_localizes_ambiguous_operand_identity() {
    let left = certified_surface_support("phase7.1 plane agreement left");
    let right = certified_cube_surface_support("phase7.1 plane agreement right");
    let expected_right_identity = right
        .receipts()
        .stage_identity()
        .receipt_identity()
        .to_string();

    let denial = PlanarBooleanCommonPlaneAgreementWorkload::for_surface_support_pair(left, right)
        .declared("phase7.1 plane agreement localized denial")
        .certify()
        .expect_err("ambiguous right operand must deny with localized machine context");

    match denial {
        PlanarBooleanCommonPlaneAgreementDenial::AmbiguousCertifiedFacePlaneWitness {
            side,
            surface_support_identity,
            plane_identity_count,
        } => {
            assert_eq!(side, PlanarBooleanCommonPlaneOperandSide::Right);
            assert_eq!(surface_support_identity, expected_right_identity);
            assert!(plane_identity_count > 1);
        }
        other => panic!("expected ambiguous right-operand denial, got {other:?}"),
    }
}

#[test]
fn planar_boolean_common_plane_agreement_rejects_blank_declaration_before_witness_work() {
    let left = certified_surface_support("phase7.1 blank declaration left");
    let right = certified_surface_support("phase7.1 blank declaration right");

    let denial = PlanarBooleanCommonPlaneAgreementWorkload::for_surface_support_pair(left, right)
        .declared("   ")
        .certify()
        .expect_err("blank declarations must deny before any plane-agreement proof is minted");

    assert_eq!(
        denial.kind(),
        PlanarBooleanCommonPlaneAgreementDenialKind::MissingDeclaration
    );
}

#[test]
fn planar_boolean_common_plane_posture_agreement_replays_to_one_shared_posture_identity() {
    let left = transform_receipts(
        "phase7.1 posture agreement parity left",
        movement_rotation_stack_sequence(),
    );
    let right = transform_receipts(
        "phase7.1 posture agreement parity right",
        movement_rotation_stack_sequence(),
    );

    let first = PlanarBooleanCommonPlanePostureAgreementWorkload::for_transform_receipt_pair(
        left.clone(),
        right.clone(),
    )
    .declared("phase7.1 posture agreement parity")
    .certify()
    .expect("equivalent transform receipts should agree on one posture");
    let replayed =
        PlanarBooleanCommonPlanePostureAgreementWorkload::for_transform_receipt_pair(left, right)
            .declared("phase7.1 posture agreement parity")
            .certify()
            .expect("replayed transform receipts should preserve the same posture agreement");

    assert_eq!(first, replayed);
    assert_eq!(
        first.left_witness().semantic_posture_identity(),
        first.right_witness().semantic_posture_identity()
    );
}

#[test]
fn planar_boolean_common_plane_posture_agreement_denies_mismatched_posture_before_frame_work() {
    let left = transform_receipts(
        "phase7.1 posture agreement left",
        movement_rotation_stack_sequence(),
    );
    let right = transform_receipts(
        "phase7.1 posture agreement right",
        TransformSequence::new().reorient(TransformReorientation::ReversesHandedness),
    );
    let expected_right_projected_identity = right.projected_workload_identity().to_string();

    let denial =
        PlanarBooleanCommonPlanePostureAgreementWorkload::for_transform_receipt_pair(left, right)
            .declared("phase7.1 posture agreement denial")
            .certify()
            .expect_err("mismatched posture receipts must deny before shared-frame work begins");

    assert_eq!(
        denial.kind(),
        PlanarBooleanCommonPlanePostureAgreementDenialKind::DistinctMovementRotationPostures
    );
    match denial {
        PlanarBooleanCommonPlanePostureAgreementDenial::DistinctMovementRotationPostures {
            right_projected_workload_identity,
            left_posture_identity,
            right_posture_identity,
            ..
        } => {
            assert_eq!(
                right_projected_workload_identity,
                expected_right_projected_identity
            );
            assert_ne!(left_posture_identity, right_posture_identity);
        }
        other => panic!("expected posture mismatch denial, got {other:?}"),
    }
}

#[test]
fn planar_boolean_common_plane_posture_agreement_rejects_blank_declaration_before_witness_work() {
    let left = transform_receipts(
        "phase7.1 posture blank declaration left",
        movement_rotation_stack_sequence(),
    );
    let right = transform_receipts(
        "phase7.1 posture blank declaration right",
        movement_rotation_stack_sequence(),
    );

    let denial =
        PlanarBooleanCommonPlanePostureAgreementWorkload::for_transform_receipt_pair(left, right)
            .declared("   ")
            .certify()
            .expect_err(
                "blank posture-agreement declarations must deny before any posture proof is minted",
            );

    assert_eq!(
        denial.kind(),
        PlanarBooleanCommonPlanePostureAgreementDenialKind::MissingDeclaration
    );
}

#[test]
fn planar_boolean_common_plane_shared_plane_identity_receipt_replays_to_one_identity() {
    let left = certified_surface_support("phase7.1 shared-plane receipt parity");
    let right = certified_surface_support("phase7.1 shared-plane receipt parity");

    let agreement =
        PlanarBooleanCommonPlaneAgreementWorkload::for_surface_support_pair(left, right)
            .declared("phase7.1 shared-plane receipt parity")
            .certify()
            .expect("equivalent supports should certify plane agreement");
    let first =
        PlanarBooleanCommonPlaneSharedPlaneIdentityReceipt::from_plane_agreement(&agreement);
    let replayed =
        PlanarBooleanCommonPlaneSharedPlaneIdentityReceipt::from_plane_agreement(&agreement);

    assert_eq!(first, replayed);
    assert_eq!(
        first.shared_plane_identity(),
        agreement.shared_plane_identity()
    );
    assert_eq!(
        first.plane_agreement_identity(),
        agreement.agreement_identity()
    );
}

fn certified_surface_support(declaration: &str) -> CertifiedSurfaceSupport {
    certified_support_from_topology_seed(
        TopologySeed::single_face_loop(4)
            .with_declaration(format!("{declaration} topology"))
            .build()
            .expect("single-face topology should build"),
        declaration,
    )
}

fn certified_cube_surface_support(declaration: &str) -> CertifiedSurfaceSupport {
    certified_support_from_topology_seed(
        TopologySeed::cube()
            .with_declaration(format!("{declaration} topology"))
            .build()
            .expect("cube topology should build"),
        declaration,
    )
}

fn certified_support_from_topology_seed(
    topology: topology::facade::TopologySeedReceipt,
    declaration: &str,
) -> CertifiedSurfaceSupport {
    let bound_geometry: BoundGeometryWorkload =
        GeometryBindingWorkload::for_topology_seed(&topology)
            .declared(format!("bind {declaration}"))
            .with_planar_faces(PlanarFaceCarrierSet::for_seed_faces(&topology))
            .with_planar_edges(PlanarEdgeCarrierSet::for_seed_edges(&topology))
            .with_planar_loops(PlanarLoopCarrierSet::for_seed_loops(&topology))
            .admit()
            .expect("geometry binding should admit");

    SurfaceSupportWorkload::for_bound_geometry(bound_geometry)
        .declared(format!("support {declaration}"))
        .with_surface_family(SurfaceFamily::Plane)
        .certify()
        .expect("surface support should certify")
}

fn transform_receipts(
    declaration: &str,
    transform_sequence: TransformSequence,
) -> TransformReceiptSet {
    let support = certified_surface_support(declaration);
    let projected = ProjectionWorkload::for_certified_surface_support(support)
        .declared(format!("project {declaration}"))
        .with_local_frame(LocalFrameBasis::from_certified_plane())
        .project()
        .expect("projection should certify");

    TransformWorkload::for_projected_workload(projected)
        .declared(format!("transform {declaration}"))
        .with_transform_sequence(transform_sequence)
        .transform()
        .expect("transform should certify")
        .receipts()
        .clone()
}

fn movement_rotation_stack_sequence() -> TransformSequence {
    TransformSequence::new()
        .translate(VectorDelta::xy(10, 0))
        .rotate(RotationTurn::quarter_turn_clockwise())
}

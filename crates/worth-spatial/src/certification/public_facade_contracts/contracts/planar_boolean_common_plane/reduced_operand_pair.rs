use topology::facade::TopologySeed;
use worth_spatial::facade::planar_boolean_common_plane::{
    PlanarBooleanCommonPlaneAgreementWorkload, PlanarBooleanCommonPlaneLocalFrameSelectionReceipt,
    PlanarBooleanCommonPlaneOperandProjectionConsumptionReceipt,
    PlanarBooleanCommonPlaneOperandSide, PlanarBooleanCommonPlaneReducedOperandPairDenialKind,
    PlanarBooleanCommonPlaneReducedOperandPairReceipt,
    PlanarBooleanCommonPlaneSharedPlaneIdentityReceipt,
};
use worth_spatial::facade::planar_contract_bundle::{
    PlanarM7ReadinessBundle, PlanarM7ReadinessReceipt, PlanarM7ReadinessSupportPosture,
};
use worth_spatial::facade::projection_workload::{LocalFrameBasis, ProjectionWorkload};
use worth_spatial::facade::surface_support::{
    CertifiedSurfaceSupport, SurfaceFamily, SurfaceSupportWorkload,
};
use worth_spatial::facade::workload_binding::{
    BoundGeometryWorkload, GeometryBindingWorkload, PlanarEdgeCarrierSet, PlanarFaceCarrierSet,
    PlanarLoopCarrierSet,
};

use crate::public_api_planar_contract_bundle::m7_readiness::fixture::{
    bundle_contracts, m7_readiness_parts,
};

#[test]
fn planar_boolean_reduced_operand_pair_replays_to_one_identity() {
    run_with_large_stack(|| {
        let (left, right) = operand_projection_receipts("phase7.1 reduced pair parity");
        let first =
            PlanarBooleanCommonPlaneReducedOperandPairReceipt::from_operand_projection_receipts(
                &left, &right,
            )
            .expect("reduced pair should certify");
        let replayed =
            PlanarBooleanCommonPlaneReducedOperandPairReceipt::from_operand_projection_receipts(
                &left, &right,
            )
            .expect("replayed reduced pair should preserve identity");

        assert_eq!(first, replayed);
        assert_eq!(
            first.left_projection_identity(),
            left.operand_projection_consumption_identity()
        );
        assert_eq!(
            first.right_projection_identity(),
            right.operand_projection_consumption_identity()
        );
        assert_eq!(
            first.left_projection_stage_identity(),
            left.projection_stage_identity()
        );
        assert_eq!(
            first.right_projection_stage_identity(),
            right.projection_stage_identity(),
            "reduced-pair proof must preserve the exact operand-local projection-stage provenance it consumed"
        );
        assert_eq!(
            first.ordering_contract().semantic_left_side(),
            PlanarBooleanCommonPlaneOperandSide::Left
        );
        assert_eq!(
            first.ordering_contract().semantic_right_side(),
            PlanarBooleanCommonPlaneOperandSide::Right
        );
    });
}

#[test]
fn planar_boolean_reduced_operand_pair_rejects_duplicate_left_operands() {
    run_with_large_stack(|| {
        let (left, _) = operand_projection_receipts("phase7.1 reduced pair duplicate left");
        let denial =
            PlanarBooleanCommonPlaneReducedOperandPairReceipt::from_operand_projection_receipts(
                &left, &left,
            )
            .expect_err("duplicate left operand projections must deny");

        assert_eq!(
            denial.kind(),
            PlanarBooleanCommonPlaneReducedOperandPairDenialKind::DuplicateOperandSide
        );
    });
}

#[test]
fn planar_boolean_reduced_operand_pair_rejects_mixed_local_frame_chains() {
    run_with_large_stack(|| {
        let (left, _right) = operand_projection_receipts("phase7.1 reduced pair one");
        let (_, foreign_right) = operand_projection_receipts("phase7.1 reduced pair two");
        let denial =
            PlanarBooleanCommonPlaneReducedOperandPairReceipt::from_operand_projection_receipts(
                &left,
                &foreign_right,
            )
            .expect_err("mixed local-frame chains must deny");

        assert!(matches!(
            denial.kind(),
            PlanarBooleanCommonPlaneReducedOperandPairDenialKind::SharedPlaneReceiptIdentityMismatch
                | PlanarBooleanCommonPlaneReducedOperandPairDenialKind::SharedPlaneIdentityMismatch
                | PlanarBooleanCommonPlaneReducedOperandPairDenialKind::PlaneAgreementIdentityMismatch
                | PlanarBooleanCommonPlaneReducedOperandPairDenialKind::LocalFrameSelectionIdentityMismatch
        ));
    });
}

fn operand_projection_receipts(
    declaration: &'static str,
) -> (
    PlanarBooleanCommonPlaneOperandProjectionConsumptionReceipt,
    PlanarBooleanCommonPlaneOperandProjectionConsumptionReceipt,
) {
    let selection = local_frame_selection_receipt(declaration);
    let projection =
        ProjectionWorkload::for_certified_surface_support(certified_surface_support(declaration))
            .declared(format!("project {declaration}"))
            .with_local_frame(LocalFrameBasis::from_common_plane_selection(&selection))
            .project()
            .expect("selected-frame projection should certify");
    let left =
        PlanarBooleanCommonPlaneOperandProjectionConsumptionReceipt::from_local_frame_selection_and_projection_receipts(
            &selection,
            projection.receipts(),
            PlanarBooleanCommonPlaneOperandSide::Left,
        )
        .expect("left operand should certify");
    let right =
        PlanarBooleanCommonPlaneOperandProjectionConsumptionReceipt::from_local_frame_selection_and_projection_receipts(
            &selection,
            projection.receipts(),
            PlanarBooleanCommonPlaneOperandSide::Right,
        )
        .expect("right operand should certify");
    (left, right)
}

fn local_frame_selection_receipt(
    declaration: &'static str,
) -> PlanarBooleanCommonPlaneLocalFrameSelectionReceipt {
    let shared_plane = shared_plane_identity_receipt(declaration);
    let readiness = m7_readiness_receipt(declaration);
    PlanarBooleanCommonPlaneLocalFrameSelectionReceipt::from_shared_plane_identity_and_m7_readiness(
        &shared_plane,
        &readiness,
    )
    .expect("local-frame selection should certify")
}

fn m7_readiness_receipt(world: &'static str) -> PlanarM7ReadinessReceipt {
    let parts = m7_readiness_parts(world);
    PlanarM7ReadinessBundle::from_certified_planar_bundle(parts.readiness)
        .with_structural_identity(parts.structural)
        .with_motion_posture(parts.motion)
        .with_retained_planar_facts(parts.retained)
        .with_projection_consumed_facts(parts.projected)
        .with_recovery_posture(parts.recovery)
        .with_diagnostics(parts.diagnostics)
        .with_support_posture(PlanarM7ReadinessSupportPosture::support_gated(
            "M7 boolean split/classify/assemble is support-gated until Milestone 7",
        ))
        .compile(&bundle_contracts(world))
        .expect("M7 readiness plan")
        .certify()
        .expect("M7 readiness receipt")
}

fn shared_plane_identity_receipt(
    declaration: &str,
) -> PlanarBooleanCommonPlaneSharedPlaneIdentityReceipt {
    let left = certified_surface_support(declaration);
    let right = certified_surface_support(declaration);
    let agreement =
        PlanarBooleanCommonPlaneAgreementWorkload::for_surface_support_pair(left, right)
            .declared(declaration)
            .certify()
            .expect("equivalent supports should certify plane agreement");
    PlanarBooleanCommonPlaneSharedPlaneIdentityReceipt::from_plane_agreement(&agreement)
}

fn certified_surface_support(declaration: &str) -> CertifiedSurfaceSupport {
    let topology = TopologySeed::single_face_loop(4)
        .with_declaration(format!("{declaration} topology"))
        .build()
        .expect("single-face topology should build");
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

fn run_with_large_stack(body: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new()
        .name("planar-boolean-reduced-operand-pair".to_string())
        .stack_size(16 * 1024 * 1024)
        .spawn(body)
        .expect("reduced-operand-pair contract thread should spawn")
        .join()
        .expect("reduced-operand-pair contract thread should finish");
}

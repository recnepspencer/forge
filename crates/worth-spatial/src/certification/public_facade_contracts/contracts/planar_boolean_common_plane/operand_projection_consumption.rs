use topology::facade::TopologySeed;
use worth_spatial::facade::planar_boolean_common_plane::{
    PlanarBooleanCommonPlaneAgreementWorkload, PlanarBooleanCommonPlaneLocalFrameSelectionReceipt,
    PlanarBooleanCommonPlaneOperandProjectionConsumptionReceipt,
    PlanarBooleanCommonPlaneOperandSide, PlanarBooleanCommonPlaneSharedPlaneIdentityReceipt,
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
fn planar_boolean_operand_projection_consumption_replays_to_one_identity() {
    run_with_large_stack(|| {
        let declaration = "phase7.1 operand projection parity";
        let support = certified_surface_support(declaration);
        let selection = local_frame_selection_receipt(declaration);
        let projection = ProjectionWorkload::for_certified_surface_support(support)
            .declared(format!("project {declaration}"))
            .with_local_frame(LocalFrameBasis::from_common_plane_selection(&selection))
            .project()
            .expect("projection should certify");

        let first =
            PlanarBooleanCommonPlaneOperandProjectionConsumptionReceipt::from_local_frame_selection_and_projection_receipts(
                &selection,
                projection.receipts(),
                PlanarBooleanCommonPlaneOperandSide::Left,
            )
            .expect("operand projection consumption should certify");
        let replayed =
            PlanarBooleanCommonPlaneOperandProjectionConsumptionReceipt::from_local_frame_selection_and_projection_receipts(
                &selection,
                projection.receipts(),
                PlanarBooleanCommonPlaneOperandSide::Left,
            )
            .expect("replayed operand projection consumption should preserve identity");

        assert_eq!(first, replayed);
        assert_eq!(
            first.local_frame_selection_identity(),
            selection.local_frame_selection_receipt_identity()
        );
        assert_eq!(
            first.shared_plane_identity(),
            selection.shared_plane_identity()
        );
        assert_eq!(
            first.projection_stage_identity(),
            projection.receipts().stage_identity().receipt_identity()
        );
        assert_eq!(
            first.projection_local_basis_identity(),
            selection.projection_local_basis_identity()
        );
        assert_eq!(
            first.projected_entity_count(),
            projection
                .receipts()
                .counters()
                .projected_topology_entities()
        );
    });
}

#[test]
fn planar_boolean_operand_b_projection_consumption_replays_to_one_identity() {
    run_with_large_stack(|| {
        let declaration = "phase7.1 operand-b projection parity";
        let support = certified_surface_support(declaration);
        let selection = local_frame_selection_receipt(declaration);
        let projection = ProjectionWorkload::for_certified_surface_support(support)
            .declared(format!("project {declaration}"))
            .with_local_frame(LocalFrameBasis::from_common_plane_selection(&selection))
            .project()
            .expect("projection should certify");

        let first =
            PlanarBooleanCommonPlaneOperandProjectionConsumptionReceipt::from_local_frame_selection_and_projection_receipts(
                &selection,
                projection.receipts(),
                PlanarBooleanCommonPlaneOperandSide::Right,
            )
            .expect("operand projection consumption should certify");
        let replayed =
            PlanarBooleanCommonPlaneOperandProjectionConsumptionReceipt::from_local_frame_selection_and_projection_receipts(
                &selection,
                projection.receipts(),
                PlanarBooleanCommonPlaneOperandSide::Right,
            )
            .expect("replayed operand projection consumption should preserve identity");

        assert_eq!(first, replayed);
        assert_eq!(
            first.projection_local_basis_identity(),
            selection.projection_local_basis_identity()
        );
    });
}

#[test]
fn planar_boolean_operand_projection_consumption_rejects_generic_plane_basis() {
    run_with_large_stack(|| {
        let declaration = "phase7.1 operand projection generic basis denial";
        let support = certified_surface_support(declaration);
        let selection = local_frame_selection_receipt(declaration);
        let projection = ProjectionWorkload::for_certified_surface_support(support)
            .declared(format!("project {declaration}"))
            .with_local_frame(LocalFrameBasis::from_certified_plane())
            .project()
            .expect("generic projection should still certify as a projection workload");

        let denial =
            PlanarBooleanCommonPlaneOperandProjectionConsumptionReceipt::from_local_frame_selection_and_projection_receipts(
                &selection,
                projection.receipts(),
                PlanarBooleanCommonPlaneOperandSide::Left,
            )
            .expect_err("operand projection consumption must reject projection receipts that were not derived from the selected common-plane frame");

        assert_eq!(
            denial.kind(),
            worth_spatial::facade::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandProjectionConsumptionDenialKind::ProjectionLocalBasisSelectionMismatch
        );
    });
}

#[test]
fn planar_boolean_operand_projection_consumption_localizes_operand_b_failure() {
    run_with_large_stack(|| {
        let declaration = "phase7.1 operand projection localize b failure";
        let selection = local_frame_selection_receipt(declaration);
        let shared_projection = ProjectionWorkload::for_certified_surface_support(
            certified_surface_support(declaration),
        )
        .declared(format!("project {declaration}"))
        .with_local_frame(LocalFrameBasis::from_common_plane_selection(&selection))
        .project()
        .expect("selected-frame projection should certify");
        let operand_a =
            PlanarBooleanCommonPlaneOperandProjectionConsumptionReceipt::from_local_frame_selection_and_projection_receipts(
                &selection,
                shared_projection.receipts(),
                PlanarBooleanCommonPlaneOperandSide::Left,
            )
            .expect("operand A should certify through the selected frame");

        let generic_projection = ProjectionWorkload::for_certified_surface_support(
            certified_surface_support("phase7.1 operand-b generic denial support"),
        )
        .declared("phase7.1 operand-b generic denial projection")
        .with_local_frame(LocalFrameBasis::from_certified_plane())
        .project()
        .expect("generic projection should still certify as a projection workload");
        let denial =
            PlanarBooleanCommonPlaneOperandProjectionConsumptionReceipt::from_local_frame_selection_and_projection_receipts(
                &selection,
                generic_projection.receipts(),
                PlanarBooleanCommonPlaneOperandSide::Right,
            )
            .expect_err("operand B must localize the selected-frame mismatch");

        assert_eq!(
            operand_a.operand_side(),
            PlanarBooleanCommonPlaneOperandSide::Left
        );
        assert_eq!(
            denial.kind(),
            worth_spatial::facade::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandProjectionConsumptionDenialKind::ProjectionLocalBasisSelectionMismatch
        );
    });
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
        .name("planar-boolean-operand-projection-consumption".to_string())
        .stack_size(16 * 1024 * 1024)
        .spawn(body)
        .expect("operand projection consumption contract thread should spawn")
        .join()
        .expect("operand projection consumption contract thread should finish");
}

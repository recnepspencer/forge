use topology::facade::TopologySeed;
use worth_kernel::workload_composition::{
    PlanarBooleanCommonPlaneLocalFrameSelectedRequest,
    PlanarBooleanCommonPlaneOperandBProjectedRequest,
    PlanarBooleanCommonPlaneOperandBProjectionConsumptionError,
    PlanarBooleanCommonPlanePlaneAgreedRequest, PlanarBooleanCommonPlanePostureAgreedRequest,
    PlanarBooleanCommonPlanePrecisionAgreedRequest, PlanarBooleanCommonPlaneReductionRequest,
    PlanarBooleanCommonPlaneScopeAdmittedRequest,
    PlanarBooleanCommonPlaneSharedPlaneIdentifiedRequest, PlanarBooleanDeclaration,
    PlanarBooleanEntryBasis, PlanarBooleanExecutionLane, PlanarBooleanFamily,
    PlanarBooleanOperandPairIdentity, PlanarBooleanOperation, WorkloadCatalog,
};
use worth_spatial::facade::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandSide;
use worth_spatial::facade::projection_workload::{LocalFrameBasis, ProjectionWorkload};
use worth_spatial::facade::surface_support::{
    CertifiedSurfaceSupport, SurfaceFamily, SurfaceSupportWorkload,
};
use worth_spatial::facade::workload_binding::{
    BoundGeometryWorkload, GeometryBindingWorkload, PlanarEdgeCarrierSet, PlanarFaceCarrierSet,
    PlanarLoopCarrierSet,
};

#[path = "public_api_planar_boolean_entry/tests/support.rs"]
mod entry_support;

#[test]
fn operand_b_projection_consumption_converges_across_construction_paths() {
    run_with_large_stack(|| {
        let pair = WorkloadCatalog::planar_boolean_clean_planar_body_pair()
            .declared("phase7.1 operand-b projection convergence")
            .build()
            .expect("pair should build");
        let expected_source_operand_workload_identity = pair
            .right()
            .workload()
            .response()
            .identity()
            .receipt_identity();

        let local_frame =
            local_frame_request_from_pair("phase7.1 operand-b projection convergence", pair);
        let ordinary =
            PlanarBooleanCommonPlaneOperandBProjectedRequest::from_local_frame_selected_request(
                local_frame.clone(),
            )
            .expect("ordinary path should certify operand-B projection consumption");
        let advanced = PlanarBooleanCommonPlaneOperandBProjectedRequest::from_parts(
            local_frame,
            ordinary.projection_receipt().clone(),
        )
        .expect("advanced path should preserve operand-B projection consumption");

        assert_eq!(
            ordinary.operand_b_projection_identity(),
            advanced.operand_b_projection_identity()
        );
        assert_eq!(
            ordinary.projection_receipt().projection_stage_identity(),
            advanced.projection_receipt().projection_stage_identity()
        );
        assert_eq!(
            ordinary.projection_receipt().projected_entity_count(),
            advanced.projection_receipt().projected_entity_count()
        );
        assert_eq!(
            ordinary
                .projection_receipt()
                .projection_local_basis_identity(),
            ordinary
                .local_frame_selected_request()
                .selection_receipt()
                .projection_local_basis_identity()
        );
        assert_eq!(
            ordinary.source_operand_workload_identity(),
            expected_source_operand_workload_identity
        );
    });
}

#[test]
fn operand_b_projection_consumption_rejects_foreign_operand_a_receipt() {
    run_with_large_stack(|| {
        let pair = WorkloadCatalog::planar_boolean_clean_planar_body_pair()
            .declared("phase7.1 operand-b projection one")
            .build()
            .expect("pair should build");

        let local_frame =
            local_frame_request_from_pair("phase7.1 operand-b projection one", pair.clone());
        let foreign_projection = ProjectionWorkload::for_certified_surface_support(
            certified_surface_support("phase7.1 operand-b foreign left support"),
        )
        .declared("phase7.1 operand-b foreign left projection")
        .with_local_frame(LocalFrameBasis::from_common_plane_selection(
            local_frame.selection_receipt(),
        ))
        .project()
        .expect(
            "left projection through selected frame should still be a valid projection workload",
        );
        let foreign =
            worth_spatial::facade::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandProjectionConsumptionReceipt::from_local_frame_selection_and_projection_receipts(
                local_frame.selection_receipt(),
                foreign_projection.receipts(),
                PlanarBooleanCommonPlaneOperandSide::Left,
            )
            .expect("foreign left operand receipt should still be buildable as a raw spatial artifact");

        let error =
            PlanarBooleanCommonPlaneOperandBProjectedRequest::from_parts(local_frame, foreign)
                .expect_err("operand-B wrapper must reject foreign/left receipt");

        assert!(matches!(
            error,
            PlanarBooleanCommonPlaneOperandBProjectionConsumptionError::RetainedOperandProjectionConsumptionDenied { .. }
                | PlanarBooleanCommonPlaneOperandBProjectionConsumptionError::ProjectionLocalBasisIdentityMismatch { .. }
                | PlanarBooleanCommonPlaneOperandBProjectionConsumptionError::OperandSideMismatch { .. }
                | PlanarBooleanCommonPlaneOperandBProjectionConsumptionError::ProjectionStageIdentityMismatch { .. }
                | PlanarBooleanCommonPlaneOperandBProjectionConsumptionError::UpstreamSurfaceSupportIdentityMismatch { .. }
                | PlanarBooleanCommonPlaneOperandBProjectionConsumptionError::CertifiedPlaneSupportIdentityMismatch { .. }
        ));
    });
}

#[test]
fn operand_b_projection_consumption_rejects_foreign_local_frame_selection_chain() {
    run_with_large_stack(|| {
        let pair = WorkloadCatalog::planar_boolean_clean_planar_body_pair()
            .declared("phase7.1 operand-b projection local-frame one")
            .build()
            .expect("pair should build");
        let other_pair = WorkloadCatalog::planar_boolean_clean_planar_body_pair()
            .declared("phase7.1 operand-b projection local-frame two")
            .build()
            .expect("other pair should build");

        let local_frame =
            local_frame_request_from_pair("phase7.1 operand-b projection local-frame one", pair);
        let foreign_local_frame = local_frame_request_from_pair(
            "phase7.1 operand-b projection local-frame two",
            other_pair,
        );
        let projection_receipt =
            PlanarBooleanCommonPlaneOperandBProjectedRequest::from_local_frame_selected_request(
                local_frame,
            )
            .expect("ordinary path should certify")
            .projection_receipt()
            .clone();

        let error = PlanarBooleanCommonPlaneOperandBProjectedRequest::from_parts(
            foreign_local_frame,
            projection_receipt,
        )
        .expect_err("foreign local-frame chain must fail");

        assert!(matches!(
            error,
            PlanarBooleanCommonPlaneOperandBProjectionConsumptionError::LocalFrameSelectionIdentityMismatch { .. }
                | PlanarBooleanCommonPlaneOperandBProjectionConsumptionError::SharedPlaneReceiptIdentityMismatch { .. }
                | PlanarBooleanCommonPlaneOperandBProjectionConsumptionError::SharedPlaneIdentityMismatch { .. }
                | PlanarBooleanCommonPlaneOperandBProjectionConsumptionError::PlaneAgreementIdentityMismatch { .. }
        ));
    });
}

fn local_frame_request_from_pair(
    readiness_scope: &'static str,
    pair: worth_kernel::workload_composition::BuiltBooleanOperandPairRecipe,
) -> PlanarBooleanCommonPlaneLocalFrameSelectedRequest {
    let declaration = PlanarBooleanDeclaration::new(
        PlanarBooleanFamily::PlanarRegions,
        PlanarBooleanOperation::Union,
        PlanarBooleanOperandPairIdentity::new(pair.operand_pair_identity())
            .expect("operand-pair identity should certify"),
        PlanarBooleanExecutionLane::BRepNow,
    )
    .from_basis(
        PlanarBooleanEntryBasis::bind(
            entry_support::certified_boolean_readiness_workload_receipt(readiness_scope),
            format!("{readiness_scope} basis"),
        )
        .expect("entry basis should certify"),
    )
    .declared_by_query(format!("{readiness_scope} declaration"))
    .bind()
    .expect("boolean declaration should bind");
    entry_support::assert_planar_boolean_query_digest(declaration.basis_query_declaration_digest());
    entry_support::assert_planar_boolean_query_digest(declaration.query_declaration_digest());

    let posture = PlanarBooleanCommonPlanePostureAgreedRequest::from_plane_agreed_request(
        PlanarBooleanCommonPlanePlaneAgreedRequest::from_scope_admitted_request(
            PlanarBooleanCommonPlaneScopeAdmittedRequest::from_reduction_request(
                PlanarBooleanCommonPlaneReductionRequest::from_declaration_receipt_and_operand_pair(
                    declaration,
                    pair,
                )
                .expect("reduction request should build"),
            )
            .expect("scope admission should certify"),
        )
        .expect("plane agreement should certify"),
    )
    .expect("posture agreement should certify");
    let precision =
        PlanarBooleanCommonPlanePrecisionAgreedRequest::from_posture_agreed_request(posture)
            .expect("precision agreement should certify");
    let shared_plane =
        PlanarBooleanCommonPlaneSharedPlaneIdentifiedRequest::from_precision_agreed_request(
            precision,
        )
        .expect("shared-plane identity should certify");
    PlanarBooleanCommonPlaneLocalFrameSelectedRequest::from_shared_plane_identified_request(
        shared_plane,
    )
    .expect("local-frame selection should certify")
}

fn run_with_large_stack(body: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new()
        .name("planar-boolean-operand-b-projection".to_string())
        .stack_size(16 * 1024 * 1024)
        .spawn(body)
        .expect("operand-B projection contract thread should spawn")
        .join()
        .expect("operand-B projection contract thread should finish");
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

use worth_kernel::workload_composition::{
    PlanarBooleanCommonPlaneLocalFrameSelectedRequest,
    PlanarBooleanCommonPlaneLocalFrameSelectionError, PlanarBooleanCommonPlanePlaneAgreedRequest,
    PlanarBooleanCommonPlanePostureAgreedRequest, PlanarBooleanCommonPlanePrecisionAgreedRequest,
    PlanarBooleanCommonPlaneReductionRequest, PlanarBooleanCommonPlaneScopeAdmittedRequest,
    PlanarBooleanCommonPlaneSharedPlaneIdentifiedRequest, PlanarBooleanDeclaration,
    PlanarBooleanEntryBasis, PlanarBooleanExecutionLane, PlanarBooleanFamily,
    PlanarBooleanOperandPairIdentity, PlanarBooleanOperation, WorkloadCatalog,
};
#[path = "public_api_planar_boolean_entry/tests/support.rs"]
mod entry_support;

#[test]
fn common_plane_local_frame_selection_converges_across_construction_paths() {
    run_with_large_stack(|| {
        let pair = WorkloadCatalog::planar_boolean_clean_planar_body_pair()
            .declared("phase7.1 local frame selection convergence")
            .build()
            .expect("pair should build");

        let shared_plane =
            shared_plane_request_from_pair("phase7.1 local frame selection convergence", pair);
        let ordinary =
            PlanarBooleanCommonPlaneLocalFrameSelectedRequest::from_shared_plane_identified_request(
                shared_plane.clone(),
            )
            .expect("ordinary path should select local frame");
        let advanced = PlanarBooleanCommonPlaneLocalFrameSelectedRequest::from_parts(
            shared_plane,
            ordinary.selection_receipt().clone(),
        )
        .expect("advanced path should certify the same local-frame selection");

        assert_eq!(
            ordinary.local_frame_selection_identity(),
            advanced.local_frame_selection_identity()
        );
        assert_eq!(
            ordinary.local_frame_fact_digest(),
            advanced.local_frame_fact_digest()
        );
        assert_eq!(ordinary.frame_identity(), advanced.frame_identity());
        assert_eq!(
            ordinary.topology_basis_identity(),
            advanced.topology_basis_identity()
        );
        assert_eq!(
            ordinary.movement_rotation_posture_identity(),
            advanced.movement_rotation_posture_identity()
        );
    });
}

#[test]
fn common_plane_local_frame_selection_rejects_foreign_local_frame_receipt() {
    run_with_large_stack(|| {
        let pair = WorkloadCatalog::planar_boolean_clean_planar_body_pair()
            .declared("phase7.1 local frame selection one")
            .build()
            .expect("pair should build");
        let other_pair = WorkloadCatalog::planar_boolean_clean_planar_body_pair()
            .declared("phase7.1 local frame selection two")
            .build()
            .expect("other pair should build");

        let selected = shared_plane_request_from_pair("phase7.1 local frame selection one", pair);
        let foreign_shared_plane =
            shared_plane_request_from_pair("phase7.1 local frame selection two", other_pair);
        let foreign_receipt =
            PlanarBooleanCommonPlaneLocalFrameSelectedRequest::from_shared_plane_identified_request(
                foreign_shared_plane,
            )
            .expect("foreign path should select local frame")
            .selection_receipt()
            .clone();

        let error = PlanarBooleanCommonPlaneLocalFrameSelectedRequest::from_parts(
            selected,
            foreign_receipt,
        )
        .expect_err("foreign local-frame selection receipt must fail");

        assert!(matches!(
            error,
            PlanarBooleanCommonPlaneLocalFrameSelectionError::SharedPlaneReceiptIdentityMismatch { .. }
                | PlanarBooleanCommonPlaneLocalFrameSelectionError::SharedPlaneIdentityMismatch { .. }
                | PlanarBooleanCommonPlaneLocalFrameSelectionError::FrameIdentityMismatch { .. }
                | PlanarBooleanCommonPlaneLocalFrameSelectionError::TopologyBasisIdentityMismatch { .. }
                | PlanarBooleanCommonPlaneLocalFrameSelectionError::MovementRotationPostureIdentityMismatch { .. }
                | PlanarBooleanCommonPlaneLocalFrameSelectionError::PlaneAgreementIdentityMismatch { .. }
        ));
    });
}

fn shared_plane_request_from_pair(
    readiness_scope: &'static str,
    pair: worth_kernel::workload_composition::BuiltBooleanOperandPairRecipe,
) -> PlanarBooleanCommonPlaneSharedPlaneIdentifiedRequest {
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
    PlanarBooleanCommonPlaneSharedPlaneIdentifiedRequest::from_precision_agreed_request(precision)
        .expect("shared-plane identity should certify")
}

fn run_with_large_stack(body: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new()
        .name("planar-boolean-local-frame-selection".to_string())
        .stack_size(16 * 1024 * 1024)
        .spawn(body)
        .expect("local-frame selection contract thread should spawn")
        .join()
        .expect("local-frame selection contract thread should finish");
}

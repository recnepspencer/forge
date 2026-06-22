use worth_kernel::workload_composition::{
    PlanarBooleanCommonPlanePlaneAgreedRequest, PlanarBooleanCommonPlanePostureAgreedRequest,
    PlanarBooleanCommonPlanePrecisionAgreedRequest, PlanarBooleanCommonPlaneReductionRequest,
    PlanarBooleanCommonPlaneScopeAdmittedRequest,
    PlanarBooleanCommonPlaneSharedPlaneIdentifiedRequest,
    PlanarBooleanCommonPlaneSharedPlaneIdentityError, PlanarBooleanDeclaration,
    PlanarBooleanEntryBasis, PlanarBooleanExecutionLane, PlanarBooleanFamily,
    PlanarBooleanOperandPairIdentity, PlanarBooleanOperation, WorkloadCatalog,
};
use worth_spatial::facade::planar_boolean_common_plane::PlanarBooleanCommonPlaneSharedPlaneIdentityReceipt;

#[path = "public_api_planar_boolean_entry/tests/support.rs"]
mod entry_support;

#[test]
fn common_plane_shared_plane_identity_converges_across_construction_paths() {
    run_with_large_stack(|| {
        let ordinary_pair = WorkloadCatalog::planar_boolean_clean_planar_body_pair()
            .declared("phase7.1 shared plane identity convergence")
            .build()
            .expect("ordinary pair should build");
        let advanced_pair = WorkloadCatalog::planar_boolean_clean_planar_body_pair()
            .declared("phase7.1 shared plane identity convergence")
            .build()
            .expect("advanced pair should build");

        let ordinary =
            PlanarBooleanCommonPlaneSharedPlaneIdentifiedRequest::from_precision_agreed_request(
                precision_agreed_request_from_pair(
                    "phase7.1 shared plane identity convergence",
                    ordinary_pair,
                ),
            )
            .expect("ordinary path should identify one shared plane");
        let advanced_precision = precision_agreed_request_from_pair(
            "phase7.1 shared plane identity convergence",
            advanced_pair,
        );
        let advanced_receipt =
            PlanarBooleanCommonPlaneSharedPlaneIdentityReceipt::from_plane_agreement(
                advanced_precision
                    .posture_agreed_request()
                    .plane_agreed_request()
                    .agreement_receipt(),
            );
        let advanced = PlanarBooleanCommonPlaneSharedPlaneIdentifiedRequest::from_parts(
            advanced_precision,
            advanced_receipt,
        )
        .expect("advanced path should identify the same shared plane");

        assert_eq!(
            ordinary.shared_plane_identity(),
            advanced.shared_plane_identity(),
            "equivalent admitted pairs must preserve one shared-plane identity"
        );
        assert_eq!(
            ordinary.shared_plane_identified_request_identity(),
            advanced.shared_plane_identified_request_identity(),
            "ordinary and advanced construction paths must converge to one shared-plane request identity"
        );
        assert_eq!(
            ordinary.shared_plane_receipt_identity(),
            advanced.shared_plane_receipt_identity()
        );
        assert_eq!(
            ordinary.plane_agreement_identity(),
            advanced.plane_agreement_identity()
        );
        assert_eq!(
            ordinary.posture_agreement_identity(),
            advanced.posture_agreement_identity()
        );
        assert_eq!(
            ordinary.precision_agreement_identity(),
            advanced.precision_agreement_identity()
        );
    });
}

#[test]
fn common_plane_shared_plane_identity_rejects_foreign_plane_agreement_receipt() {
    run_with_large_stack(|| {
        let pair = WorkloadCatalog::planar_boolean_clean_planar_body_pair()
            .declared("phase7.1 shared plane identity one")
            .build()
            .expect("first pair should build");
        let other_pair = WorkloadCatalog::planar_boolean_clean_planar_body_pair()
            .declared("phase7.1 shared plane identity two")
            .build()
            .expect("second pair should build");

        let precision =
            precision_agreed_request_from_pair("phase7.1 shared plane identity one", pair);
        let foreign_precision =
            precision_agreed_request_from_pair("phase7.1 shared plane identity two", other_pair);
        let foreign_receipt =
            PlanarBooleanCommonPlaneSharedPlaneIdentityReceipt::from_plane_agreement(
                foreign_precision
                    .posture_agreed_request()
                    .plane_agreed_request()
                    .agreement_receipt(),
            );

        let error = PlanarBooleanCommonPlaneSharedPlaneIdentifiedRequest::from_parts(
            precision,
            foreign_receipt,
        )
        .expect_err(
            "foreign shared-plane receipt must not satisfy the identified request boundary",
        );

        assert!(matches!(
            error,
            PlanarBooleanCommonPlaneSharedPlaneIdentityError::PlaneAgreementIdentityMismatch { .. }
        ));
        assert!(error
            .human_reason()
            .contains("same certified common-plane agreement"));
    });
}

fn precision_agreed_request_from_pair(
    readiness_scope: &'static str,
    pair: worth_kernel::workload_composition::BuiltBooleanOperandPairRecipe,
) -> PlanarBooleanCommonPlanePrecisionAgreedRequest {
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

    PlanarBooleanCommonPlanePrecisionAgreedRequest::from_posture_agreed_request(posture)
        .expect("precision agreement should certify")
}

fn run_with_large_stack(body: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new()
        .name("planar-boolean-shared-plane-identity".to_string())
        .stack_size(16 * 1024 * 1024)
        .spawn(body)
        .expect("shared-plane identity contract thread should spawn")
        .join()
        .expect("shared-plane identity contract thread should finish");
}

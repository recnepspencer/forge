use worth_kernel::workload_composition::{
    PlanarBooleanCommonPlanePlaneAgreedRequest, PlanarBooleanCommonPlanePostureAgreedRequest,
    PlanarBooleanCommonPlanePrecisionAgreedRequest,
    PlanarBooleanCommonPlanePrecisionAgreementError, PlanarBooleanCommonPlaneReductionRequest,
    PlanarBooleanCommonPlaneScopeAdmittedRequest, PlanarBooleanDeclaration,
    PlanarBooleanEntryBasis, PlanarBooleanExecutionLane, PlanarBooleanFamily,
    PlanarBooleanOperandPairIdentity, PlanarBooleanOperation, WorkloadCatalog,
};
use worth_spatial::facade::planar_boolean_common_plane::PlanarBooleanCommonPlanePrecisionAgreementReceipt;

#[path = "public_api_planar_boolean_entry/tests/support.rs"]
mod entry_support;

#[test]
fn common_plane_precision_agreement_converges_across_construction_paths() {
    run_with_large_stack(|| {
        let pair = WorkloadCatalog::planar_boolean_clean_planar_body_pair()
            .declared("phase7.1 precision agreement convergence")
            .build()
            .expect("pair should build");

        let posture = posture_request_from_pair("phase7.1 precision agreement convergence", pair);
        let ordinary = PlanarBooleanCommonPlanePrecisionAgreedRequest::from_posture_agreed_request(
            posture.clone(),
        )
        .expect("ordinary path should certify precision agreement");
        let advanced = PlanarBooleanCommonPlanePrecisionAgreedRequest::from_parts(
            posture,
            ordinary.precision_receipt().clone(),
        )
        .expect("advanced path should certify the same precision agreement");

        assert_eq!(
            ordinary.precision_agreement_identity(),
            advanced.precision_agreement_identity()
        );
        assert_eq!(
            ordinary.precision_fact_digest(),
            advanced.precision_fact_digest()
        );
        assert_eq!(
            ordinary.local_frame_fact_digest(),
            advanced.local_frame_fact_digest()
        );
    });
}

#[test]
fn common_plane_precision_agreement_rejects_missing_or_foreign_precision_boundary() {
    run_with_large_stack(|| {
        let pair = WorkloadCatalog::planar_boolean_clean_planar_body_pair()
            .declared("phase7.1 precision agreement one")
            .build()
            .expect("pair should build");
        let bare_posture = bare_posture_request_from_pair(pair.clone());
        let missing = PlanarBooleanCommonPlanePrecisionAgreedRequest::from_posture_agreed_request(
            bare_posture,
        )
        .expect_err("precision agreement must deny missing 7.0 declaration boundary");
        assert_eq!(
            missing,
            PlanarBooleanCommonPlanePrecisionAgreementError::MissingBooleanDeclarationBoundary
        );

        let posture = posture_request_from_pair("phase7.1 precision agreement one", pair);
        let expected_precision =
            PlanarBooleanCommonPlanePrecisionAgreedRequest::from_posture_agreed_request(
                posture.clone(),
            )
            .expect("ordinary path should certify precision agreement");
        let foreign_receipt =
            PlanarBooleanCommonPlanePrecisionAgreementReceipt::from_certified_parts(
                format!("{}-foreign", expected_precision.precision_fact_digest()),
                expected_precision.local_frame_fact_digest(),
                expected_precision.topology_basis_identity(),
                expected_precision.movement_rotation_posture_identity(),
            );
        let foreign =
            PlanarBooleanCommonPlanePrecisionAgreedRequest::from_parts(posture, foreign_receipt)
                .expect_err("foreign precision receipt must fail");
        assert!(matches!(
            foreign,
            PlanarBooleanCommonPlanePrecisionAgreementError::PrecisionFactDigestMismatch { .. }
        ));
    });
}

fn posture_request_from_pair(
    readiness_scope: &'static str,
    pair: worth_kernel::workload_composition::BuiltBooleanOperandPairRecipe,
) -> PlanarBooleanCommonPlanePostureAgreedRequest {
    PlanarBooleanCommonPlanePostureAgreedRequest::from_plane_agreed_request(
        PlanarBooleanCommonPlanePlaneAgreedRequest::from_scope_admitted_request(
            PlanarBooleanCommonPlaneScopeAdmittedRequest::from_reduction_request(
                PlanarBooleanCommonPlaneReductionRequest::from_declaration_receipt_and_operand_pair(
                    declaration_for_pair(readiness_scope, pair.clone()),
                    pair,
                )
                .expect("reduction request should build"),
            )
            .expect("scope admission should certify"),
        )
        .expect("plane agreement should certify"),
    )
    .expect("posture agreement should certify")
}

fn bare_posture_request_from_pair(
    pair: worth_kernel::workload_composition::BuiltBooleanOperandPairRecipe,
) -> PlanarBooleanCommonPlanePostureAgreedRequest {
    PlanarBooleanCommonPlanePostureAgreedRequest::from_plane_agreed_request(
        PlanarBooleanCommonPlanePlaneAgreedRequest::from_scope_admitted_request(
            PlanarBooleanCommonPlaneScopeAdmittedRequest::from_reduction_request(
                PlanarBooleanCommonPlaneReductionRequest::from_operand_pair_recipe(pair)
                    .expect("reduction request should build"),
            )
            .expect("scope admission should certify"),
        )
        .expect("plane agreement should certify"),
    )
    .expect("posture agreement should certify")
}

fn declaration_for_pair(
    readiness_scope: &'static str,
    pair: worth_kernel::workload_composition::BuiltBooleanOperandPairRecipe,
) -> worth_kernel::workload_composition::PlanarBooleanDeclarationReceipt {
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
    declaration
}

fn run_with_large_stack(body: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new()
        .name("planar-boolean-precision-agreement".to_string())
        .stack_size(16 * 1024 * 1024)
        .spawn(body)
        .expect("precision agreement contract thread should spawn")
        .join()
        .expect("precision agreement contract thread should finish");
}

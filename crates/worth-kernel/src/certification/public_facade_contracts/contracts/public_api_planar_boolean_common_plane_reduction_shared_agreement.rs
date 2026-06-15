use worth_kernel::workload_composition::{
    PlanarBooleanCommonPlanePlaneAgreedRequest, PlanarBooleanCommonPlanePostureAgreedRequest,
    PlanarBooleanCommonPlanePostureAgreementError, PlanarBooleanCommonPlaneReductionRequest,
    PlanarBooleanCommonPlaneScopeAdmittedRequest, WorkloadCatalog,
};
use worth_spatial::facade::planar_boolean_common_plane::PlanarBooleanCommonPlanePostureAgreementDenial;

#[test]
fn common_plane_plane_agreement_converges_to_one_shared_plane_receipt() {
    super::run_with_large_stack(|| {
        let ordinary_pair = WorkloadCatalog::planar_boolean_clean_planar_body_pair()
            .declared("phase7.1 plane agreement convergence")
            .build()
            .expect("ordinary clean planar body pair should build");
        let advanced_pair = WorkloadCatalog::planar_boolean_clean_planar_body_pair()
            .declared("phase7.1 plane agreement convergence")
            .build()
            .expect("advanced clean planar body pair should build");

        let ordinary = PlanarBooleanCommonPlanePlaneAgreedRequest::from_scope_admitted_request(
            PlanarBooleanCommonPlaneScopeAdmittedRequest::from_reduction_request(
                PlanarBooleanCommonPlaneReductionRequest::from_operand_pair_recipe(ordinary_pair)
                    .expect("ordinary reduction request should build"),
            )
            .expect("ordinary request should admit scope"),
        )
        .expect("ordinary request should certify plane agreement");
        let advanced = PlanarBooleanCommonPlanePlaneAgreedRequest::from_scope_admitted_request(
            PlanarBooleanCommonPlaneScopeAdmittedRequest::from_reduction_request(
                PlanarBooleanCommonPlaneReductionRequest::from_built_pair_and_construction(
                    advanced_pair.clone(),
                    advanced_pair.construction_receipt(),
                )
                .expect("advanced reduction request should build"),
            )
            .expect("advanced request should admit scope"),
        )
        .expect("advanced request should certify plane agreement");

        assert_eq!(
            ordinary.shared_plane_identity(),
            advanced.shared_plane_identity(),
            "equivalent admitted pairs must certify one shared plane identity"
        );
        assert_eq!(
            ordinary.plane_agreement_identity(),
            advanced.plane_agreement_identity(),
            "ordinary and advanced request construction must converge to one plane-agreement identity"
        );
        assert_eq!(ordinary.request_identity(), advanced.request_identity());
        assert_eq!(
            ordinary.scope_admission_identity(),
            advanced.scope_admission_identity()
        );
        assert_eq!(
            ordinary
                .agreement_receipt()
                .left_witness()
                .plane_identity_digest(),
            ordinary
                .agreement_receipt()
                .right_witness()
                .plane_identity_digest()
        );
    });
}

#[test]
fn common_plane_posture_agreement_converges_to_one_shared_posture_receipt() {
    super::run_with_large_stack(|| {
        let ordinary_pair = WorkloadCatalog::planar_boolean_clean_planar_body_pair()
            .declared("phase7.1 posture agreement convergence")
            .build()
            .expect("ordinary clean planar body pair should build");
        let advanced_pair = WorkloadCatalog::planar_boolean_clean_planar_body_pair()
            .declared("phase7.1 posture agreement convergence")
            .build()
            .expect("advanced clean planar body pair should build");

        let ordinary = PlanarBooleanCommonPlanePostureAgreedRequest::from_plane_agreed_request(
            PlanarBooleanCommonPlanePlaneAgreedRequest::from_scope_admitted_request(
                PlanarBooleanCommonPlaneScopeAdmittedRequest::from_reduction_request(
                    PlanarBooleanCommonPlaneReductionRequest::from_operand_pair_recipe(
                        ordinary_pair,
                    )
                    .expect("ordinary reduction request should build"),
                )
                .expect("ordinary request should admit scope"),
            )
            .expect("ordinary request should certify plane agreement"),
        )
        .expect("ordinary request should certify posture agreement");
        let advanced = PlanarBooleanCommonPlanePostureAgreedRequest::from_plane_agreed_request(
            PlanarBooleanCommonPlanePlaneAgreedRequest::from_scope_admitted_request(
                PlanarBooleanCommonPlaneScopeAdmittedRequest::from_reduction_request(
                    PlanarBooleanCommonPlaneReductionRequest::from_built_pair_and_construction(
                        advanced_pair.clone(),
                        advanced_pair.construction_receipt(),
                    )
                    .expect("advanced reduction request should build"),
                )
                .expect("advanced request should admit scope"),
            )
            .expect("advanced request should certify plane agreement"),
        )
        .expect("advanced request should certify posture agreement");

        assert_eq!(
            ordinary.shared_posture_identity(),
            advanced.shared_posture_identity(),
            "equivalent admitted pairs must certify one shared posture identity"
        );
        assert_eq!(
            ordinary.posture_agreement_identity(),
            advanced.posture_agreement_identity(),
            "ordinary and advanced request construction must converge to one posture-agreement identity"
        );
        assert_eq!(ordinary.request_identity(), advanced.request_identity());
        assert_eq!(
            ordinary.plane_agreement_identity(),
            advanced.plane_agreement_identity()
        );
    });
}

#[test]
fn common_plane_posture_agreement_denies_real_mismatched_posture_pair_before_precision_work() {
    super::run_with_large_stack(|| {
        let pair = WorkloadCatalog::planar_boolean_mismatched_posture_pair()
            .declared("phase7.1 posture denial real pair")
            .build()
            .expect("mismatched posture pair should build as a real admitted workload pair");

        let error = PlanarBooleanCommonPlanePostureAgreedRequest::from_plane_agreed_request(
            PlanarBooleanCommonPlanePlaneAgreedRequest::from_scope_admitted_request(
                PlanarBooleanCommonPlaneScopeAdmittedRequest::from_reduction_request(
                    PlanarBooleanCommonPlaneReductionRequest::from_operand_pair_recipe(pair)
                        .expect("reduction request should build"),
                )
                .expect("mismatched posture pair should admit the same closed planar body scope"),
            )
            .expect("mismatched posture pair should still certify plane agreement"),
        )
        .expect_err("real mismatched posture pair must deny before precision work begins");

        assert!(matches!(
            error,
            PlanarBooleanCommonPlanePostureAgreementError::SpatialPostureAgreementDenied { .. }
        ));
        assert!(!error.request_identity().trim().is_empty());
        assert!(!error.scope_admission_identity().trim().is_empty());
        assert!(!error.plane_agreement_identity().trim().is_empty());
        match error.spatial_denial() {
            PlanarBooleanCommonPlanePostureAgreementDenial::DistinctMovementRotationPostures {
                left_posture_identity,
                right_posture_identity,
                ..
            } => assert_ne!(left_posture_identity, right_posture_identity),
            other => panic!("expected posture mismatch denial, got {other:?}"),
        }
    });
}

#[test]
fn common_plane_posture_denial_converges_across_request_construction_paths() {
    super::run_with_large_stack(|| {
        let ordinary_pair = WorkloadCatalog::planar_boolean_mismatched_posture_pair()
            .declared("phase7.1 posture denial convergence")
            .build()
            .expect("ordinary mismatched posture pair should build");
        let advanced_pair = WorkloadCatalog::planar_boolean_mismatched_posture_pair()
            .declared("phase7.1 posture denial convergence")
            .build()
            .expect("advanced mismatched posture pair should build");

        let ordinary_error =
            PlanarBooleanCommonPlanePostureAgreedRequest::from_plane_agreed_request(
                PlanarBooleanCommonPlanePlaneAgreedRequest::from_scope_admitted_request(
                    PlanarBooleanCommonPlaneScopeAdmittedRequest::from_reduction_request(
                        PlanarBooleanCommonPlaneReductionRequest::from_operand_pair_recipe(
                            ordinary_pair,
                        )
                        .expect("ordinary reduction request should build"),
                    )
                    .expect("ordinary mismatched posture pair should admit scope"),
                )
                .expect("ordinary mismatched posture pair should certify plane agreement"),
            )
            .expect_err("ordinary mismatched posture pair must deny during posture agreement");
        let advanced_error =
            PlanarBooleanCommonPlanePostureAgreedRequest::from_plane_agreed_request(
                PlanarBooleanCommonPlanePlaneAgreedRequest::from_scope_admitted_request(
                    PlanarBooleanCommonPlaneScopeAdmittedRequest::from_reduction_request(
                        PlanarBooleanCommonPlaneReductionRequest::from_built_pair_and_construction(
                            advanced_pair.clone(),
                            advanced_pair.construction_receipt(),
                        )
                        .expect("advanced reduction request should build"),
                    )
                    .expect("advanced mismatched posture pair should admit scope"),
                )
                .expect("advanced mismatched posture pair should certify plane agreement"),
            )
            .expect_err("advanced mismatched posture pair must deny during posture agreement");

        assert!(matches!(
            ordinary_error,
            PlanarBooleanCommonPlanePostureAgreementError::SpatialPostureAgreementDenied { .. }
        ));
        assert!(matches!(
            advanced_error,
            PlanarBooleanCommonPlanePostureAgreementError::SpatialPostureAgreementDenied { .. }
        ));
        assert_eq!(
            ordinary_error.request_identity(),
            advanced_error.request_identity(),
            "ordinary and advanced denial paths must preserve one request identity"
        );
        assert_eq!(
            ordinary_error.operand_pair_identity(),
            advanced_error.operand_pair_identity(),
            "ordinary and advanced denial paths must preserve one operand-pair identity"
        );
        assert_eq!(
            ordinary_error.scope_admission_identity(),
            advanced_error.scope_admission_identity()
        );
        assert_eq!(
            ordinary_error.plane_agreement_identity(),
            advanced_error.plane_agreement_identity()
        );
    });
}

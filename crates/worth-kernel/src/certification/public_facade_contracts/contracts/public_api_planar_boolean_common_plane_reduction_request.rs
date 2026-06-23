use worth_kernel::workload_composition::{
    PlanarBooleanCommonPlaneReductionRequest, PlanarBooleanCommonPlaneReductionRequestError,
    WorkloadCatalog,
};

#[test]
fn common_plane_reduction_request_common_and_advanced_paths_converge() {
    super::run_with_large_stack(|| {
        let ordinary_pair = WorkloadCatalog::planar_boolean_clean_planar_body_pair()
            .declared("phase7.1 common plane request convergence")
            .build()
            .expect("catalog pair should build");
        let advanced_pair = WorkloadCatalog::planar_boolean_clean_planar_body_pair()
            .declared("phase7.1 common plane request convergence")
            .build()
            .expect("duplicate catalog pair should build");

        let ordinary =
            PlanarBooleanCommonPlaneReductionRequest::from_operand_pair_recipe(ordinary_pair)
                .expect("ordinary common-plane request should build");
        let construction = advanced_pair.construction_receipt();
        let advanced = PlanarBooleanCommonPlaneReductionRequest::from_built_pair_and_construction(
            advanced_pair,
            construction,
        )
        .expect("advanced common-plane request should build");

        assert_eq!(
            ordinary.request_identity(),
            advanced.request_identity(),
            "ordinary and advanced request construction must converge to one request identity"
        );
        assert_eq!(
            ordinary.operand_pair_identity(),
            advanced.operand_pair_identity(),
            "request must preserve the admitted operand pair identity"
        );
        assert_eq!(
            ordinary.construction_receipt().operand_pair_identity(),
            advanced.operand_pair_identity()
        );
        assert_eq!(
            ordinary.declaration().query_declaration_digest(),
            advanced.declaration().query_declaration_digest()
        );
        assert_eq!(
            ordinary.support().query_support_digest(),
            advanced.support().query_support_digest()
        );
        assert_eq!(
            ordinary
                .left()
                .workload()
                .response()
                .identity()
                .receipt_identity(),
            advanced
                .left()
                .workload()
                .response()
                .identity()
                .receipt_identity()
        );
        assert_eq!(
            ordinary
                .right()
                .workload()
                .response()
                .identity()
                .receipt_identity(),
            advanced
                .right()
                .workload()
                .response()
                .identity()
                .receipt_identity()
        );
    });
}

#[test]
fn common_plane_reduction_request_rejects_mismatched_pair_construction_receipt() {
    super::run_with_large_stack(|| {
        let pair = WorkloadCatalog::planar_boolean_clean_planar_body_pair()
            .declared("phase7.1 pair one")
            .build()
            .expect("first catalog pair should build");
        let other_pair = WorkloadCatalog::planar_boolean_clean_planar_body_pair()
            .declared("phase7.1 pair two")
            .build()
            .expect("second catalog pair should build");

        let error = PlanarBooleanCommonPlaneReductionRequest::from_built_pair_and_construction(
            pair,
            other_pair.construction_receipt(),
        )
        .expect_err("mismatched pair-construction proof must be rejected");

        match error {
            PlanarBooleanCommonPlaneReductionRequestError::OperandPairIdentityMismatch {
                ref expected_operand_pair_identity,
                ref actual_operand_pair_identity,
            } => {
                assert_ne!(
                    expected_operand_pair_identity, actual_operand_pair_identity,
                    "mismatch denial must preserve both sides of the identity disagreement"
                );
                assert!(!expected_operand_pair_identity.trim().is_empty());
                assert!(!actual_operand_pair_identity.trim().is_empty());
            }
            PlanarBooleanCommonPlaneReductionRequestError::DeclarationOperandPairIdentityMismatch {
                ..
            } => {
                panic!("pair-construction mismatch should not classify as declaration mismatch")
            }
        }
        assert!(error
            .human_reason()
            .contains("same admitted boolean operand pair"));
        assert!(error.expected_operand_pair_identity().is_some());
        assert!(error.actual_operand_pair_identity().is_some());
    });
}

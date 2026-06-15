use worth_kernel::workload_composition::{
    PlanarBooleanCommonPlaneAdmittedOperandScope, PlanarBooleanCommonPlaneReductionRequest,
    PlanarBooleanCommonPlaneScopeAdmissionError, PlanarBooleanCommonPlaneScopeAdmittedRequest,
    WorkloadCatalog, WorkloadCatalogRecipeKind,
};

#[test]
fn common_plane_scope_admission_preserves_one_narrow_admitted_shape() {
    super::run_with_large_stack(|| {
        let ordinary_pair = WorkloadCatalog::planar_boolean_clean_planar_body_pair()
            .declared("phase7.1 scope admission convergence")
            .build()
            .expect("ordinary clean planar body pair should build");
        let advanced_pair = WorkloadCatalog::planar_boolean_clean_planar_body_pair()
            .declared("phase7.1 scope admission convergence")
            .build()
            .expect("advanced clean planar body pair should build");

        let ordinary = PlanarBooleanCommonPlaneScopeAdmittedRequest::from_reduction_request(
            PlanarBooleanCommonPlaneReductionRequest::from_operand_pair_recipe(ordinary_pair)
                .expect("ordinary reduction request should build"),
        )
        .expect("ordinary clean planar body pair should admit");
        let advanced = PlanarBooleanCommonPlaneScopeAdmittedRequest::from_reduction_request(
            PlanarBooleanCommonPlaneReductionRequest::from_built_pair_and_construction(
                advanced_pair.clone(),
                advanced_pair.construction_receipt(),
            )
            .expect("advanced reduction request should build"),
        )
        .expect("advanced clean planar body pair should admit");

        assert_eq!(
            ordinary.admitted_scope(),
            PlanarBooleanCommonPlaneAdmittedOperandScope::ClosedPlanarBodyPair
        );
        assert_eq!(
            advanced.admitted_scope(),
            PlanarBooleanCommonPlaneAdmittedOperandScope::ClosedPlanarBodyPair
        );
        assert_eq!(
            ordinary.admitted_scope(),
            advanced.admitted_scope(),
            "equivalent admitted recipes must converge to the same admitted scope class"
        );
        assert_eq!(
            ordinary.scope_admission_identity(),
            advanced.scope_admission_identity(),
            "ordinary and advanced request construction must converge to one scope-admission identity"
        );
        assert_eq!(
            ordinary.request_identity(),
            ordinary.reduction_request().request_identity()
        );
        assert_eq!(
            advanced.request_identity(),
            advanced.reduction_request().request_identity()
        );
        assert_eq!(
            ordinary.operand_pair_identity(),
            advanced.operand_pair_identity()
        );
        assert!(!ordinary.scope_admission_identity().trim().is_empty());
        assert!(!advanced.scope_admission_identity().trim().is_empty());
    });
}

#[test]
fn common_plane_scope_admission_denies_non_clean_pair_families_before_plane_work() {
    super::run_with_large_stack(|| {
        for (recipe, expected_recipe) in [
            (
                WorkloadCatalog::planar_boolean_coplanar_overlap_pair()
                    .declared("phase7.1 scope denial overlap")
                    .build()
                    .expect("coplanar overlap pair should build"),
                WorkloadCatalogRecipeKind::BooleanCoplanarOverlapPair,
            ),
            (
                WorkloadCatalog::planar_boolean_thin_feature_pair()
                    .declared("phase7.1 scope denial thin")
                    .build()
                    .expect("thin feature pair should build"),
                WorkloadCatalogRecipeKind::BooleanThinFeaturePair,
            ),
            (
                WorkloadCatalog::planar_boolean_high_valence_contact_pair()
                    .declared("phase7.1 scope denial high valence")
                    .build()
                    .expect("high valence contact pair should build"),
                WorkloadCatalogRecipeKind::BooleanHighValenceContactPair,
            ),
        ] {
            let error = PlanarBooleanCommonPlaneScopeAdmittedRequest::from_reduction_request(
                PlanarBooleanCommonPlaneReductionRequest::from_operand_pair_recipe(recipe)
                    .expect("reduction request should build"),
            )
            .expect_err("unsupported pair family must deny during scope admission");

            assert_eq!(error.actual_recipe(), expected_recipe);
            assert_eq!(
                error.admitted_scope(),
                "closed planar body pair admitted for common-plane reduction"
            );
            assert!(!error.request_identity().trim().is_empty());
            assert!(!error.operand_pair_identity().trim().is_empty());
            assert!(matches!(
                error,
                PlanarBooleanCommonPlaneScopeAdmissionError::UnsupportedOperandPairRecipe { .. }
            ));
            assert!(error.human_reason().contains("before plane agreement"));
        }
    });
}

#[test]
fn common_plane_scope_admission_denial_class_converges_across_request_construction_paths() {
    super::run_with_large_stack(|| {
        let ordinary_pair = WorkloadCatalog::planar_boolean_thin_feature_pair()
            .declared("phase7.1 scope denial convergence")
            .build()
            .expect("ordinary thin-feature pair should build");
        let advanced_pair = WorkloadCatalog::planar_boolean_thin_feature_pair()
            .declared("phase7.1 scope denial convergence")
            .build()
            .expect("advanced thin-feature pair should build");

        let ordinary_error = PlanarBooleanCommonPlaneScopeAdmittedRequest::from_reduction_request(
            PlanarBooleanCommonPlaneReductionRequest::from_operand_pair_recipe(ordinary_pair)
                .expect("ordinary reduction request should build"),
        )
        .expect_err("ordinary thin-feature pair must deny during scope admission");
        let advanced_error = PlanarBooleanCommonPlaneScopeAdmittedRequest::from_reduction_request(
            PlanarBooleanCommonPlaneReductionRequest::from_built_pair_and_construction(
                advanced_pair.clone(),
                advanced_pair.construction_receipt(),
            )
            .expect("advanced reduction request should build"),
        )
        .expect_err("advanced thin-feature pair must deny during scope admission");

        assert!(matches!(
            ordinary_error,
            PlanarBooleanCommonPlaneScopeAdmissionError::UnsupportedOperandPairRecipe { .. }
        ));
        assert!(matches!(
            advanced_error,
            PlanarBooleanCommonPlaneScopeAdmissionError::UnsupportedOperandPairRecipe { .. }
        ));
        assert_eq!(
            ordinary_error.actual_recipe(),
            advanced_error.actual_recipe()
        );
        assert_eq!(
            ordinary_error.admitted_scope(),
            advanced_error.admitted_scope()
        );
        assert_eq!(
            ordinary_error.request_identity(),
            advanced_error.request_identity(),
            "ordinary and advanced request construction must preserve one denial request identity"
        );
        assert_eq!(
            ordinary_error.operand_pair_identity(),
            advanced_error.operand_pair_identity(),
            "ordinary and advanced denial paths must preserve one operand-pair identity"
        );
    });
}

use std::f64::consts::TAU;

use worth_geom::facade::ParameterDomain;
use worth_spatial::facade::bindings::{
    author_primitive_rebinding_declaration, BindingContinuityClass, NeighborhoodBindingFamily,
};

use super::proof_fixture::{
    anchored_curved_surface, anchored_curved_surface_declaration, anchored_ellipsoid_surface,
    anchored_ellipsoid_surface_declaration,
};
use crate::binding::tests::support::{
    admitted_rebinding_handle, anchored_surface_candidate_from_declaration,
    anchored_surface_prior_fact_from_declaration, canonical_geometry,
    certification_bundle_for_pair, rebind_surface_on_face, replacement_neighborhood,
    scoped_branch_head_inspection_basis, triaxial_ellipsoid_geometry,
};

#[test]
fn binding_layer_certification_bundle_proves_curved_and_asymmetric_pressure_do_not_reopen_earlier_shortcuts(
) {
    let curved_prior = anchored_curved_surface_declaration(
        "face-curved",
        ParameterDomain::cylinder(),
        [TAU + 0.25, 0.5],
        canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
    );
    let curved_exact = anchored_curved_surface_declaration(
        "face-curved",
        ParameterDomain::cylinder(),
        [0.25, 0.5],
        canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
    );
    let curved_planarized = anchored_curved_surface_declaration(
        "face-curved",
        ParameterDomain::plane(),
        [0.25, 0.5],
        canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
    );
    let curved_bundle = certification_bundle_for_pair(
        admitted_rebinding_handle("rebinding-closeout-curved"),
        scoped_branch_head_inspection_basis("branch:rebinding-closeout-curved"),
        author_primitive_rebinding_declaration(
            crate::binding::tests::support::replace_surface_binding(
                anchored_surface_prior_fact_from_declaration(
                    &curved_prior,
                    "curved-closeout-left-prior",
                ),
                replacement_neighborhood(
                    NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                    "face-curved",
                    vec![
                        anchored_surface_candidate_from_declaration(
                            "planarized",
                            &curved_planarized,
                            "curved-closeout-left-planarized",
                        )
                        .expect("planarized"),
                        anchored_surface_candidate_from_declaration(
                            "canonical",
                            &curved_exact,
                            "curved-closeout-left-canonical",
                        )
                        .expect("canonical"),
                    ],
                ),
            ),
        ),
        author_primitive_rebinding_declaration(
            crate::binding::tests::support::replace_surface_binding(
                anchored_surface_prior_fact_from_declaration(
                    &curved_prior,
                    "curved-closeout-right-prior",
                ),
                replacement_neighborhood(
                    NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                    "face-curved",
                    vec![
                        anchored_surface_candidate_from_declaration(
                            "canonical",
                            &curved_exact,
                            "curved-closeout-right-canonical",
                        )
                        .expect("canonical"),
                        anchored_surface_candidate_from_declaration(
                            "planarized",
                            &curved_planarized,
                            "curved-closeout-right-planarized",
                        )
                        .expect("planarized"),
                    ],
                ),
            ),
        ),
        "curved-left",
        "curved-right",
    );

    assert_eq!(
        anchored_curved_surface(
            "face-curved",
            ParameterDomain::cylinder(),
            [TAU + 0.25, 0.5],
            canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]])
        ),
        anchored_curved_surface(
            "face-curved",
            ParameterDomain::cylinder(),
            [0.25, 0.5],
            canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]])
        )
    );
    assert_ne!(
        anchored_curved_surface(
            "face-curved",
            ParameterDomain::cylinder(),
            [TAU + 0.25, 0.5],
            canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]])
        ),
        anchored_curved_surface(
            "face-curved",
            ParameterDomain::plane(),
            [0.25, 0.5],
            canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]])
        )
    );
    assert_eq!(
        curved_bundle.deterministic_outcome_class(),
        worth_spatial::facade::bindings::RebindingOutcomeClass::Preserved
    );
    assert_eq!(
        curved_bundle.deterministic_continuity_class(),
        BindingContinuityClass::Exact
    );
    assert_eq!(
        rebind_surface_on_face(
            anchored_surface_prior_fact_from_declaration(
                &curved_prior,
                "curved-closeout-direct-planarized-prior",
            ),
            replacement_neighborhood(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-curved",
                vec![anchored_surface_candidate_from_declaration(
                    "planarized",
                    &curved_planarized,
                    "curved-closeout-direct-planarized",
                )
                .expect("planarized")],
            ),
        )
        .expect("planarized decision")
        .outcome_class(),
        worth_spatial::facade::bindings::RebindingOutcomeClass::CorrespondenceOnly
    );

    assert_asymmetric_closeout_pressure();
}

fn assert_asymmetric_closeout_pressure() {
    let asymmetric_prior = anchored_ellipsoid_surface_declaration(
        "face-ellipsoid",
        triaxial_ellipsoid_geometry(
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
            [5.0, 3.0, 2.0],
            [[5.0, 0.0, 0.0], [0.0, 3.0, 0.0]],
        ),
    );
    let asymmetric_exact = anchored_ellipsoid_surface_declaration(
        "face-ellipsoid",
        triaxial_ellipsoid_geometry(
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
            [5.0, 3.0, 2.0],
            [[5.0, 0.0, 0.0], [0.0, 3.0, 0.0]],
        ),
    );
    let asymmetric_swapped = anchored_ellipsoid_surface_declaration(
        "face-ellipsoid",
        triaxial_ellipsoid_geometry(
            [1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [0.0, 1.0, 0.0],
            [5.0, 2.0, 3.0],
            [[5.0, 0.0, 0.0], [0.0, 0.0, 2.0]],
        ),
    );
    let asymmetric_bundle = certification_bundle_for_pair(
        admitted_rebinding_handle("rebinding-closeout-asymmetric"),
        scoped_branch_head_inspection_basis("branch:rebinding-closeout-asymmetric"),
        author_primitive_rebinding_declaration(
            crate::binding::tests::support::replace_surface_binding(
                anchored_surface_prior_fact_from_declaration(
                    &asymmetric_prior,
                    "asymmetric-closeout-left-prior",
                ),
                replacement_neighborhood(
                    NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                    "face-ellipsoid",
                    vec![
                        anchored_surface_candidate_from_declaration(
                            "axis-swapped",
                            &asymmetric_swapped,
                            "asymmetric-closeout-left-swapped",
                        )
                        .expect("swapped"),
                        anchored_surface_candidate_from_declaration(
                            "exact",
                            &asymmetric_exact,
                            "asymmetric-closeout-left-exact",
                        )
                        .expect("exact"),
                    ],
                ),
            ),
        ),
        author_primitive_rebinding_declaration(
            crate::binding::tests::support::replace_surface_binding(
                anchored_surface_prior_fact_from_declaration(
                    &asymmetric_prior,
                    "asymmetric-closeout-right-prior",
                ),
                replacement_neighborhood(
                    NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                    "face-ellipsoid",
                    vec![
                        anchored_surface_candidate_from_declaration(
                            "exact",
                            &asymmetric_exact,
                            "asymmetric-closeout-right-exact",
                        )
                        .expect("exact"),
                        anchored_surface_candidate_from_declaration(
                            "axis-swapped",
                            &asymmetric_swapped,
                            "asymmetric-closeout-right-swapped",
                        )
                        .expect("swapped"),
                    ],
                ),
            ),
        ),
        "asymmetric-left",
        "asymmetric-right",
    );

    assert_ne!(
        anchored_ellipsoid_surface(
            "face-ellipsoid",
            triaxial_ellipsoid_geometry(
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 0.0, 1.0],
                [5.0, 3.0, 2.0],
                [[5.0, 0.0, 0.0], [0.0, 3.0, 0.0]],
            ),
        ),
        anchored_ellipsoid_surface(
            "face-ellipsoid",
            triaxial_ellipsoid_geometry(
                [1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0],
                [0.0, 1.0, 0.0],
                [5.0, 2.0, 3.0],
                [[5.0, 0.0, 0.0], [0.0, 0.0, 2.0]],
            ),
        )
    );
    assert_eq!(
        asymmetric_bundle.deterministic_outcome_class(),
        worth_spatial::facade::bindings::RebindingOutcomeClass::Preserved
    );
    assert_eq!(
        asymmetric_bundle.deterministic_continuity_class(),
        BindingContinuityClass::Exact
    );
    assert_eq!(
        rebind_surface_on_face(
            anchored_surface_prior_fact_from_declaration(
                &asymmetric_prior,
                "asymmetric-closeout-direct-swapped-prior-a",
            ),
            replacement_neighborhood(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-ellipsoid",
                vec![anchored_surface_candidate_from_declaration(
                    "swapped",
                    &asymmetric_swapped,
                    "asymmetric-closeout-direct-swapped-a",
                )
                .expect("swapped")],
            ),
        )
        .expect("swapped decision")
        .continuity_class(),
        BindingContinuityClass::CorrespondenceOnly
    );
    assert_eq!(
        rebind_surface_on_face(
            anchored_surface_prior_fact_from_declaration(
                &asymmetric_prior,
                "asymmetric-closeout-direct-swapped-prior-b",
            ),
            replacement_neighborhood(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-ellipsoid",
                vec![anchored_surface_candidate_from_declaration(
                    "swapped",
                    &asymmetric_swapped,
                    "asymmetric-closeout-direct-swapped-b",
                )
                .expect("swapped")],
            ),
        )
        .expect("swapped decision")
        .outcome_class(),
        worth_spatial::facade::bindings::RebindingOutcomeClass::CorrespondenceOnly
    );
}

use std::f64::consts::TAU;

use worth_geom::facade::{ParameterDomain, ParameterSpacePoint};
use worth_primitives::{PrimitiveConstructionFamilyContractRegistry, PrimitiveWitnessDescriptor};
use worth_spatial::facade::bindings::{
    author_primitive_anchor_binding_declaration, author_primitive_rebinding_declaration,
    AnchorCarrierOwnership, AuthorPrimitiveAnchorBindingIntent, BindingContinuityClass,
    CarrierOwnedParameterPointAnchorSpec, FaceBindingSite, FaceSurfaceBindingSpec,
    NeighborhoodBindingFamily, PrimitiveAnchorBindingDeclarationEntry, VertexBindingSite,
    VertexGeometryBindingSpec, VertexGeometryProvenanceKind, VertexToleranceRegime,
};

use super::super::support::{
    admitted_rebinding_handle, anchored_surface_candidate_from_declaration,
    anchored_surface_prior_fact_from_declaration, canonical_geometry,
    certification_bundle_for_pair, orthotope_contract, rebind_surface_on_face,
    rebinding_candidate_from_binding_declaration, rebinding_prior_fact_from_binding_declaration,
    rebinding_receipt_for_entry, replacement_neighborhood, scoped_branch_head_inspection_basis,
    triaxial_ellipsoid_geometry,
};

#[test]
fn binding_layer_certification_bundle_proves_determinism_replay_and_inspection_parity_under_hostile_order_variation(
) {
    let prior = anchored_planar_surface("face-old", [0.25, 0.5], 1.0);
    let exact = anchored_planar_surface("face-new-a", [0.25, 0.5], 1.0);
    let weaker = anchored_planar_surface("face-new-b", [0.25, 0.5], 2.0);
    let left_entry = author_primitive_rebinding_declaration(
        crate::binding::tests::support::replace_surface_binding(
            anchored_surface_prior_fact_from_declaration(&prior, "closeout-left-prior"),
            replacement_neighborhood(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-old",
                vec![
                    anchored_surface_candidate_from_declaration(
                        "weaker",
                        &weaker,
                        "closeout-left-weaker",
                    )
                    .expect("weaker"),
                    anchored_surface_candidate_from_declaration(
                        "exact",
                        &exact,
                        "closeout-left-exact",
                    )
                    .expect("exact"),
                ],
            ),
        ),
    );
    let right_entry = author_primitive_rebinding_declaration(
        crate::binding::tests::support::replace_surface_binding(
            anchored_surface_prior_fact_from_declaration(&prior, "closeout-right-prior"),
            replacement_neighborhood(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-old",
                vec![
                    anchored_surface_candidate_from_declaration(
                        "exact",
                        &exact,
                        "closeout-right-exact",
                    )
                    .expect("exact"),
                    anchored_surface_candidate_from_declaration(
                        "weaker",
                        &weaker,
                        "closeout-right-weaker",
                    )
                    .expect("weaker"),
                ],
            ),
        ),
    );
    let handle = admitted_rebinding_handle("phase-sixteen-host-order");
    let branch_basis = scoped_branch_head_inspection_basis("branch:phase-sixteen-host-order");
    let bundle = certification_bundle_for_pair(
        handle,
        branch_basis,
        left_entry.clone(),
        right_entry.clone(),
        "branch-evidence:left",
        "branch-evidence:right",
    );

    assert_eq!(
        bundle.deterministic_outcome_class(),
        worth_spatial::facade::bindings::RebindingOutcomeClass::Ambiguous
    );
    assert_eq!(
        bundle.deterministic_continuity_class(),
        BindingContinuityClass::Ambiguous
    );
    assert_eq!(
        bundle.binding_identity(),
        rebinding_receipt_for_entry(&left_entry, "closeout-left")
            .expect("left receipt")
            .prior_binding_identity()
    );
    assert!(bundle.selected_candidate_identity().is_none());
    assert!(!bundle.historical_digest().is_empty());
    assert!(!bundle.historical_inspection_digest().is_empty());
    assert!(!bundle.branch_local_digest().is_empty());
    assert!(!bundle.branch_local_inspection_digest().is_empty());
    assert!(!bundle.replay_digest().is_empty());
    assert_eq!(bundle.replay_ordinary_kind(), "ambiguous");
    assert!(!bundle.report_digest().is_empty());

    let denied_left = author_primitive_rebinding_declaration(
        crate::binding::tests::support::replace_surface_binding(
            rebinding_prior_fact_from_binding_declaration(
                &vertex_binding_declaration("vertex-old"),
                "closeout-denied-left-prior",
            ),
            replacement_neighborhood(
                NeighborhoodBindingFamily::VertexGeometry,
                "vertex-old",
                vec![
                    rebinding_candidate_from_binding_declaration(
                        "a",
                        &vertex_binding_declaration("vertex-new-a"),
                        "closeout-denied-left-a",
                    )
                    .expect("a"),
                    rebinding_candidate_from_binding_declaration(
                        "b",
                        &vertex_binding_declaration("vertex-new-b"),
                        "closeout-denied-left-b",
                    )
                    .expect("b"),
                ],
            ),
        ),
    );
    let denied_right = author_primitive_rebinding_declaration(
        crate::binding::tests::support::replace_surface_binding(
            rebinding_prior_fact_from_binding_declaration(
                &vertex_binding_declaration("vertex-old"),
                "closeout-denied-right-prior",
            ),
            replacement_neighborhood(
                NeighborhoodBindingFamily::VertexGeometry,
                "vertex-old",
                vec![
                    rebinding_candidate_from_binding_declaration(
                        "b",
                        &vertex_binding_declaration("vertex-new-b"),
                        "closeout-denied-right-b",
                    )
                    .expect("b"),
                    rebinding_candidate_from_binding_declaration(
                        "a",
                        &vertex_binding_declaration("vertex-new-a"),
                        "closeout-denied-right-a",
                    )
                    .expect("a"),
                ],
            ),
        ),
    );
    let denied_bundle = certification_bundle_for_pair(
        admitted_rebinding_handle("phase-sixteen-host-order-denied"),
        scoped_branch_head_inspection_basis("branch:phase-sixteen-host-order-denied"),
        denied_left,
        denied_right,
        "branch-evidence:denied-left",
        "branch-evidence:denied-right",
    );

    assert_eq!(
        denied_bundle.deterministic_outcome_class(),
        worth_spatial::facade::bindings::RebindingOutcomeClass::Unsupported
    );
    assert_eq!(denied_bundle.replay_ordinary_kind(), "unsupported");
}

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
        admitted_rebinding_handle("phase-sixteen-curved-closeout"),
        scoped_branch_head_inspection_basis("branch:phase-sixteen-curved"),
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
        admitted_rebinding_handle("phase-sixteen-asymmetric-closeout"),
        scoped_branch_head_inspection_basis("branch:phase-sixteen-asymmetric"),
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

fn anchored_planar_surface(
    face_id: &str,
    point: [f64; 2],
    width: f64,
) -> PrimitiveAnchorBindingDeclarationEntry {
    anchored_curved_surface_declaration(
        face_id,
        ParameterDomain::plane(),
        point,
        canonical_geometry([[0.0, 0.0, 0.0], [width, 0.0, 0.0]]),
    )
}

fn anchored_curved_surface_declaration(
    face_id: &str,
    domain: ParameterDomain,
    point: [f64; 2],
    geometry: worth_primitives::PrimitiveGeometryIdentityBundle,
) -> PrimitiveAnchorBindingDeclarationEntry {
    author_primitive_anchor_binding_declaration(
        AuthorPrimitiveAnchorBindingIntent::attach_parameter_space_point_to_face(
            FaceSurfaceBindingSpec::new(
                FaceBindingSite::new(face_id),
                orthotope_contract(),
                geometry,
            ),
            CarrierOwnedParameterPointAnchorSpec::new(
                AnchorCarrierOwnership::for_face_surface(face_id, domain).expect("ownership"),
                ParameterSpacePoint::try_new(point).expect("parameter"),
            )
            .expect("anchor"),
        ),
    )
}

fn anchored_curved_surface(
    face_id: &str,
    domain: ParameterDomain,
    point: [f64; 2],
    geometry: worth_primitives::PrimitiveGeometryIdentityBundle,
) -> String {
    anchored_curved_surface_runtime(face_id, domain, point, geometry)
}

fn anchored_curved_surface_runtime(
    face_id: &str,
    domain: ParameterDomain,
    point: [f64; 2],
    geometry: worth_primitives::PrimitiveGeometryIdentityBundle,
) -> String {
    anchored_surface_prior_fact_from_declaration(
        &anchored_curved_surface_declaration(face_id, domain, point, geometry),
        "binding-layer-closeout-curved-identity",
    )
    .prior_binding_identity()
    .to_string()
}

fn anchored_ellipsoid_surface(
    face_id: &str,
    geometry: worth_primitives::PrimitiveGeometryIdentityBundle,
) -> String {
    anchored_curved_surface_runtime(
        face_id,
        ParameterDomain::triaxial_ellipsoid(),
        [0.25, 0.4],
        geometry,
    )
}

fn anchored_ellipsoid_surface_declaration(
    face_id: &str,
    geometry: worth_primitives::PrimitiveGeometryIdentityBundle,
) -> PrimitiveAnchorBindingDeclarationEntry {
    anchored_curved_surface_declaration(
        face_id,
        ParameterDomain::triaxial_ellipsoid(),
        [0.25, 0.4],
        geometry,
    )
}

fn vertex_binding_declaration(
    vertex_id: &str,
) -> worth_spatial::facade::bindings::PrimitiveBindingDeclarationEntry {
    worth_spatial::facade::bindings::author_primitive_binding_declaration(
        worth_spatial::facade::bindings::AuthorPrimitiveBindingIntent::attach_vertex_geometry(
            VertexGeometryBindingSpec::new(
                VertexBindingSite::new(vertex_id),
                PrimitiveConstructionFamilyContractRegistry::contract_for(
                    &PrimitiveWitnessDescriptor::Orthotope,
                ),
                canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
                VertexGeometryProvenanceKind::CanonicalWitness,
                VertexToleranceRegime::ExactBits,
            ),
        ),
    )
}

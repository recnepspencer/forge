use std::f64::consts::TAU;

use worth_geom::facade::{ParameterDomain, ParameterSpacePoint};
use worth_primitives::{PrimitiveConstructionFamilyContractRegistry, PrimitiveWitnessDescriptor};
use worth_spatial::facade::bindings::{
    attach_parameter_space_point_to_face, attach_vertex_geometry, evaluate_continuity,
    rebind_surface_on_face, AnchorCarrierOwnership, BindingContinuityClass,
    CarrierOwnedParameterPointAnchorSpec, FaceBindingSite, FaceSurfaceBindingSpec,
    NeighborhoodBindingFamily, ReplacementCandidate, SpatialAdmittedPrimitiveBinding,
    VertexBindingSite, VertexGeometryBindingSpec, VertexGeometryProvenanceKind,
    VertexToleranceRegime,
};

use crate::facade::authoring::binding::{
    author_primitive_rebinding_declaration, AuthorPrimitiveRebindingIntent,
};

use super::super::support::{
    admitted_rebinding_handle, canonical_geometry, certification_bundle_for_pair,
    orthotope_contract, replacement_neighborhood, scoped_branch_head_inspection_basis,
    triaxial_ellipsoid_geometry,
};

#[test]
fn binding_layer_certification_bundle_proves_determinism_replay_and_inspection_parity_under_hostile_order_variation(
) {
    let prior = anchored_planar_surface("face-old", [0.25, 0.5], 1.0);
    let exact = anchored_planar_surface("face-new-a", [0.25, 0.5], 1.0);
    let weaker = anchored_planar_surface("face-new-b", [0.25, 0.5], 2.0);
    let left_entry = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_surface_binding(
            prior.clone(),
            replacement_neighborhood(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-old",
                vec![
                    ReplacementCandidate::new("weaker", weaker.clone()).expect("weaker"),
                    ReplacementCandidate::new("exact", exact.clone()).expect("exact"),
                ],
            ),
        ),
    );
    let right_entry = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_surface_binding(
            prior,
            replacement_neighborhood(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-old",
                vec![
                    ReplacementCandidate::new("exact", exact).expect("exact"),
                    ReplacementCandidate::new("weaker", weaker).expect("weaker"),
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
        left_entry
            .clone()
            .admit()
            .expect("left decision")
            .explanation()
            .prior_identity()
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
        AuthorPrimitiveRebindingIntent::replace_surface_binding(
            vertex_binding("vertex-old"),
            replacement_neighborhood(
                NeighborhoodBindingFamily::VertexGeometry,
                "vertex-old",
                vec![
                    ReplacementCandidate::new("a", vertex_binding("vertex-new-a")).expect("a"),
                    ReplacementCandidate::new("b", vertex_binding("vertex-new-b")).expect("b"),
                ],
            ),
        ),
    );
    let denied_right = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_surface_binding(
            vertex_binding("vertex-old"),
            replacement_neighborhood(
                NeighborhoodBindingFamily::VertexGeometry,
                "vertex-old",
                vec![
                    ReplacementCandidate::new("b", vertex_binding("vertex-new-b")).expect("b"),
                    ReplacementCandidate::new("a", vertex_binding("vertex-new-a")).expect("a"),
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
    let curved_prior = anchored_curved_surface(
        "face-curved",
        ParameterDomain::cylinder(),
        [TAU + 0.25, 0.5],
        canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
    );
    let curved_exact = anchored_curved_surface(
        "face-curved",
        ParameterDomain::cylinder(),
        [0.25, 0.5],
        canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
    );
    let curved_planarized = anchored_curved_surface(
        "face-curved",
        ParameterDomain::plane(),
        [0.25, 0.5],
        canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
    );
    let curved_bundle = certification_bundle_for_pair(
        admitted_rebinding_handle("phase-sixteen-curved-closeout"),
        scoped_branch_head_inspection_basis("branch:phase-sixteen-curved"),
        author_primitive_rebinding_declaration(
            AuthorPrimitiveRebindingIntent::replace_surface_binding(
                curved_prior.clone(),
                replacement_neighborhood(
                    NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                    "face-curved",
                    vec![
                        ReplacementCandidate::new("planarized", curved_planarized.clone())
                            .expect("planarized"),
                        ReplacementCandidate::new("canonical", curved_exact.clone())
                            .expect("canonical"),
                    ],
                ),
            ),
        ),
        author_primitive_rebinding_declaration(
            AuthorPrimitiveRebindingIntent::replace_surface_binding(
                curved_prior.clone(),
                replacement_neighborhood(
                    NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                    "face-curved",
                    vec![
                        ReplacementCandidate::new("canonical", curved_exact.clone())
                            .expect("canonical"),
                        ReplacementCandidate::new("planarized", curved_planarized.clone())
                            .expect("planarized"),
                    ],
                ),
            ),
        ),
        "curved-left",
        "curved-right",
    );

    assert_eq!(curved_prior.identity(), curved_exact.identity());
    assert_ne!(curved_prior.identity(), curved_planarized.identity());
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
            curved_prior,
            replacement_neighborhood(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-curved",
                vec![
                    ReplacementCandidate::new("planarized", curved_planarized).expect("planarized")
                ],
            ),
        )
        .expect("planarized decision")
        .outcome_class(),
        worth_spatial::facade::bindings::RebindingOutcomeClass::CorrespondenceOnly
    );

    let asymmetric_prior = anchored_ellipsoid_surface(
        "face-ellipsoid",
        triaxial_ellipsoid_geometry(
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
            [5.0, 3.0, 2.0],
            [[5.0, 0.0, 0.0], [0.0, 3.0, 0.0]],
        ),
    );
    let asymmetric_exact = anchored_ellipsoid_surface(
        "face-ellipsoid",
        triaxial_ellipsoid_geometry(
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
            [5.0, 3.0, 2.0],
            [[5.0, 0.0, 0.0], [0.0, 3.0, 0.0]],
        ),
    );
    let asymmetric_swapped = anchored_ellipsoid_surface(
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
            AuthorPrimitiveRebindingIntent::replace_surface_binding(
                asymmetric_prior.clone(),
                replacement_neighborhood(
                    NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                    "face-ellipsoid",
                    vec![
                        ReplacementCandidate::new("axis-swapped", asymmetric_swapped.clone())
                            .expect("swapped"),
                        ReplacementCandidate::new("exact", asymmetric_exact.clone())
                            .expect("exact"),
                    ],
                ),
            ),
        ),
        author_primitive_rebinding_declaration(
            AuthorPrimitiveRebindingIntent::replace_surface_binding(
                asymmetric_prior.clone(),
                replacement_neighborhood(
                    NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                    "face-ellipsoid",
                    vec![
                        ReplacementCandidate::new("exact", asymmetric_exact.clone())
                            .expect("exact"),
                        ReplacementCandidate::new("axis-swapped", asymmetric_swapped.clone())
                            .expect("swapped"),
                    ],
                ),
            ),
        ),
        "asymmetric-left",
        "asymmetric-right",
    );

    assert_ne!(asymmetric_prior.identity(), asymmetric_swapped.identity());
    assert_eq!(
        asymmetric_bundle.deterministic_outcome_class(),
        worth_spatial::facade::bindings::RebindingOutcomeClass::Preserved
    );
    assert_eq!(
        asymmetric_bundle.deterministic_continuity_class(),
        BindingContinuityClass::Exact
    );
    assert_eq!(
        evaluate_continuity(
            &asymmetric_prior,
            &replacement_neighborhood(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-ellipsoid",
                vec![
                    ReplacementCandidate::new("swapped", asymmetric_swapped.clone())
                        .expect("swapped")
                ],
            ),
        )
        .expect("swapped continuity")
        .continuity_class(),
        BindingContinuityClass::CorrespondenceOnly
    );
    assert_eq!(
        rebind_surface_on_face(
            asymmetric_prior,
            replacement_neighborhood(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-ellipsoid",
                vec![ReplacementCandidate::new("swapped", asymmetric_swapped).expect("swapped")],
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
) -> SpatialAdmittedPrimitiveBinding {
    anchored_curved_surface(
        face_id,
        ParameterDomain::plane(),
        point,
        canonical_geometry([[0.0, 0.0, 0.0], [width, 0.0, 0.0]]),
    )
}

fn anchored_curved_surface(
    face_id: &str,
    domain: ParameterDomain,
    point: [f64; 2],
    geometry: worth_primitives::PrimitiveGeometryIdentityBundle,
) -> SpatialAdmittedPrimitiveBinding {
    SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(
        attach_parameter_space_point_to_face(
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
        )
        .expect("binding"),
    )
}

fn anchored_ellipsoid_surface(
    face_id: &str,
    geometry: worth_primitives::PrimitiveGeometryIdentityBundle,
) -> SpatialAdmittedPrimitiveBinding {
    anchored_curved_surface(
        face_id,
        ParameterDomain::triaxial_ellipsoid(),
        [0.25, 0.4],
        geometry,
    )
}

fn vertex_binding(vertex_id: &str) -> SpatialAdmittedPrimitiveBinding {
    SpatialAdmittedPrimitiveBinding::VertexGeometry(
        attach_vertex_geometry(VertexGeometryBindingSpec::new(
            VertexBindingSite::new(vertex_id),
            PrimitiveConstructionFamilyContractRegistry::contract_for(
                &PrimitiveWitnessDescriptor::Orthotope,
            ),
            canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
            VertexGeometryProvenanceKind::CanonicalWitness,
            VertexToleranceRegime::ExactBits,
        ))
        .expect("vertex geometry binding"),
    )
}

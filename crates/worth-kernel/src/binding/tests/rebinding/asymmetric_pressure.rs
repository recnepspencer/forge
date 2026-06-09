use forge_query::facade::ForgeQueryOrdinaryOutcome;
use worth_geom::facade::ParameterDomain;
use worth_spatial::facade::bindings::{
    author_primitive_anchor_binding_declaration, author_primitive_rebinding_declaration,
    AnchorCarrierOwnership, AuthorPrimitiveAnchorBindingIntent, BindingContinuityClass,
    CarrierOwnedParameterPointAnchorSpec, FaceBindingSite, FaceSurfaceBindingSpec,
    LocalTopologyReplacementNeighborhood, NeighborhoodBindingFamily,
    PrimitiveAnchorBindingDeclarationEntry, RebindingOutcomeClass,
};

use super::super::support::{
    admitted_rebinding_handle, anchored_surface_candidate_from_declaration,
    anchored_surface_prior_fact_from_declaration, inspect_progressed_rebinding_entry,
    orthotope_contract, progress_rebinding_entry, rebind_surface_on_face,
    rebinding_ordinary_outcome_for_entry, rebinding_receipt_for_entry, replacement_neighborhood,
    triaxial_ellipsoid_geometry,
};

fn anchored_face_binding_declaration(
    face_id: &str,
    geometry_identity: worth_primitives::PrimitiveGeometryIdentityBundle,
) -> PrimitiveAnchorBindingDeclarationEntry {
    author_primitive_anchor_binding_declaration(
        AuthorPrimitiveAnchorBindingIntent::attach_parameter_space_point_to_face(
            FaceSurfaceBindingSpec::new(
                FaceBindingSite::new(face_id),
                orthotope_contract(),
                geometry_identity,
            ),
            CarrierOwnedParameterPointAnchorSpec::new(
                AnchorCarrierOwnership::for_face_surface(
                    face_id,
                    ParameterDomain::triaxial_ellipsoid(),
                )
                .expect("ownership"),
                worth_geom::facade::ParameterSpacePoint::try_new([0.25, 0.4]).expect("parameter"),
            )
            .expect("anchor spec"),
        ),
    )
}

fn admitted_face_surface_point_anchor_identity(
    declaration: &PrimitiveAnchorBindingDeclarationEntry,
) -> String {
    crate::binding::tests::support::anchored_surface_prior_fact_from_declaration(
        declaration,
        "asymmetric-pressure-anchor-identity",
    )
    .prior_binding_identity()
    .to_string()
}

#[test]
fn triaxial_ellipsoid_breaks_symmetry_dependent_binding_identity_and_anchor_reuse() {
    let handle = admitted_rebinding_handle("asymmetric-identity-pressure");
    let canonical = anchored_face_binding_declaration(
        "face-ellipsoid",
        triaxial_ellipsoid_geometry(
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
            [5.0, 3.0, 2.0],
            [[5.0, 0.0, 0.0], [0.0, 3.0, 0.0]],
        ),
    );
    let axis_swapped = anchored_face_binding_declaration(
        "face-ellipsoid",
        triaxial_ellipsoid_geometry(
            [1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [0.0, 1.0, 0.0],
            [5.0, 2.0, 3.0],
            [[5.0, 0.0, 0.0], [0.0, 0.0, 2.0]],
        ),
    );
    let neighborhood = replacement_neighborhood(
        NeighborhoodBindingFamily::FaceSurfacePointAnchor,
        "face-ellipsoid",
        vec![anchored_surface_candidate_from_declaration(
            "axis-swapped",
            &axis_swapped,
            "asymmetric-identity-axis-swapped",
        )
        .expect("candidate")],
    );
    let rebinding_entry = author_primitive_rebinding_declaration(
        crate::binding::tests::support::replace_surface_binding(
            anchored_surface_prior_fact_from_declaration(
                &canonical,
                "asymmetric-identity-canonical-prior",
            ),
            neighborhood.clone(),
        ),
    );
    let progression = progress_rebinding_entry(&rebinding_entry, &handle);
    let inspection = inspect_progressed_rebinding_entry(&handle, progression.clone());
    let outcome = rebinding_ordinary_outcome_for_entry(&rebinding_entry, &handle);
    let decision = rebinding_receipt_for_entry(&rebinding_entry, "asymmetric-identity-decision")
        .expect("decision");

    assert_ne!(
        admitted_face_surface_point_anchor_identity(&canonical),
        admitted_face_surface_point_anchor_identity(&axis_swapped)
    );
    assert_eq!(
        continuity_class_for_surface_rebinding(&canonical, neighborhood),
        BindingContinuityClass::CorrespondenceOnly
    );
    assert_eq!(
        decision.continuity_class(),
        BindingContinuityClass::CorrespondenceOnly
    );
    assert_eq!(
        Some(progression.progression_digest()),
        inspection.progression_digest()
    );
    assert!(matches!(outcome, ForgeQueryOrdinaryOutcome::Bound(_)));
}

#[test]
fn triaxial_ellipsoid_rebinding_and_continuity_do_not_reuse_axis_interchange_shortcuts() {
    let handle = admitted_rebinding_handle("asymmetric-rebinding-pressure");
    let prior = anchored_face_binding_declaration(
        "face-ellipsoid",
        triaxial_ellipsoid_geometry(
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
            [5.0, 3.0, 2.0],
            [[5.0, 0.0, 0.0], [0.0, 3.0, 0.0]],
        ),
    );
    let exact = anchored_face_binding_declaration(
        "face-ellipsoid",
        triaxial_ellipsoid_geometry(
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
            [5.0, 3.0, 2.0],
            [[5.0, 0.0, 0.0], [0.0, 3.0, 0.0]],
        ),
    );
    let axis_swapped = anchored_face_binding_declaration(
        "face-ellipsoid",
        triaxial_ellipsoid_geometry(
            [1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [0.0, 1.0, 0.0],
            [5.0, 2.0, 3.0],
            [[5.0, 0.0, 0.0], [0.0, 0.0, 2.0]],
        ),
    );

    let axis_swapped_entry = author_primitive_rebinding_declaration(
        crate::binding::tests::support::replace_surface_binding(
            anchored_surface_prior_fact_from_declaration(&prior, "asymmetric-rebinding-prior"),
            replacement_neighborhood(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-ellipsoid",
                vec![anchored_surface_candidate_from_declaration(
                    "axis-swapped",
                    &axis_swapped,
                    "asymmetric-rebinding-axis-swapped",
                )
                .expect("candidate")],
            ),
        ),
    );
    let axis_swapped_progression = progress_rebinding_entry(&axis_swapped_entry, &handle);
    let axis_swapped_inspection =
        inspect_progressed_rebinding_entry(&handle, axis_swapped_progression.clone());
    let axis_swapped_outcome = rebinding_ordinary_outcome_for_entry(&axis_swapped_entry, &handle);
    let axis_swapped_decision =
        rebinding_receipt_for_entry(&axis_swapped_entry, "asymmetric-axis-swapped")
            .expect("decision");

    let exact_decision = rebind_surface_on_face(
        anchored_surface_prior_fact_from_declaration(&prior, "asymmetric-pressure-direct-prior"),
        replacement_neighborhood(
            NeighborhoodBindingFamily::FaceSurfacePointAnchor,
            "face-ellipsoid",
            vec![anchored_surface_candidate_from_declaration(
                "exact",
                &exact,
                "asymmetric-direct-exact",
            )
            .expect("candidate")],
        ),
    )
    .expect("exact decision");

    assert_eq!(
        exact_decision.outcome_class(),
        RebindingOutcomeClass::Preserved
    );
    assert_eq!(
        axis_swapped_decision.outcome_class(),
        RebindingOutcomeClass::CorrespondenceOnly
    );
    assert_eq!(
        axis_swapped_decision.continuity_class(),
        BindingContinuityClass::CorrespondenceOnly
    );
    assert_eq!(
        Some(axis_swapped_progression.progression_digest()),
        axis_swapped_inspection.progression_digest()
    );
    assert!(matches!(
        axis_swapped_outcome,
        ForgeQueryOrdinaryOutcome::Bound(_)
    ));
    assert_ne!(
        exact_decision.selected_candidate_identity(),
        axis_swapped_decision.selected_candidate_identity()
    );
}

fn continuity_class_for_surface_rebinding(
    prior_binding: &PrimitiveAnchorBindingDeclarationEntry,
    neighborhood: LocalTopologyReplacementNeighborhood,
) -> BindingContinuityClass {
    let entry = author_primitive_rebinding_declaration(
        crate::binding::tests::support::replace_surface_binding(
            anchored_surface_prior_fact_from_declaration(prior_binding, "asymmetric-surface-prior"),
            neighborhood,
        ),
    );
    rebinding_receipt_for_entry(&entry, "asymmetric-surface")
        .expect("surface rebinding receipt")
        .continuity_class()
}

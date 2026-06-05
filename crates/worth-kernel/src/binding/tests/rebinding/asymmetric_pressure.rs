use forge_query::facade::ForgeQueryOrdinaryOutcome;
use worth_geom::facade::ParameterDomain;
use worth_spatial::facade::bindings::{
    attach_parameter_space_point_to_face, evaluate_continuity, rebind_surface_on_face,
    AnchorCarrierOwnership, BindingContinuityClass, CarrierOwnedParameterPointAnchorSpec,
    FaceBindingSite, FaceSurfaceBindingSpec, LocalTopologyReplacementNeighborhood,
    NeighborhoodBindingFamily, RebindingOutcomeClass, ReplacementCandidate,
    ReplacementCandidateSet, SpatialAdmittedPrimitiveBinding,
};

use crate::facade::authoring::binding::{
    author_primitive_rebinding_declaration, AuthorPrimitiveRebindingIntent,
};

use super::super::support::{
    admitted_rebinding_handle, inspect_progressed_rebinding_entry, orthotope_contract,
    progress_rebinding_entry, triaxial_ellipsoid_geometry,
};

fn anchored_face_binding(
    face_id: &str,
    geometry_identity: worth_primitives::PrimitiveGeometryIdentityBundle,
) -> SpatialAdmittedPrimitiveBinding {
    SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(
        attach_parameter_space_point_to_face(
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
        )
        .expect("anchored face binding"),
    )
}

#[test]
fn triaxial_ellipsoid_breaks_symmetry_dependent_binding_identity_and_anchor_reuse() {
    let handle = admitted_rebinding_handle("asymmetric-identity-pressure");
    let canonical = anchored_face_binding(
        "face-ellipsoid",
        triaxial_ellipsoid_geometry(
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
            [5.0, 3.0, 2.0],
            [[5.0, 0.0, 0.0], [0.0, 3.0, 0.0]],
        ),
    );
    let axis_swapped = anchored_face_binding(
        "face-ellipsoid",
        triaxial_ellipsoid_geometry(
            [1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [0.0, 1.0, 0.0],
            [5.0, 2.0, 3.0],
            [[5.0, 0.0, 0.0], [0.0, 0.0, 2.0]],
        ),
    );
    let neighborhood = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::FaceSurfacePointAnchor,
        "face-ellipsoid",
        ReplacementCandidateSet::new(vec![ReplacementCandidate::new(
            "axis-swapped",
            axis_swapped.clone(),
        )
        .expect("candidate")])
        .expect("candidate set"),
    )
    .expect("neighborhood");
    let rebinding_entry = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_surface_binding(
            canonical.clone(),
            neighborhood.clone(),
        ),
    );
    let progression = progress_rebinding_entry(&rebinding_entry, &handle);
    let inspection = inspect_progressed_rebinding_entry(&handle, progression.clone());
    let outcome = rebinding_entry.ordinary_outcome_with_query(&handle);
    let decision = rebinding_entry.clone().admit().expect("decision");

    assert_ne!(canonical.identity(), axis_swapped.identity());
    assert_eq!(
        evaluate_continuity(&canonical, &neighborhood)
            .expect("continuity")
            .continuity_class(),
        BindingContinuityClass::CorrespondenceOnly
    );
    assert_eq!(
        decision.explanation().continuity_class(),
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
    let prior = anchored_face_binding(
        "face-ellipsoid",
        triaxial_ellipsoid_geometry(
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
            [5.0, 3.0, 2.0],
            [[5.0, 0.0, 0.0], [0.0, 3.0, 0.0]],
        ),
    );
    let exact = anchored_face_binding(
        "face-ellipsoid",
        triaxial_ellipsoid_geometry(
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
            [5.0, 3.0, 2.0],
            [[5.0, 0.0, 0.0], [0.0, 3.0, 0.0]],
        ),
    );
    let axis_swapped = anchored_face_binding(
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
        AuthorPrimitiveRebindingIntent::replace_surface_binding(
            prior.clone(),
            LocalTopologyReplacementNeighborhood::new(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-ellipsoid",
                ReplacementCandidateSet::new(vec![ReplacementCandidate::new(
                    "axis-swapped",
                    axis_swapped.clone(),
                )
                .expect("candidate")])
                .expect("candidate set"),
            )
            .expect("swapped neighborhood"),
        ),
    );
    let axis_swapped_progression = progress_rebinding_entry(&axis_swapped_entry, &handle);
    let axis_swapped_inspection =
        inspect_progressed_rebinding_entry(&handle, axis_swapped_progression.clone());
    let axis_swapped_outcome = axis_swapped_entry.ordinary_outcome_with_query(&handle);
    let axis_swapped_decision = axis_swapped_entry.clone().admit().expect("decision");

    let exact_decision = rebind_surface_on_face(
        prior,
        LocalTopologyReplacementNeighborhood::new(
            NeighborhoodBindingFamily::FaceSurfacePointAnchor,
            "face-ellipsoid",
            ReplacementCandidateSet::new(vec![
                ReplacementCandidate::new("exact", exact.clone()).expect("candidate")
            ])
            .expect("candidate set"),
        )
        .expect("exact neighborhood"),
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
        axis_swapped_decision.explanation().continuity_class(),
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
        exact_decision.selected_binding().unwrap().identity(),
        axis_swapped_decision.selected_binding().unwrap().identity()
    );
}

#![cfg(test)]

use worth_geom::facade::{ParameterDomain, ParameterSpacePoint};
use worth_primitives::{
    PrimitiveConstructionFamilyContractRegistry, PrimitiveCurvedSupportIdentity,
    PrimitiveGeometryIdentityBundle, PrimitiveTriaxialEllipsoidIdentity, PrimitiveVertexIdentity,
    PrimitiveWitnessDescriptor,
};

use crate::facade::bindings::{
    attach_parameter_space_point_to_face, evaluate_continuity, rebind_surface_on_face,
    AnchorCarrierOwnership, BindingContinuityClass, CarrierOwnedParameterPointAnchorSpec,
    FaceBindingSite, FaceSurfaceBindingSpec, LocalTopologyReplacementNeighborhood,
    NeighborhoodBindingFamily, RebindingOutcomeClass, ReplacementCandidate,
    ReplacementCandidateSet, SpatialAdmittedPrimitiveBinding,
};

fn triaxial_ellipsoid_geometry(
    axis_u: [f64; 3],
    axis_v: [f64; 3],
    axis_w: [f64; 3],
    radii: [f64; 3],
) -> PrimitiveGeometryIdentityBundle {
    PrimitiveGeometryIdentityBundle::with_curved_support(
        vec![],
        vec![PrimitiveCurvedSupportIdentity::TriaxialEllipsoid(
            PrimitiveTriaxialEllipsoidIdentity::new(
                [0.0, 0.0, 0.0],
                axis_u,
                axis_v,
                axis_w,
                radii[0],
                radii[1],
                radii[2],
            ),
        )],
        vec![
            PrimitiveVertexIdentity::from_position([radii[0], 0.0, 0.0]),
            PrimitiveVertexIdentity::from_position([0.0, radii[1], 0.0]),
        ],
    )
}

fn anchored_face_binding(
    face_id: &str,
    geometry_identity: PrimitiveGeometryIdentityBundle,
) -> SpatialAdmittedPrimitiveBinding {
    let contract = PrimitiveConstructionFamilyContractRegistry::contract_for(
        &PrimitiveWitnessDescriptor::Orthotope,
    );
    SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(
        attach_parameter_space_point_to_face(
            FaceSurfaceBindingSpec::new(FaceBindingSite::new(face_id), contract, geometry_identity),
            CarrierOwnedParameterPointAnchorSpec::new(
                AnchorCarrierOwnership::for_face_surface(
                    face_id,
                    ParameterDomain::triaxial_ellipsoid(),
                )
                .expect("ownership"),
                ParameterSpacePoint::try_new([0.25, 0.4]).expect("parameter"),
            )
            .expect("anchor spec"),
        )
        .expect("anchored face binding"),
    )
}

#[test]
fn triaxial_ellipsoid_breaks_symmetry_dependent_binding_identity_and_anchor_reuse() {
    let canonical = anchored_face_binding(
        "face-ellipsoid",
        triaxial_ellipsoid_geometry(
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
            [5.0, 3.0, 2.0],
        ),
    );
    let axis_swapped = anchored_face_binding(
        "face-ellipsoid",
        triaxial_ellipsoid_geometry(
            [1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [0.0, 1.0, 0.0],
            [5.0, 2.0, 3.0],
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

    assert_eq!(canonical.completeness().is_complete(), true);
    assert_eq!(axis_swapped.completeness().is_complete(), true);
    assert_ne!(canonical.identity(), axis_swapped.identity());
    assert_eq!(
        evaluate_continuity(&canonical, &neighborhood)
            .expect("continuity")
            .continuity_class(),
        BindingContinuityClass::CorrespondenceOnly
    );
}

#[test]
fn triaxial_ellipsoid_rebinding_and_continuity_do_not_reuse_axis_interchange_shortcuts() {
    let prior = anchored_face_binding(
        "face-ellipsoid",
        triaxial_ellipsoid_geometry(
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
            [5.0, 3.0, 2.0],
        ),
    );
    let exact = anchored_face_binding(
        "face-ellipsoid",
        triaxial_ellipsoid_geometry(
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
            [5.0, 3.0, 2.0],
        ),
    );
    let axis_swapped = anchored_face_binding(
        "face-ellipsoid",
        triaxial_ellipsoid_geometry(
            [1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [0.0, 1.0, 0.0],
            [5.0, 2.0, 3.0],
        ),
    );

    let exact_decision = rebind_surface_on_face(
        prior.clone(),
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
    let axis_swapped_decision = rebind_surface_on_face(
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
    )
    .expect("axis-swapped decision");

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
    assert_ne!(
        prior.identity(),
        axis_swapped_decision.selected_binding().unwrap().identity()
    );
}

#![cfg(test)]

use worth_geom::facade::{ParameterDomain, ParameterSpacePoint};
use worth_primitives::{
    PrimitiveConstructionFamilyContractRegistry, PrimitiveCurvedSupportIdentity,
    PrimitiveGeometryIdentityBundle, PrimitiveTriaxialEllipsoidIdentity, PrimitiveVertexIdentity,
    PrimitiveWitnessDescriptor,
};

use crate::bindings::anchors::{AnchorCarrierOwnership, CarrierOwnedParameterPointAnchorSpec};
use crate::bindings::authority::{FaceBindingSite, FaceSurfaceBindingSpec};
use crate::bindings::query_native_anchor_binding_authoring::{
    author_primitive_anchor_binding_declaration, AuthorPrimitiveAnchorBindingIntent,
};
use crate::bindings::query_native_declared_target_identity_fact::anchor_binding_declaration_fact;
use crate::bindings::rebinding::{
    evaluate_continuity_internal as evaluate_continuity, BindingContinuityClass,
    LocalTopologyReplacementNeighborhood, NeighborhoodBindingFamily, RebindingOutcomeClass,
    ReplacementCandidateSet,
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

fn anchored_face_binding_declaration(
    face_id: &str,
    geometry_identity: PrimitiveGeometryIdentityBundle,
) -> crate::bindings::query_native_anchor_binding_authoring::PrimitiveAnchorBindingDeclarationEntry
{
    let contract = PrimitiveConstructionFamilyContractRegistry::contract_for(
        &PrimitiveWitnessDescriptor::Orthotope,
    );
    author_primitive_anchor_binding_declaration(
        AuthorPrimitiveAnchorBindingIntent::attach_parameter_space_point_to_face(
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
        ),
    )
}

#[test]
fn triaxial_ellipsoid_breaks_symmetry_dependent_binding_identity_and_anchor_reuse() {
    let canonical_declaration = anchored_face_binding_declaration(
        "face-ellipsoid",
        triaxial_ellipsoid_geometry(
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
            [5.0, 3.0, 2.0],
        ),
    );
    let canonical_fact =
        anchor_binding_declaration_fact(&canonical_declaration).expect("canonical fact");
    let axis_swapped_declaration = anchored_face_binding_declaration(
        "face-ellipsoid",
        triaxial_ellipsoid_geometry(
            [1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [0.0, 1.0, 0.0],
            [5.0, 2.0, 3.0],
        ),
    );
    let axis_swapped_fact =
        anchor_binding_declaration_fact(&axis_swapped_declaration).expect("axis-swapped fact");
    let neighborhood = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::FaceSurfacePointAnchor,
        "face-ellipsoid",
        ReplacementCandidateSet::new(vec![super::rebinding_candidate_from_anchor_declaration(
            "axis-swapped",
            &axis_swapped_declaration,
            "asymmetric-pressure-axis-swapped-candidate",
        )
        .expect("candidate")])
        .expect("candidate set"),
    )
    .expect("neighborhood");

    assert!(canonical_fact.completeness().is_complete());
    assert!(axis_swapped_fact.completeness().is_complete());
    assert_ne!(
        canonical_fact.binding_identity().as_str(),
        axis_swapped_fact.binding_identity().as_str()
    );
    assert_eq!(
        evaluate_continuity(
            &super::rebinding_prior_fact_from_anchor_declaration(
                &canonical_declaration,
                "asymmetric-pressure-canonical-prior",
            ),
            &neighborhood,
        )
        .expect("continuity")
        .continuity_class(),
        BindingContinuityClass::CorrespondenceOnly
    );
}

#[test]
fn triaxial_ellipsoid_rebinding_and_continuity_do_not_reuse_axis_interchange_shortcuts() {
    let prior_declaration = anchored_face_binding_declaration(
        "face-ellipsoid",
        triaxial_ellipsoid_geometry(
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
            [5.0, 3.0, 2.0],
        ),
    );
    let exact_declaration = anchored_face_binding_declaration(
        "face-ellipsoid",
        triaxial_ellipsoid_geometry(
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
            [5.0, 3.0, 2.0],
        ),
    );
    let axis_swapped_declaration = anchored_face_binding_declaration(
        "face-ellipsoid",
        triaxial_ellipsoid_geometry(
            [1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [0.0, 1.0, 0.0],
            [5.0, 2.0, 3.0],
        ),
    );
    let prior_identity = anchor_binding_declaration_fact(&prior_declaration)
        .expect("prior fact")
        .binding_identity()
        .as_str()
        .to_string();

    let exact_decision = super::rebind_surface_on_face_from_fact(
        super::rebinding_prior_fact_from_anchor_declaration(
            &prior_declaration,
            "asymmetric-pressure-exact-prior",
        ),
        LocalTopologyReplacementNeighborhood::new(
            NeighborhoodBindingFamily::FaceSurfacePointAnchor,
            "face-ellipsoid",
            ReplacementCandidateSet::new(vec![super::rebinding_candidate_from_anchor_declaration(
                "exact",
                &exact_declaration,
                "asymmetric-pressure-exact-candidate",
            )
            .expect("candidate")])
            .expect("candidate set"),
        )
        .expect("exact neighborhood"),
    )
    .expect("exact decision");
    let axis_swapped_decision = super::rebind_surface_on_face_from_fact(
        super::rebinding_prior_fact_from_anchor_declaration(
            &prior_declaration,
            "asymmetric-pressure-swapped-prior",
        ),
        LocalTopologyReplacementNeighborhood::new(
            NeighborhoodBindingFamily::FaceSurfacePointAnchor,
            "face-ellipsoid",
            ReplacementCandidateSet::new(vec![super::rebinding_candidate_from_anchor_declaration(
                "axis-swapped",
                &axis_swapped_declaration,
                "asymmetric-pressure-swapped-candidate",
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
        axis_swapped_decision.continuity_class(),
        BindingContinuityClass::CorrespondenceOnly
    );
    assert_ne!(
        prior_identity.as_str(),
        axis_swapped_decision.selected_candidate_identity().unwrap()
    );
}

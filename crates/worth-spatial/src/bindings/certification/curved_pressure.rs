#![cfg(test)]

use std::f64::consts::{PI, TAU};

use worth_geom::facade::{ParameterDomain, ParameterSpacePoint, PolygonalTrimmedParameterRegion};
use worth_primitives::{
    PrimitiveConstructionFamilyContractRegistry, PrimitiveGeometryIdentityBundle,
    PrimitiveSupportPlaneIdentity, PrimitiveVertexIdentity, PrimitiveWitnessDescriptor,
};

use crate::bindings::anchors::{
    AnchorCarrierOwnership, CarrierOwnedParameterPointAnchorSpec, SpatialAnchorAuthorityError,
};
use crate::bindings::authority::{FaceBindingSite, FaceSurfaceBindingSpec};
use crate::bindings::query_native_anchor_binding_authoring::{
    author_primitive_anchor_binding_declaration, AuthorPrimitiveAnchorBindingIntent,
};
use crate::bindings::query_native_declared_target_identity_fact::anchor_binding_declaration_fact;
use crate::bindings::rebinding::{
    evaluate_continuity_internal as evaluate_continuity, BindingContinuityClass,
    LocalTopologyReplacementNeighborhood, MotionAwareBindingPosture, NeighborhoodBindingFamily,
    RebindingOutcomeClass, ReplacementCandidateSet,
};

fn planar_geometry() -> PrimitiveGeometryIdentityBundle {
    PrimitiveGeometryIdentityBundle::new(
        vec![PrimitiveSupportPlaneIdentity::new(
            "0".to_string(),
            "0".to_string(),
            "1".to_string(),
            "0".to_string(),
        )],
        vec![
            PrimitiveVertexIdentity::from_position([0.0, 0.0, 0.0]),
            PrimitiveVertexIdentity::from_position([1.0, 0.0, 0.0]),
        ],
    )
}

fn anchored_face_binding_declaration(
    face_id: &str,
    domain: ParameterDomain,
    point: [f64; 2],
) -> crate::bindings::query_native_anchor_binding_authoring::PrimitiveAnchorBindingDeclarationEntry
{
    let contract = PrimitiveConstructionFamilyContractRegistry::contract_for(
        &PrimitiveWitnessDescriptor::Orthotope,
    );
    author_primitive_anchor_binding_declaration(
        AuthorPrimitiveAnchorBindingIntent::attach_parameter_space_point_to_face(
            FaceSurfaceBindingSpec::new(FaceBindingSite::new(face_id), contract, planar_geometry()),
            CarrierOwnedParameterPointAnchorSpec::new(
                AnchorCarrierOwnership::for_face_surface(face_id, domain).expect("ownership"),
                ParameterSpacePoint::try_new(point).expect("parameter"),
            )
            .expect("anchor spec"),
        ),
    )
}

fn trimmed_face_binding_declaration(
    face_id: &str,
    outer_boundary: [[f64; 2]; 4],
    point: [f64; 2],
) -> crate::bindings::query_native_anchor_binding_authoring::PrimitiveAnchorBindingDeclarationEntry
{
    let trimmed_region = PolygonalTrimmedParameterRegion::new(
        ParameterDomain::plane(),
        outer_boundary
            .into_iter()
            .map(|coords| ParameterSpacePoint::try_new(coords).expect("boundary point"))
            .collect(),
        vec![],
    )
    .expect("trimmed region");
    let contract = PrimitiveConstructionFamilyContractRegistry::contract_for(
        &PrimitiveWitnessDescriptor::Orthotope,
    );
    author_primitive_anchor_binding_declaration(
        AuthorPrimitiveAnchorBindingIntent::attach_parameter_space_point_to_face(
            FaceSurfaceBindingSpec::new(FaceBindingSite::new(face_id), contract, planar_geometry()),
            CarrierOwnedParameterPointAnchorSpec::new(
                AnchorCarrierOwnership::for_trimmed_face_surface(face_id, trimmed_region)
                    .expect("ownership"),
                ParameterSpacePoint::try_new(point).expect("parameter"),
            )
            .expect("anchor spec"),
        ),
    )
}

#[test]
fn curved_carrier_pressure_breaks_planar_anchor_and_rebinding_shortcuts() {
    let prior_declaration = anchored_face_binding_declaration(
        "face-curved",
        ParameterDomain::cylinder(),
        [TAU + 0.25, 0.5],
    );
    let preserved_declaration =
        anchored_face_binding_declaration("face-curved", ParameterDomain::cylinder(), [0.25, 0.5]);
    let planarized_declaration =
        anchored_face_binding_declaration("face-curved", ParameterDomain::plane(), [0.25, 0.5]);
    let prior_identity = anchor_binding_declaration_fact(&prior_declaration)
        .expect("prior fact")
        .binding_identity()
        .as_str()
        .to_string();
    let preserved_identity = anchor_binding_declaration_fact(&preserved_declaration)
        .expect("preserved fact")
        .binding_identity()
        .as_str()
        .to_string();

    let curved_neighborhood = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::FaceSurfacePointAnchor,
        "face-curved",
        ReplacementCandidateSet::new(vec![super::rebinding_candidate_from_anchor_declaration(
            "curved",
            &preserved_declaration,
            "curved-pressure-curved-candidate",
        )
        .expect("candidate")])
        .expect("candidate set"),
    )
    .expect("curved neighborhood");
    let curved_continuity = evaluate_continuity(
        &super::rebinding_prior_fact_from_anchor_declaration(
            &prior_declaration,
            "curved-pressure-curved-continuity-prior",
        ),
        &curved_neighborhood,
    )
    .expect("continuity");
    let curved_decision = super::rebind_surface_on_face_from_fact(
        super::rebinding_prior_fact_from_anchor_declaration(
            &prior_declaration,
            "curved-pressure-curved-prior",
        ),
        curved_neighborhood,
    )
    .expect("decision");

    let planarized_neighborhood = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::FaceSurfacePointAnchor,
        "face-curved",
        ReplacementCandidateSet::new(vec![super::rebinding_candidate_from_anchor_declaration(
            "planarized",
            &planarized_declaration,
            "curved-pressure-planarized-candidate",
        )
        .expect("candidate")])
        .expect("candidate set"),
    )
    .expect("planarized neighborhood");
    let planarized_continuity = evaluate_continuity(
        &super::rebinding_prior_fact_from_anchor_declaration(
            &prior_declaration,
            "curved-pressure-planarized-continuity-prior",
        ),
        &planarized_neighborhood,
    )
    .expect("continuity");
    let planarized_decision = super::rebind_surface_on_face_from_fact(
        super::rebinding_prior_fact_from_anchor_declaration(
            &prior_declaration,
            "curved-pressure-planarized-prior",
        ),
        planarized_neighborhood,
    )
    .expect("decision");

    assert_eq!(prior_identity, preserved_identity);
    assert_eq!(
        curved_continuity.continuity_class(),
        BindingContinuityClass::Exact
    );
    assert_eq!(
        curved_decision.outcome_class(),
        RebindingOutcomeClass::Preserved
    );
    assert_eq!(
        curved_decision.motion_posture(),
        MotionAwareBindingPosture::Unresolved
    );

    assert_ne!(
        prior_identity.as_str(),
        planarized_decision.selected_candidate_identity().unwrap()
    );
    assert_eq!(
        planarized_continuity.continuity_class(),
        BindingContinuityClass::CorrespondenceOnly
    );
    assert_eq!(
        planarized_decision.outcome_class(),
        RebindingOutcomeClass::CorrespondenceOnly
    );
    assert_eq!(
        planarized_decision.continuity_class(),
        BindingContinuityClass::CorrespondenceOnly
    );
}

#[test]
fn curved_binding_and_rebinding_do_not_fall_back_to_planarized_identity_or_domain_assumptions() {
    let periodic_declaration = anchored_face_binding_declaration(
        "face-periodic",
        ParameterDomain::cylinder(),
        [TAU + 0.25, 0.5],
    );
    let canonical_declaration = anchored_face_binding_declaration(
        "face-periodic",
        ParameterDomain::cylinder(),
        [0.25, 0.5],
    );
    let planarized_declaration =
        anchored_face_binding_declaration("face-periodic", ParameterDomain::plane(), [0.25, 0.5]);
    let trimmed_a_declaration = trimmed_face_binding_declaration(
        "face-trimmed",
        [[0.0, 0.0], [3.0, 0.0], [3.0, 3.0], [0.0, 3.0]],
        [1.0, 1.0],
    );
    let trimmed_b_declaration = trimmed_face_binding_declaration(
        "face-trimmed",
        [[0.5, 0.0], [3.0, 0.0], [3.0, 3.0], [0.5, 3.0]],
        [1.0, 1.0],
    );
    let periodic_identity = anchor_binding_declaration_fact(&periodic_declaration)
        .expect("periodic fact")
        .binding_identity()
        .as_str()
        .to_string();
    let canonical_identity = anchor_binding_declaration_fact(&canonical_declaration)
        .expect("canonical fact")
        .binding_identity()
        .as_str()
        .to_string();
    let planarized_identity = anchor_binding_declaration_fact(&planarized_declaration)
        .expect("planarized fact")
        .binding_identity()
        .as_str()
        .to_string();
    let trimmed_a_identity = anchor_binding_declaration_fact(&trimmed_a_declaration)
        .expect("trimmed a fact")
        .binding_identity()
        .as_str()
        .to_string();
    let trimmed_b_identity = anchor_binding_declaration_fact(&trimmed_b_declaration)
        .expect("trimmed b fact")
        .binding_identity()
        .as_str()
        .to_string();
    let trimmed_neighborhood = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::FaceSurfacePointAnchor,
        "face-trimmed",
        ReplacementCandidateSet::new(vec![super::rebinding_candidate_from_anchor_declaration(
            "trimmed-b",
            &trimmed_b_declaration,
            "curved-pressure-trimmed-b-candidate",
        )
        .expect("candidate")])
        .expect("candidate set"),
    )
    .expect("trimmed neighborhood");

    let denied = CarrierOwnedParameterPointAnchorSpec::new(
        AnchorCarrierOwnership::for_face_surface("face-sphere", ParameterDomain::sphere())
            .expect("ownership"),
        ParameterSpacePoint::try_new([0.25, PI]).expect("parameter"),
    )
    .expect_err("sphere latitude outside admitted domain should deny");

    assert_eq!(periodic_identity, canonical_identity);
    assert_ne!(periodic_identity, planarized_identity);
    assert_ne!(trimmed_a_identity, trimmed_b_identity);
    assert_eq!(
        evaluate_continuity(
            &super::rebinding_prior_fact_from_anchor_declaration(
                &trimmed_a_declaration,
                "curved-pressure-trimmed-continuity-prior",
            ),
            &trimmed_neighborhood,
        )
        .expect("trimmed continuity")
        .continuity_class(),
        BindingContinuityClass::CorrespondenceOnly
    );
    let trimmed_decision = super::rebind_surface_on_face_from_fact(
        super::rebinding_prior_fact_from_anchor_declaration(
            &trimmed_a_declaration,
            "curved-pressure-trimmed-prior",
        ),
        trimmed_neighborhood,
    )
    .expect("trimmed decision");
    assert_eq!(
        trimmed_decision.outcome_class(),
        RebindingOutcomeClass::CorrespondenceOnly
    );
    assert!(matches!(
        denied,
        SpatialAnchorAuthorityError::ParameterDomainViolation(_)
    ));
}

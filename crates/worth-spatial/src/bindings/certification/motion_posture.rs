#![cfg(test)]

use worth_primitives::{
    PrimitiveConstructionBirthSynopsisContract, PrimitiveConstructionFamilyContractRegistry,
    PrimitiveGeometryIdentityBundle, PrimitiveSupportPlaneIdentity, PrimitiveVertexIdentity,
    PrimitiveWitnessDescriptor,
};

use crate::bindings::authority::{
    CoedgeBindingSite, CoedgePCurveBindingSpec, EdgeBindingSite, EdgeCurveBindingSpec,
    FaceBindingSite, FaceSurfaceBindingSpec, VertexBindingSite, VertexGeometryBindingSpec,
    VertexGeometryProvenanceKind, VertexToleranceRegime,
};
use crate::bindings::query_native_binding_authoring::{
    author_primitive_binding_declaration, AuthorPrimitiveBindingIntent,
};
use crate::bindings::rebinding::{
    evaluate_binding_motion_posture_internal as evaluate_binding_motion_posture,
    BindingMotionSemanticsInput, LocalTopologyReplacementNeighborhood, MotionAwareBindingPosture,
    NeighborhoodBindingFamily, ReplacementCandidateSet,
};

fn plane_geometry(vertices: [[f64; 3]; 2]) -> PrimitiveGeometryIdentityBundle {
    PrimitiveGeometryIdentityBundle::new(
        vec![PrimitiveSupportPlaneIdentity::new(
            "0".to_string(),
            "0".to_string(),
            "1".to_string(),
            "0".to_string(),
        )],
        vertices
            .into_iter()
            .map(PrimitiveVertexIdentity::from_position)
            .collect(),
    )
}

fn surface_binding_declaration(
    face_id: &str,
    geometry: PrimitiveGeometryIdentityBundle,
) -> crate::bindings::query_native_binding_authoring::PrimitiveBindingDeclarationEntry {
    let contract = PrimitiveConstructionFamilyContractRegistry::contract_for(
        &PrimitiveWitnessDescriptor::Orthotope,
    );
    author_primitive_binding_declaration(AuthorPrimitiveBindingIntent::attach_surface_to_face(
        FaceSurfaceBindingSpec::new(FaceBindingSite::new(face_id), contract, geometry),
    ))
}

fn all_phase_six_bindings(
    contract: PrimitiveConstructionBirthSynopsisContract,
) -> [crate::bindings::query_native_binding_authoring::PrimitiveBindingDeclarationEntry; 4] {
    [
        author_primitive_binding_declaration(AuthorPrimitiveBindingIntent::attach_surface_to_face(
            FaceSurfaceBindingSpec::new(
                FaceBindingSite::new("face-1"),
                contract,
                plane_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
            ),
        )),
        author_primitive_binding_declaration(AuthorPrimitiveBindingIntent::attach_curve_to_edge(
            EdgeCurveBindingSpec::new(
                EdgeBindingSite::new("edge-1"),
                contract,
                plane_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
            ),
        )),
        author_primitive_binding_declaration(
            AuthorPrimitiveBindingIntent::attach_pcurve_to_coedge(CoedgePCurveBindingSpec::new(
                CoedgeBindingSite::new("coedge-1"),
                contract,
                plane_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
            )),
        ),
        author_primitive_binding_declaration(AuthorPrimitiveBindingIntent::attach_vertex_geometry(
            VertexGeometryBindingSpec::new(
                VertexBindingSite::new("vertex-1"),
                contract,
                plane_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
                VertexGeometryProvenanceKind::CanonicalWitness,
                VertexToleranceRegime::ExactBits,
            ),
        )),
    ]
}

#[test]
fn motion_aware_binding_posture_distinguishes_preserved_transformed_invalidated_and_unresolved() {
    let contract = PrimitiveConstructionFamilyContractRegistry::contract_for(
        &PrimitiveWitnessDescriptor::Orthotope,
    );
    let bindings = all_phase_six_bindings(contract);

    for declaration in bindings {
        let prior_fact = super::rebinding_prior_fact_from_binding_declaration(
            &declaration,
            "motion-posture-phase-six-prior",
        );
        assert_eq!(
            evaluate_binding_motion_posture(
                &prior_fact,
                BindingMotionSemanticsInput::rotated_with_carrier(0.0),
            )
            .expect("preserved"),
            MotionAwareBindingPosture::Preserved
        );
        assert_eq!(
            evaluate_binding_motion_posture(
                &prior_fact,
                BindingMotionSemanticsInput::moved_with_carrier(),
            )
            .expect("transformed"),
            MotionAwareBindingPosture::TransformedWithCarrier
        );
        assert_eq!(
            evaluate_binding_motion_posture(
                &prior_fact,
                BindingMotionSemanticsInput::reoriented_with_carrier(),
            )
            .expect("unresolved"),
            MotionAwareBindingPosture::Unresolved
        );
        assert_eq!(
            evaluate_binding_motion_posture(
                &prior_fact,
                BindingMotionSemanticsInput::invalidated_by_local_topology_replacement(),
            )
            .expect("invalidated"),
            MotionAwareBindingPosture::Invalidated
        );
    }
}

#[test]
fn motion_posture_is_not_rederived_from_rebinding_candidate_presence() {
    let prior_declaration = surface_binding_declaration(
        "face-old",
        plane_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
    );
    let successor_declaration = surface_binding_declaration(
        "face-new",
        plane_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
    );
    let alternate_declaration = surface_binding_declaration(
        "face-alt",
        plane_geometry([[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]]),
    );
    let neighborhood = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::FaceSurface,
        "face-old",
        ReplacementCandidateSet::new(vec![super::rebinding_candidate_from_binding_declaration(
            "successor",
            &successor_declaration,
            "motion-posture-successor-candidate",
        )
        .expect("candidate")])
        .expect("candidate set"),
    )
    .expect("neighborhood");
    let richer_neighborhood = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::FaceSurface,
        "face-old",
        ReplacementCandidateSet::new(vec![
            super::rebinding_candidate_from_binding_declaration(
                "successor",
                &surface_binding_declaration(
                    "face-new-rich",
                    plane_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
                ),
                "motion-posture-successor-rich-candidate",
            )
            .expect("candidate"),
            super::rebinding_candidate_from_binding_declaration(
                "alternate",
                &alternate_declaration,
                "motion-posture-alternate-candidate",
            )
            .expect("alternate"),
        ])
        .expect("candidate set"),
    )
    .expect("richer neighborhood");
    let alternate = super::rebinding_candidate_from_binding_declaration(
        "alternate",
        &alternate_declaration,
        "motion-posture-alternate-identity",
    )
    .expect("alternate candidate");
    let prior = super::rebinding_prior_fact_from_binding_declaration(
        &prior_declaration,
        "motion-posture-identity-prior",
    );
    let prior_fact = super::rebinding_prior_fact_from_binding_declaration(
        &prior_declaration,
        "motion-posture-invalidated-prior",
    );
    let invalidated = evaluate_binding_motion_posture(
        &prior_fact,
        BindingMotionSemanticsInput::invalidated_by_local_topology_replacement(),
    )
    .expect("invalidated");
    let rebinding = super::rebind_surface_on_face_from_fact(
        super::rebinding_prior_fact_from_binding_declaration(
            &prior_declaration,
            "motion-posture-prior",
        ),
        neighborhood,
    )
    .expect("rebinding");
    let richer_rebinding = super::rebind_surface_on_face_from_fact(
        super::rebinding_prior_fact_from_binding_declaration(
            &prior_declaration,
            "motion-posture-prior-rich",
        ),
        richer_neighborhood,
    )
    .expect("richer rebinding");

    assert_eq!(invalidated, MotionAwareBindingPosture::Invalidated);
    assert_eq!(
        rebinding.motion_posture(),
        MotionAwareBindingPosture::Unresolved
    );
    assert!(rebinding.selected_candidate_identity().is_some());
    assert_eq!(
        richer_rebinding.motion_posture(),
        MotionAwareBindingPosture::Unresolved
    );
    assert_ne!(rebinding.motion_posture(), invalidated);
    assert_eq!(
        rebinding.motion_posture(),
        richer_rebinding.motion_posture()
    );
    assert_ne!(alternate.binding_identity(), prior.prior_binding_identity());
}

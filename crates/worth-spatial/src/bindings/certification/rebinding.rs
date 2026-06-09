#![cfg(test)]

use worth_primitives::{
    PrimitiveConstructionFamilyContractRegistry, PrimitiveGeometryIdentityBundle,
    PrimitiveSupportPlaneIdentity, PrimitiveVertexIdentity, PrimitiveWitnessDescriptor,
};

use crate::bindings::authority::{
    EdgeBindingSite, EdgeCurveBindingSpec, FaceBindingSite, FaceSurfaceBindingSpec,
    VertexBindingSite, VertexGeometryBindingSpec, VertexGeometryProvenanceKind,
    VertexToleranceRegime,
};
use crate::bindings::query_native_binding_authoring::{
    author_primitive_binding_declaration, AuthorPrimitiveBindingIntent,
};
use crate::bindings::rebinding::{
    evaluate_continuity_internal as evaluate_continuity, BindingContinuityClass,
    LocalTopologyReplacementNeighborhood, MotionAwareBindingPosture, NeighborhoodBindingFamily,
    RebindingOutcomeClass, ReplacementCandidateSet,
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

fn edge_binding_declaration(
    edge_id: &str,
    geometry: PrimitiveGeometryIdentityBundle,
) -> crate::bindings::query_native_binding_authoring::PrimitiveBindingDeclarationEntry {
    let contract = PrimitiveConstructionFamilyContractRegistry::contract_for(
        &PrimitiveWitnessDescriptor::Orthotope,
    );
    author_primitive_binding_declaration(AuthorPrimitiveBindingIntent::attach_curve_to_edge(
        EdgeCurveBindingSpec::new(EdgeBindingSite::new(edge_id), contract, geometry),
    ))
}

fn vertex_binding_declaration(
    vertex_id: &str,
    geometry: PrimitiveGeometryIdentityBundle,
) -> crate::bindings::query_native_binding_authoring::PrimitiveBindingDeclarationEntry {
    let contract = PrimitiveConstructionFamilyContractRegistry::contract_for(
        &PrimitiveWitnessDescriptor::Orthotope,
    );
    author_primitive_binding_declaration(AuthorPrimitiveBindingIntent::attach_vertex_geometry(
        VertexGeometryBindingSpec::new(
            VertexBindingSite::new(vertex_id),
            contract,
            geometry,
            VertexGeometryProvenanceKind::CanonicalWitness,
            VertexToleranceRegime::ExactBits,
        ),
    ))
}

#[test]
fn rebinding_authority_keeps_candidate_order_out_of_surface_decisions() {
    let prior_declaration = surface_binding_declaration(
        "face-old",
        plane_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
    );
    let exact_declaration = surface_binding_declaration(
        "face-new-a",
        plane_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
    );
    let weaker_declaration = surface_binding_declaration(
        "face-new-b",
        plane_geometry([[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]]),
    );
    let left = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::FaceSurface,
        "face-old",
        ReplacementCandidateSet::new(vec![
            super::rebinding_candidate_from_binding_declaration(
                "weaker",
                &weaker_declaration,
                "rebinding-order-weaker-left",
            )
            .expect("weaker"),
            super::rebinding_candidate_from_binding_declaration(
                "exact",
                &exact_declaration,
                "rebinding-order-exact-left",
            )
            .expect("exact"),
        ])
        .expect("candidate set"),
    )
    .expect("left");
    let right = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::FaceSurface,
        "face-old",
        ReplacementCandidateSet::new(vec![
            super::rebinding_candidate_from_binding_declaration(
                "exact",
                &exact_declaration,
                "rebinding-order-exact-right",
            )
            .expect("exact"),
            super::rebinding_candidate_from_binding_declaration(
                "weaker",
                &weaker_declaration,
                "rebinding-order-weaker-right",
            )
            .expect("weaker"),
        ])
        .expect("candidate set"),
    )
    .expect("right");

    let left_decision = super::rebind_surface_on_face_from_fact(
        super::rebinding_prior_fact_from_binding_declaration(
            &prior_declaration,
            "rebinding-order-prior-left",
        ),
        left,
    )
    .expect("left decision");
    let right_decision = super::rebind_surface_on_face_from_fact(
        super::rebinding_prior_fact_from_binding_declaration(
            &prior_declaration,
            "rebinding-order-prior-right",
        ),
        right,
    )
    .expect("right decision");
    let prior = super::rebinding_prior_fact_from_binding_declaration(
        &prior_declaration,
        "rebinding-order-prior-identity",
    );
    let exact = super::rebinding_candidate_from_binding_declaration(
        "exact",
        &exact_declaration,
        "rebinding-order-exact-identity",
    )
    .expect("exact identity");
    let weaker = super::rebinding_candidate_from_binding_declaration(
        "weaker",
        &weaker_declaration,
        "rebinding-order-weaker-identity",
    )
    .expect("weaker identity");

    assert_eq!(
        exact.binding_identity(),
        right_decision.selected_candidate_identity().unwrap()
    );
    assert_eq!(weaker.binding_identity() != exact.binding_identity(), true);
    assert_eq!(
        left_decision.outcome_class(),
        RebindingOutcomeClass::ExactReattachment
    );
    assert_eq!(
        left_decision.outcome_class(),
        right_decision.outcome_class()
    );
    assert_eq!(
        left_decision.motion_posture(),
        MotionAwareBindingPosture::Unresolved
    );
    assert_eq!(
        left_decision.selected_candidate_identity(),
        right_decision.selected_candidate_identity()
    );
    assert_eq!(
        Some(exact.binding_identity()),
        left_decision.selected_candidate_identity()
    );
    assert_eq!(
        prior.prior_binding_identity() != exact.binding_identity(),
        true
    );
}

#[test]
fn rebinding_authority_keeps_edge_curve_ambiguity_typed() {
    let prior_declaration = edge_binding_declaration(
        "edge-old",
        plane_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
    );
    let a_declaration =
        edge_binding_declaration("edge-a", plane_geometry([[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]]));
    let b_declaration =
        edge_binding_declaration("edge-b", plane_geometry([[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]]));
    let neighborhood = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::EdgeCurve,
        "edge-old",
        ReplacementCandidateSet::new(vec![
            super::rebinding_candidate_from_binding_declaration(
                "a",
                &a_declaration,
                "rebinding-ambiguity-a",
            )
            .expect("a"),
            super::rebinding_candidate_from_binding_declaration(
                "b",
                &b_declaration,
                "rebinding-ambiguity-b",
            )
            .expect("b"),
        ])
        .expect("candidates"),
    )
    .expect("neighborhood");

    let decision = super::rebind_curve_on_edge_from_fact(
        super::rebinding_prior_fact_from_binding_declaration(
            &prior_declaration,
            "rebinding-ambiguity-prior",
        ),
        neighborhood,
    )
    .expect("decision");
    let prior = super::rebinding_prior_fact_from_binding_declaration(
        &prior_declaration,
        "rebinding-ambiguity-prior-identity",
    );
    let a = super::rebinding_candidate_from_binding_declaration(
        "a",
        &a_declaration,
        "rebinding-ambiguity-a-identity",
    )
    .expect("a identity");
    let b = super::rebinding_candidate_from_binding_declaration(
        "b",
        &b_declaration,
        "rebinding-ambiguity-b-identity",
    )
    .expect("b identity");

    assert_eq!(a.binding_identity() != b.binding_identity(), true);
    assert_eq!(prior.prior_binding_identity() != a.binding_identity(), true);
    assert_eq!(decision.outcome_class(), RebindingOutcomeClass::Ambiguous);
    assert_eq!(
        decision.motion_posture(),
        MotionAwareBindingPosture::Unresolved
    );
    assert!(decision.selected_candidate_identity().is_none());
}

#[test]
fn rebinding_authority_preserves_when_prior_binding_remains_in_local_neighborhood() {
    let prior_declaration = surface_binding_declaration(
        "face-old",
        plane_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
    );
    let neighborhood = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::FaceSurface,
        "face-old",
        ReplacementCandidateSet::new(vec![super::rebinding_candidate_from_binding_declaration(
            "preserved",
            &prior_declaration,
            "rebinding-preserved-candidate",
        )
        .expect("preserved")])
        .expect("candidate set"),
    )
    .expect("neighborhood");

    let decision = super::rebind_surface_on_face_from_fact(
        super::rebinding_prior_fact_from_binding_declaration(
            &prior_declaration,
            "rebinding-preserved-prior",
        ),
        neighborhood,
    )
    .expect("decision");
    assert_eq!(decision.outcome_class(), RebindingOutcomeClass::Preserved);
    assert_eq!(
        decision.motion_posture(),
        MotionAwareBindingPosture::Unresolved
    );
    assert_eq!(
        decision.selected_candidate_identity(),
        Some(
            super::rebinding_prior_fact_from_binding_declaration(
                &prior_declaration,
                "rebinding-preserved-prior-identity",
            )
            .prior_binding_identity()
        )
    );
}

#[test]
fn rebinding_continuity_preserves_partial_vs_denied_incomplete_distinction() {
    let prior_declaration = edge_binding_declaration(
        "edge-old",
        plane_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
    );
    let partial_declaration = edge_binding_declaration(
        "edge-partial",
        PrimitiveGeometryIdentityBundle::new(
            vec![],
            vec![PrimitiveVertexIdentity::from_position([0.0, 0.0, 0.0])],
        ),
    );
    let denied_declaration = edge_binding_declaration(
        "edge-denied",
        PrimitiveGeometryIdentityBundle::new(vec![], vec![]),
    );
    let neighborhood = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::EdgeCurve,
        "edge-old",
        ReplacementCandidateSet::new(vec![
            super::rebinding_candidate_from_binding_declaration(
                "partial",
                &partial_declaration,
                "rebinding-partial-candidate",
            )
            .expect("partial"),
            super::rebinding_candidate_from_binding_declaration(
                "denied",
                &denied_declaration,
                "rebinding-denied-candidate",
            )
            .expect("denied"),
        ])
        .expect("candidates"),
    )
    .expect("neighborhood");

    let continuity = evaluate_continuity(
        &super::rebinding_prior_fact_from_binding_declaration(
            &prior_declaration,
            "rebinding-partial-continuity-prior",
        ),
        &neighborhood,
    )
    .expect("continuity");
    let partial = super::rebinding_candidate_from_binding_declaration(
        "partial",
        &partial_declaration,
        "rebinding-partial-identity",
    )
    .expect("partial identity");
    let denied = super::rebinding_candidate_from_binding_declaration(
        "denied",
        &denied_declaration,
        "rebinding-denied-identity",
    )
    .expect("denied identity");

    assert_eq!(
        partial.binding_identity() != denied.binding_identity(),
        true
    );
    assert_eq!(
        continuity.continuity_class(),
        BindingContinuityClass::InsufficientEvidenceFromAdmittedPartial
    );
    let denied_only_neighborhood = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::EdgeCurve,
        "edge-old",
        ReplacementCandidateSet::new(vec![super::rebinding_candidate_from_binding_declaration(
            "denied",
            &edge_binding_declaration(
                "edge-denied-only",
                PrimitiveGeometryIdentityBundle::new(vec![], vec![]),
            ),
            "rebinding-denied-only-candidate",
        )
        .expect("denied only")])
        .expect("candidates"),
    )
    .expect("denied-only neighborhood");
    let denied_only_continuity = evaluate_continuity(
        &super::rebinding_prior_fact_from_binding_declaration(
            &prior_declaration,
            "rebinding-denied-only-continuity-prior",
        ),
        &denied_only_neighborhood,
    )
    .expect("denied-only continuity");
    assert_eq!(
        denied_only_continuity.continuity_class(),
        BindingContinuityClass::InsufficientEvidenceFromDeniedIncomplete
    );

    let decision = super::rebind_curve_on_edge_from_fact(
        super::rebinding_prior_fact_from_binding_declaration(
            &prior_declaration,
            "rebinding-orphaned-prior",
        ),
        neighborhood,
    )
    .expect("decision");
    assert_eq!(decision.outcome_class(), RebindingOutcomeClass::Orphaned);
}

#[test]
fn vertex_rebinding_uses_the_same_local_neighborhood_law_as_other_core_families() {
    let prior_declaration = vertex_binding_declaration(
        "vertex-old",
        plane_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
    );
    let successor_declaration = vertex_binding_declaration(
        "vertex-new",
        plane_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
    );
    let neighborhood = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::VertexGeometry,
        "vertex-old",
        ReplacementCandidateSet::new(vec![super::rebinding_candidate_from_binding_declaration(
            "successor",
            &successor_declaration,
            "rebinding-vertex-successor-candidate",
        )
        .expect("candidate")])
        .expect("candidate set"),
    )
    .expect("neighborhood");

    let decision = super::rebind_geometry_on_vertex_from_fact(
        super::rebinding_prior_fact_from_binding_declaration(
            &prior_declaration,
            "rebinding-vertex-prior",
        ),
        neighborhood,
    )
    .expect("decision");
    let prior = super::rebinding_prior_fact_from_binding_declaration(
        &prior_declaration,
        "rebinding-vertex-prior-identity",
    );
    let successor = super::rebinding_candidate_from_binding_declaration(
        "successor",
        &successor_declaration,
        "rebinding-vertex-successor-identity",
    )
    .expect("successor identity");

    assert_eq!(
        decision.outcome_class(),
        RebindingOutcomeClass::ExactReattachment
    );
    assert_eq!(
        decision.neighborhood_family(),
        NeighborhoodBindingFamily::VertexGeometry
    );
    assert_eq!(
        decision.selected_candidate_identity(),
        Some(successor.binding_identity())
    );
    assert_eq!(
        decision.motion_posture(),
        MotionAwareBindingPosture::Unresolved
    );
    assert_eq!(
        prior.prior_binding_identity() != successor.binding_identity(),
        true
    );
}

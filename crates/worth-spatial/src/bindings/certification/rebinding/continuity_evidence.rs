use worth_primitives::{PrimitiveGeometryIdentityBundle, PrimitiveVertexIdentity};

use crate::bindings::rebinding::{
    evaluate_continuity_internal as evaluate_continuity, BindingContinuityClass,
    LocalTopologyReplacementNeighborhood, NeighborhoodBindingFamily, RebindingOutcomeClass,
    ReplacementCandidateSet,
};

#[test]
fn rebinding_continuity_preserves_partial_vs_denied_incomplete_distinction() {
    let prior_declaration = super::edge_binding_declaration(
        "edge-old",
        super::plane_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
    );
    let partial_declaration = super::edge_binding_declaration(
        "edge-partial",
        PrimitiveGeometryIdentityBundle::new(
            vec![],
            vec![PrimitiveVertexIdentity::from_position([0.0, 0.0, 0.0])],
        ),
    );
    let denied_declaration = super::edge_binding_declaration(
        "edge-denied",
        PrimitiveGeometryIdentityBundle::new(vec![], vec![]),
    );
    let neighborhood = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::EdgeCurve,
        "edge-old",
        ReplacementCandidateSet::new(vec![
            super::super::rebinding_candidate_from_binding_declaration(
                "partial",
                &partial_declaration,
                "rebinding-partial-candidate",
            )
            .expect("partial"),
            super::super::rebinding_candidate_from_binding_declaration(
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
        &super::super::rebinding_prior_fact_from_binding_declaration(
            &prior_declaration,
            "rebinding-partial-continuity-prior",
        ),
        &neighborhood,
    )
    .expect("continuity");
    let partial = super::super::rebinding_candidate_from_binding_declaration(
        "partial",
        &partial_declaration,
        "rebinding-partial-identity",
    )
    .expect("partial identity");
    let denied = super::super::rebinding_candidate_from_binding_declaration(
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
    assert_denied_only_incomplete_evidence(&prior_declaration);

    let decision = super::super::rebind_curve_on_edge_from_fact(
        super::super::rebinding_prior_fact_from_binding_declaration(
            &prior_declaration,
            "rebinding-orphaned-prior",
        ),
        neighborhood,
    )
    .expect("decision");
    assert_eq!(decision.outcome_class(), RebindingOutcomeClass::Orphaned);
}

fn assert_denied_only_incomplete_evidence(
    prior_declaration: &crate::bindings::query_native_binding_authoring::PrimitiveBindingDeclarationEntry,
) {
    let denied_only_neighborhood = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::EdgeCurve,
        "edge-old",
        ReplacementCandidateSet::new(vec![
            super::super::rebinding_candidate_from_binding_declaration(
                "denied",
                &super::edge_binding_declaration(
                    "edge-denied-only",
                    PrimitiveGeometryIdentityBundle::new(vec![], vec![]),
                ),
                "rebinding-denied-only-candidate",
            )
            .expect("denied only"),
        ])
        .expect("candidates"),
    )
    .expect("denied-only neighborhood");
    let denied_only_continuity = evaluate_continuity(
        &super::super::rebinding_prior_fact_from_binding_declaration(
            prior_declaration,
            "rebinding-denied-only-continuity-prior",
        ),
        &denied_only_neighborhood,
    )
    .expect("denied-only continuity");
    assert_eq!(
        denied_only_continuity.continuity_class(),
        BindingContinuityClass::InsufficientEvidenceFromDeniedIncomplete
    );
}

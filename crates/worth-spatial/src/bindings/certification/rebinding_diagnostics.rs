#![cfg(test)]

use std::collections::BTreeSet;
use worth_primitives::{
    PrimitiveConstructionFamilyContractRegistry, PrimitiveGeometryIdentityBundle,
    PrimitiveVertexIdentity, PrimitiveWitnessDescriptor,
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
    LocalTopologyReplacementNeighborhood, NeighborhoodBindingFamily, RebindingOutcomeClass,
    ReplacementCandidateSet, UnsupportedRebindingReason,
};

fn plane_geometry(vertices: [[f64; 3]; 2]) -> worth_primitives::PrimitiveGeometryIdentityBundle {
    worth_primitives::PrimitiveGeometryIdentityBundle::new(
        vec![worth_primitives::PrimitiveSupportPlaneIdentity::new(
            "0".to_string(),
            "0".to_string(),
            "1".to_string(),
            "0".to_string(),
        )],
        vertices
            .into_iter()
            .map(worth_primitives::PrimitiveVertexIdentity::from_position)
            .collect(),
    )
}

fn surface_binding_declaration(
    face_id: &str,
    geometry: worth_primitives::PrimitiveGeometryIdentityBundle,
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
    geometry: worth_primitives::PrimitiveGeometryIdentityBundle,
) -> crate::bindings::query_native_binding_authoring::PrimitiveBindingDeclarationEntry {
    let contract = PrimitiveConstructionFamilyContractRegistry::contract_for(
        &PrimitiveWitnessDescriptor::Orthotope,
    );
    author_primitive_binding_declaration(AuthorPrimitiveBindingIntent::attach_curve_to_edge(
        EdgeCurveBindingSpec::new(EdgeBindingSite::new(edge_id), contract, geometry),
    ))
}

fn pcurve_binding_declaration(
    coedge_id: &str,
    contract: worth_primitives::PrimitiveConstructionBirthSynopsisContract,
    geometry: worth_primitives::PrimitiveGeometryIdentityBundle,
) -> crate::bindings::query_native_binding_authoring::PrimitiveBindingDeclarationEntry {
    author_primitive_binding_declaration(AuthorPrimitiveBindingIntent::attach_pcurve_to_coedge(
        crate::bindings::authority::CoedgePCurveBindingSpec::new(
            crate::bindings::authority::CoedgeBindingSite::new(coedge_id),
            contract,
            geometry,
        ),
    ))
}

fn vertex_binding_declaration(
    vertex_id: &str,
    geometry: worth_primitives::PrimitiveGeometryIdentityBundle,
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
fn continuity_classes_remain_typed_and_no_winner_diagnostics_remain_honest() {
    let contract = PrimitiveConstructionFamilyContractRegistry::contract_for(
        &PrimitiveWitnessDescriptor::Orthotope,
    );
    let prior_surface_declaration = surface_binding_declaration(
        "face-old",
        plane_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
    );
    let successor_surface_declaration = surface_binding_declaration(
        "face-successor",
        plane_geometry([[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]]),
    );
    let successor_neighborhood = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::FaceSurface,
        "face-old",
        ReplacementCandidateSet::new(vec![super::rebinding_candidate_from_binding_declaration(
            "successor",
            &successor_surface_declaration,
            "rebinding-diagnostics-successor-candidate",
        )
        .expect("candidate")])
        .expect("candidate set"),
    )
    .expect("neighborhood");
    let successor_continuity = evaluate_continuity(
        &super::rebinding_prior_fact_from_binding_declaration(
            &prior_surface_declaration,
            "rebinding-diagnostics-successor-continuity-prior",
        ),
        &successor_neighborhood,
    )
    .expect("continuity");
    let successor_decision = super::rebind_surface_on_face_from_fact(
        super::rebinding_prior_fact_from_binding_declaration(
            &prior_surface_declaration,
            "rebinding-diagnostics-successor-prior",
        ),
        successor_neighborhood,
    )
    .expect("decision");

    let prior_edge_declaration = edge_binding_declaration(
        "edge-old",
        plane_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
    );
    let partial_neighborhood = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::EdgeCurve,
        "edge-old",
        ReplacementCandidateSet::new(vec![super::rebinding_candidate_from_binding_declaration(
            "partial",
            &edge_binding_declaration(
                "edge-partial",
                PrimitiveGeometryIdentityBundle::new(
                    vec![],
                    vec![PrimitiveVertexIdentity::from_position([0.0, 0.0, 0.0])],
                ),
            ),
            "rebinding-diagnostics-partial-candidate",
        )
        .expect("candidate")])
        .expect("candidate set"),
    )
    .expect("partial neighborhood");
    let partial_continuity = evaluate_continuity(
        &super::rebinding_prior_fact_from_binding_declaration(
            &prior_edge_declaration,
            "rebinding-diagnostics-partial-continuity-prior",
        ),
        &partial_neighborhood,
    )
    .expect("partial continuity");

    let ambiguous_a_declaration =
        edge_binding_declaration("edge-a", plane_geometry([[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]]));
    let ambiguous_b_declaration =
        edge_binding_declaration("edge-b", plane_geometry([[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]]));
    let ambiguous_decision = super::rebind_curve_on_edge_from_fact(
        super::rebinding_prior_fact_from_binding_declaration(
            &prior_edge_declaration,
            "rebinding-diagnostics-ambiguous-prior",
        ),
        LocalTopologyReplacementNeighborhood::new(
            NeighborhoodBindingFamily::EdgeCurve,
            "edge-old",
            ReplacementCandidateSet::new(vec![
                super::rebinding_candidate_from_binding_declaration(
                    "a",
                    &ambiguous_a_declaration,
                    "rebinding-diagnostics-ambiguous-a",
                )
                .expect("candidate a"),
                super::rebinding_candidate_from_binding_declaration(
                    "b",
                    &ambiguous_b_declaration,
                    "rebinding-diagnostics-ambiguous-b",
                )
                .expect("candidate b"),
            ])
            .expect("candidate set"),
        )
        .expect("ambiguous neighborhood"),
    )
    .expect("ambiguous decision");

    let orphaned_prior_declaration = pcurve_binding_declaration(
        "coedge-old",
        PrimitiveConstructionFamilyContractRegistry::contract_for(
            &PrimitiveWitnessDescriptor::ShellWithHole {
                outer_loop_edge_count: 4,
                hole_loop_edge_counts: vec![3],
            },
        ),
        plane_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
    );
    let orphaned_decision = super::rebind_pcurve_on_coedge_from_fact(
        super::rebinding_prior_fact_from_binding_declaration(
            &orphaned_prior_declaration,
            "rebinding-diagnostics-orphaned-prior",
        ),
        LocalTopologyReplacementNeighborhood::new(
            NeighborhoodBindingFamily::CoedgePCurve,
            "coedge-old",
            ReplacementCandidateSet::new(vec![
                super::rebinding_candidate_from_binding_declaration(
                    "weak",
                    &pcurve_binding_declaration(
                        "coedge-new",
                        contract,
                        plane_geometry([[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]]),
                    ),
                    "rebinding-diagnostics-orphaned-candidate",
                )
                .expect("candidate"),
            ])
            .expect("candidate set"),
        )
        .expect("orphaned neighborhood"),
    )
    .expect("orphaned decision");

    let vertex_prior_declaration = vertex_binding_declaration(
        "vertex-old",
        plane_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
    );
    let unsupported_decision = super::rebind_surface_on_face_from_fact(
        super::rebinding_prior_fact_from_binding_declaration(
            &vertex_prior_declaration,
            "rebinding-diagnostics-unsupported-prior",
        ),
        LocalTopologyReplacementNeighborhood::new(
            NeighborhoodBindingFamily::VertexGeometry,
            "vertex-old",
            ReplacementCandidateSet::new(vec![
                super::rebinding_candidate_from_binding_declaration(
                    "vertex-successor",
                    &vertex_prior_declaration,
                    "rebinding-diagnostics-unsupported-candidate",
                )
                .expect("candidate"),
            ])
            .expect("candidate set"),
        )
        .expect("unsupported neighborhood"),
    )
    .expect("unsupported decision");

    assert_eq!(
        successor_continuity.continuity_class(),
        BindingContinuityClass::AuthoritativeSuccessor
    );
    assert_eq!(
        partial_continuity.continuity_class(),
        BindingContinuityClass::InsufficientEvidenceFromAdmittedPartial
    );
    assert_eq!(
        successor_decision.outcome_class(),
        RebindingOutcomeClass::ContinuityJustifiedReattachment
    );
    assert!(successor_decision.selected_candidate_identity().is_some());

    assert_eq!(
        ambiguous_decision.outcome_class(),
        RebindingOutcomeClass::Ambiguous
    );
    assert_eq!(
        ambiguous_decision
            .candidate_labels()
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>(),
        BTreeSet::from(["a".to_string(), "b".to_string()])
    );
    assert_eq!(
        ambiguous_decision
            .candidate_site_identities()
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>(),
        BTreeSet::from(["edge-a".to_string(), "edge-b".to_string()])
    );
    assert!(ambiguous_decision.selected_candidate_identity().is_none());

    assert_eq!(
        orphaned_decision.outcome_class(),
        RebindingOutcomeClass::Orphaned
    );
    assert_eq!(orphaned_decision.candidate_labels(), ["weak"]);
    assert!(orphaned_decision.selected_candidate_identity().is_none());

    assert_eq!(
        unsupported_decision.outcome_class(),
        RebindingOutcomeClass::Unsupported
    );
    assert_eq!(
        unsupported_decision.unsupported_reason(),
        Some(
            UnsupportedRebindingReason::RequestedRebindingFamilyDoesNotAdmitBindingFamily {
                requested: NeighborhoodBindingFamily::FaceSurface,
                actual: NeighborhoodBindingFamily::VertexGeometry,
            },
        )
    );
    assert!(unsupported_decision.selected_candidate_identity().is_none());
}

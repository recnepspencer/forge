#![cfg(test)]

use std::collections::BTreeSet;
use worth_primitives::{
    PrimitiveConstructionFamilyContractRegistry, PrimitiveGeometryIdentityBundle,
    PrimitiveVertexIdentity, PrimitiveWitnessDescriptor,
};

use crate::facade::bindings::{
    attach_curve_to_edge, attach_pcurve_to_coedge, attach_surface_to_face, attach_vertex_geometry,
    evaluate_continuity, rebind_curve_on_edge, rebind_pcurve_on_coedge, rebind_surface_on_face,
    BindingContinuityClass, CoedgeBindingSite, CoedgePCurveBindingSpec, EdgeBindingSite,
    EdgeCurveBindingSpec, FaceBindingSite, FaceSurfaceBindingSpec,
    LocalTopologyReplacementNeighborhood, NeighborhoodBindingFamily, RebindingOutcomeClass,
    ReplacementCandidate, ReplacementCandidateSet, SpatialAdmittedPrimitiveBinding,
    UnsupportedRebindingReason, VertexBindingSite, VertexGeometryBindingSpec,
    VertexGeometryProvenanceKind, VertexToleranceRegime,
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

#[test]
fn continuity_classes_remain_typed_and_no_winner_diagnostics_remain_honest() {
    let contract = PrimitiveConstructionFamilyContractRegistry::contract_for(
        &PrimitiveWitnessDescriptor::Orthotope,
    );
    let prior_surface = SpatialAdmittedPrimitiveBinding::FaceSurface(
        attach_surface_to_face(FaceSurfaceBindingSpec::new(
            FaceBindingSite::new("face-old"),
            contract,
            plane_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
        ))
        .expect("prior surface"),
    );
    let successor_surface = SpatialAdmittedPrimitiveBinding::FaceSurface(
        attach_surface_to_face(FaceSurfaceBindingSpec::new(
            FaceBindingSite::new("face-successor"),
            contract,
            plane_geometry([[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]]),
        ))
        .expect("successor surface"),
    );
    let successor_neighborhood = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::FaceSurface,
        "face-old",
        ReplacementCandidateSet::new(vec![ReplacementCandidate::new(
            "successor",
            successor_surface,
        )
        .expect("candidate")])
        .expect("candidate set"),
    )
    .expect("neighborhood");
    let successor_continuity =
        evaluate_continuity(&prior_surface, &successor_neighborhood).expect("continuity");
    let successor_decision =
        rebind_surface_on_face(prior_surface, successor_neighborhood).expect("decision");

    let prior_edge = SpatialAdmittedPrimitiveBinding::EdgeCurve(
        attach_curve_to_edge(EdgeCurveBindingSpec::new(
            EdgeBindingSite::new("edge-old"),
            contract,
            plane_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
        ))
        .expect("prior edge"),
    );
    let partial_neighborhood = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::EdgeCurve,
        "edge-old",
        ReplacementCandidateSet::new(vec![ReplacementCandidate::new(
            "partial",
            SpatialAdmittedPrimitiveBinding::EdgeCurve(
                attach_curve_to_edge(EdgeCurveBindingSpec::new(
                    EdgeBindingSite::new("edge-partial"),
                    contract,
                    PrimitiveGeometryIdentityBundle::new(
                        vec![],
                        vec![PrimitiveVertexIdentity::from_position([0.0, 0.0, 0.0])],
                    ),
                ))
                .expect("partial edge"),
            ),
        )
        .expect("candidate")])
        .expect("candidate set"),
    )
    .expect("partial neighborhood");
    let partial_continuity =
        evaluate_continuity(&prior_edge, &partial_neighborhood).expect("partial continuity");

    let ambiguous_decision = rebind_curve_on_edge(
        prior_edge,
        LocalTopologyReplacementNeighborhood::new(
            NeighborhoodBindingFamily::EdgeCurve,
            "edge-old",
            ReplacementCandidateSet::new(vec![
                ReplacementCandidate::new(
                    "a",
                    SpatialAdmittedPrimitiveBinding::EdgeCurve(
                        attach_curve_to_edge(EdgeCurveBindingSpec::new(
                            EdgeBindingSite::new("edge-a"),
                            contract,
                            plane_geometry([[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]]),
                        ))
                        .expect("edge a"),
                    ),
                )
                .expect("candidate a"),
                ReplacementCandidate::new(
                    "b",
                    SpatialAdmittedPrimitiveBinding::EdgeCurve(
                        attach_curve_to_edge(EdgeCurveBindingSpec::new(
                            EdgeBindingSite::new("edge-b"),
                            contract,
                            plane_geometry([[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]]),
                        ))
                        .expect("edge b"),
                    ),
                )
                .expect("candidate b"),
            ])
            .expect("candidate set"),
        )
        .expect("ambiguous neighborhood"),
    )
    .expect("ambiguous decision");

    let orphaned_decision = rebind_pcurve_on_coedge(
        SpatialAdmittedPrimitiveBinding::CoedgePCurve(
            attach_pcurve_to_coedge(CoedgePCurveBindingSpec::new(
                CoedgeBindingSite::new("coedge-old"),
                PrimitiveConstructionFamilyContractRegistry::contract_for(
                    &PrimitiveWitnessDescriptor::ShellWithHole {
                        outer_loop_edge_count: 4,
                        hole_loop_edge_counts: vec![3],
                    },
                ),
                plane_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
            ))
            .expect("prior coedge"),
        ),
        LocalTopologyReplacementNeighborhood::new(
            NeighborhoodBindingFamily::CoedgePCurve,
            "coedge-old",
            ReplacementCandidateSet::new(vec![ReplacementCandidate::new(
                "weak",
                SpatialAdmittedPrimitiveBinding::CoedgePCurve(
                    attach_pcurve_to_coedge(CoedgePCurveBindingSpec::new(
                        CoedgeBindingSite::new("coedge-new"),
                        contract,
                        plane_geometry([[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]]),
                    ))
                    .expect("weak candidate"),
                ),
            )
            .expect("candidate")])
            .expect("candidate set"),
        )
        .expect("orphaned neighborhood"),
    )
    .expect("orphaned decision");

    let vertex_prior = attach_vertex_geometry(VertexGeometryBindingSpec::new(
        VertexBindingSite::new("vertex-old"),
        contract,
        plane_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
        VertexGeometryProvenanceKind::CanonicalWitness,
        VertexToleranceRegime::ExactBits,
    ))
    .expect("vertex prior");
    let unsupported_decision = rebind_surface_on_face(
        SpatialAdmittedPrimitiveBinding::VertexGeometry(vertex_prior.clone()),
        LocalTopologyReplacementNeighborhood::new(
            NeighborhoodBindingFamily::VertexGeometry,
            "vertex-old",
            ReplacementCandidateSet::new(vec![ReplacementCandidate::new(
                "vertex-successor",
                SpatialAdmittedPrimitiveBinding::VertexGeometry(vertex_prior),
            )
            .expect("candidate")])
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
    assert!(successor_decision
        .explanation()
        .selected_candidate_identity()
        .is_some());

    assert_eq!(
        ambiguous_decision.outcome_class(),
        RebindingOutcomeClass::Ambiguous
    );
    assert_eq!(
        ambiguous_decision
            .explanation()
            .candidate_labels()
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>(),
        BTreeSet::from(["a".to_string(), "b".to_string()])
    );
    assert_eq!(
        ambiguous_decision
            .explanation()
            .candidate_site_identities()
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>(),
        BTreeSet::from(["edge-a".to_string(), "edge-b".to_string()])
    );
    assert!(ambiguous_decision
        .explanation()
        .selected_candidate_identity()
        .is_none());

    assert_eq!(
        orphaned_decision.outcome_class(),
        RebindingOutcomeClass::Orphaned
    );
    assert_eq!(orphaned_decision.explanation().candidate_labels(), ["weak"]);
    assert!(orphaned_decision
        .explanation()
        .selected_candidate_identity()
        .is_none());

    assert_eq!(
        unsupported_decision.outcome_class(),
        RebindingOutcomeClass::Unsupported
    );
    assert_eq!(
        unsupported_decision.explanation().unsupported_reason(),
        Some(
            UnsupportedRebindingReason::RequestedRebindingFamilyDoesNotAdmitBindingFamily {
                requested: NeighborhoodBindingFamily::FaceSurface,
                actual: NeighborhoodBindingFamily::VertexGeometry,
            },
        )
    );
    assert!(unsupported_decision
        .explanation()
        .selected_candidate_identity()
        .is_none());
}

mod asymmetric_pressure;
mod canonical_replay;
mod continuity;
mod contribution_workflow;
mod curved_pressure;
mod diagnostics;
mod grouped_workflow;
mod mutation_evidence;
mod outcome_transport;
mod outcomes;
mod projection_consumption;

use worth_spatial::facade::bindings::{
    author_primitive_binding_declaration, author_primitive_rebinding_declaration,
    AuthorPrimitiveBindingIntent, AuthorPrimitiveRebindingIntent, CoedgeBindingSite,
    CoedgePCurveBindingSpec, EdgeBindingSite, EdgeCurveBindingSpec,
    LocalTopologyReplacementNeighborhood, MotionAwareBindingPosture, NeighborhoodBindingFamily,
    PrimitiveBindingDeclarationEntry, RebindingOutcomeClass, ReplacementCandidateSet,
    VertexBindingSite, VertexGeometryBindingSpec, VertexGeometryProvenanceKind,
    VertexToleranceRegime,
};

use super::support::{
    canonical_geometry, orthotope_contract, rebinding_candidate_from_binding_declaration,
    rebinding_prior_fact_from_binding_declaration, rebinding_receipt_for_entry,
    shell_with_hole_contract,
};

fn edge_curve_binding_declaration(
    edge_id: &str,
    vertices: [[f64; 3]; 2],
    contract: worth_primitives::PrimitiveConstructionBirthSynopsisContract,
) -> PrimitiveBindingDeclarationEntry {
    author_primitive_binding_declaration(AuthorPrimitiveBindingIntent::attach_curve_to_edge(
        EdgeCurveBindingSpec::new(
            EdgeBindingSite::new(edge_id),
            contract,
            canonical_geometry(vertices),
        ),
    ))
}

fn coedge_pcurve_binding_declaration(
    coedge_id: &str,
    vertices: [[f64; 3]; 2],
    contract: worth_primitives::PrimitiveConstructionBirthSynopsisContract,
) -> PrimitiveBindingDeclarationEntry {
    author_primitive_binding_declaration(AuthorPrimitiveBindingIntent::attach_pcurve_to_coedge(
        CoedgePCurveBindingSpec::new(
            CoedgeBindingSite::new(coedge_id),
            contract,
            canonical_geometry(vertices),
        ),
    ))
}

fn vertex_geometry_binding_declaration(
    vertex_id: &str,
    vertices: [[f64; 3]; 2],
) -> PrimitiveBindingDeclarationEntry {
    author_primitive_binding_declaration(AuthorPrimitiveBindingIntent::attach_vertex_geometry(
        VertexGeometryBindingSpec::new(
            VertexBindingSite::new(vertex_id),
            orthotope_contract(),
            canonical_geometry(vertices),
            VertexGeometryProvenanceKind::CanonicalWitness,
            VertexToleranceRegime::ExactBits,
        ),
    ))
}

#[test]
fn host_order_variation_does_not_change_rebinding_outcome_or_diagnostics() {
    let prior_edge = edge_curve_binding_declaration(
        "edge-old",
        [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
        shell_with_hole_contract(),
    );
    let ambiguous_a = edge_curve_binding_declaration(
        "edge-a",
        [[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]],
        shell_with_hole_contract(),
    );
    let ambiguous_b = edge_curve_binding_declaration(
        "edge-b",
        [[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]],
        shell_with_hole_contract(),
    );
    let first = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::EdgeCurve,
        "edge-old",
        ReplacementCandidateSet::new(vec![
            rebinding_candidate_from_binding_declaration("a", &ambiguous_a, "rebinding-mod-a")
                .expect("candidate a"),
            rebinding_candidate_from_binding_declaration("b", &ambiguous_b, "rebinding-mod-b")
                .expect("candidate b"),
        ])
        .expect("candidate set"),
    )
    .expect("first");
    let second = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::EdgeCurve,
        "edge-old",
        ReplacementCandidateSet::new(vec![
            rebinding_candidate_from_binding_declaration(
                "b",
                &ambiguous_b,
                "rebinding-mod-b-second",
            )
            .expect("candidate b"),
            rebinding_candidate_from_binding_declaration(
                "a",
                &ambiguous_a,
                "rebinding-mod-a-second",
            )
            .expect("candidate a"),
        ])
        .expect("candidate set"),
    )
    .expect("second");

    let first_entry = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_curve_binding(
            rebinding_prior_fact_from_binding_declaration(&prior_edge, "rebinding-mod-first-prior"),
            first,
        ),
    );
    let first_decision =
        rebinding_receipt_for_entry(&first_entry, "rebinding-mod-first").expect("first decision");
    let second_entry = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_curve_binding(
            rebinding_prior_fact_from_binding_declaration(
                &prior_edge,
                "rebinding-mod-second-prior",
            ),
            second,
        ),
    );
    let second_decision = rebinding_receipt_for_entry(&second_entry, "rebinding-mod-second")
        .expect("second decision");

    assert_eq!(
        first_decision.outcome_class(),
        RebindingOutcomeClass::Ambiguous
    );
    assert_eq!(
        first_decision.outcome_class(),
        second_decision.outcome_class()
    );
    assert_eq!(
        first_decision.continuity_class(),
        second_decision.continuity_class()
    );
    assert_eq!(
        first_decision.motion_posture(),
        MotionAwareBindingPosture::Unresolved
    );
    assert_eq!(
        second_decision.motion_posture(),
        MotionAwareBindingPosture::Unresolved
    );
    assert!(first_decision.selected_candidate_identity().is_none());
    assert!(second_decision.selected_candidate_identity().is_none());

    let prior_coedge = coedge_pcurve_binding_declaration(
        "coedge-old",
        [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
        shell_with_hole_contract(),
    );
    let weak_candidate = coedge_pcurve_binding_declaration(
        "coedge-new",
        [[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]],
        orthotope_contract(),
    );
    let weak_neighborhood = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::CoedgePCurve,
        "coedge-old",
        ReplacementCandidateSet::new(vec![rebinding_candidate_from_binding_declaration(
            "weak",
            &weak_candidate,
            "rebinding-mod-weak",
        )
        .expect("weak candidate")])
        .expect("candidate set"),
    )
    .expect("weak neighborhood");

    let weak_entry = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_pcurve_binding(
            rebinding_prior_fact_from_binding_declaration(
                &prior_coedge,
                "rebinding-mod-weak-prior",
            ),
            weak_neighborhood,
        ),
    );
    let weak_decision =
        rebinding_receipt_for_entry(&weak_entry, "rebinding-mod-weak").expect("weak decision");

    assert_eq!(
        weak_decision.outcome_class(),
        RebindingOutcomeClass::Orphaned
    );
    assert_ne!(
        weak_decision.continuity_class(),
        first_decision.continuity_class()
    );
    assert_eq!(
        weak_decision.motion_posture(),
        MotionAwareBindingPosture::Unresolved
    );
    assert!(weak_decision.selected_candidate_identity().is_none());
}

#[test]
fn vertex_rebinding_uses_the_same_local_neighborhood_law_as_other_core_families() {
    let prior =
        vertex_geometry_binding_declaration("vertex-old", [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]);
    let successor =
        vertex_geometry_binding_declaration("vertex-new", [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]);
    let successor_candidate = rebinding_candidate_from_binding_declaration(
        "successor",
        &successor,
        "rebinding-mod-geometry-successor",
    )
    .expect("successor candidate");
    let neighborhood = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::VertexGeometry,
        "vertex-old",
        ReplacementCandidateSet::new(vec![successor_candidate.clone()]).expect("candidate set"),
    )
    .expect("neighborhood");

    let kernel_entry = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_geometry_binding(
            rebinding_prior_fact_from_binding_declaration(&prior, "rebinding-mod-geometry-prior"),
            neighborhood,
        ),
    );
    let kernel_receipt = rebinding_receipt_for_entry(&kernel_entry, "rebinding-mod-geometry")
        .expect("kernel receipt");

    assert_eq!(
        kernel_receipt.outcome_class(),
        RebindingOutcomeClass::ExactReattachment
    );
    assert_eq!(
        kernel_receipt.neighborhood_family(),
        NeighborhoodBindingFamily::VertexGeometry
    );
    assert_eq!(
        kernel_receipt.selected_candidate_identity(),
        Some(successor_candidate.binding_identity())
    );
    assert_eq!(
        kernel_receipt.motion_posture(),
        MotionAwareBindingPosture::Unresolved
    );
}

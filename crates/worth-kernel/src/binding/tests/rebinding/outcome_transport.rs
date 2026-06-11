use forge_query::facade::ForgeQueryOrdinaryOutcome;
use worth_primitives::{
    PrimitiveConstructionBirthSynopsisContract, PrimitiveConstructionFamilyContractRegistry,
    PrimitiveWitnessDescriptor,
};
use worth_spatial::facade::bindings::{
    author_primitive_binding_declaration, author_primitive_rebinding_declaration,
    AuthorPrimitiveBindingIntent, CoedgeBindingSite, CoedgePCurveBindingSpec, EdgeBindingSite,
    EdgeCurveBindingSpec, FaceBindingSite, FaceSurfaceBindingSpec,
    LocalTopologyReplacementNeighborhood, NeighborhoodBindingFamily,
    PrimitiveBindingDeclarationEntry, RebindingOutcomeClass, ReplacementCandidateSet,
    VertexBindingSite, VertexGeometryBindingSpec, VertexGeometryProvenanceKind,
    VertexToleranceRegime,
};

use super::super::support::{
    admitted_rebinding_handle, canonical_geometry, orthotope_contract,
    rebinding_candidate_from_binding_declaration, rebinding_ordinary_outcome_for_entry,
    rebinding_prior_fact_from_binding_declaration, rebinding_receipt_for_entry,
    shell_with_hole_contract,
};

fn face_surface_binding_declaration(
    face_id: &str,
    persistent_name: &str,
    vertices: [[f64; 3]; 2],
) -> PrimitiveBindingDeclarationEntry {
    author_primitive_binding_declaration(AuthorPrimitiveBindingIntent::attach_surface_to_face(
        FaceSurfaceBindingSpec::new(
            FaceBindingSite::new(face_id).with_persistent_name(persistent_name),
            orthotope_contract(),
            canonical_geometry(vertices),
        ),
    ))
}

fn edge_curve_binding_declaration(
    edge_id: &str,
    vertices: [[f64; 3]; 2],
    contract: PrimitiveConstructionBirthSynopsisContract,
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
    contract: PrimitiveConstructionBirthSynopsisContract,
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
            PrimitiveConstructionFamilyContractRegistry::contract_for(
                &PrimitiveWitnessDescriptor::Orthotope,
            ),
            canonical_geometry(vertices),
            VertexGeometryProvenanceKind::CanonicalWitness,
            VertexToleranceRegime::ExactBits,
        ),
    ))
}

#[test]
fn rebinding_outcome_transport_through_kernel_and_query_does_not_collapse_denial_shape() {
    let handle = admitted_rebinding_handle("rebinding-outcome-transport");

    let supported_entry = author_primitive_rebinding_declaration(
        crate::binding::tests::support::replace_surface_binding(
            rebinding_prior_fact_from_binding_declaration(
                &face_surface_binding_declaration(
                    "face-old",
                    "surface-alpha",
                    [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
                ),
                "rebinding-outcome-supported-prior",
            ),
            LocalTopologyReplacementNeighborhood::new(
                NeighborhoodBindingFamily::FaceSurface,
                "face-old",
                ReplacementCandidateSet::new(vec![rebinding_candidate_from_binding_declaration(
                    "successor",
                    &face_surface_binding_declaration(
                        "face-new",
                        "surface-beta",
                        [[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]],
                    ),
                    "rebinding-outcome-supported-candidate",
                )
                .expect("candidate")])
                .expect("candidate set"),
            )
            .expect("neighborhood"),
        ),
    );
    let supported = rebinding_ordinary_outcome_for_entry(&supported_entry, &handle);
    let supported_decision =
        rebinding_receipt_for_entry(&supported_entry, "rebinding-outcome-supported")
            .expect("supported decision");

    let ambiguous_prior = edge_curve_binding_declaration(
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
    let ambiguous_entry = author_primitive_rebinding_declaration(
        crate::binding::tests::support::replace_curve_binding(
            rebinding_prior_fact_from_binding_declaration(
                &ambiguous_prior,
                "rebinding-outcome-ambiguous-prior",
            ),
            LocalTopologyReplacementNeighborhood::new(
                NeighborhoodBindingFamily::EdgeCurve,
                "edge-old",
                ReplacementCandidateSet::new(vec![
                    rebinding_candidate_from_binding_declaration(
                        "a",
                        &ambiguous_a,
                        "rebinding-outcome-ambiguous-a",
                    )
                    .expect("a"),
                    rebinding_candidate_from_binding_declaration(
                        "b",
                        &ambiguous_b,
                        "rebinding-outcome-ambiguous-b",
                    )
                    .expect("b"),
                ])
                .expect("candidate set"),
            )
            .expect("neighborhood"),
        ),
    );
    let ambiguous = rebinding_ordinary_outcome_for_entry(&ambiguous_entry, &handle);
    let ambiguous_decision =
        rebinding_receipt_for_entry(&ambiguous_entry, "rebinding-outcome-ambiguous")
            .expect("ambiguous decision");

    let orphaned_prior = coedge_pcurve_binding_declaration(
        "coedge-old",
        [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
        shell_with_hole_contract(),
    );
    let orphaned_candidate = coedge_pcurve_binding_declaration(
        "coedge-new",
        [[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]],
        orthotope_contract(),
    );
    let orphaned_entry = author_primitive_rebinding_declaration(
        crate::binding::tests::support::replace_pcurve_binding(
            rebinding_prior_fact_from_binding_declaration(
                &orphaned_prior,
                "rebinding-outcome-orphaned-prior",
            ),
            LocalTopologyReplacementNeighborhood::new(
                NeighborhoodBindingFamily::CoedgePCurve,
                "coedge-old",
                ReplacementCandidateSet::new(vec![rebinding_candidate_from_binding_declaration(
                    "weak",
                    &orphaned_candidate,
                    "rebinding-outcome-orphaned-candidate",
                )
                .expect("candidate")])
                .expect("candidate set"),
            )
            .expect("neighborhood"),
        ),
    );
    let orphaned = rebinding_ordinary_outcome_for_entry(&orphaned_entry, &handle);
    let orphaned_decision =
        rebinding_receipt_for_entry(&orphaned_entry, "rebinding-outcome-orphaned")
            .expect("orphaned decision");

    let unsupported_prior =
        vertex_geometry_binding_declaration("vertex-old", [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]);
    let unsupported_candidate =
        vertex_geometry_binding_declaration("vertex-new", [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]);
    let unsupported_entry = author_primitive_rebinding_declaration(
        crate::binding::tests::support::replace_surface_binding(
            rebinding_prior_fact_from_binding_declaration(
                &unsupported_prior,
                "rebinding-outcome-unsupported-prior",
            ),
            LocalTopologyReplacementNeighborhood::new(
                NeighborhoodBindingFamily::VertexGeometry,
                "vertex-old",
                ReplacementCandidateSet::new(vec![rebinding_candidate_from_binding_declaration(
                    "vertex-successor",
                    &unsupported_candidate,
                    "rebinding-outcome-unsupported-candidate",
                )
                .expect("candidate")])
                .expect("candidate set"),
            )
            .expect("neighborhood"),
        ),
    );
    let unsupported = rebinding_ordinary_outcome_for_entry(&unsupported_entry, &handle);
    let unsupported_decision =
        rebinding_receipt_for_entry(&unsupported_entry, "rebinding-outcome-unsupported")
            .expect("unsupported decision");

    assert_eq!(
        supported_decision.outcome_class(),
        RebindingOutcomeClass::ContinuityJustifiedReattachment
    );
    assert_eq!(
        ambiguous_decision.outcome_class(),
        RebindingOutcomeClass::Ambiguous
    );
    assert_eq!(
        orphaned_decision.outcome_class(),
        RebindingOutcomeClass::Orphaned
    );
    assert_eq!(
        unsupported_decision.outcome_class(),
        RebindingOutcomeClass::Unsupported
    );
    assert!(matches!(supported, ForgeQueryOrdinaryOutcome::Bound(_)));
    assert!(matches!(ambiguous, ForgeQueryOrdinaryOutcome::Ambiguous(_)));
    assert!(matches!(
        orphaned,
        ForgeQueryOrdinaryOutcome::RebindRequired(_)
    ));
    assert!(matches!(
        unsupported,
        ForgeQueryOrdinaryOutcome::Unsupported(_)
    ));
}

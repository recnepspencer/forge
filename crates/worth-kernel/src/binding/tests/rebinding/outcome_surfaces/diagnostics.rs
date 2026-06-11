use forge_query::facade::ForgeQueryOrdinaryOutcome;
use std::collections::BTreeSet;
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
    UnsupportedRebindingReason, VertexBindingSite, VertexGeometryBindingSpec,
    VertexGeometryProvenanceKind, VertexToleranceRegime,
};

use crate::binding::tests::support::{
    admitted_rebinding_handle, assert_workflow_artifact_parity, canonical_geometry,
    canonical_text_entries_for_rebinding, inspect_progressed_rebinding_entry, orthotope_contract,
    progress_rebinding_entry, rebinding_candidate_from_binding_declaration,
    rebinding_ordinary_outcome_for_entry, rebinding_prior_fact_from_binding_declaration,
    rebinding_receipt_for_entry, rebinding_workflow_artifacts, shell_with_hole_contract,
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
fn rebinding_diagnostics_preserve_candidate_inventory_and_no_winner_cases_without_false_authority()
{
    let handle = admitted_rebinding_handle("rebinding-diagnostics");

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
                "rebinding-diagnostics-ambiguous-prior",
            ),
            LocalTopologyReplacementNeighborhood::new(
                NeighborhoodBindingFamily::EdgeCurve,
                "edge-old",
                ReplacementCandidateSet::new(vec![
                    rebinding_candidate_from_binding_declaration(
                        "a",
                        &ambiguous_a,
                        "rebinding-diagnostics-ambiguous-a",
                    )
                    .expect("a"),
                    rebinding_candidate_from_binding_declaration(
                        "b",
                        &ambiguous_b,
                        "rebinding-diagnostics-ambiguous-b",
                    )
                    .expect("b"),
                ])
                .expect("candidate set"),
            )
            .expect("neighborhood"),
        ),
    );
    let ambiguous_progression = progress_rebinding_entry(&ambiguous_entry, &handle);
    let ambiguous_inspection =
        inspect_progressed_rebinding_entry(&handle, ambiguous_progression.clone());
    let ambiguous_workflow = rebinding_workflow_artifacts(&ambiguous_entry, &handle);
    let ambiguous_decision =
        rebinding_receipt_for_entry(&ambiguous_entry, "rebinding-diagnostics-ambiguous")
            .expect("ambiguous decision");
    let ambiguous_outcome = rebinding_ordinary_outcome_for_entry(&ambiguous_entry, &handle);

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
                "rebinding-diagnostics-orphaned-prior",
            ),
            LocalTopologyReplacementNeighborhood::new(
                NeighborhoodBindingFamily::CoedgePCurve,
                "coedge-old",
                ReplacementCandidateSet::new(vec![rebinding_candidate_from_binding_declaration(
                    "weak",
                    &orphaned_candidate,
                    "rebinding-diagnostics-orphaned-candidate",
                )
                .expect("candidate")])
                .expect("candidate set"),
            )
            .expect("neighborhood"),
        ),
    );
    let orphaned_decision =
        rebinding_receipt_for_entry(&orphaned_entry, "rebinding-diagnostics-orphaned")
            .expect("orphaned decision");
    let orphaned_outcome = rebinding_ordinary_outcome_for_entry(&orphaned_entry, &handle);

    let unsupported_prior =
        vertex_geometry_binding_declaration("vertex-old", [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]);
    let unsupported_entry = author_primitive_rebinding_declaration(
        crate::binding::tests::support::replace_surface_binding(
            rebinding_prior_fact_from_binding_declaration(
                &unsupported_prior,
                "rebinding-diagnostics-unsupported-prior",
            ),
            LocalTopologyReplacementNeighborhood::new(
                NeighborhoodBindingFamily::VertexGeometry,
                "vertex-old",
                ReplacementCandidateSet::new(vec![rebinding_candidate_from_binding_declaration(
                    "vertex-successor",
                    &unsupported_prior,
                    "rebinding-diagnostics-unsupported-candidate",
                )
                .expect("candidate")])
                .expect("candidate set"),
            )
            .expect("neighborhood"),
        ),
    );
    let unsupported_decision =
        rebinding_receipt_for_entry(&unsupported_entry, "rebinding-diagnostics-unsupported")
            .expect("unsupported decision");
    let unsupported_outcome = rebinding_ordinary_outcome_for_entry(&unsupported_entry, &handle);

    let supported_entry = author_primitive_rebinding_declaration(
        crate::binding::tests::support::replace_surface_binding(
            rebinding_prior_fact_from_binding_declaration(
                &face_surface_binding_declaration(
                    "face-old",
                    "surface-alpha",
                    [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
                ),
                "rebinding-diagnostics-supported-prior",
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
                    "rebinding-diagnostics-supported-candidate",
                )
                .expect("candidate")])
                .expect("candidate set"),
            )
            .expect("neighborhood"),
        ),
    );
    let supported_decision =
        rebinding_receipt_for_entry(&supported_entry, "rebinding-diagnostics-supported")
            .expect("supported decision");

    let ambiguous_text = canonical_text_entries_for_rebinding(&ambiguous_entry);
    let ambiguous_labels: BTreeSet<_> = ambiguous_decision
        .candidate_labels()
        .iter()
        .cloned()
        .collect();
    let ambiguous_sites: BTreeSet<_> = ambiguous_decision
        .candidate_site_identities()
        .iter()
        .cloned()
        .collect();

    assert_eq!(
        ambiguous_decision.outcome_class(),
        RebindingOutcomeClass::Ambiguous
    );
    assert_eq!(
        ambiguous_labels,
        BTreeSet::from(["a".to_string(), "b".to_string()])
    );
    assert_eq!(
        ambiguous_sites,
        BTreeSet::from(["edge-a".to_string(), "edge-b".to_string()])
    );
    assert!(ambiguous_decision.selected_candidate_identity().is_none());
    assert!(ambiguous_decision.selected_candidate_label().is_none());
    assert_eq!(
        ambiguous_text
            .get("rebinding.neighborhood.candidate_count")
            .map(String::as_str),
        Some("2")
    );
    assert_eq!(
        Some(ambiguous_progression.progression_digest()),
        ambiguous_inspection.progression_digest()
    );
    assert_workflow_artifact_parity(
        &ambiguous_workflow,
        &handle,
        ambiguous_progression.clone(),
        ambiguous_entry.clone(),
    );
    assert!(matches!(
        ambiguous_outcome,
        ForgeQueryOrdinaryOutcome::Ambiguous(_)
    ));

    assert_eq!(
        orphaned_decision.outcome_class(),
        RebindingOutcomeClass::Orphaned
    );
    assert_eq!(orphaned_decision.candidate_labels(), ["weak"]);
    assert_eq!(
        orphaned_decision.candidate_site_identities(),
        ["coedge-new"]
    );
    assert!(orphaned_decision.selected_candidate_identity().is_none());
    assert!(orphaned_decision.selected_candidate_label().is_none());
    assert!(matches!(
        orphaned_outcome,
        ForgeQueryOrdinaryOutcome::RebindRequired(_)
    ));

    assert_eq!(
        unsupported_decision.outcome_class(),
        RebindingOutcomeClass::Unsupported
    );
    assert_eq!(
        unsupported_decision.candidate_labels(),
        ["vertex-successor"]
    );
    assert_eq!(
        unsupported_decision.candidate_site_identities(),
        ["vertex-old"]
    );
    assert!(unsupported_decision.selected_candidate_identity().is_none());
    assert!(unsupported_decision.selected_candidate_label().is_none());
    assert_eq!(
        unsupported_decision.unsupported_reason(),
        Some(
            UnsupportedRebindingReason::RequestedRebindingFamilyDoesNotAdmitBindingFamily {
                requested: NeighborhoodBindingFamily::FaceSurface,
                actual: NeighborhoodBindingFamily::VertexGeometry,
            },
        )
    );
    assert!(matches!(
        unsupported_outcome,
        ForgeQueryOrdinaryOutcome::Unsupported(_)
    ));

    assert_eq!(
        supported_decision.outcome_class(),
        RebindingOutcomeClass::ContinuityJustifiedReattachment
    );
    assert_eq!(supported_decision.candidate_labels(), ["successor"]);
    assert!(supported_decision.selected_candidate_identity().is_some());
    assert_eq!(
        supported_decision.selected_candidate_label(),
        Some("successor")
    );
}

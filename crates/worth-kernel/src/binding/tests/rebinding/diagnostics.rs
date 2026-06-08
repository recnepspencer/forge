use forge_query::facade::ForgeQueryOrdinaryOutcome;
use std::collections::BTreeSet;
use worth_primitives::{PrimitiveConstructionFamilyContractRegistry, PrimitiveWitnessDescriptor};
use worth_spatial::facade::bindings::{
    attach_curve_to_edge, attach_pcurve_to_coedge, attach_surface_to_face, attach_vertex_geometry,
    CoedgeBindingSite, CoedgePCurveBindingSpec, EdgeBindingSite, EdgeCurveBindingSpec,
    FaceBindingSite, FaceSurfaceBindingSpec, LocalTopologyReplacementNeighborhood,
    NeighborhoodBindingFamily, RebindingOutcomeClass, ReplacementCandidate,
    ReplacementCandidateSet, SpatialAdmittedPrimitiveBinding, UnsupportedRebindingReason,
    VertexBindingSite, VertexGeometryBindingSpec, VertexGeometryProvenanceKind,
    VertexToleranceRegime,
};

use crate::facade::authoring::binding::{
    author_primitive_rebinding_declaration, AuthorPrimitiveRebindingIntent,
};

use super::super::support::{
    admitted_rebinding_handle, assert_workflow_artifact_parity, canonical_geometry,
    canonical_text_entries_for_rebinding, inspect_progressed_rebinding_entry, orthotope_contract,
    progress_rebinding_entry, rebinding_workflow_artifacts, shell_with_hole_contract,
};

fn face_surface_binding(
    face_id: &str,
    persistent_name: &str,
    vertices: [[f64; 3]; 2],
) -> SpatialAdmittedPrimitiveBinding {
    SpatialAdmittedPrimitiveBinding::FaceSurface(
        attach_surface_to_face(FaceSurfaceBindingSpec::new(
            FaceBindingSite::new(face_id).with_persistent_name(persistent_name),
            orthotope_contract(),
            canonical_geometry(vertices),
        ))
        .expect("face surface binding"),
    )
}

#[test]
fn rebinding_diagnostics_preserve_candidate_inventory_and_no_winner_cases_without_false_authority()
{
    let handle = admitted_rebinding_handle("rebinding-diagnostics");

    let ambiguous_entry = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_curve_binding(
            SpatialAdmittedPrimitiveBinding::EdgeCurve(
                attach_curve_to_edge(EdgeCurveBindingSpec::new(
                    EdgeBindingSite::new("edge-old"),
                    shell_with_hole_contract(),
                    canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
                ))
                .expect("prior"),
            ),
            LocalTopologyReplacementNeighborhood::new(
                NeighborhoodBindingFamily::EdgeCurve,
                "edge-old",
                ReplacementCandidateSet::new(vec![
                    ReplacementCandidate::new(
                        "a",
                        SpatialAdmittedPrimitiveBinding::EdgeCurve(
                            attach_curve_to_edge(EdgeCurveBindingSpec::new(
                                EdgeBindingSite::new("edge-a"),
                                shell_with_hole_contract(),
                                canonical_geometry([[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]]),
                            ))
                            .expect("edge a"),
                        ),
                    )
                    .expect("a"),
                    ReplacementCandidate::new(
                        "b",
                        SpatialAdmittedPrimitiveBinding::EdgeCurve(
                            attach_curve_to_edge(EdgeCurveBindingSpec::new(
                                EdgeBindingSite::new("edge-b"),
                                shell_with_hole_contract(),
                                canonical_geometry([[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]]),
                            ))
                            .expect("edge b"),
                        ),
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
    let ambiguous_decision = ambiguous_entry.clone().admit().expect("ambiguous decision");
    let ambiguous_outcome = ambiguous_entry.ordinary_outcome_with_query(&handle);

    let orphaned_entry = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_pcurve_binding(
            SpatialAdmittedPrimitiveBinding::CoedgePCurve(
                attach_pcurve_to_coedge(CoedgePCurveBindingSpec::new(
                    CoedgeBindingSite::new("coedge-old"),
                    shell_with_hole_contract(),
                    canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
                ))
                .expect("prior"),
            ),
            LocalTopologyReplacementNeighborhood::new(
                NeighborhoodBindingFamily::CoedgePCurve,
                "coedge-old",
                ReplacementCandidateSet::new(vec![ReplacementCandidate::new(
                    "weak",
                    SpatialAdmittedPrimitiveBinding::CoedgePCurve(
                        attach_pcurve_to_coedge(CoedgePCurveBindingSpec::new(
                            CoedgeBindingSite::new("coedge-new"),
                            orthotope_contract(),
                            canonical_geometry([[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]]),
                        ))
                        .expect("weak candidate"),
                    ),
                )
                .expect("candidate")])
                .expect("candidate set"),
            )
            .expect("neighborhood"),
        ),
    );
    let orphaned_decision = orphaned_entry.clone().admit().expect("orphaned decision");
    let orphaned_outcome = orphaned_entry.ordinary_outcome_with_query(&handle);

    let unsupported_prior = attach_vertex_geometry(VertexGeometryBindingSpec::new(
        VertexBindingSite::new("vertex-old"),
        PrimitiveConstructionFamilyContractRegistry::contract_for(
            &PrimitiveWitnessDescriptor::Orthotope,
        ),
        canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
        VertexGeometryProvenanceKind::CanonicalWitness,
        VertexToleranceRegime::ExactBits,
    ))
    .expect("vertex prior");
    let unsupported_entry = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_surface_binding(
            SpatialAdmittedPrimitiveBinding::VertexGeometry(unsupported_prior.clone()),
            LocalTopologyReplacementNeighborhood::new(
                NeighborhoodBindingFamily::VertexGeometry,
                "vertex-old",
                ReplacementCandidateSet::new(vec![ReplacementCandidate::new(
                    "vertex-successor",
                    SpatialAdmittedPrimitiveBinding::VertexGeometry(unsupported_prior),
                )
                .expect("candidate")])
                .expect("candidate set"),
            )
            .expect("neighborhood"),
        ),
    );
    let unsupported_decision = unsupported_entry
        .clone()
        .admit()
        .expect("unsupported decision");
    let unsupported_outcome = unsupported_entry.ordinary_outcome_with_query(&handle);

    let supported_entry = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_surface_binding(
            face_surface_binding(
                "face-old",
                "surface-alpha",
                [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
            ),
            LocalTopologyReplacementNeighborhood::new(
                NeighborhoodBindingFamily::FaceSurface,
                "face-old",
                ReplacementCandidateSet::new(vec![ReplacementCandidate::new(
                    "successor",
                    face_surface_binding(
                        "face-new",
                        "surface-beta",
                        [[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]],
                    ),
                )
                .expect("candidate")])
                .expect("candidate set"),
            )
            .expect("neighborhood"),
        ),
    );
    let supported_decision = supported_entry.clone().admit().expect("supported decision");

    let ambiguous_text = canonical_text_entries_for_rebinding(&ambiguous_entry);
    let ambiguous_labels: BTreeSet<_> = ambiguous_decision
        .explanation()
        .candidate_labels()
        .iter()
        .cloned()
        .collect();
    let ambiguous_sites: BTreeSet<_> = ambiguous_decision
        .explanation()
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
    assert!(ambiguous_decision
        .explanation()
        .selected_candidate_identity()
        .is_none());
    assert!(ambiguous_decision
        .explanation()
        .selected_candidate_label()
        .is_none());
    assert_eq!(
        ambiguous_text.get("candidate_count").map(String::as_str),
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
    assert_eq!(orphaned_decision.explanation().candidate_labels(), ["weak"]);
    assert_eq!(
        orphaned_decision.explanation().candidate_site_identities(),
        ["coedge-new"]
    );
    assert!(orphaned_decision
        .explanation()
        .selected_candidate_identity()
        .is_none());
    assert!(orphaned_decision
        .explanation()
        .selected_candidate_label()
        .is_none());
    assert!(matches!(
        orphaned_outcome,
        ForgeQueryOrdinaryOutcome::RebindRequired(_)
    ));

    assert_eq!(
        unsupported_decision.outcome_class(),
        RebindingOutcomeClass::Unsupported
    );
    assert_eq!(
        unsupported_decision.explanation().candidate_labels(),
        ["vertex-successor"]
    );
    assert_eq!(
        unsupported_decision
            .explanation()
            .candidate_site_identities(),
        ["vertex-old"]
    );
    assert!(unsupported_decision
        .explanation()
        .selected_candidate_identity()
        .is_none());
    assert!(unsupported_decision
        .explanation()
        .selected_candidate_label()
        .is_none());
    assert_eq!(
        unsupported_decision.explanation().unsupported_reason(),
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
    assert_eq!(
        supported_decision.explanation().candidate_labels(),
        ["successor"]
    );
    assert!(supported_decision
        .explanation()
        .selected_candidate_identity()
        .is_some());
    assert_eq!(
        supported_decision.explanation().selected_candidate_label(),
        Some("successor")
    );
}

use worth_primitives::{PrimitiveConstructionFamilyContractRegistry, PrimitiveWitnessDescriptor};
use worth_spatial::facade::bindings::{
    author_primitive_binding_declaration, author_primitive_rebinding_declaration,
    primitive_rebinding_mutation_evidence, AuthorPrimitiveBindingIntent, CoedgeBindingSite,
    CoedgePCurveBindingSpec, FaceBindingSite, FaceSurfaceBindingSpec,
    LocalTopologyReplacementNeighborhood, NeighborhoodBindingFamily,
    PrimitiveBindingDeclarationEntry, ReplacementCandidateSet,
};
use worth_spatial::facade::bindings::{BindingContinuityClass, RebindingOutcomeClass};

use super::super::support::{
    admitted_rebinding_handle, canonical_geometry, orthotope_contract,
    rebinding_candidate_from_binding_declaration, rebinding_prior_fact_from_binding_declaration,
};

fn face_surface_binding_declaration(
    face_id: &str,
    vertices: [[f64; 3]; 2],
) -> PrimitiveBindingDeclarationEntry {
    author_primitive_binding_declaration(AuthorPrimitiveBindingIntent::attach_surface_to_face(
        FaceSurfaceBindingSpec::new(
            FaceBindingSite::new(face_id),
            orthotope_contract(),
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

#[test]
fn rebinding_mutation_evidence_preserves_route_and_target_identity_for_admitted_replacements() {
    let prior = face_surface_binding_declaration("face-old", [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]);
    let successor =
        face_surface_binding_declaration("face-new", [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]);
    let declaration = author_primitive_rebinding_declaration(
        crate::binding::tests::support::replace_surface_binding(
            rebinding_prior_fact_from_binding_declaration(
                &prior,
                "rebinding-mutation-evidence-prior",
            ),
            LocalTopologyReplacementNeighborhood::new(
                NeighborhoodBindingFamily::FaceSurface,
                "face-old",
                ReplacementCandidateSet::new(vec![rebinding_candidate_from_binding_declaration(
                    "successor",
                    &successor,
                    "rebinding-mutation-evidence-successor",
                )
                .expect("candidate")])
                .expect("candidate set"),
            )
            .expect("neighborhood"),
        ),
    );
    let handle = admitted_rebinding_handle("rebinding-mutation-evidence");
    let evidence = primitive_rebinding_mutation_evidence(&declaration, &handle).expect("evidence");

    assert_eq!(evidence.prior_site_identity(), "face-old");
    assert_eq!(
        evidence.neighborhood_family(),
        NeighborhoodBindingFamily::FaceSurface
    );
    assert_eq!(
        evidence.outcome_class(),
        RebindingOutcomeClass::ExactReattachment
    );
    assert_eq!(evidence.continuity_class(), BindingContinuityClass::Exact);
    assert_eq!(evidence.selected_candidate_label(), Some("successor"));
    assert_eq!(
        evidence.selected_candidate_site_identity(),
        Some("face-new")
    );
    assert!(evidence.progression_digest().is_some());
    assert!(evidence.route_plan_digest().is_some());
    assert!(!evidence.receipt_digest().is_empty());
    assert!(!evidence.envelope_digest().is_empty());
    assert!(!evidence.evidence_digest().is_empty());
}

#[test]
fn rebinding_mutation_evidence_preserves_orphaned_projection_truth_without_fabricating_success() {
    let prior = coedge_pcurve_binding_declaration(
        "coedge-old",
        [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
        PrimitiveConstructionFamilyContractRegistry::contract_for(
            &PrimitiveWitnessDescriptor::ShellWithHole {
                outer_loop_edge_count: 6,
                hole_loop_edge_counts: vec![3],
            },
        ),
    );
    let weak = coedge_pcurve_binding_declaration(
        "coedge-new",
        [[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]],
        orthotope_contract(),
    );
    let declaration = author_primitive_rebinding_declaration(
        crate::binding::tests::support::replace_pcurve_binding(
            rebinding_prior_fact_from_binding_declaration(
                &prior,
                "rebinding-mutation-evidence-orphaned-prior",
            ),
            LocalTopologyReplacementNeighborhood::new(
                NeighborhoodBindingFamily::CoedgePCurve,
                "coedge-old",
                ReplacementCandidateSet::new(vec![rebinding_candidate_from_binding_declaration(
                    "weak",
                    &weak,
                    "rebinding-mutation-evidence-orphaned-weak",
                )
                .expect("candidate")])
                .expect("candidate set"),
            )
            .expect("neighborhood"),
        ),
    );
    let handle = admitted_rebinding_handle("rebinding-mutation-evidence-orphaned");

    let evidence = primitive_rebinding_mutation_evidence(&declaration, &handle).expect("evidence");

    assert_eq!(evidence.prior_site_identity(), "coedge-old");
    assert_eq!(
        evidence.neighborhood_family(),
        NeighborhoodBindingFamily::CoedgePCurve
    );
    assert_eq!(evidence.outcome_class(), RebindingOutcomeClass::Orphaned);
    assert_eq!(
        evidence.continuity_class(),
        BindingContinuityClass::InsufficientEvidenceFromAdmittedPartial
    );
    assert_eq!(evidence.selected_candidate_label(), None);
    assert_eq!(evidence.selected_candidate_site_identity(), None);
    assert!(evidence.progression_digest().is_some());
    assert!(evidence.route_plan_digest().is_some());
    assert!(!evidence.receipt_digest().is_empty());
    assert!(!evidence.envelope_digest().is_empty());
    assert!(!evidence.evidence_digest().is_empty());
}

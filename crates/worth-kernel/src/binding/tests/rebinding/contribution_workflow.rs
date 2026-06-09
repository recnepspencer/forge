use forge_query::facade::ForgeQueryContributionComposedClassification;
use worth_primitives::{PrimitiveConstructionFamilyContractRegistry, PrimitiveWitnessDescriptor};
use worth_spatial::facade::bindings::{
    author_primitive_binding_declaration, author_primitive_rebinding_declaration,
    AuthorPrimitiveBindingIntent, EdgeBindingSite, EdgeCurveBindingSpec, FaceBindingSite,
    FaceSurfaceBindingSpec, LocalTopologyReplacementNeighborhood, NeighborhoodBindingFamily,
    PrimitiveBindingDeclarationEntry, ReplacementCandidateSet,
};
use worth_spatial::facade::neighborhood::primitive_rebinding_contribution_workflow;

use super::super::support::{
    admitted_rebinding_handle, canonical_geometry, orthotope_contract,
    rebinding_candidate_from_binding_declaration, rebinding_prior_fact_from_binding_declaration,
    shell_with_hole_contract,
};

fn face_surface_binding_declaration(
    face_id: &str,
    vertices: [[f64; 3]; 2],
    contract: worth_primitives::PrimitiveConstructionBirthSynopsisContract,
) -> PrimitiveBindingDeclarationEntry {
    author_primitive_binding_declaration(AuthorPrimitiveBindingIntent::attach_surface_to_face(
        FaceSurfaceBindingSpec::new(
            FaceBindingSite::new(face_id),
            contract,
            canonical_geometry(vertices),
        ),
    ))
}

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

#[test]
fn contribution_workflow_preserves_exact_reattachment_continuity_truth() {
    let contract = PrimitiveConstructionFamilyContractRegistry::contract_for(
        &PrimitiveWitnessDescriptor::Orthotope,
    );
    let prior = face_surface_binding_declaration(
        "face-proof-old",
        [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
        contract,
    );
    let successor = face_surface_binding_declaration(
        "face-proof-new",
        [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
        contract,
    );
    let declaration = author_primitive_rebinding_declaration(
        crate::binding::tests::support::replace_surface_binding(
            rebinding_prior_fact_from_binding_declaration(
                &prior,
                "rebinding-contribution-exact-prior",
            ),
            LocalTopologyReplacementNeighborhood::new(
                NeighborhoodBindingFamily::FaceSurface,
                "face-proof-old",
                ReplacementCandidateSet::new(vec![rebinding_candidate_from_binding_declaration(
                    "successor",
                    &successor,
                    "rebinding-contribution-exact-successor",
                )
                .expect("candidate")])
                .expect("candidate set"),
            )
            .expect("neighborhood"),
        ),
    );
    let handle = admitted_rebinding_handle("rebinding-contribution-exact");
    let proof = handle.orchestrate_declaration_with_contributions_proof(
        primitive_rebinding_contribution_workflow(declaration, &handle),
    );

    assert_eq!(
        proof.composition_classification(),
        Some(ForgeQueryContributionComposedClassification::FullyAdmitted)
    );
    assert_eq!(proof.intent_results().len(), 1);
    assert_eq!(
        proof.intent_results()[0].semantic_code(),
        "worth.spatial.rebinding.continuity.exact_reattachment"
    );
}

#[test]
fn contribution_workflow_preserves_orphaned_rebind_context_truth() {
    let prior = edge_curve_binding_declaration(
        "edge-proof-old",
        [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
        shell_with_hole_contract(),
    );
    let weak_candidate = edge_curve_binding_declaration(
        "edge-proof-new",
        [[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]],
        orthotope_contract(),
    );
    let declaration = author_primitive_rebinding_declaration(
        crate::binding::tests::support::replace_curve_binding(
            rebinding_prior_fact_from_binding_declaration(
                &prior,
                "rebinding-contribution-orphaned-prior",
            ),
            LocalTopologyReplacementNeighborhood::new(
                NeighborhoodBindingFamily::EdgeCurve,
                "edge-proof-old",
                ReplacementCandidateSet::new(vec![rebinding_candidate_from_binding_declaration(
                    "weak",
                    &weak_candidate,
                    "rebinding-contribution-orphaned-weak",
                )
                .expect("weak")])
                .expect("candidate set"),
            )
            .expect("neighborhood"),
        ),
    );
    let handle = admitted_rebinding_handle("rebinding-contribution-orphaned");
    let proof = handle.orchestrate_declaration_with_contributions_proof(
        primitive_rebinding_contribution_workflow(declaration, &handle),
    );

    assert_eq!(
        proof.composition_classification(),
        Some(ForgeQueryContributionComposedClassification::FullyAdmitted)
    );
    assert_eq!(
        proof.intent_results()[0].semantic_code(),
        "worth.spatial.rebinding.explanation.orphaned"
    );
}

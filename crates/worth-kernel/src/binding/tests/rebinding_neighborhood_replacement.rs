use crate::binding::tests::support::{
    admitted_rebinding_handle, canonical_geometry, orthotope_contract,
    rebinding_candidate_from_binding_declaration, rebinding_prior_fact_from_binding_declaration,
};
use worth_spatial::facade::bindings::{
    author_primitive_binding_declaration, author_primitive_rebinding_declaration,
    primitive_rebinding_mutation_evidence, AuthorPrimitiveBindingIntent, FaceBindingSite,
    FaceSurfaceBindingSpec, LocalTopologyReplacementNeighborhood, NeighborhoodBindingFamily,
    ReplacementCandidateSet,
};
use worth_spatial::facade::neighborhood::{
    primitive_rebinding_neighborhood_replacement_facts,
    primitive_rebinding_neighborhood_replacement_source, topology_neighborhood_replacement_entry,
    TopologyNeighborhoodReplacementScope,
};

#[test]
fn rebinding_neighborhood_replacement_receipts_attach_to_mutation_evidence() {
    let prior_declaration = author_primitive_binding_declaration(
        AuthorPrimitiveBindingIntent::attach_surface_to_face(FaceSurfaceBindingSpec::new(
            FaceBindingSite::new("face-old"),
            orthotope_contract(),
            canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
        )),
    );
    let prior_identity = rebinding_prior_fact_from_binding_declaration(
        &prior_declaration,
        "neighborhood-replacement-prior-identity",
    )
    .prior_binding_identity()
    .to_string();
    let successor_declaration = author_primitive_binding_declaration(
        AuthorPrimitiveBindingIntent::attach_surface_to_face(FaceSurfaceBindingSpec::new(
            FaceBindingSite::new("face-new-a"),
            orthotope_contract(),
            canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
        )),
    );
    let declaration = author_primitive_rebinding_declaration(
        crate::binding::tests::support::replace_surface_binding(
            rebinding_prior_fact_from_binding_declaration(
                &prior_declaration,
                "neighborhood-replacement-prior",
            ),
            LocalTopologyReplacementNeighborhood::new(
                NeighborhoodBindingFamily::FaceSurface,
                "face-old",
                ReplacementCandidateSet::new(vec![rebinding_candidate_from_binding_declaration(
                    "successor-a",
                    &successor_declaration,
                    "neighborhood-replacement-successor-a",
                )
                .expect("candidate a")])
                .expect("candidate set"),
            )
            .expect("neighborhood"),
        ),
    );
    let handle = admitted_rebinding_handle("rebinding-neighborhood-replacement");
    let replacement_entry = topology_neighborhood_replacement_entry(
        primitive_rebinding_neighborhood_replacement_source(&declaration, &handle)
            .expect("replacement source"),
    );
    let facts = primitive_rebinding_neighborhood_replacement_facts(&replacement_entry, &handle)
        .expect("replacement facts");
    let evidence = primitive_rebinding_mutation_evidence(&declaration, &handle).expect("evidence");

    assert_eq!(
        facts.replacement_scope(),
        TopologyNeighborhoodReplacementScope::LocalNeighborhood
    );
    assert_eq!(facts.existing_target_identity_basis(), prior_identity);
    assert_eq!(facts.affected_target_identities().len(), 1);
    assert_eq!(
        evidence.neighborhood_replacement().fact_digest(),
        facts.fact_digest()
    );
    assert_eq!(
        evidence
            .neighborhood_replacement()
            .replacement_neighborhood_identity(),
        facts.replacement_neighborhood_identity()
    );
}

use crate::binding::tests::support::{admitted_rebinding_handle, orthotope_contract};
use forge_query::facade::{ForgeQueryGroupedContributionInput, ForgeQueryGroupedDeclarationInput};
use worth_spatial::facade::bindings::{
    author_primitive_binding_declaration, author_primitive_rebinding_declaration,
    AuthorPrimitiveBindingIntent, FaceBindingSite, FaceSurfaceBindingSpec,
    LocalTopologyReplacementNeighborhood, NeighborhoodBindingFamily,
    PrimitiveBindingDeclarationEntry, PrimitiveRebindingDeclarationEntry, ReplacementCandidateSet,
};
use worth_spatial::facade::neighborhood::{
    primitive_rebinding_contribution_workflow, primitive_rebinding_local_neighborhood,
    primitive_rebinding_local_neighborhood_contributions,
};

use crate::binding::tests::support::{
    canonical_geometry, rebinding_candidate_from_binding_declaration,
    rebinding_prior_fact_from_binding_declaration,
};

fn face_surface_binding_declaration(site: &'static str) -> PrimitiveBindingDeclarationEntry {
    author_primitive_binding_declaration(AuthorPrimitiveBindingIntent::attach_surface_to_face(
        FaceSurfaceBindingSpec::new(
            FaceBindingSite::new(site),
            orthotope_contract(),
            canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
        ),
    ))
}

fn grouped_rebinding_entry(
    prior: &PrimitiveBindingDeclarationEntry,
    source_site: &'static str,
    candidate_label: &'static str,
    candidate: &PrimitiveBindingDeclarationEntry,
) -> PrimitiveRebindingDeclarationEntry {
    author_primitive_rebinding_declaration(crate::binding::tests::support::replace_surface_binding(
        rebinding_prior_fact_from_binding_declaration(prior, "grouped-rebinding-prior"),
        LocalTopologyReplacementNeighborhood::new(
            NeighborhoodBindingFamily::FaceSurface,
            source_site,
            ReplacementCandidateSet::new(vec![rebinding_candidate_from_binding_declaration(
                candidate_label,
                candidate,
                "grouped-rebinding-candidate",
            )
            .expect("candidate")])
            .expect("candidate set"),
        )
        .expect("neighborhood"),
    ))
}

#[test]
fn grouped_rebinding_helper_and_generic_builder_lower_to_same_query_shape() {
    let prior_a = face_surface_binding_declaration("face-grouped-prior-a");
    let prior_b = face_surface_binding_declaration("face-grouped-prior-b");
    let successor_a = face_surface_binding_declaration("face-grouped-successor-a");
    let successor_b = face_surface_binding_declaration("face-grouped-successor-b");
    let declaration_a = grouped_rebinding_entry(
        &prior_a,
        "face-grouped-prior-a",
        "successor-a",
        &successor_a,
    );
    let declaration_b = grouped_rebinding_entry(
        &prior_b,
        "face-grouped-prior-b",
        "successor-b",
        &successor_b,
    );
    let helper_input = primitive_rebinding_local_neighborhood(declaration_a.clone())
        .with_member(declaration_b.clone())
        .with_shared_rationale("rebinding grouped helper should preserve local neighborhood truth");
    let generic_input = ForgeQueryGroupedDeclarationInput::local_neighborhood(declaration_a)
        .with_members([declaration_b])
        .with_shared_rationale("rebinding grouped helper should preserve local neighborhood truth");
    let handle = admitted_rebinding_handle("grouped-rebinding-identity-pressure");

    let helper_declaration = match handle.declare_grouped(helper_input) {
        Ok(value) => value,
        Err(stop) => panic!("helper grouped declaration should admit: {stop:?}"),
    };
    let generic_declaration = match handle.declare_grouped(generic_input) {
        Ok(value) => value,
        Err(stop) => panic!("generic grouped declaration should admit: {stop:?}"),
    };
    let helper_support = handle.grouped_support_report(&helper_declaration);
    let generic_support = handle.grouped_support_report(&generic_declaration);
    let helper_outcome = match handle.orchestrate_grouped(helper_declaration.clone()) {
        Ok(value) => value,
        Err(_) => panic!("helper grouped orchestration should admit"),
    };
    let generic_outcome = match handle.orchestrate_grouped(generic_declaration.clone()) {
        Ok(value) => value,
        Err(_) => panic!("generic grouped orchestration should admit"),
    };

    assert_eq!(
        helper_declaration.group_digest(),
        generic_declaration.group_digest()
    );
    assert_eq!(
        helper_declaration.semantics(),
        generic_declaration.semantics()
    );
    assert_eq!(helper_declaration.members().len(), 2);
    assert_eq!(
        helper_declaration.members()[0]
            .declaration()
            .declaration_digest(),
        generic_declaration.members()[0]
            .declaration()
            .declaration_digest()
    );
    assert_eq!(
        helper_declaration.members()[1]
            .declaration()
            .declaration_digest(),
        generic_declaration.members()[1]
            .declaration()
            .declaration_digest()
    );
    assert_eq!(helper_support.statuses(), generic_support.statuses());
    assert_eq!(
        helper_outcome.declaration().group_digest(),
        generic_outcome.declaration().group_digest()
    );
    assert_eq!(helper_outcome.member_envelopes().len(), 2);
    assert_eq!(
        helper_outcome.member_envelopes()[0]
            .envelope()
            .declaration_digest(),
        generic_outcome.member_envelopes()[0]
            .envelope()
            .declaration_digest()
    );
    assert_eq!(
        helper_outcome.member_envelopes()[1]
            .envelope()
            .declaration_digest(),
        generic_outcome.member_envelopes()[1]
            .envelope()
            .declaration_digest()
    );
}

#[test]
fn grouped_rebinding_contribution_helper_and_generic_builder_lower_to_same_query_shape() {
    let prior_a = face_surface_binding_declaration("face-grouped-contrib-prior-a");
    let prior_b = face_surface_binding_declaration("face-grouped-contrib-prior-b");
    let successor_a = face_surface_binding_declaration("face-grouped-contrib-successor-a");
    let successor_b = face_surface_binding_declaration("face-grouped-contrib-successor-b");
    let declaration_a = grouped_rebinding_entry(
        &prior_a,
        "face-grouped-contrib-prior-a",
        "successor-a",
        &successor_a,
    );
    let declaration_b = grouped_rebinding_entry(
        &prior_b,
        "face-grouped-contrib-prior-b",
        "successor-b",
        &successor_b,
    );
    let handle = admitted_rebinding_handle("grouped-rebinding-contribution-pressure");
    let helper_input = primitive_rebinding_local_neighborhood_contributions(
        primitive_rebinding_local_neighborhood(declaration_a.clone())
            .with_member(declaration_b.clone())
            .with_shared_rationale(
                "rebinding grouped contribution helper should preserve member-local continuity truth",
            ),
        &handle,
    );
    let generic_input = ForgeQueryGroupedContributionInput::new(
        ForgeQueryGroupedDeclarationInput::local_neighborhood(declaration_a.clone())
            .with_member(declaration_b.clone())
            .with_shared_rationale(
                "rebinding grouped contribution helper should preserve member-local continuity truth",
            ),
    )
    .with_member_contribution(
        0,
        primitive_rebinding_contribution_workflow(declaration_a, &handle)
            .contributions()[0]
            .clone(),
    )
    .with_member_contribution(
        1,
        primitive_rebinding_contribution_workflow(declaration_b, &handle)
            .contributions()[0]
            .clone(),
    );

    let helper = match handle.grouped_contributions_checked(helper_input) {
        Ok(value) => value,
        Err(_) => panic!("helper grouped contribution lane should admit"),
    };
    let generic = match handle.grouped_contributions_checked(generic_input) {
        Ok(value) => value,
        Err(_) => panic!("generic grouped contribution lane should admit"),
    };

    assert_eq!(
        helper.declaration().group_digest(),
        generic.declaration().group_digest()
    );
    assert_eq!(helper.members().len(), generic.members().len());
    assert_eq!(
        helper.members()[0].0.aspect_record(),
        generic.members()[0].0.aspect_record()
    );
    assert_eq!(
        helper.members()[0].1.composition_digest(),
        generic.members()[0].1.composition_digest()
    );
    assert_eq!(
        helper.members()[1].1.composition_digest(),
        generic.members()[1].1.composition_digest()
    );
}

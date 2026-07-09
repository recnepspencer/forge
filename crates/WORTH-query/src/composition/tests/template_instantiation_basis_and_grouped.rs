use crate::authoring::{
    AspectFieldSelector, GuidedAuthoringPath, OrderingSelector, RootEntityKey, TraversalSelector,
};
use crate::composition::{
    BasisScopeEvidence, GuidedCompositionPath, QueryTemplateDescriptor, TemplateBindingSet,
    TemplateFamily, TemplateParameterSlot,
};
use crate::harness::fixtures::execution_preflights;
use crate::query_context::{
    admit_query_basis_context, bind_query_basis_context, QueryBasisContextRequest,
    QueryContextBindingSource,
};

use super::template_instantiation_support::{
    assert_template_instantiation_artifacts, display_name_equality_predicate,
    template_collection_query, template_collection_shape, template_detail_query,
    template_detail_shape,
};

#[test]
fn detail_template_instantiation_preserves_basis_metadata_when_present() {
    let preflight = execution_preflights::direct_runtime_preflight();
    let binding = bind_query_basis_context(
        QueryBasisContextRequest::current_branch_head(),
        QueryContextBindingSource::RuntimeCurrent(&preflight),
    )
    .unwrap();
    let admitted = admit_query_basis_context(binding).unwrap();
    let direct_query =
        crate::authoring::RawAuthoredQuery::detail_builder(RootEntityKey::new("user").unwrap())
            .project(AspectFieldSelector::new("identity", "id").unwrap())
            .project(AspectFieldSelector::new("profile", "display_name").unwrap())
            .where_equal(
                crate::authoring::EqualityPredicate::new(
                    "profile",
                    "display_name",
                    crate::authoring::ScalarPredicateValue::String("Alice".to_string()),
                )
                .unwrap(),
            )
            .build()
            .unwrap();
    let direct =
        GuidedAuthoringPath::canonicalize_detail(direct_query, template_detail_shape()).unwrap();
    let evidence = BasisScopeEvidence::from_admitted_context_for_canonical_query(
        &admitted,
        direct.query().digest(),
    );

    let predicate_slot = TemplateParameterSlot::predicate("name_filter");
    let template =
        QueryTemplateDescriptor::detail(template_detail_query(), template_detail_shape())
            .with_slot(predicate_slot.clone())
            .with_basis_evidence(evidence.clone());
    let bindings = TemplateBindingSet::new()
        .bind_predicate(&predicate_slot, display_name_equality_predicate("Alice"));
    let (artifact, expanded) =
        GuidedCompositionPath::instantiate_detail_template(template, bindings).unwrap();
    let composed = GuidedCompositionPath::canonicalize_expanded(expanded).unwrap();

    assert_eq!(artifact.template_family(), TemplateFamily::DetailTemplate);
    assert_eq!(
        artifact
            .basis_evidence()
            .expect("basis evidence should be preserved on the artifact")
            .basis_digest(),
        admitted.basis_digest()
    );
    assert_eq!(
        composed.composition().basis_digest(),
        Some(admitted.basis_digest())
    );
}

#[test]
fn template_instantiation_binding_digest_distinguishes_different_predicate_values() {
    let predicate_slot = TemplateParameterSlot::predicate("name_filter");
    let template =
        QueryTemplateDescriptor::detail(template_detail_query(), template_detail_shape())
            .with_slot(predicate_slot.clone());

    let (alice_artifact, alice_expanded) = GuidedCompositionPath::instantiate_detail_template(
        template.clone(),
        TemplateBindingSet::new()
            .bind_predicate(&predicate_slot, display_name_equality_predicate("Alice")),
    )
    .unwrap();
    let (bob_artifact, bob_expanded) = GuidedCompositionPath::instantiate_detail_template(
        template,
        TemplateBindingSet::new()
            .bind_predicate(&predicate_slot, display_name_equality_predicate("Bob")),
    )
    .unwrap();
    let alice_composed = GuidedCompositionPath::canonicalize_expanded(alice_expanded).unwrap();
    let bob_composed = GuidedCompositionPath::canonicalize_expanded(bob_expanded).unwrap();

    assert_ne!(
        alice_artifact.binding_digest(),
        bob_artifact.binding_digest(),
        "binding identity must change when the bound predicate value changes"
    );
    assert_ne!(
        alice_composed.composition().template_binding_digest(),
        bob_composed.composition().template_binding_digest(),
        "composition report must preserve distinct binding digests for distinct bindings"
    );
    assert_ne!(
        alice_composed.canonical().query().digest(),
        bob_composed.canonical().query().digest(),
        "canonical queries should diverge for different bound predicate values"
    );
}

#[test]
fn grouped_collection_template_instantiation_preserves_canonical_parity_and_binding_artifacts() {
    let direct_query =
        crate::authoring::RawAuthoredQuery::collection_builder(RootEntityKey::new("user").unwrap())
            .project(AspectFieldSelector::new("identity", "id").unwrap())
            .project(AspectFieldSelector::new("profile", "display_name").unwrap())
            .order_by(OrderingSelector::ascending("profile", "display_name").unwrap())
            .traverse(TraversalSelector::bounded("manager", 1).unwrap())
            .build()
            .unwrap();
    let direct =
        GuidedAuthoringPath::canonicalize_collection(direct_query, template_collection_shape())
            .unwrap();

    let traversal_slot = TemplateParameterSlot::traversal("manager_hop");
    let template = QueryTemplateDescriptor::grouped_collection(
        template_collection_query(),
        template_collection_shape(),
    )
    .with_slot(traversal_slot.clone());
    let bindings = TemplateBindingSet::new().bind_traversal(
        &traversal_slot,
        TraversalSelector::bounded("manager", 1).unwrap(),
    );
    let (artifact, expanded) =
        GuidedCompositionPath::instantiate_collection_template(template, bindings).unwrap();
    let composed = GuidedCompositionPath::canonicalize_expanded(expanded).unwrap();

    assert_eq!(
        direct.query().digest(),
        composed.canonical().query().digest()
    );
    assert_eq!(
        direct.result_shape().digest(),
        composed.canonical().result_shape().digest()
    );
    assert_template_instantiation_artifacts(
        &artifact,
        &composed,
        TemplateFamily::GroupedCollectionTemplate,
        1,
        1,
    );
}

#[test]
fn grouped_collection_template_instantiation_preserves_basis_metadata_when_present() {
    let preflight = execution_preflights::direct_runtime_preflight();
    let binding = bind_query_basis_context(
        QueryBasisContextRequest::current_branch_head(),
        QueryContextBindingSource::RuntimeCurrent(&preflight),
    )
    .unwrap();
    let admitted = admit_query_basis_context(binding).unwrap();
    let direct_query =
        crate::authoring::RawAuthoredQuery::collection_builder(RootEntityKey::new("user").unwrap())
            .project(AspectFieldSelector::new("identity", "id").unwrap())
            .project(AspectFieldSelector::new("profile", "display_name").unwrap())
            .order_by(OrderingSelector::ascending("profile", "display_name").unwrap())
            .traverse(TraversalSelector::bounded("manager", 1).unwrap())
            .build()
            .unwrap();
    let direct =
        GuidedAuthoringPath::canonicalize_collection(direct_query, template_collection_shape())
            .unwrap();
    let evidence = BasisScopeEvidence::from_admitted_context_for_canonical_query(
        &admitted,
        direct.query().digest(),
    );

    let traversal_slot = TemplateParameterSlot::traversal("manager_hop");
    let template = QueryTemplateDescriptor::grouped_collection(
        template_collection_query(),
        template_collection_shape(),
    )
    .with_slot(traversal_slot.clone())
    .with_basis_evidence(evidence.clone());
    let bindings = TemplateBindingSet::new().bind_traversal(
        &traversal_slot,
        TraversalSelector::bounded("manager", 1).unwrap(),
    );
    let (artifact, expanded) =
        GuidedCompositionPath::instantiate_collection_template(template, bindings).unwrap();
    let composed = GuidedCompositionPath::canonicalize_expanded(expanded).unwrap();

    assert_eq!(
        artifact.template_family(),
        TemplateFamily::GroupedCollectionTemplate
    );
    assert_eq!(
        artifact
            .basis_evidence()
            .expect("grouped collection template should preserve basis evidence")
            .basis_digest(),
        admitted.basis_digest()
    );
    assert_eq!(
        composed.composition().basis_digest(),
        Some(admitted.basis_digest())
    );
}

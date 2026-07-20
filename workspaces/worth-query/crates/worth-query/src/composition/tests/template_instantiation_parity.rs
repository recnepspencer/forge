use crate::authoring::{
    AspectFieldSelector, GuidedAuthoringPath, OrderingSelector, QueryFamily, RootEntityKey,
    TraversalSelector,
};
use crate::composition::{
    GuidedCompositionPath, QueryTemplateDescriptor, TemplateBindingSet, TemplateFamily,
    TemplateParameterSlot,
};

use super::template_instantiation_support::{
    assert_template_instantiation_artifacts, template_collection_query, template_collection_shape,
    template_detail_query, template_detail_shape, template_identity_only_collection_query,
};

#[test]
fn detail_template_instantiation_preserves_canonical_parity_and_binding_artifacts() {
    let direct_query =
        crate::authoring::RawAuthoredQuery::detail_builder(RootEntityKey::new("user").unwrap())
            .project(AspectFieldSelector::new("identity", "id").unwrap())
            .project(AspectFieldSelector::new("profile", "display_name").unwrap())
            .where_equal(
                crate::authoring::EqualityPredicate::new(
                    "profile",
                    "display_name",
                    crate::authoring::WorthQueryPredicateOperand::string("Alice".to_string()),
                )
                .unwrap(),
            )
            .build()
            .unwrap();
    let direct =
        GuidedAuthoringPath::canonicalize_detail(direct_query, template_detail_shape()).unwrap();

    let predicate_slot = TemplateParameterSlot::predicate("name_filter");
    let template =
        QueryTemplateDescriptor::detail(template_detail_query(), template_detail_shape())
            .with_slot(predicate_slot.clone());
    let bindings = TemplateBindingSet::new().bind_predicate(
        &predicate_slot,
        crate::authoring::PredicateSelector::Equality(
            crate::authoring::EqualityPredicate::new(
                "profile",
                "display_name",
                crate::authoring::WorthQueryPredicateOperand::string("Alice".to_string()),
            )
            .unwrap(),
        ),
    );
    let (artifact, expanded) =
        GuidedCompositionPath::instantiate_detail_template(template, bindings).unwrap();
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
        TemplateFamily::DetailTemplate,
        1,
        1,
    );
}

#[test]
fn collection_template_instantiation_preserves_projection_slot_parity_and_binding_artifacts() {
    let direct_query =
        crate::authoring::RawAuthoredQuery::collection_builder(RootEntityKey::new("user").unwrap())
            .project(AspectFieldSelector::new("identity", "id").unwrap())
            .project(AspectFieldSelector::new("profile", "display_name").unwrap())
            .build()
            .unwrap();
    let direct =
        GuidedAuthoringPath::canonicalize_collection(direct_query, template_collection_shape())
            .unwrap();

    let projection_slot = TemplateParameterSlot::projection("display_name_projection");
    let template = QueryTemplateDescriptor::collection(
        template_identity_only_collection_query(),
        template_collection_shape(),
    )
    .with_slot(projection_slot.clone());
    let bindings = TemplateBindingSet::new().bind_projection(
        &projection_slot,
        AspectFieldSelector::new("profile", "display_name").unwrap(),
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
        TemplateFamily::CollectionTemplate,
        1,
        1,
    );
}

#[test]
fn collection_template_instantiation_preserves_canonical_parity_and_binding_artifacts() {
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
    let template = QueryTemplateDescriptor::collection(
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
        TemplateFamily::CollectionTemplate,
        1,
        1,
    );
}

#[test]
fn collection_template_instantiation_preserves_multi_slot_canonical_parity_and_binding_width() {
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

    let ordering_slot = TemplateParameterSlot::ordering("name_first");
    let traversal_slot = TemplateParameterSlot::traversal("manager_hop");
    let template = QueryTemplateDescriptor::collection(
        crate::authoring::RawAuthoredQuery::collection_builder(RootEntityKey::new("user").unwrap())
            .project(AspectFieldSelector::new("identity", "id").unwrap())
            .project(AspectFieldSelector::new("profile", "display_name").unwrap())
            .build()
            .unwrap(),
        template_collection_shape(),
    )
    .with_slot(ordering_slot.clone())
    .with_slot(traversal_slot.clone());
    let bindings = TemplateBindingSet::new()
        .bind_ordering(
            &ordering_slot,
            OrderingSelector::ascending("profile", "display_name").unwrap(),
        )
        .bind_traversal(
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
        TemplateFamily::CollectionTemplate,
        2,
        2,
    );
}

#[test]
fn template_instantiation_preserves_lowered_query_family() {
    let traversal_slot = TemplateParameterSlot::traversal("manager_hop");
    let template = QueryTemplateDescriptor::collection(
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
        composed.canonical().query().family(),
        &QueryFamily::Collection
    );
    assert_eq!(
        artifact.template_family(),
        TemplateFamily::CollectionTemplate
    );
}

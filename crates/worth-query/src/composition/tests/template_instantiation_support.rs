use crate::authoring::{
    AspectFieldSelector, AuthoredResultShapeField, EqualityPredicate, OrderingSelector,
    RootEntityKey, WorthQueryPredicateOperand,
};
use crate::composition::{
    ComposedCanonicalQueryBundle, QueryCompositionFamily, QueryTemplateDescriptor,
    TemplateBindingSet, TemplateFamily, TemplateInstantiationArtifact, TemplateParameterSlot,
};

pub(super) fn template_detail_query() -> crate::authoring::DetailAuthoredQuery {
    crate::authoring::RawAuthoredQuery::detail_builder(RootEntityKey::new("user").unwrap())
        .project(AspectFieldSelector::new("identity", "id").unwrap())
        .project(AspectFieldSelector::new("profile", "display_name").unwrap())
        .build()
        .unwrap()
}

pub(super) fn template_detail_shape() -> crate::authoring::DetailAuthoredResultShape {
    crate::authoring::RawAuthoredResultShape::detail_builder()
        .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
        .field(AuthoredResultShapeField::new("profile", "display_name", "display_name").unwrap())
        .build()
        .unwrap()
}

pub(super) fn template_collection_query() -> crate::authoring::CollectionAuthoredQuery {
    crate::authoring::RawAuthoredQuery::collection_builder(RootEntityKey::new("user").unwrap())
        .project(AspectFieldSelector::new("identity", "id").unwrap())
        .project(AspectFieldSelector::new("profile", "display_name").unwrap())
        .order_by(OrderingSelector::ascending("profile", "display_name").unwrap())
        .build()
        .unwrap()
}

pub(super) fn template_identity_only_collection_query() -> crate::authoring::CollectionAuthoredQuery
{
    crate::authoring::RawAuthoredQuery::collection_builder(RootEntityKey::new("user").unwrap())
        .project(AspectFieldSelector::new("identity", "id").unwrap())
        .build()
        .unwrap()
}

pub(super) fn template_collection_shape() -> crate::authoring::CollectionAuthoredResultShape {
    crate::authoring::RawAuthoredResultShape::collection_builder()
        .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
        .field(AuthoredResultShapeField::new("profile", "display_name", "display_name").unwrap())
        .build()
        .unwrap()
}

pub(super) fn display_name_equality_predicate(value: &str) -> crate::authoring::PredicateSelector {
    crate::authoring::PredicateSelector::Equality(
        EqualityPredicate::new(
            "profile",
            "display_name",
            WorthQueryPredicateOperand::string(value.to_string()),
        )
        .unwrap(),
    )
}

pub(super) fn equality_binding(slot: &TemplateParameterSlot) -> TemplateBindingSet {
    TemplateBindingSet::new().bind_predicate(slot, display_name_equality_predicate("Alice"))
}

pub(super) fn assert_template_instantiation_artifacts(
    artifact: &TemplateInstantiationArtifact,
    composed: &ComposedCanonicalQueryBundle,
    expected_family: TemplateFamily,
    expected_slot_count: usize,
    expected_binding_width: usize,
) {
    assert_eq!(artifact.template_family(), expected_family);
    assert_eq!(
        composed.composition().family(),
        QueryCompositionFamily::TemplateInstantiation
    );
    assert_eq!(
        composed.composition().template_binding_digest(),
        Some(artifact.binding_digest())
    );
    assert_ne!(artifact.binding_digest().as_str(), "");
    assert_ne!(
        composed.composition().composition_digest().as_str(),
        artifact.binding_digest().as_str(),
        "composition digest should not collapse to raw binding identity"
    );
    assert_eq!(
        composed.composition().counters().template_slot_count(),
        expected_slot_count
    );
    assert_eq!(
        composed.composition().counters().template_binding_width(),
        expected_binding_width
    );
    assert_eq!(
        composed
            .composition()
            .counters()
            .template_rediscovery_count(),
        0
    );
}

pub(super) fn observed_inspector_deferred_template() -> QueryTemplateDescriptor<
    crate::authoring::DetailFamily,
    crate::authoring::DetailResultShapeFamily,
> {
    QueryTemplateDescriptor::observed_inspector_deferred_for_test(
        template_detail_query(),
        template_detail_shape(),
    )
}

pub(super) fn focused_inspector_deferred_template() -> QueryTemplateDescriptor<
    crate::authoring::DetailFamily,
    crate::authoring::DetailResultShapeFamily,
> {
    QueryTemplateDescriptor::focused_inspector_deferred_for_test(
        template_detail_query(),
        template_detail_shape(),
    )
}

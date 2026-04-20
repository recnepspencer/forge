use forge_query::facade::{
    AspectFieldSelector, AuthoredResultShapeField, CollectionQueryBuilder,
    CollectionResultShapeBuilder, GuidedCompositionPath, QueryTemplateDescriptor, RootEntityKey,
    TemplateBindingSet,
};

fn main() {
    let query = CollectionQueryBuilder::new(RootEntityKey::new("user").unwrap())
        .project(AspectFieldSelector::new("identity", "id").unwrap())
        .build()
        .unwrap();
    let shape = CollectionResultShapeBuilder::new()
        .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
        .build()
        .unwrap();
    let template = QueryTemplateDescriptor::collection(query, shape);

    let _ = GuidedCompositionPath::instantiate_detail_template(template, TemplateBindingSet::new());
}

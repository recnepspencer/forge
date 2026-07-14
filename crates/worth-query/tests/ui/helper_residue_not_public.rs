use worth_query::facade::foundation::{AspectFieldSelector, AuthoredResultShapeField, GuidedAuthoringPath, RootEntityKey};

fn main() {
    let query = worth_query::facade::foundation::DetailQueryBuilder::new(RootEntityKey::new("task").unwrap())
        .project(AspectFieldSelector::new("title", "text").unwrap())
        .build()
        .unwrap();
    let shape = worth_query::facade::foundation::DetailResultShapeBuilder::new()
        .field(AuthoredResultShapeField::new("title", "text", "title").unwrap())
        .build()
        .unwrap();

    let _ = GuidedAuthoringPath::pair_detail(query, shape)
        .unwrap()
        .with_helper_residue_for_test("builder_history");
}

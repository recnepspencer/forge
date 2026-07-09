use worth_query::facade::{
    AspectFieldSelector, AuthoredResultShapeField, DetailQueryBuilder, DetailResultShapeBuilder,
    GuidedAuthoringPath, GuidedCompositionPath, RootEntityKey,
};

fn main() {
    let query = DetailQueryBuilder::new(RootEntityKey::new("user").unwrap())
        .project(AspectFieldSelector::new("identity", "id").unwrap())
        .build()
        .unwrap();
    let shape = DetailResultShapeBuilder::new()
        .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
        .build()
        .unwrap();
    let canonical = GuidedAuthoringPath::canonicalize_detail(query, shape).unwrap();
    let shape_again = DetailResultShapeBuilder::new()
        .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
        .build()
        .unwrap();

    let _ = GuidedCompositionPath::expand_detail_scopes(
        canonical,
        shape_again,
        std::iter::empty(),
    );
}

use worth_query::facade::{
    optimizer_input_from_narrowed_policy_query, AspectFieldSelector, AuthoredResultShapeField,
    DetailQueryBuilder, DetailResultShapeBuilder, GuidedAuthoringPath, RootEntityKey,
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

    let _ = optimizer_input_from_narrowed_policy_query(canonical.query());
}

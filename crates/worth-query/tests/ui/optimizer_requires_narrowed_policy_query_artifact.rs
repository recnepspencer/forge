use worth_query::facade::foundation::{AspectFieldSelector, AuthoredResultShapeField, DetailQueryBuilder, DetailResultShapeBuilder, GuidedAuthoringPath, RootEntityKey};
use worth_query::facade::policy::optimizer_input_from_narrowed_policy_query;

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

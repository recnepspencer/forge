use worth_query_decl::facade::{
    application_query::ApplicationQueryResultShapeBuilder,
    application_schema::{ApplicationEntityRef, ApplicationRelationRef},
};

struct Schema;
struct Parent;
struct Child;
struct ParentChild;
struct Query;
struct ParentResult;
struct ChildResult;

fn main() {
    let parent = ApplicationEntityRef::<Schema, Parent>::from_schema_identifier("Parent");
    let child = ApplicationEntityRef::<Schema, Child>::from_schema_identifier("Child");
    let relation =
        ApplicationRelationRef::<Schema, ParentChild, Parent, Child>::from_schema_identifiers(
            "ParentChild",
            "Parent",
            "Child",
        );
    let nested =
        ApplicationQueryResultShapeBuilder::<Schema, Query, Child, ChildResult>::new(child);
    let _ = ApplicationQueryResultShapeBuilder::<Schema, Query, Parent, ParentResult>::new(parent)
        .relation(relation, nested);
}

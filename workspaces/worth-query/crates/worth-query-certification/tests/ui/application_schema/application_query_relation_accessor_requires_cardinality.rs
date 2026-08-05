use worth_query_decl::facade::application_query::{
    ApplicationQueryResultRelationRef, ExactlyOneResult, ForwardResultTraversal,
};
use worth_query_execution::facade::primary_graph::WorthQueryApplicationProjectionRow;

struct Schema;
struct Query;
struct Slot;
struct Relation;
struct Parent;
struct Child;

fn project_one_as_many(
    row: &WorthQueryApplicationProjectionRow<'_, Schema, Query>,
    selector: ApplicationQueryResultRelationRef<
        Query,
        Slot,
        Schema,
        Relation,
        Parent,
        Child,
        ForwardResultTraversal,
        ExactlyOneResult,
    >,
) {
    let _ = row.many(selector);
}

fn main() {}

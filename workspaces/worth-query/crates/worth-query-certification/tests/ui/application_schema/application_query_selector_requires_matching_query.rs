use worth_query_decl::facade::{
    application_query::ApplicationQueryResultFieldRef,
    application_schema::{
        EqualityPredicate, NoApplicationCurrency, ReadOnly,
    },
};
use worth_query_execution::facade::primary_graph::WorthQueryApplicationProjectionRow;

struct Schema;
struct Query;
struct ForeignQuery;
struct Slot;
struct Entity;
struct Aspect;
struct Field;

fn project_foreign_selector(
    row: &WorthQueryApplicationProjectionRow<'_, Schema, Query>,
    selector: ApplicationQueryResultFieldRef<
        ForeignQuery,
        Slot,
        Schema,
        Entity,
        Aspect,
        Field,
        u64,
        ReadOnly,
        EqualityPredicate,
        NoApplicationCurrency,
    >,
) {
    let _ = row.field(selector);
}

fn main() {}

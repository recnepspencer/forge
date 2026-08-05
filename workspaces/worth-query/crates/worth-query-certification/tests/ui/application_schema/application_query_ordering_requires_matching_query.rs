use worth_query_decl::facade::{
    application_query::{
        ApplicationQueryDefinitionBuilder, ApplicationQueryOrderingDirection,
        ApplicationQueryResultFieldRef,
    },
    application_schema::{
        EqualityPredicate, NoApplicationCurrency, ReadOnly,
    },
};

struct Schema;
struct Query;
struct ForeignQuery;
struct Parameters;
struct Result;
struct Scope;
struct Slot;
struct Entity;
struct Aspect;
struct Field;

fn order_with_foreign_selector(
    builder: ApplicationQueryDefinitionBuilder<Schema, Query, Parameters, Result, Scope>,
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
    let _ = builder.order_by(selector, ApplicationQueryOrderingDirection::Ascending);
}

fn main() {}

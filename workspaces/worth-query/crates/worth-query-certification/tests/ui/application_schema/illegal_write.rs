use worth_query_decl::facade::application_schema::{
    ApplicationFieldRef, ApplicationOperationRef, EqualityPredicate, ReadOnly,
    TypedOperationBuilder,
};

struct Schema;
struct Entity;
struct Aspect;
struct Field;
struct Operation;
struct Input;

fn main() {
    let operation =
        ApplicationOperationRef::<Schema, Operation, Input>::from_schema_identifier("Operation");
    let field = ApplicationFieldRef::<
        Schema,
        Entity,
        Aspect,
        Field,
        u64,
        ReadOnly,
        EqualityPredicate,
    >::from_schema_identifiers("Entity", "Aspect", "Field");
    let _ = TypedOperationBuilder::new(operation)
        .input(Input)
        .set(field, 7_u64);
}

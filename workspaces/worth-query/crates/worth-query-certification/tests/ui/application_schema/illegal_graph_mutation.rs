use worth_query_decl::facade::application_schema::{
    ApplicationEntityRef, ApplicationOperationRef, ApplicationRelationRef, TypedOperationBuilder,
};

struct Schema;
struct Operation;
struct Entity;
struct Relation;
struct From;
struct To;

fn main() {
    let operation =
        ApplicationOperationRef::<Schema, Operation, ()>::from_schema_identifier("Operation");
    let relation =
        ApplicationRelationRef::<Schema, Relation, From, To>::from_schema_identifiers(
            "Relation", "From", "To",
        );
    let _ = TypedOperationBuilder::new(operation)
        .input(())
        .link(relation);

    let entity = ApplicationEntityRef::<Schema, Entity>::from_schema_identifier("Entity");
    let _ = TypedOperationBuilder::new(operation)
        .input(())
        .delete(entity);
}

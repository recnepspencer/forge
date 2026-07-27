use worth_query_decl::facade::application_schema::{
    ApplicationEntityRef, ApplicationFieldRef, EqualityPredicate, ReadOnly,
    TypedReadDeclarationBuilder,
};

struct FirstSchema;
struct SecondSchema;
struct FirstEntity;
struct SecondEntity;
struct Aspect;
struct Field;

fn main() {
    let entity =
        ApplicationEntityRef::<FirstSchema, FirstEntity>::from_schema_identifier("FirstEntity");
    let foreign = ApplicationFieldRef::<
        SecondSchema,
        SecondEntity,
        Aspect,
        Field,
        u64,
        ReadOnly,
        EqualityPredicate,
    >::from_schema_identifiers("SecondEntity", "Aspect", "Field");
    let _ = TypedReadDeclarationBuilder::new(entity).project(foreign);
}

use worth_query_decl::facade::application_schema::{
    ApplicationEntityRef, ApplicationFieldRef, NoEqualityPredicate, ReadOnly,
    TypedReadDeclarationBuilder,
};

struct Schema;
struct Entity;
struct Aspect;
struct Field;

fn main() {
    let entity = ApplicationEntityRef::<Schema, Entity>::from_schema_identifier("Entity");
    let field = ApplicationFieldRef::<
        Schema,
        Entity,
        Aspect,
        Field,
        u64,
        ReadOnly,
        NoEqualityPredicate,
    >::from_schema_identifiers("Entity", "Aspect", "Field");
    let _ = TypedReadDeclarationBuilder::new(entity).where_equal(field, 7_u64);
}

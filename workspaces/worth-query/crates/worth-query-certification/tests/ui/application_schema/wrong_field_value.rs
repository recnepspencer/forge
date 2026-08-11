use worth_query_decl::facade::application_schema::{
    ApplicationAspectMarkerIdentity, ApplicationEntityMarkerIdentity, ApplicationEntityRef,
    ApplicationFieldMarkerIdentity, ApplicationFieldRef, EqualityPredicate, ReadOnly,
    TypedReadDeclarationBuilder,
};

struct Schema;
struct Entity;
struct Aspect;
struct Field;

impl ApplicationEntityMarkerIdentity for Entity {
    type Schema = Schema;
    const IDENTIFIER: &'static str = "Entity";
}
impl ApplicationAspectMarkerIdentity for Aspect {
    type Schema = Schema;
    type Entity = Entity;
    const IDENTIFIER: &'static str = "Aspect";
}
impl ApplicationFieldMarkerIdentity for Field {
    type Schema = Schema;
    type Entity = Entity;
    type Aspect = Aspect;
    const IDENTIFIER: &'static str = "Field";
}

fn main() {
    let entity = ApplicationEntityRef::<Schema, Entity>::from_schema_identifier("Entity");
    let field = ApplicationFieldRef::<
        Schema,
        Entity,
        Aspect,
        Field,
        u64,
        ReadOnly,
        EqualityPredicate,
    >::from_schema_types();
    let _ = TypedReadDeclarationBuilder::new(entity).where_equal(field, String::from("wrong"));
}

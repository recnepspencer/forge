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
    const ASPECT_IDENTITY: worth_query_decl::facade::application_schema::AspectIdentity =
        worth_query_decl::facade::application_schema::AspectIdentity(0x91612006);
    const CONTRACT_REVISION: worth_query_decl::facade::application_schema::AspectContractRevision =
        worth_query_decl::facade::application_schema::AspectContractRevision(1);
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

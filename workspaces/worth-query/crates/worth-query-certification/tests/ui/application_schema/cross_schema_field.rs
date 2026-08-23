use worth_query_decl::facade::application_schema::{
    ApplicationAspectMarkerIdentity, ApplicationEntityMarkerIdentity, ApplicationEntityRef,
    ApplicationFieldMarkerIdentity, ApplicationFieldRef, EqualityPredicate, ReadOnly,
    TypedReadDeclarationBuilder,
};

struct FirstSchema;
struct SecondSchema;
struct FirstEntity;
struct SecondEntity;
struct Aspect;
struct Field;

impl ApplicationEntityMarkerIdentity for SecondEntity {
    type Schema = SecondSchema;
    const IDENTIFIER: &'static str = "SecondEntity";
}

impl ApplicationAspectMarkerIdentity for Aspect {
    type Schema = SecondSchema;
    type Entity = SecondEntity;
    const IDENTIFIER: &'static str = "Aspect";
    const ASPECT_IDENTITY: worth_query_decl::facade::application_schema::AspectIdentity =
        worth_query_decl::facade::application_schema::AspectIdentity(0x91612001);
    const CONTRACT_REVISION: worth_query_decl::facade::application_schema::AspectContractRevision =
        worth_query_decl::facade::application_schema::AspectContractRevision(1);
}

impl ApplicationFieldMarkerIdentity for Field {
    type Schema = SecondSchema;
    type Entity = SecondEntity;
    type Aspect = Aspect;
    const IDENTIFIER: &'static str = "Field";
}

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
    >::from_schema_types();
    let _ = TypedReadDeclarationBuilder::new(entity).project(foreign);
}

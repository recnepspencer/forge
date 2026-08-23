use worth_query_decl::facade::application_schema::{
    ApplicationAspectMarkerIdentity, ApplicationEntityMarkerIdentity,
    ApplicationFieldMarkerIdentity, ApplicationFieldRef, ApplicationOperationRef,
    EqualityPredicate, ReadOnly, TypedOperationBuilder,
};

struct Schema;
struct Entity;
struct Aspect;
struct Field;
struct Operation;
struct Input;

impl ApplicationEntityMarkerIdentity for Entity {
    type Schema = Schema;
    const IDENTIFIER: &'static str = "Entity";
}

impl ApplicationAspectMarkerIdentity for Aspect {
    type Schema = Schema;
    type Entity = Entity;
    const IDENTIFIER: &'static str = "Aspect";
    const ASPECT_IDENTITY: worth_query_decl::facade::application_schema::AspectIdentity =
        worth_query_decl::facade::application_schema::AspectIdentity(0x91612002);
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
    >::from_schema_types();
    let _ = TypedOperationBuilder::new(operation)
        .input(Input)
        .set(field, 7_u64);
}

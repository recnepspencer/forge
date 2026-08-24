use worth_query_decl::facade::application_schema::{
    ApplicationAspectMarkerIdentity, ApplicationEntityMarkerIdentity,
    ApplicationFieldMarkerIdentity, ApplicationFieldRef, EqualityPredicate, OperationExpectsFact,
    ReadOnly, TypedMutationPreconditions,
};

struct Schema;
struct Operation;
struct Account;
struct Institution;
struct InstitutionFacts;
struct Status;

impl OperationExpectsFact<Operation> for Status {}
impl ApplicationEntityMarkerIdentity for Institution {
    type Schema = Schema;
    const IDENTIFIER: &'static str = "Institution";
}
impl ApplicationAspectMarkerIdentity for InstitutionFacts {
    type Schema = Schema;
    type Entity = Institution;
    const IDENTIFIER: &'static str = "InstitutionFacts";
    const ASPECT_IDENTITY: worth_query_decl::facade::application_schema::AspectIdentity =
        worth_query_decl::facade::application_schema::AspectIdentity(0x91612004);
    const CONTRACT_REVISION: worth_query_decl::facade::application_schema::AspectContractRevision =
        worth_query_decl::facade::application_schema::AspectContractRevision(1);
}
impl ApplicationFieldMarkerIdentity for Status {
    type Schema = Schema;
    type Entity = Institution;
    type Aspect = InstitutionFacts;
    const IDENTIFIER: &'static str = "Status";
}

fn main() {
    let institution_status = ApplicationFieldRef::<
        Schema,
        Institution,
        InstitutionFacts,
        Status,
        String,
        ReadOnly,
        EqualityPredicate,
    >::from_schema_types();
    let _: TypedMutationPreconditions<Schema, Operation, Account> =
        TypedMutationPreconditions::new().expect_fact(institution_status, "open".to_owned());
}

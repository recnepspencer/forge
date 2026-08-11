use worth_query_decl::facade::application_schema::{
    ApplicationAspectMarkerIdentity, ApplicationEntityMarkerIdentity,
    ApplicationFieldMarkerIdentity, ApplicationFieldRef, EqualityPredicate, OperationExpectsFact,
    ReadOnly, TypedMutationPreconditions,
};

struct Schema;
struct Operation;
struct Account;
struct AccountFacts;
struct Status;

impl OperationExpectsFact<Operation> for Status {}
impl ApplicationEntityMarkerIdentity for Account {
    type Schema = Schema;
    const IDENTIFIER: &'static str = "Account";
}
impl ApplicationAspectMarkerIdentity for AccountFacts {
    type Schema = Schema;
    type Entity = Account;
    const IDENTIFIER: &'static str = "AccountFacts";
}
impl ApplicationFieldMarkerIdentity for Status {
    type Schema = Schema;
    type Entity = Account;
    type Aspect = AccountFacts;
    const IDENTIFIER: &'static str = "Status";
}

fn main() {
    let status = ApplicationFieldRef::<
        Schema,
        Account,
        AccountFacts,
        Status,
        String,
        ReadOnly,
        EqualityPredicate,
    >::from_schema_types();
    let _: TypedMutationPreconditions<Schema, Operation, Account> =
        TypedMutationPreconditions::new().expect_version(status, "open".to_owned());
}

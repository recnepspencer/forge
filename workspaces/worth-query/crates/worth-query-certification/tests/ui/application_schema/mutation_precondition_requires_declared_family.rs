use worth_query_decl::facade::application_schema::{
    ApplicationFieldRef, EqualityPredicate, OperationExpectsFact, ReadOnly,
    TypedMutationPreconditions,
};

struct Schema;
struct Operation;
struct Account;
struct AccountFacts;
struct Status;

impl OperationExpectsFact<Operation> for Status {}

fn main() {
    let status = ApplicationFieldRef::<
        Schema,
        Account,
        AccountFacts,
        Status,
        String,
        ReadOnly,
        EqualityPredicate,
    >::from_schema_identifiers("Account", "AccountFacts", "Status");
    let _: TypedMutationPreconditions<Schema, Operation, Account> =
        TypedMutationPreconditions::new().expect_version(status, "open".to_owned());
}

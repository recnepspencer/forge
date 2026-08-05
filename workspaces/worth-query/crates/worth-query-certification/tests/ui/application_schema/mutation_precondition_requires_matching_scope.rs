use worth_query_decl::facade::application_schema::{
    ApplicationFieldRef, EqualityPredicate, OperationExpectsFact, ReadOnly,
    TypedMutationPreconditions,
};

struct Schema;
struct Operation;
struct Account;
struct Institution;
struct InstitutionFacts;
struct Status;

impl OperationExpectsFact<Operation> for Status {}

fn main() {
    let institution_status =
        ApplicationFieldRef::<
            Schema,
            Institution,
            InstitutionFacts,
            Status,
            String,
            ReadOnly,
            EqualityPredicate,
        >::from_schema_identifiers("Institution", "InstitutionFacts", "Status");
    let _: TypedMutationPreconditions<Schema, Operation, Account> =
        TypedMutationPreconditions::new().expect_fact(institution_status, "open".to_owned());
}

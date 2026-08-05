use worth_query_decl::facade::application_schema::{
    ApplicationOperationRef, TypedOperationBuilder,
};

struct Schema;
struct Operation;

struct ExpectedInput;
struct WrongInput;

fn main() {
    let operation =
        ApplicationOperationRef::<Schema, Operation, ExpectedInput>::from_schema_identifier(
            "Operation",
        );
    let _ = TypedOperationBuilder::new(operation).input(WrongInput);
}

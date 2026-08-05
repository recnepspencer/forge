#[path = "capability/contract.rs"]
mod contract;
#[path = "capability/declaration.rs"]
mod declaration;
#[path = "capability/elevated.rs"]
mod elevated;
#[path = "capability/governed_input.rs"]
mod governed_input;

pub use declaration::*;
pub use elevated::*;
pub use governed_input::*;

pub(super) fn install(
    schema: worth_query_declaration::facade::application_schema::ApplicationSchemaDeclarationBuilder<
        super::IdentityExecutionSchema,
    >,
) -> worth_query_declaration::facade::application_schema::ApplicationSchemaDeclarationBuilder<
    super::IdentityExecutionSchema,
> {
    elevated::install(contract::install(schema))
}

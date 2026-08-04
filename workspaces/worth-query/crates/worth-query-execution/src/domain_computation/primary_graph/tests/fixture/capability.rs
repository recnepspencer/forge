#[path = "capability/contract.rs"]
mod contract;
#[path = "capability/declaration.rs"]
mod declaration;
#[path = "capability/elevated.rs"]
mod elevated;

pub use declaration::*;
pub use elevated::*;

pub(super) fn install(
    schema: worth_query_declaration::facade::application_schema::ApplicationSchemaDeclarationBuilder<
        super::IdentityExecutionSchema,
    >,
) -> worth_query_declaration::facade::application_schema::ApplicationSchemaDeclarationBuilder<
    super::IdentityExecutionSchema,
> {
    elevated::install(contract::install(schema))
}

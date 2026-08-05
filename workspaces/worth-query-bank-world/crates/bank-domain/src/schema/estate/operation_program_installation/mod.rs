mod freeze_account;
mod lifecycle;
mod notify_death;
mod open_estate_case;
mod recognize_executor;

use worth_query_decl::facade::application_schema::ApplicationSchemaDeclarationBuilder;

use crate::schema::BankSchema;

pub(super) fn install(
    schema: ApplicationSchemaDeclarationBuilder<BankSchema>,
) -> ApplicationSchemaDeclarationBuilder<BankSchema> {
    let schema = lifecycle::install(schema);
    let schema = freeze_account::install(schema);
    let schema = recognize_executor::install(schema);
    let schema = notify_death::install(schema);
    open_estate_case::install(schema)
}

use worth_query_decl::facade::application_schema::ApplicationSchemaDeclarationBuilder;

use super::EstateCapabilityScopePolicy;
use crate::schema::BankSchema;

pub(super) fn install(
    schema: ApplicationSchemaDeclarationBuilder<BankSchema>,
) -> ApplicationSchemaDeclarationBuilder<BankSchema> {
    schema.policy(EstateCapabilityScopePolicy::reference())
}

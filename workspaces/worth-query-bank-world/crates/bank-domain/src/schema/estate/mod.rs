mod authority_relation_installation;
mod capability_installation;
mod entities;
mod estate_relation_installation;
mod fields;
mod member_installation;
mod policies;
mod policy_installation;
mod relations;
mod values;

pub use entities::*;
pub use fields::*;
pub use policies::*;
pub use relations::*;

use worth_query_decl::facade::application_schema::ApplicationSchemaDeclarationBuilder;

use crate::schema::BankSchema;

pub(crate) fn install_estate_world(
    schema: ApplicationSchemaDeclarationBuilder<BankSchema>,
) -> ApplicationSchemaDeclarationBuilder<BankSchema> {
    let schema = member_installation::install(schema);
    let schema = capability_installation::install(schema);
    let schema = estate_relation_installation::install(schema);
    let schema = authority_relation_installation::install(schema);
    policy_installation::install(schema)
}

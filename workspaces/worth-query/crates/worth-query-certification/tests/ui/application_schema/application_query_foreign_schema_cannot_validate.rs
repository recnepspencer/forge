use worth_query_host::facade::declaration::application_schema::{
    ApplicationSchema, ApplicationSchemaDeclaration, ApplicationSchemaDeclarationBuilder,
    ApplicationSchemaDeclarationDenial,
};
use worth_query_host::facade::domain::{
    WorthQueryInstalledApplicationQuery, WorthQueryInstalledApplicationSchema,
};

struct Schema;
struct ForeignSchema;
struct Query;
struct Parameters;
struct QueryResult;
struct Scope;

impl ApplicationSchema for Schema {
    const OWNER: &'static str = "owner";
    const NAME: &'static str = "Schema";
    const MAJOR: u32 = 1;
    const MINOR: u32 = 0;

    fn declaration(
    ) -> Result<ApplicationSchemaDeclaration<Self>, ApplicationSchemaDeclarationDenial> {
        ApplicationSchemaDeclarationBuilder::for_schema().build()
    }
}

impl ApplicationSchema for ForeignSchema {
    const OWNER: &'static str = "owner";
    const NAME: &'static str = "ForeignSchema";
    const MAJOR: u32 = 1;
    const MINOR: u32 = 0;

    fn declaration(
    ) -> Result<ApplicationSchemaDeclaration<Self>, ApplicationSchemaDeclarationDenial> {
        ApplicationSchemaDeclarationBuilder::for_schema().build()
    }
}

fn foreign_schema_cannot_validate_query(
    schema: &WorthQueryInstalledApplicationSchema<ForeignSchema>,
    query: &WorthQueryInstalledApplicationQuery<Schema, Query, Parameters, QueryResult, Scope>,
) {
    schema.validate_installed_query(query).unwrap();
}

fn main() {}

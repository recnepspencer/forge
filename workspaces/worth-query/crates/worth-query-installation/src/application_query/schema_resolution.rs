use worth_query_declaration::facade::{
    application_query::ApplicationQueryReference,
    application_schema::{ApplicationSchema, ApplicationSchemaMember},
};

use crate::application_schema::WorthQueryInstalledApplicationSchema;

use super::{
    WorthQueryApplicationQueryInstallationDenial, WorthQueryApplicationQueryInstallationDenialKind,
    WorthQueryInstalledApplicationQuery,
};

impl<Schema> WorthQueryInstalledApplicationSchema<Schema>
where
    Schema: ApplicationSchema,
{
    pub fn application_query<Query, Parameters, QueryResult, Scope>(
        &self,
        reference: ApplicationQueryReference<Schema, Query, Parameters, QueryResult, Scope>,
    ) -> Result<
        WorthQueryInstalledApplicationQuery<Schema, Query, Parameters, QueryResult, Scope>,
        WorthQueryApplicationQueryInstallationDenial,
    > {
        let definition = self
            .schema
            .members()
            .iter()
            .find_map(|member| match member {
                ApplicationSchemaMember::ApplicationQuery { definition }
                    if definition.name() == reference.name() =>
                {
                    Some(definition)
                }
                _ => None,
            })
            .ok_or_else(|| {
                WorthQueryApplicationQueryInstallationDenial::new(
                    WorthQueryApplicationQueryInstallationDenialKind::QueryNotInstalled,
                    reference.name(),
                )
            })?;
        if !definition.matches_reference(reference) {
            return Err(WorthQueryApplicationQueryInstallationDenial::new(
                WorthQueryApplicationQueryInstallationDenialKind::QueryMeaningChanged,
                reference.name(),
            ));
        }
        WorthQueryInstalledApplicationQuery::from_installed_schema(self, definition)
    }
}

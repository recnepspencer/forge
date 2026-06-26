use forge_query::facade::{
    ForgeQueryGraphObligationRegistration, ForgeQueryGraphObligationRegistrationCatalog,
};

use super::registration_lowering::registration_from_family_record;
use super::registration_projection_row::WorthTopologyQueryGraphObligationRegistrationProjectionRow;
use crate::validator_invariant_catalog::{
    WorthTopologyLegalityCatalogError, WorthTopologyLegalityFamilyRecord,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTopologyQueryGraphObligationCatalogProjection {
    query_catalog: ForgeQueryGraphObligationRegistrationCatalog,
    registration_projection_rows: Vec<WorthTopologyQueryGraphObligationRegistrationProjectionRow>,
    query_registration_count: usize,
    projection_digest: String,
}

impl WorthTopologyQueryGraphObligationCatalogProjection {
    pub(in crate::validator_invariant_catalog) fn from_family_records(
        records: &[WorthTopologyLegalityFamilyRecord],
    ) -> Result<Self, WorthTopologyLegalityCatalogError> {
        let registrations = records
            .iter()
            .map(registration_from_family_record)
            .collect::<Result<Vec<_>, _>>()?;
        let registration_projection_rows = records
            .iter()
            .zip(registrations.iter())
            .map(|(record, registration)| {
                WorthTopologyQueryGraphObligationRegistrationProjectionRow::from_registration(
                    record,
                    registration,
                )
            })
            .collect::<Vec<_>>();
        Self::from_registrations(registrations, registration_projection_rows)
    }

    fn from_registrations(
        registrations: Vec<ForgeQueryGraphObligationRegistration>,
        registration_projection_rows: Vec<
            WorthTopologyQueryGraphObligationRegistrationProjectionRow,
        >,
    ) -> Result<Self, WorthTopologyLegalityCatalogError> {
        let query_catalog =
            ForgeQueryGraphObligationRegistrationCatalog::from_registrations(registrations)
                .map_err(|error| {
                    WorthTopologyLegalityCatalogError::QueryRegistration(error.to_string())
                })?;
        let projection_row_digest = registration_projection_rows
            .iter()
            .map(|row| row.row_digest())
            .collect::<Vec<_>>()
            .join("|");
        let projection_digest = format!(
            "worth-topo-query-graph-obligation-catalog:{}:{}",
            query_catalog.registration_count(),
            query_catalog.catalog_digest(),
        );
        let projection_digest =
            format!("{projection_digest}:registration-projection-rows:{projection_row_digest}");
        Ok(Self {
            query_registration_count: query_catalog.registration_count(),
            query_catalog,
            registration_projection_rows,
            projection_digest,
        })
    }

    pub const fn query_catalog(&self) -> &ForgeQueryGraphObligationRegistrationCatalog {
        &self.query_catalog
    }

    pub const fn query_registration_count(&self) -> usize {
        self.query_registration_count
    }

    pub fn registration_projection_rows(
        &self,
    ) -> &[WorthTopologyQueryGraphObligationRegistrationProjectionRow] {
        &self.registration_projection_rows
    }

    pub fn projection_digest(&self) -> &str {
        &self.projection_digest
    }
}

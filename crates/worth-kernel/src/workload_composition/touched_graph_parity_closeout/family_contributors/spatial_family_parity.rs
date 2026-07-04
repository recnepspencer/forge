use schema::facade::platform::authority::touched_graph_parity_closeout::{
    TouchedGraphParityClaimKind, TouchedGraphParityFamilyKind,
};
use worth_spatial::touched_graph_parity_closeout::{
    SpatialContributorCatalogRowKind, SpatialFamilyContributorCatalog,
    SpatialFamilyContributorCatalogRow,
};

use crate::workload_composition::planner_owned_routing::current_worth_touched_graph_conflict_selected_route_packet;

use super::spatial_family_catalog::{
    current_spatial_family_contributor_catalog, validate_spatial_family_contributor_catalog,
    SpatialFamilyContributorCatalogErrorKind,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialFamilyParityErrorKind {
    CurrentSelectedRouteUnavailable,
    MissingSpatialCatalog,
    MissingWorthLocalResidueSource,
    LocalQueryGapFabricationStillAuthoritative,
    OperatorLocalLanguageStillAuthoritative,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpatialFamilyParityError {
    kind: SpatialFamilyParityErrorKind,
    detail: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpatialFamilyParityRow {
    kind: SpatialContributorCatalogRowKind,
    family_kind: TouchedGraphParityFamilyKind,
    current_packet_or_digest_source: &'static str,
    public_closeout_digest_source: &'static str,
    support_posture_source: &'static str,
    consumer_residue_source: &'static str,
    worth_local_residue_source: Option<&'static str>,
    selected_identity_fields_produced: &'static [&'static str],
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpatialFamilyParityClaim {
    kind: TouchedGraphParityClaimKind,
    selected_route_identity_digest: String,
    selected_family_identity: String,
    selected_product_identity_digest: String,
    witness_identity_digest: Option<String>,
    rows: Vec<SpatialFamilyParityRow>,
    catalog_digest: String,
}

pub fn current_spatial_family_parity_claim(
) -> Result<SpatialFamilyParityClaim, SpatialFamilyParityError> {
    let catalog = current_spatial_family_contributor_catalog().map_err(|error| {
        let kind = match error.kind() {
            SpatialFamilyContributorCatalogErrorKind::MissingWorthLocalResidueSource => {
                SpatialFamilyParityErrorKind::MissingWorthLocalResidueSource
            }
            SpatialFamilyContributorCatalogErrorKind::LocalQueryGapFabricationStillAuthoritative => {
                SpatialFamilyParityErrorKind::LocalQueryGapFabricationStillAuthoritative
            }
            SpatialFamilyContributorCatalogErrorKind::OperatorLocalLanguageStillAuthoritative => {
                SpatialFamilyParityErrorKind::OperatorLocalLanguageStillAuthoritative
            }
            _ => SpatialFamilyParityErrorKind::MissingSpatialCatalog,
        };
        SpatialFamilyParityError::new(kind, error.detail())
    })?;

    spatial_family_parity_claim_from_catalog(&catalog)
}

pub(crate) fn spatial_family_parity_claim_from_catalog(
    catalog: &SpatialFamilyContributorCatalog,
) -> Result<SpatialFamilyParityClaim, SpatialFamilyParityError> {
    validate_spatial_family_contributor_catalog(catalog).map_err(|error| {
        let kind = match error.kind() {
            SpatialFamilyContributorCatalogErrorKind::MissingWorthLocalResidueSource => {
                SpatialFamilyParityErrorKind::MissingWorthLocalResidueSource
            }
            SpatialFamilyContributorCatalogErrorKind::LocalQueryGapFabricationStillAuthoritative => {
                SpatialFamilyParityErrorKind::LocalQueryGapFabricationStillAuthoritative
            }
            SpatialFamilyContributorCatalogErrorKind::OperatorLocalLanguageStillAuthoritative => {
                SpatialFamilyParityErrorKind::OperatorLocalLanguageStillAuthoritative
            }
            _ => SpatialFamilyParityErrorKind::MissingSpatialCatalog,
        };
        SpatialFamilyParityError::new(kind, error.detail())
    })?;
    let selected_route =
        current_worth_touched_graph_conflict_selected_route_packet().map_err(|_| {
            SpatialFamilyParityError::new(
                SpatialFamilyParityErrorKind::CurrentSelectedRouteUnavailable,
                "spatial family parity claim requires the current selected-route packet",
            )
        })?;
    Ok(SpatialFamilyParityClaim {
        kind: TouchedGraphParityClaimKind::DeclareOnceFamilyParity,
        selected_route_identity_digest: selected_route.selected_route_identity_digest().to_string(),
        selected_family_identity: selected_route.selected_family_identity().to_string(),
        selected_product_identity_digest: selected_route
            .selected_product_identity_digest()
            .to_string(),
        witness_identity_digest: selected_route
            .selected_witness_identity_digest()
            .map(str::to_string),
        rows: catalog
            .rows()
            .iter()
            .map(SpatialFamilyParityRow::from_catalog_row)
            .collect(),
        catalog_digest: catalog.catalog_digest().to_string(),
    })
}

impl SpatialFamilyParityRow {
    fn from_catalog_row(row: &SpatialFamilyContributorCatalogRow) -> Self {
        Self {
            kind: row.kind(),
            family_kind: row.family_kind(),
            current_packet_or_digest_source: row.current_packet_or_digest_source(),
            public_closeout_digest_source: row.public_closeout_digest_source(),
            support_posture_source: row.support_posture_source(),
            consumer_residue_source: row.consumer_residue_source(),
            worth_local_residue_source: row.worth_local_residue_source(),
            selected_identity_fields_produced: row.selected_identity_fields_produced(),
        }
    }

    pub const fn kind(&self) -> SpatialContributorCatalogRowKind {
        self.kind
    }

    pub const fn family_kind(&self) -> TouchedGraphParityFamilyKind {
        self.family_kind
    }

    pub const fn current_packet_or_digest_source(&self) -> &'static str {
        self.current_packet_or_digest_source
    }

    pub const fn public_closeout_digest_source(&self) -> &'static str {
        self.public_closeout_digest_source
    }

    pub const fn support_posture_source(&self) -> &'static str {
        self.support_posture_source
    }

    pub const fn consumer_residue_source(&self) -> &'static str {
        self.consumer_residue_source
    }

    pub const fn worth_local_residue_source(&self) -> Option<&'static str> {
        self.worth_local_residue_source
    }

    pub const fn selected_identity_fields_produced(&self) -> &'static [&'static str] {
        self.selected_identity_fields_produced
    }
}

impl SpatialFamilyParityClaim {
    pub const fn kind(&self) -> TouchedGraphParityClaimKind {
        self.kind
    }

    pub fn selected_route_identity_digest(&self) -> &str {
        &self.selected_route_identity_digest
    }

    pub fn selected_family_identity(&self) -> &str {
        &self.selected_family_identity
    }

    pub fn selected_product_identity_digest(&self) -> &str {
        &self.selected_product_identity_digest
    }

    pub fn witness_identity_digest(&self) -> Option<&str> {
        self.witness_identity_digest.as_deref()
    }

    pub fn rows(&self) -> &[SpatialFamilyParityRow] {
        &self.rows
    }

    pub fn catalog_digest(&self) -> &str {
        &self.catalog_digest
    }
}

impl SpatialFamilyParityError {
    fn new(kind: SpatialFamilyParityErrorKind, detail: impl Into<String>) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub const fn kind(&self) -> SpatialFamilyParityErrorKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

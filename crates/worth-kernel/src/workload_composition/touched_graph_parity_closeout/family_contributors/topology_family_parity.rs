use schema::facade::platform::authority::touched_graph_parity_closeout::{
    TouchedGraphParityClaimKind, TouchedGraphParityFamilyKind,
};
use topology::touched_graph_parity_closeout::{
    TopologyFamilyContributorCatalog, TopologyFamilyContributorCatalogRow,
};

use crate::workload_composition::planner_owned_routing::current_worth_touched_graph_conflict_selected_route_packet;

use super::topology_family_catalog::{
    current_topology_family_contributor_catalog, validate_topology_family_contributor_catalog,
    TopologyFamilyContributorCatalogErrorKind,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TopologyFamilyDeclareOnceParityErrorKind {
    CurrentSelectedRouteUnavailable,
    MissingDeclareOnceCatalog,
    OperatorLocalRoutingStillAuthoritative,
    EntityFallbackStillAuthoritative,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyFamilyDeclareOnceParityError {
    kind: TopologyFamilyDeclareOnceParityErrorKind,
    detail: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyFamilyDeclareOnceParityRow {
    family_kind: TouchedGraphParityFamilyKind,
    current_packet_or_function: &'static str,
    operator_or_stage_coverage: Vec<String>,
    selected_identity_fields_produced: &'static [&'static str],
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyFamilyDeclareOnceParityClaim {
    kind: TouchedGraphParityClaimKind,
    selected_route_identity_digest: String,
    selected_family_identity: String,
    selected_product_identity_digest: String,
    witness_identity_digest: Option<String>,
    rows: Vec<TopologyFamilyDeclareOnceParityRow>,
    catalog_digest: String,
}

pub fn current_topology_family_declare_once_parity_claim(
) -> Result<TopologyFamilyDeclareOnceParityClaim, TopologyFamilyDeclareOnceParityError> {
    let catalog = current_topology_family_contributor_catalog().map_err(|error| {
        let kind = match error.kind() {
            TopologyFamilyContributorCatalogErrorKind::OperatorLocalRoutingStillAuthoritative => {
                TopologyFamilyDeclareOnceParityErrorKind::OperatorLocalRoutingStillAuthoritative
            }
            TopologyFamilyContributorCatalogErrorKind::EntityFallbackStillAuthoritative => {
                TopologyFamilyDeclareOnceParityErrorKind::EntityFallbackStillAuthoritative
            }
            _ => TopologyFamilyDeclareOnceParityErrorKind::MissingDeclareOnceCatalog,
        };
        TopologyFamilyDeclareOnceParityError::new(kind, error.detail())
    })?;

    topology_family_declare_once_parity_claim_from_catalog(&catalog)
}

pub(crate) fn topology_family_declare_once_parity_claim_from_catalog(
    catalog: &TopologyFamilyContributorCatalog,
) -> Result<TopologyFamilyDeclareOnceParityClaim, TopologyFamilyDeclareOnceParityError> {
    validate_topology_family_contributor_catalog(catalog).map_err(|error| {
        let kind = match error.kind() {
            TopologyFamilyContributorCatalogErrorKind::OperatorLocalRoutingStillAuthoritative => {
                TopologyFamilyDeclareOnceParityErrorKind::OperatorLocalRoutingStillAuthoritative
            }
            TopologyFamilyContributorCatalogErrorKind::EntityFallbackStillAuthoritative => {
                TopologyFamilyDeclareOnceParityErrorKind::EntityFallbackStillAuthoritative
            }
            _ => TopologyFamilyDeclareOnceParityErrorKind::MissingDeclareOnceCatalog,
        };
        TopologyFamilyDeclareOnceParityError::new(kind, error.detail())
    })?;
    let selected_route =
        current_worth_touched_graph_conflict_selected_route_packet().map_err(|_| {
            TopologyFamilyDeclareOnceParityError::new(
                TopologyFamilyDeclareOnceParityErrorKind::CurrentSelectedRouteUnavailable,
                "topology family parity claim requires the current selected-route packet",
            )
        })?;
    Ok(TopologyFamilyDeclareOnceParityClaim {
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
            .map(TopologyFamilyDeclareOnceParityRow::from_catalog_row)
            .collect(),
        catalog_digest: catalog.catalog_digest().to_string(),
    })
}

impl TopologyFamilyDeclareOnceParityRow {
    fn from_catalog_row(row: &TopologyFamilyContributorCatalogRow) -> Self {
        Self {
            family_kind: row.family_kind(),
            current_packet_or_function: row.current_packet_or_function(),
            operator_or_stage_coverage: row.operator_or_stage_coverage().to_vec(),
            selected_identity_fields_produced: row.selected_identity_fields_produced(),
        }
    }

    pub const fn family_kind(&self) -> TouchedGraphParityFamilyKind {
        self.family_kind
    }

    pub const fn current_packet_or_function(&self) -> &'static str {
        self.current_packet_or_function
    }

    pub fn operator_or_stage_coverage(&self) -> &[String] {
        &self.operator_or_stage_coverage
    }

    pub const fn selected_identity_fields_produced(&self) -> &'static [&'static str] {
        self.selected_identity_fields_produced
    }
}

impl TopologyFamilyDeclareOnceParityClaim {
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

    pub fn rows(&self) -> &[TopologyFamilyDeclareOnceParityRow] {
        &self.rows
    }

    pub fn catalog_digest(&self) -> &str {
        &self.catalog_digest
    }
}

impl TopologyFamilyDeclareOnceParityError {
    fn new(kind: TopologyFamilyDeclareOnceParityErrorKind, detail: impl Into<String>) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub const fn kind(&self) -> TopologyFamilyDeclareOnceParityErrorKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

use schema::facade::platform::authority::touched_graph_parity_closeout::{
    TouchedGraphParityClaimKind, TouchedGraphParityFamilyKind,
};
use topology::facade::TopologyDerivedReuseDecisionPosture;
use worth_spatial::facade::evidence_lookup_reuse_route::EvidenceLookupReuseDecisionPosture;

use crate::workload_composition::planner_owned_routing::{
    current_worth_touched_graph_conflict_milestone_fifteen_seed,
    current_worth_touched_graph_conflict_public_proof_input,
    current_worth_touched_graph_conflict_selected_route_packet,
};

use super::contributor_catalog::{
    current_reuse_family_contributor_catalog, validate_reuse_family_contributor_catalog,
    ReuseFamilyContributorCatalog,
};
use super::error::ReuseFamilyContributorCatalogErrorKind;
use super::row::{ReuseFamilyContributorCatalogRow, ReuseFamilyContributorRowKind};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReuseFamilyParityErrorKind {
    CurrentSelectedRouteUnavailable,
    CurrentPublicProofUnavailable,
    MissingReuseCatalog,
    MismatchedEquivalenceIdentity,
    MismatchedReuseIdentity,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReuseFamilyParityError {
    kind: ReuseFamilyParityErrorKind,
    detail: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReuseFamilyParityRow {
    kind: ReuseFamilyContributorRowKind,
    family_kind: TouchedGraphParityFamilyKind,
    current_packet_or_identity_source: &'static str,
    carried_equivalence_or_compatibility_source: &'static str,
    carried_reuse_or_denial_source: &'static str,
    route_packet_identity: String,
    topology_equivalence_policy_identity_digest: String,
    topology_selected_compatibility_basis_identity_digest: String,
    topology_selected_reuse_basis_identity_digest: String,
    topology_posture: TopologyDerivedReuseDecisionPosture,
    topology_rebuild_denial_identity_digest: Option<String>,
    spatial_equivalence_policy_identity_digest: String,
    spatial_selected_compatibility_basis_identity_digest: String,
    spatial_selected_reuse_basis_identity_digest: String,
    spatial_posture: EvidenceLookupReuseDecisionPosture,
    spatial_rebuild_denial_identity_digest: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReuseFamilyParityClaim {
    kind: TouchedGraphParityClaimKind,
    selected_route_identity_digest: String,
    selected_family_identity: String,
    selected_product_identity_digest: String,
    witness_identity_digest: Option<String>,
    rows: Vec<ReuseFamilyParityRow>,
}

pub fn current_reuse_family_parity_claim() -> Result<ReuseFamilyParityClaim, ReuseFamilyParityError>
{
    let catalog = current_reuse_family_contributor_catalog().map_err(|error| {
        ReuseFamilyParityError::new(map_catalog_error_kind(error.kind()), error.detail())
    })?;
    reuse_family_parity_claim_from_catalog(&catalog)
}

pub(crate) fn reuse_family_parity_claim_from_catalog(
    catalog: &ReuseFamilyContributorCatalog,
) -> Result<ReuseFamilyParityClaim, ReuseFamilyParityError> {
    validate_reuse_family_contributor_catalog(catalog).map_err(|error| {
        ReuseFamilyParityError::new(map_catalog_error_kind(error.kind()), error.detail())
    })?;
    let selected_route =
        current_worth_touched_graph_conflict_selected_route_packet().map_err(|error| {
            ReuseFamilyParityError::new(
                ReuseFamilyParityErrorKind::CurrentSelectedRouteUnavailable,
                error.detail(),
            )
        })?;
    let public_proof =
        current_worth_touched_graph_conflict_public_proof_input().map_err(|error| {
            ReuseFamilyParityError::new(
                ReuseFamilyParityErrorKind::CurrentPublicProofUnavailable,
                error.detail(),
            )
        })?;
    let milestone_fifteen_seed = current_worth_touched_graph_conflict_milestone_fifteen_seed()
        .map_err(|error| {
            ReuseFamilyParityError::new(
                ReuseFamilyParityErrorKind::CurrentPublicProofUnavailable,
                error.detail(),
            )
        })?;

    for row in catalog.rows() {
        if row.route_packet_identity()
            != selected_route.compiled_product_reuse_route_packet_identity()
            || row.route_packet_identity()
                != public_proof.compiled_product_reuse_route_packet_identity()
            || row.topology_selected_family_identity() != selected_route.selected_family_identity()
            || row.spatial_selected_family_identity()
                != selected_route.spatial_selected_family_identity()
            || row.topology_selected_product_identity_digest()
                != selected_route.selected_product_identity_digest()
            || row.spatial_selected_product_identity_digest()
                != selected_route.spatial_selected_product_identity_digest()
        {
            return Err(ReuseFamilyParityError::new(
                row_error_kind(row.kind()),
                format!(
                    "reuse-family parity requires {} row to carry the exact selected-route and public-proof compiled-product family/product identities",
                    row.kind().as_str()
                ),
            ));
        }
        if row.topology_equivalence_policy_identity_digest()
            != selected_route.selected_equivalence_policy_identity_digest()
            || row.spatial_equivalence_policy_identity_digest()
                != selected_route.spatial_equivalence_policy_identity_digest()
            || row.topology_equivalence_policy_identity_digest()
                != milestone_fifteen_seed.topology_equivalence_policy_identity_digest()
            || row.spatial_equivalence_policy_identity_digest()
                != public_proof.spatial_equivalence_policy_identity_digest()
        {
            return Err(ReuseFamilyParityError::new(
                row_error_kind(row.kind()),
                format!(
                    "reuse-family parity requires {} row to carry the exact equivalence policy identities admitted by selected-route and public-proof surfaces",
                    row.kind().as_str()
                ),
            ));
        }
        if row.certified_topology_equivalence_basis_digest()
            != milestone_fifteen_seed.topology_query_selected_compatibility_basis_identity_digest()
            || row.certified_spatial_equivalence_basis_digest()
                != selected_route.spatial_selected_compatibility_basis_identity_digest()
            || row.certified_spatial_equivalence_basis_digest()
                != public_proof.spatial_selected_compatibility_basis_identity_digest()
        {
            return Err(ReuseFamilyParityError::new(
                row_error_kind(row.kind()),
                format!(
                    "reuse-family parity requires {} row to carry the exact topology and spatial compatibility basis identities admitted by selected-route and public-proof proof surfaces",
                    row.kind().as_str()
                ),
            ));
        }
        match row.kind() {
            ReuseFamilyContributorRowKind::Equivalence => {}
            ReuseFamilyContributorRowKind::Reuse => {
                if row.topology_selected_reuse_basis_identity_digest()
                    != selected_route.selected_reuse_basis_identity_digest()
                    || row.topology_selected_reuse_basis_identity_digest()
                        != milestone_fifteen_seed
                            .topology_query_selected_reuse_basis_identity_digest()
                    || row.spatial_selected_reuse_basis_identity_digest()
                        != selected_route.spatial_selected_reuse_basis_identity_digest()
                    || row.spatial_selected_reuse_basis_identity_digest()
                        != milestone_fifteen_seed.spatial_selected_reuse_basis_identity_digest()
                    || row.topology_reuse_decision_identity_digest()
                        != public_proof.selected_witness_identity_digest()
                    || row.topology_posture() != selected_route.topology_reuse_posture()
                    || public_proof.topology_reuse_posture() != Some(row.topology_posture())
                    || row.topology_rebuild_denial_identity_digest()
                        != public_proof.rebuild_denial_identity_digest()
                    || row.spatial_reuse_decision_identity_digest()
                        != public_proof.spatial_reuse_decision_identity_digest()
                    || row.spatial_posture() != selected_route.spatial_reuse_posture()
                    || public_proof.spatial_reuse_posture() != Some(row.spatial_posture())
                    || row.spatial_rebuild_denial_identity_digest()
                        != public_proof.spatial_rebuild_denial_identity_digest()
                {
                    return Err(ReuseFamilyParityError::new(
                        ReuseFamilyParityErrorKind::MismatchedReuseIdentity,
                        "reuse-family reuse row must carry the exact reuse basis, posture, decision, and denial identities admitted by selected-route and public-proof witnesses",
                    ));
                }
            }
        }
    }

    Ok(ReuseFamilyParityClaim {
        kind: TouchedGraphParityClaimKind::SelectedRouteParity,
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
            .map(ReuseFamilyParityRow::from_catalog_row)
            .collect(),
    })
}

impl ReuseFamilyParityRow {
    fn from_catalog_row(row: &ReuseFamilyContributorCatalogRow) -> Self {
        Self {
            kind: row.kind(),
            family_kind: row.family_kind(),
            current_packet_or_identity_source: row.current_packet_or_identity_source(),
            carried_equivalence_or_compatibility_source: row
                .carried_equivalence_or_compatibility_source(),
            carried_reuse_or_denial_source: row.carried_reuse_or_denial_source(),
            route_packet_identity: row.route_packet_identity().to_string(),
            topology_equivalence_policy_identity_digest: row
                .topology_equivalence_policy_identity_digest()
                .to_string(),
            topology_selected_compatibility_basis_identity_digest: row
                .certified_topology_equivalence_basis_digest()
                .to_string(),
            topology_selected_reuse_basis_identity_digest: row
                .topology_selected_reuse_basis_identity_digest()
                .to_string(),
            topology_posture: row.topology_posture(),
            topology_rebuild_denial_identity_digest: row
                .topology_rebuild_denial_identity_digest()
                .map(str::to_string),
            spatial_equivalence_policy_identity_digest: row
                .spatial_equivalence_policy_identity_digest()
                .to_string(),
            spatial_selected_compatibility_basis_identity_digest: row
                .certified_spatial_equivalence_basis_digest()
                .to_string(),
            spatial_selected_reuse_basis_identity_digest: row
                .spatial_selected_reuse_basis_identity_digest()
                .to_string(),
            spatial_posture: row.spatial_posture(),
            spatial_rebuild_denial_identity_digest: row
                .spatial_rebuild_denial_identity_digest()
                .map(str::to_string),
        }
    }

    pub const fn kind(&self) -> ReuseFamilyContributorRowKind {
        self.kind
    }
    pub const fn family_kind(&self) -> TouchedGraphParityFamilyKind {
        self.family_kind
    }
    pub const fn current_packet_or_identity_source(&self) -> &'static str {
        self.current_packet_or_identity_source
    }
    pub const fn carried_equivalence_or_compatibility_source(&self) -> &'static str {
        self.carried_equivalence_or_compatibility_source
    }
    pub const fn carried_reuse_or_denial_source(&self) -> &'static str {
        self.carried_reuse_or_denial_source
    }
    pub fn route_packet_identity(&self) -> &str {
        &self.route_packet_identity
    }
}

impl ReuseFamilyParityClaim {
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
    pub fn rows(&self) -> &[ReuseFamilyParityRow] {
        &self.rows
    }
}

impl ReuseFamilyParityError {
    fn new(kind: ReuseFamilyParityErrorKind, detail: impl Into<String>) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub const fn kind(&self) -> ReuseFamilyParityErrorKind {
        self.kind
    }
    pub fn detail(&self) -> &str {
        &self.detail
    }
}

fn map_catalog_error_kind(
    kind: ReuseFamilyContributorCatalogErrorKind,
) -> ReuseFamilyParityErrorKind {
    match kind {
        ReuseFamilyContributorCatalogErrorKind::CurrentSurfaceUnavailable => {
            ReuseFamilyParityErrorKind::CurrentSelectedRouteUnavailable
        }
        ReuseFamilyContributorCatalogErrorKind::MissingRequiredRow => {
            ReuseFamilyParityErrorKind::MissingReuseCatalog
        }
        ReuseFamilyContributorCatalogErrorKind::MissingCarriedIdentity
        | ReuseFamilyContributorCatalogErrorKind::MismatchedReuseSemantics => {
            ReuseFamilyParityErrorKind::MismatchedEquivalenceIdentity
        }
    }
}

fn row_error_kind(kind: ReuseFamilyContributorRowKind) -> ReuseFamilyParityErrorKind {
    match kind {
        ReuseFamilyContributorRowKind::Equivalence => {
            ReuseFamilyParityErrorKind::MismatchedEquivalenceIdentity
        }
        ReuseFamilyContributorRowKind::Reuse => ReuseFamilyParityErrorKind::MismatchedReuseIdentity,
    }
}

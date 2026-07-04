use schema::facade::platform::authority::touched_graph_parity_closeout::TouchedGraphParityClaimKind;

use crate::workload_composition::planner_owned_routing::{
    current_worth_touched_graph_conflict_public_closeout,
    current_worth_touched_graph_conflict_public_facade_with_artifact_policy,
    current_worth_touched_graph_conflict_selected_route_packet,
    require_matching_projection_authority,
    WorthTouchedGraphConflictDerivedDiagnosticArtifactPolicy,
};

use super::contributor_catalog::{
    current_public_projection_contributor_catalog, validate_public_projection_contributor_catalog,
    PublicProjectionContributorCatalog,
};
use super::error::{
    PublicProjectionContributorCatalogError, PublicProjectionContributorCatalogErrorKind,
};
use super::row::{PublicProjectionContributorCatalogRow, PublicProjectionContributorRowKind};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PublicProjectionParityErrorKind {
    CurrentSelectedRouteUnavailable,
    CurrentPublicProofUnavailable,
    MissingProjectionCatalog,
    MismatchedProjectionAuthority,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicProjectionParityError {
    kind: PublicProjectionParityErrorKind,
    detail: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicProjectionParityRow {
    kind: PublicProjectionContributorRowKind,
    current_packet_or_identity_source: &'static str,
    carried_projection_authority_source: &'static str,
    selected_route_identity_digest: String,
    selected_family_identity: String,
    selected_product_identity_digest: String,
    selected_witness_identity_digest: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicProjectionParityClaim {
    kind: TouchedGraphParityClaimKind,
    selected_route_identity_digest: String,
    rows: Vec<PublicProjectionParityRow>,
}

pub fn current_public_projection_parity_claim(
) -> Result<PublicProjectionParityClaim, PublicProjectionParityError> {
    let catalog = current_public_projection_contributor_catalog().map_err(|error| {
        PublicProjectionParityError::new(map_catalog_error_kind(error.kind()), error.detail())
    })?;
    public_projection_parity_claim_from_catalog(&catalog)
}

pub(crate) fn public_projection_parity_claim_from_catalog(
    catalog: &PublicProjectionContributorCatalog,
) -> Result<PublicProjectionParityClaim, PublicProjectionParityError> {
    validate_public_projection_contributor_catalog(catalog).map_err(|error| {
        PublicProjectionParityError::new(map_catalog_error_kind(error.kind()), error.detail())
    })?;
    let selected_route =
        current_worth_touched_graph_conflict_selected_route_packet().map_err(|error| {
            PublicProjectionParityError::new(
                PublicProjectionParityErrorKind::CurrentSelectedRouteUnavailable,
                error.detail(),
            )
        })?;
    let public_closeout =
        current_worth_touched_graph_conflict_public_closeout().map_err(|error| {
            PublicProjectionParityError::new(
                PublicProjectionParityErrorKind::CurrentPublicProofUnavailable,
                error.detail(),
            )
        })?;
    let public_facade = current_worth_touched_graph_conflict_public_facade_with_artifact_policy(
        WorthTouchedGraphConflictDerivedDiagnosticArtifactPolicy::MinimalOperationalTruth,
    )
    .map_err(|error| {
        PublicProjectionParityError::new(
            PublicProjectionParityErrorKind::CurrentPublicProofUnavailable,
            error.detail(),
        )
    })?;
    require_matching_projection_authority(&public_closeout, public_facade.derived_diagnostics())
        .map_err(|detail| {
            PublicProjectionParityError::new(
                PublicProjectionParityErrorKind::MismatchedProjectionAuthority,
                detail,
            )
        })?;

    for row in catalog.rows() {
        if row.selected_route_identity_digest() != selected_route.selected_route_identity_digest()
            || row.selected_family_identity() != public_closeout.selected_family_identity()
            || row.selected_product_identity_digest()
                != public_closeout.selected_product_identity_digest()
        {
            return Err(PublicProjectionParityError::new(
                PublicProjectionParityErrorKind::MismatchedProjectionAuthority,
                format!(
                    "public projection parity requires {} row to carry the exact selected-route, family, and product identities",
                    row.kind().as_str()
                ),
            ));
        }

        match row.kind() {
            PublicProjectionContributorRowKind::PublicProof => {
                let inspection = public_facade.public_proof();
                let seed = inspection.milestone_fifteen_seed();
                if row.selected_witness_identity_digest()
                    != inspection.selected_witness_identity_digest()
                    || row.proof_chain_digest() != Some(inspection.proof_chain_digest())
                    || row.milestone_fifteen_seed_digest() != Some(seed.seed_digest())
                    || row.residue_digest() != Some(inspection.residue_chain().residue_digest())
                    || row.source_firewall_digest() != Some(inspection.source_firewall_digest())
                    || row.source_firewall_digest() != Some(selected_route.source_firewall_digest())
                {
                    return Err(PublicProjectionParityError::new(
                        PublicProjectionParityErrorKind::MismatchedProjectionAuthority,
                        "public projection parity requires public-proof rows to carry the exact witness, seed, residue, and source-firewall digests",
                    ));
                }
            }
            PublicProjectionContributorRowKind::DerivedDiagnostics => {
                let diagnostics = public_facade.derived_diagnostics();
                if row.selected_witness_identity_digest()
                    != diagnostics.selected_witness_identity_digest()
                    || row.decision_trace_identity_digest()
                        != Some(diagnostics.decision_trace_identity_digest())
                {
                    return Err(PublicProjectionParityError::new(
                        PublicProjectionParityErrorKind::MismatchedProjectionAuthority,
                        "public projection parity requires derived-diagnostics rows to carry the exact witness and decision-trace identities",
                    ));
                }
            }
        }
    }

    Ok(PublicProjectionParityClaim {
        kind: TouchedGraphParityClaimKind::PublicProjectionParity,
        selected_route_identity_digest: selected_route.selected_route_identity_digest().to_string(),
        rows: catalog
            .rows()
            .iter()
            .map(PublicProjectionParityRow::from_catalog_row)
            .collect(),
    })
}

impl PublicProjectionParityRow {
    fn from_catalog_row(row: &PublicProjectionContributorCatalogRow) -> Self {
        Self {
            kind: row.kind(),
            current_packet_or_identity_source: row.current_packet_or_identity_source(),
            carried_projection_authority_source: row.carried_projection_authority_source(),
            selected_route_identity_digest: row.selected_route_identity_digest().to_string(),
            selected_family_identity: row.selected_family_identity().to_string(),
            selected_product_identity_digest: row.selected_product_identity_digest().to_string(),
            selected_witness_identity_digest: row
                .selected_witness_identity_digest()
                .map(str::to_string),
        }
    }

    pub const fn kind(&self) -> PublicProjectionContributorRowKind {
        self.kind
    }
    pub const fn family_kind(&self) -> schema::facade::platform::authority::touched_graph_parity_closeout::TouchedGraphParityFamilyKind {
        self.kind.family_kind()
    }
    pub const fn current_packet_or_identity_source(&self) -> &'static str {
        self.current_packet_or_identity_source
    }
    pub const fn carried_projection_authority_source(&self) -> &'static str {
        self.carried_projection_authority_source
    }
}

impl PublicProjectionParityClaim {
    pub const fn kind(&self) -> TouchedGraphParityClaimKind {
        self.kind
    }
    pub fn selected_route_identity_digest(&self) -> &str {
        &self.selected_route_identity_digest
    }
    pub fn rows(&self) -> &[PublicProjectionParityRow] {
        &self.rows
    }
}

impl PublicProjectionParityError {
    fn new(kind: PublicProjectionParityErrorKind, detail: impl Into<String>) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub const fn kind(&self) -> PublicProjectionParityErrorKind {
        self.kind
    }
    pub fn detail(&self) -> &str {
        &self.detail
    }
}

fn map_catalog_error_kind(
    kind: PublicProjectionContributorCatalogErrorKind,
) -> PublicProjectionParityErrorKind {
    match kind {
        PublicProjectionContributorCatalogErrorKind::CurrentSurfaceUnavailable => {
            PublicProjectionParityErrorKind::CurrentPublicProofUnavailable
        }
        PublicProjectionContributorCatalogErrorKind::MissingRequiredRow => {
            PublicProjectionParityErrorKind::MissingProjectionCatalog
        }
        PublicProjectionContributorCatalogErrorKind::MissingCarriedIdentity
        | PublicProjectionContributorCatalogErrorKind::MismatchedProjectionAuthority => {
            PublicProjectionParityErrorKind::MismatchedProjectionAuthority
        }
    }
}

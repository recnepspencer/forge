use schema::facade::platform::authority::touched_graph_parity_closeout::{
    TouchedGraphParityClaimKind, TouchedGraphParityFamilyKind,
};

use crate::workload_composition::planner_owned_routing::{
    current_worth_touched_graph_conflict_selected_route_packet,
    current_worth_workload_ordinary_consumer_cutover,
};

use super::contributor_catalog::{
    current_replay_undo_family_contributor_catalog,
    validate_replay_undo_family_contributor_catalog, ReplayUndoFamilyContributorCatalog,
};
use super::error::ReplayUndoFamilyContributorCatalogErrorKind;
use super::row::{ReplayUndoContributorRowKind, ReplayUndoFamilyContributorCatalogRow};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReplayUndoFamilyParityErrorKind {
    CurrentSelectedRouteUnavailable,
    CurrentCutoverUnavailable,
    MissingReplayUndoCatalog,
    MismatchedReplayIdentity,
    MismatchedUndoIdentity,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayUndoFamilyParityError {
    kind: ReplayUndoFamilyParityErrorKind,
    detail: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayUndoFamilyParityRow {
    kind: ReplayUndoContributorRowKind,
    family_kind: TouchedGraphParityFamilyKind,
    current_packet_or_identity_source: &'static str,
    carried_scope_identity_source: &'static str,
    carried_witness_or_boundary_source: &'static str,
    route_packet_identity: String,
    transaction_packet_identity: String,
    carried_scope_identity: String,
    carried_boundary_proof_digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayUndoFamilyParityClaim {
    kind: TouchedGraphParityClaimKind,
    selected_route_identity_digest: String,
    selected_family_identity: String,
    selected_product_identity_digest: String,
    witness_identity_digest: Option<String>,
    rows: Vec<ReplayUndoFamilyParityRow>,
}

pub fn current_replay_undo_family_parity_claim(
) -> Result<ReplayUndoFamilyParityClaim, ReplayUndoFamilyParityError> {
    let catalog = current_replay_undo_family_contributor_catalog().map_err(|error| {
        ReplayUndoFamilyParityError::new(map_catalog_error_kind(error.kind()), error.detail())
    })?;
    replay_undo_family_parity_claim_from_catalog(&catalog)
}

pub(crate) fn replay_undo_family_parity_claim_from_catalog(
    catalog: &ReplayUndoFamilyContributorCatalog,
) -> Result<ReplayUndoFamilyParityClaim, ReplayUndoFamilyParityError> {
    validate_replay_undo_family_contributor_catalog(catalog).map_err(|error| {
        ReplayUndoFamilyParityError::new(map_catalog_error_kind(error.kind()), error.detail())
    })?;
    let selected_route =
        current_worth_touched_graph_conflict_selected_route_packet().map_err(|error| {
            ReplayUndoFamilyParityError::new(
                ReplayUndoFamilyParityErrorKind::CurrentSelectedRouteUnavailable,
                error.detail(),
            )
        })?;
    let cutover = current_worth_workload_ordinary_consumer_cutover().map_err(|error| {
        ReplayUndoFamilyParityError::new(
            ReplayUndoFamilyParityErrorKind::CurrentCutoverUnavailable,
            format!("{error:?}"),
        )
    })?;

    for row in catalog.rows() {
        let selected_scope_matches = match row.kind() {
            ReplayUndoContributorRowKind::Replay => selected_route
                .replay_scope_identities()
                .iter()
                .any(|identity| identity == row.carried_scope_identity()),
            ReplayUndoContributorRowKind::Undo => selected_route
                .undo_scope_identities()
                .iter()
                .any(|identity| identity == row.carried_scope_identity()),
        };
        let cutover_scope_matches = match row.kind() {
            ReplayUndoContributorRowKind::Replay => cutover
                .replay_scope_identities()
                .iter()
                .any(|identity| identity == row.carried_scope_identity()),
            ReplayUndoContributorRowKind::Undo => cutover
                .undo_scope_identities()
                .iter()
                .any(|identity| identity == row.carried_scope_identity()),
        };
        if !selected_scope_matches || !cutover_scope_matches {
            return Err(ReplayUndoFamilyParityError::new(
                row_error_kind(row.kind()),
                format!(
                    "replay/undo parity requires {} row to carry the exact scope identity admitted by both selected-route and ordinary cutover witnesses",
                    row.kind().as_str()
                ),
            ));
        }
        if !selected_route
            .replay_undo_boundary_proof_digests()
            .iter()
            .any(|digest| digest == row.carried_boundary_proof_digest())
            || !cutover
                .replay_undo_boundary_proof_digests()
                .iter()
                .any(|digest| digest == row.carried_boundary_proof_digest())
            || !selected_route
                .transaction_packet_identities()
                .iter()
                .any(|identity| identity == row.transaction_packet_identity())
            || !cutover
                .transaction_packet_identities()
                .iter()
                .any(|identity| identity == row.transaction_packet_identity())
            || row.route_packet_identity() != selected_route.replay_undo_route_packet_identity()
        {
            return Err(ReplayUndoFamilyParityError::new(
                row_error_kind(row.kind()),
                format!(
                    "replay/undo parity requires {} row to carry the exact route packet, transaction packet, and boundary proof identities used by the selected-route chain",
                    row.kind().as_str()
                ),
            ));
        }
    }

    Ok(ReplayUndoFamilyParityClaim {
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
            .map(ReplayUndoFamilyParityRow::from_catalog_row)
            .collect(),
    })
}

impl ReplayUndoFamilyParityRow {
    fn from_catalog_row(row: &ReplayUndoFamilyContributorCatalogRow) -> Self {
        Self {
            kind: row.kind(),
            family_kind: row.family_kind(),
            current_packet_or_identity_source: row.current_packet_or_identity_source(),
            carried_scope_identity_source: row.carried_scope_identity_source(),
            carried_witness_or_boundary_source: row.carried_witness_or_boundary_source(),
            route_packet_identity: row.route_packet_identity().to_string(),
            transaction_packet_identity: row.transaction_packet_identity().to_string(),
            carried_scope_identity: row.carried_scope_identity().to_string(),
            carried_boundary_proof_digest: row.carried_boundary_proof_digest().to_string(),
        }
    }

    pub const fn kind(&self) -> ReplayUndoContributorRowKind {
        self.kind
    }

    pub const fn family_kind(&self) -> TouchedGraphParityFamilyKind {
        self.family_kind
    }

    pub const fn current_packet_or_identity_source(&self) -> &'static str {
        self.current_packet_or_identity_source
    }

    pub const fn carried_scope_identity_source(&self) -> &'static str {
        self.carried_scope_identity_source
    }

    pub const fn carried_witness_or_boundary_source(&self) -> &'static str {
        self.carried_witness_or_boundary_source
    }

    pub fn route_packet_identity(&self) -> &str {
        &self.route_packet_identity
    }

    pub fn transaction_packet_identity(&self) -> &str {
        &self.transaction_packet_identity
    }

    pub fn carried_scope_identity(&self) -> &str {
        &self.carried_scope_identity
    }

    pub fn carried_boundary_proof_digest(&self) -> &str {
        &self.carried_boundary_proof_digest
    }
}

impl ReplayUndoFamilyParityClaim {
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

    pub fn rows(&self) -> &[ReplayUndoFamilyParityRow] {
        &self.rows
    }
}

impl ReplayUndoFamilyParityError {
    fn new(kind: ReplayUndoFamilyParityErrorKind, detail: impl Into<String>) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub const fn kind(&self) -> ReplayUndoFamilyParityErrorKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

fn map_catalog_error_kind(
    kind: ReplayUndoFamilyContributorCatalogErrorKind,
) -> ReplayUndoFamilyParityErrorKind {
    match kind {
        ReplayUndoFamilyContributorCatalogErrorKind::CurrentSurfaceUnavailable => {
            ReplayUndoFamilyParityErrorKind::CurrentSelectedRouteUnavailable
        }
        ReplayUndoFamilyContributorCatalogErrorKind::MissingRequiredRow => {
            ReplayUndoFamilyParityErrorKind::MissingReplayUndoCatalog
        }
        ReplayUndoFamilyContributorCatalogErrorKind::MissingCarriedIdentity
        | ReplayUndoFamilyContributorCatalogErrorKind::MismatchedRouteFamily => {
            ReplayUndoFamilyParityErrorKind::MismatchedReplayIdentity
        }
    }
}

fn row_error_kind(kind: ReplayUndoContributorRowKind) -> ReplayUndoFamilyParityErrorKind {
    match kind {
        ReplayUndoContributorRowKind::Replay => {
            ReplayUndoFamilyParityErrorKind::MismatchedReplayIdentity
        }
        ReplayUndoContributorRowKind::Undo => {
            ReplayUndoFamilyParityErrorKind::MismatchedUndoIdentity
        }
    }
}

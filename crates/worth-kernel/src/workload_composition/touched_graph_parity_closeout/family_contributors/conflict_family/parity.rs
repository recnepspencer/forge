use schema::facade::platform::authority::touched_graph_parity_closeout::{
    TouchedGraphParityClaimKind, TouchedGraphParityFamilyKind,
};

use crate::workload_composition::{
    current_worth_touched_graph_conflict_milestone_fifteen_seed,
    current_worth_touched_graph_conflict_selected_route_packet,
};

use super::contributor_catalog::{
    current_conflict_family_contributor_catalog, validate_conflict_family_contributor_catalog,
    ConflictFamilyContributorCatalog,
};
use super::error::ConflictFamilyContributorCatalogErrorKind;
use super::row::{
    ConflictFamilyContributorCatalogRow, ConflictFamilyContributorRowKind,
    ConflictFamilyDenialWitnessKind,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConflictFamilyParityErrorKind {
    CurrentSelectedRouteUnavailable,
    CurrentMilestoneFifteenSeedUnavailable,
    MissingConflictFamilyCatalog,
    MismatchedConflictIdentity,
    MismatchedIndependenceIdentity,
    MismatchedBatchAdmissionIdentity,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConflictFamilyParityError {
    kind: ConflictFamilyParityErrorKind,
    detail: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConflictFamilyParityRow {
    kind: ConflictFamilyContributorRowKind,
    family_kind: TouchedGraphParityFamilyKind,
    current_packet_or_identity_source: &'static str,
    carried_overlap_or_plan_source: &'static str,
    carried_witness_source: &'static str,
    current_packet_identity: String,
    selected_batch_plan_digest: String,
    overlap_identity_digests: Vec<String>,
    selected_conflict_plan_digests: Vec<String>,
    independence_proof_digests: Vec<String>,
    denial_witness_identity: Option<String>,
    denial_witness_kind: Option<ConflictFamilyDenialWitnessKind>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConflictFamilyParityClaim {
    kind: TouchedGraphParityClaimKind,
    selected_route_identity_digest: String,
    selected_family_identity: String,
    selected_product_identity_digest: String,
    witness_identity_digest: Option<String>,
    rows: Vec<ConflictFamilyParityRow>,
}

pub fn current_conflict_family_parity_claim(
) -> Result<ConflictFamilyParityClaim, ConflictFamilyParityError> {
    let catalog = current_conflict_family_contributor_catalog().map_err(|error| {
        ConflictFamilyParityError::new(map_catalog_error_kind(error.kind()), error.detail())
    })?;
    conflict_family_parity_claim_from_catalog(&catalog)
}

pub(crate) fn conflict_family_parity_claim_from_catalog(
    catalog: &ConflictFamilyContributorCatalog,
) -> Result<ConflictFamilyParityClaim, ConflictFamilyParityError> {
    validate_conflict_family_contributor_catalog(catalog).map_err(|error| {
        ConflictFamilyParityError::new(map_catalog_error_kind(error.kind()), error.detail())
    })?;
    let selected_route =
        current_worth_touched_graph_conflict_selected_route_packet().map_err(|error| {
            ConflictFamilyParityError::new(
                ConflictFamilyParityErrorKind::CurrentSelectedRouteUnavailable,
                error.detail(),
            )
        })?;
    let seed = current_worth_touched_graph_conflict_milestone_fifteen_seed().map_err(|error| {
        ConflictFamilyParityError::new(
            ConflictFamilyParityErrorKind::CurrentMilestoneFifteenSeedUnavailable,
            format!("{error:?}"),
        )
    })?;
    let conflict_pre_execution_identity =
        selected_route.conflict_family_conflict_pre_execution_identity();
    let independence_pre_execution_identity =
        selected_route.conflict_family_independence_pre_execution_identity();
    let batch_pre_execution_identity =
        selected_route.conflict_family_batch_pre_execution_identity();

    for row in catalog.rows() {
        let kind = row.kind();
        let overlap_matches = row.overlap_identity_digests() == seed.overlap_identity_digests();
        let witness_contract_matches = match kind {
            ConflictFamilyContributorRowKind::Conflict
            | ConflictFamilyContributorRowKind::Independence => {
                row.denial_witness_identity()
                    == selected_route.conflict_independence_denial_witness_identity()
                    && row.denial_witness_kind()
                        == selected_route
                            .conflict_independence_denial_witness_kind()
                            .map(ConflictFamilyDenialWitnessKind::ConflictIndependence)
            }
            ConflictFamilyContributorRowKind::BatchAdmission => {
                row.denial_witness_identity()
                    == selected_route.batch_admission_denial_witness_identity()
                    && row.denial_witness_kind()
                        == selected_route
                            .batch_admission_denial_witness_kind()
                            .map(ConflictFamilyDenialWitnessKind::BatchAdmission)
            }
        };
        let identity_matches = match kind {
            ConflictFamilyContributorRowKind::Conflict => {
                row.current_packet_identity() == conflict_pre_execution_identity
                    && row.selected_conflict_plan_digests()
                        == selected_route.selected_conflict_plan_digests()
                    && row.selected_conflict_plan_digests() == seed.selected_conflict_plan_digests()
                    && overlap_matches
                    && witness_contract_matches
            }
            ConflictFamilyContributorRowKind::Independence => {
                row.current_packet_identity() == independence_pre_execution_identity
                    && row.independence_proof_digests()
                        == selected_route.independence_proof_digests()
                    && row.independence_proof_digests() == seed.independence_proof_digests()
                    && overlap_matches
                    && witness_contract_matches
            }
            ConflictFamilyContributorRowKind::BatchAdmission => {
                row.current_packet_identity() == batch_pre_execution_identity
                    && row.selected_conflict_plan_digests()
                        == selected_route.selected_conflict_plan_digests()
                    && row.independence_proof_digests()
                        == selected_route.independence_proof_digests()
                    && row.selected_batch_plan_digest()
                        == selected_route.selected_batch_plan_digest()
                    && row.selected_batch_plan_digest() == seed.selected_batch_plan_digest()
                    && row
                        .supporting_packet_identities()
                        .iter()
                        .any(|identity| identity == &conflict_pre_execution_identity)
                    && row
                        .supporting_packet_identities()
                        .iter()
                        .any(|identity| identity == &independence_pre_execution_identity)
                    && overlap_matches
                    && witness_contract_matches
            }
        };
        if !identity_matches {
            return Err(ConflictFamilyParityError::new(
                row_error_kind(kind),
                format!(
                    "conflict-family parity requires {} row to carry the exact overlap, plan, and witness identities admitted by the selected-route semantic graph",
                    kind.as_str()
                ),
            ));
        }
    }

    Ok(ConflictFamilyParityClaim {
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
            .map(ConflictFamilyParityRow::from_catalog_row)
            .collect(),
    })
}

impl ConflictFamilyParityRow {
    fn from_catalog_row(row: &ConflictFamilyContributorCatalogRow) -> Self {
        Self {
            kind: row.kind(),
            family_kind: row.family_kind(),
            current_packet_or_identity_source: row.current_packet_or_identity_source(),
            carried_overlap_or_plan_source: row.carried_overlap_or_plan_source(),
            carried_witness_source: row.carried_witness_source(),
            current_packet_identity: row.current_packet_identity().to_string(),
            selected_batch_plan_digest: row.selected_batch_plan_digest().to_string(),
            overlap_identity_digests: row.overlap_identity_digests().to_vec(),
            selected_conflict_plan_digests: row.selected_conflict_plan_digests().to_vec(),
            independence_proof_digests: row.independence_proof_digests().to_vec(),
            denial_witness_identity: row.denial_witness_identity().map(str::to_string),
            denial_witness_kind: row.denial_witness_kind(),
        }
    }

    pub const fn kind(&self) -> ConflictFamilyContributorRowKind {
        self.kind
    }
    pub const fn family_kind(&self) -> TouchedGraphParityFamilyKind {
        self.family_kind
    }
    pub const fn current_packet_or_identity_source(&self) -> &'static str {
        self.current_packet_or_identity_source
    }
    pub const fn carried_overlap_or_plan_source(&self) -> &'static str {
        self.carried_overlap_or_plan_source
    }
    pub const fn carried_witness_source(&self) -> &'static str {
        self.carried_witness_source
    }
    pub fn current_packet_identity(&self) -> &str {
        &self.current_packet_identity
    }
    pub fn selected_batch_plan_digest(&self) -> &str {
        &self.selected_batch_plan_digest
    }
    pub const fn denial_witness_kind(&self) -> Option<ConflictFamilyDenialWitnessKind> {
        self.denial_witness_kind
    }
}

impl ConflictFamilyParityClaim {
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
    pub fn rows(&self) -> &[ConflictFamilyParityRow] {
        &self.rows
    }
}

impl ConflictFamilyParityError {
    fn new(kind: ConflictFamilyParityErrorKind, detail: impl Into<String>) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub const fn kind(&self) -> ConflictFamilyParityErrorKind {
        self.kind
    }
    pub fn detail(&self) -> &str {
        &self.detail
    }
}

fn map_catalog_error_kind(
    kind: ConflictFamilyContributorCatalogErrorKind,
) -> ConflictFamilyParityErrorKind {
    match kind {
        ConflictFamilyContributorCatalogErrorKind::CurrentSurfaceUnavailable => {
            ConflictFamilyParityErrorKind::CurrentSelectedRouteUnavailable
        }
        ConflictFamilyContributorCatalogErrorKind::MissingRequiredRow => {
            ConflictFamilyParityErrorKind::MissingConflictFamilyCatalog
        }
        ConflictFamilyContributorCatalogErrorKind::MissingCarriedIdentity
        | ConflictFamilyContributorCatalogErrorKind::MismatchedRouteFamily => {
            ConflictFamilyParityErrorKind::MismatchedConflictIdentity
        }
    }
}

fn row_error_kind(kind: ConflictFamilyContributorRowKind) -> ConflictFamilyParityErrorKind {
    match kind {
        ConflictFamilyContributorRowKind::Conflict => {
            ConflictFamilyParityErrorKind::MismatchedConflictIdentity
        }
        ConflictFamilyContributorRowKind::Independence => {
            ConflictFamilyParityErrorKind::MismatchedIndependenceIdentity
        }
        ConflictFamilyContributorRowKind::BatchAdmission => {
            ConflictFamilyParityErrorKind::MismatchedBatchAdmissionIdentity
        }
    }
}

use crate::workload_composition::planner_owned_routing::{
    current_worth_touched_graph_conflict_public_facade_with_artifact_policy,
    WorthTouchedGraphConflictDerivedDiagnosticArtifactPolicy, WorthTouchedGraphConflictPublicFacade,
};
use crate::workload_composition::touched_graph_parity_closeout::family_contributors::{
    KernelTouchedGraphParityCoverageContributor, KernelTouchedGraphParityCoverageError,
};

use super::derived_diagnostics_row::{
    current_derived_diagnostics_contributor_row, derived_diagnostics_contributor_row_from_public_facade,
};
use super::error::{
    PublicProjectionContributorCatalogError, PublicProjectionContributorCatalogErrorKind,
};
use super::public_proof_row::{
    current_public_proof_contributor_row, public_proof_contributor_row_from_public_facade,
};
use super::row::{
    public_projection_family_coverage_contributor_rows_from_catalog,
    PublicProjectionContributorCatalogRow, PublicProjectionContributorRowKind,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicProjectionContributorCatalog {
    rows: Vec<PublicProjectionContributorCatalogRow>,
}

pub fn current_public_projection_contributor_catalog(
) -> Result<PublicProjectionContributorCatalog, PublicProjectionContributorCatalogError> {
    let public_facade = current_worth_touched_graph_conflict_public_facade_with_artifact_policy(
        WorthTouchedGraphConflictDerivedDiagnosticArtifactPolicy::MinimalOperationalTruth,
    )
    .map_err(|error| {
        PublicProjectionContributorCatalogError::new(
            PublicProjectionContributorCatalogErrorKind::CurrentSurfaceUnavailable,
            error.detail(),
        )
    })?;
    public_projection_contributor_catalog_from_public_facade(&public_facade)
}

pub(crate) fn public_projection_family_coverage_contributor_rows(
) -> Result<Vec<KernelTouchedGraphParityCoverageContributor>, KernelTouchedGraphParityCoverageError>
{
    let catalog = current_public_projection_contributor_catalog()
        .map_err(|error| KernelTouchedGraphParityCoverageError::new(error.detail()))?;
    public_projection_family_coverage_contributor_rows_from_catalog(catalog.rows())
}

pub(crate) fn public_projection_contributor_catalog_from_public_facade(
    public_facade: &WorthTouchedGraphConflictPublicFacade,
) -> Result<PublicProjectionContributorCatalog, PublicProjectionContributorCatalogError> {
    PublicProjectionContributorCatalog::new_with_authority(
        vec![
            public_proof_contributor_row_from_public_facade(public_facade)?,
            derived_diagnostics_contributor_row_from_public_facade(public_facade)?,
        ],
        public_facade,
    )
}

pub(crate) fn public_projection_family_coverage_contributor_rows_from_public_facade(
    public_facade: &WorthTouchedGraphConflictPublicFacade,
) -> Result<Vec<KernelTouchedGraphParityCoverageContributor>, KernelTouchedGraphParityCoverageError>
{
    let catalog = public_projection_contributor_catalog_from_public_facade(public_facade)
        .map_err(|error| KernelTouchedGraphParityCoverageError::new(error.detail()))?;
    public_projection_family_coverage_contributor_rows_from_catalog(catalog.rows())
}

pub(crate) fn current_public_proof_coverage_contributor(
) -> Result<KernelTouchedGraphParityCoverageContributor, KernelTouchedGraphParityCoverageError> {
    current_public_projection_contributor_catalog()
        .map(|catalog| {
            catalog
                .rows()
                .iter()
                .find(|row| row.kind() == PublicProjectionContributorRowKind::PublicProof)
                .expect("public-proof row")
                .coverage_contributor()
                .clone()
        })
        .map_err(|error| KernelTouchedGraphParityCoverageError::new(error.detail()))
}

pub(crate) fn current_derived_diagnostics_coverage_contributor(
) -> Result<KernelTouchedGraphParityCoverageContributor, KernelTouchedGraphParityCoverageError> {
    current_public_projection_contributor_catalog()
        .map(|catalog| {
            catalog
                .rows()
                .iter()
                .find(|row| row.kind() == PublicProjectionContributorRowKind::DerivedDiagnostics)
                .expect("derived-diagnostics row")
                .coverage_contributor()
                .clone()
        })
        .map_err(|error| KernelTouchedGraphParityCoverageError::new(error.detail()))
}

impl PublicProjectionContributorCatalog {
    pub fn new(
        rows: Vec<PublicProjectionContributorCatalogRow>,
    ) -> Result<Self, PublicProjectionContributorCatalogError> {
        validate_catalog_rows(&rows)?;
        Ok(Self { rows })
    }

    pub(crate) fn new_with_authority(
        rows: Vec<PublicProjectionContributorCatalogRow>,
        public_facade: &WorthTouchedGraphConflictPublicFacade,
    ) -> Result<Self, PublicProjectionContributorCatalogError> {
        validate_catalog_rows_against_public_facade(&rows, public_facade)?;
        Ok(Self { rows })
    }

    pub fn rows(&self) -> &[PublicProjectionContributorCatalogRow] {
        &self.rows
    }

    #[cfg(test)]
    pub(crate) fn new_unvalidated_for_testing(
        rows: Vec<PublicProjectionContributorCatalogRow>,
    ) -> Self {
        Self { rows }
    }
}

pub(crate) fn validate_public_projection_contributor_catalog(
    catalog: &PublicProjectionContributorCatalog,
) -> Result<(), PublicProjectionContributorCatalogError> {
    validate_catalog_rows(catalog.rows())
}

fn validate_catalog_rows(
    rows: &[PublicProjectionContributorCatalogRow],
) -> Result<(), PublicProjectionContributorCatalogError> {
    let public_facade = current_worth_touched_graph_conflict_public_facade_with_artifact_policy(
        WorthTouchedGraphConflictDerivedDiagnosticArtifactPolicy::MinimalOperationalTruth,
    )
    .map_err(|error| {
        PublicProjectionContributorCatalogError::new(
            PublicProjectionContributorCatalogErrorKind::CurrentSurfaceUnavailable,
            error.detail(),
        )
    })?;
    validate_catalog_rows_against_public_facade(rows, &public_facade)
}

fn validate_catalog_rows_against_public_facade(
    rows: &[PublicProjectionContributorCatalogRow],
    public_facade: &WorthTouchedGraphConflictPublicFacade,
) -> Result<(), PublicProjectionContributorCatalogError> {
    if rows.len() != 2 {
        return Err(PublicProjectionContributorCatalogError::new(
            PublicProjectionContributorCatalogErrorKind::MissingRequiredRow,
            "public projection contributor catalog requires exactly public-proof and derived-diagnostics rows",
        ));
    }

    let mut has_public_proof = false;
    let mut has_derived_diagnostics = false;
    for row in rows {
        match row.kind() {
            PublicProjectionContributorRowKind::PublicProof => has_public_proof = true,
            PublicProjectionContributorRowKind::DerivedDiagnostics => {
                has_derived_diagnostics = true
            }
        }
        if row.family_kind() != row.kind().family_kind() {
            return Err(PublicProjectionContributorCatalogError::new(
                PublicProjectionContributorCatalogErrorKind::MismatchedProjectionAuthority,
                "public projection contributor row must preserve its exact public-proof or derived-diagnostics family kind",
            ));
        }
        if row.ordinary_path_live_caller_surface()
            != "current_worth_workload_ordinary_consumer_sweep_closeout"
            || row.ordinary_path_live_caller_path()
                != "crates/worth-kernel/src/workload_composition/worth_workload/ordinary_consumer_sweep/closeout.rs"
        {
            return Err(PublicProjectionContributorCatalogError::new(
                PublicProjectionContributorCatalogErrorKind::MissingCarriedIdentity,
                "public projection contributor row must name the exact ordinary consumer sweep caller seam",
            ));
        }
    }

    if !(has_public_proof && has_derived_diagnostics) {
        return Err(PublicProjectionContributorCatalogError::new(
            PublicProjectionContributorCatalogErrorKind::MissingRequiredRow,
            "public projection contributor catalog requires one public-proof row and one derived-diagnostics row",
        ));
    }
    let public_proof = public_facade.public_proof();
    let diagnostics = public_facade.derived_diagnostics();
    for row in rows {
        match row.kind() {
            PublicProjectionContributorRowKind::PublicProof => {
                if row.selected_route_identity_digest() != public_proof.selected_route_identity_digest()
                    || row.selected_family_identity() != public_proof.selected_family_identity()
                    || row.selected_product_identity_digest()
                        != public_proof.selected_product_identity_digest()
                    || row.selected_witness_identity_digest()
                        != public_proof.selected_witness_identity_digest()
                    || row.proof_chain_digest() != Some(public_proof.proof_chain_digest())
                    || row.milestone_fifteen_seed_digest()
                        != Some(public_proof.milestone_fifteen_seed().seed_digest())
                    || row.residue_digest() != Some(public_proof.residue_chain().residue_digest())
                    || row.source_firewall_digest() != Some(public_proof.source_firewall_digest())
                {
                    return Err(PublicProjectionContributorCatalogError::new(
                        PublicProjectionContributorCatalogErrorKind::MismatchedProjectionAuthority,
                        "public-proof contributor row diverged from the carried public-facade proof inspection",
                    ));
                }
            }
            PublicProjectionContributorRowKind::DerivedDiagnostics => {
                if row.selected_route_identity_digest() != diagnostics.selected_route_identity_digest()
                    || row.selected_family_identity() != diagnostics.selected_family_identity()
                    || row.selected_product_identity_digest()
                        != diagnostics.selected_product_identity_digest()
                    || row.selected_witness_identity_digest()
                        != diagnostics.selected_witness_identity_digest()
                    || row.decision_trace_identity_digest()
                        != Some(diagnostics.decision_trace_identity_digest())
                {
                    return Err(PublicProjectionContributorCatalogError::new(
                        PublicProjectionContributorCatalogErrorKind::MismatchedProjectionAuthority,
                        "derived-diagnostics contributor row diverged from the carried public-facade diagnostic projection",
                    ));
                }
            }
        }
    }

    Ok(())
}

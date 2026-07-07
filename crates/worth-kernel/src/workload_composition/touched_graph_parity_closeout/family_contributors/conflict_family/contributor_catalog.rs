use schema::facade::platform::authority::touched_graph_parity_closeout::TouchedGraphParityFamilyKind;

use crate::workload_composition::touched_graph_parity_closeout::family_contributors::{
    KernelTouchedGraphParityCoverageContributor, KernelTouchedGraphParityCoverageError,
};

use super::batch_admission_row::current_batch_admission_contributor_row;
use super::conflict_row::current_conflict_contributor_row;
use super::error::{
    ConflictFamilyContributorCatalogError, ConflictFamilyContributorCatalogErrorKind,
};
use super::independence_row::current_independence_contributor_row;
use super::row::{
    conflict_family_coverage_contributor_rows_from_catalog, ConflictFamilyContributorCatalogRow,
    ConflictFamilyContributorRowKind,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConflictFamilyContributorCatalog {
    rows: Vec<ConflictFamilyContributorCatalogRow>,
}

pub fn current_conflict_family_contributor_catalog(
) -> Result<ConflictFamilyContributorCatalog, ConflictFamilyContributorCatalogError> {
    ConflictFamilyContributorCatalog::new(vec![
        current_conflict_contributor_row()?,
        current_independence_contributor_row()?,
        current_batch_admission_contributor_row()?,
    ])
}

pub(crate) fn conflict_family_coverage_contributor_rows(
) -> Result<Vec<KernelTouchedGraphParityCoverageContributor>, KernelTouchedGraphParityCoverageError>
{
    let catalog = current_conflict_family_contributor_catalog()
        .map_err(|error| KernelTouchedGraphParityCoverageError::new(error.detail()))?;
    conflict_family_coverage_contributor_rows_from_catalog(catalog.rows())
}

#[cfg(test)]
pub(crate) fn current_conflict_coverage_contributor(
) -> Result<KernelTouchedGraphParityCoverageContributor, KernelTouchedGraphParityCoverageError> {
    coverage_row_for(ConflictFamilyContributorRowKind::Conflict)
}

#[cfg(test)]
pub(crate) fn current_independence_coverage_contributor(
) -> Result<KernelTouchedGraphParityCoverageContributor, KernelTouchedGraphParityCoverageError> {
    coverage_row_for(ConflictFamilyContributorRowKind::Independence)
}

#[cfg(test)]
pub(crate) fn current_batch_admission_coverage_contributor(
) -> Result<KernelTouchedGraphParityCoverageContributor, KernelTouchedGraphParityCoverageError> {
    coverage_row_for(ConflictFamilyContributorRowKind::BatchAdmission)
}

#[cfg(test)]
fn coverage_row_for(
    kind: ConflictFamilyContributorRowKind,
) -> Result<KernelTouchedGraphParityCoverageContributor, KernelTouchedGraphParityCoverageError> {
    current_conflict_family_contributor_catalog()
        .map(|catalog| {
            catalog
                .rows()
                .iter()
                .find(|row| row.kind() == kind)
                .expect("conflict-family contributor row")
                .coverage_contributor()
                .clone()
        })
        .map_err(|error| KernelTouchedGraphParityCoverageError::new(error.detail()))
}

impl ConflictFamilyContributorCatalog {
    pub fn new(
        rows: Vec<ConflictFamilyContributorCatalogRow>,
    ) -> Result<Self, ConflictFamilyContributorCatalogError> {
        validate_catalog_shape(&rows)?;
        Ok(Self { rows })
    }

    pub fn rows(&self) -> &[ConflictFamilyContributorCatalogRow] {
        &self.rows
    }

    #[cfg(test)]
    pub(crate) fn new_unvalidated_for_testing(
        rows: Vec<ConflictFamilyContributorCatalogRow>,
    ) -> Self {
        Self { rows }
    }
}

pub(crate) fn validate_conflict_family_contributor_catalog(
    catalog: &ConflictFamilyContributorCatalog,
) -> Result<(), ConflictFamilyContributorCatalogError> {
    validate_catalog_shape(catalog.rows())
}

fn validate_catalog_shape(
    rows: &[ConflictFamilyContributorCatalogRow],
) -> Result<(), ConflictFamilyContributorCatalogError> {
    if rows.len() != 3 {
        return Err(ConflictFamilyContributorCatalogError::new(
            ConflictFamilyContributorCatalogErrorKind::MissingRequiredRow,
            "conflict-family contributor catalog requires conflict, independence, and batch-admission rows",
        ));
    }

    let mut has_conflict = false;
    let mut has_independence = false;
    let mut has_batch = false;
    for row in rows {
        match row.kind() {
            ConflictFamilyContributorRowKind::Conflict => has_conflict = true,
            ConflictFamilyContributorRowKind::Independence => has_independence = true,
            ConflictFamilyContributorRowKind::BatchAdmission => has_batch = true,
        }
        if row.family_kind() != TouchedGraphParityFamilyKind::ConflictIndependenceBatchAdmission {
            return Err(ConflictFamilyContributorCatalogError::new(
                ConflictFamilyContributorCatalogErrorKind::MismatchedRouteFamily,
                "conflict-family contributor row must remain in the shared ConflictIndependenceBatchAdmission family kind",
            ));
        }
        if row.current_packet_identity().is_empty()
            || row.overlap_identity_digests().is_empty()
            || row.selected_identity_fields_produced().is_empty()
            || row.denial_witness_identity().is_some() != row.denial_witness_kind().is_some()
        {
            return Err(ConflictFamilyContributorCatalogError::new(
                ConflictFamilyContributorCatalogErrorKind::MissingCarriedIdentity,
                "conflict-family contributor row must carry explicit current identity, overlap/plan inputs, and complete witness contracts",
            ));
        }
        match row.kind() {
            ConflictFamilyContributorRowKind::Conflict => {
                if row.selected_conflict_plan_digests().is_empty() {
                    return Err(ConflictFamilyContributorCatalogError::new(
                        ConflictFamilyContributorCatalogErrorKind::MissingCarriedIdentity,
                        "conflict row must carry selected conflict plan identities alongside overlap and witness contracts",
                    ));
                }
            }
            ConflictFamilyContributorRowKind::Independence => {
                if row.independence_proof_digests().is_empty() {
                    return Err(ConflictFamilyContributorCatalogError::new(
                        ConflictFamilyContributorCatalogErrorKind::MissingCarriedIdentity,
                        "independence row must carry proof identities alongside overlap and witness contracts",
                    ));
                }
            }
            ConflictFamilyContributorRowKind::BatchAdmission => {
                if row.selected_batch_plan_digest().is_empty()
                    || row.selected_conflict_plan_digests().is_empty()
                    || row.independence_proof_digests().is_empty()
                    || row.supporting_packet_identities().len() < 2
                {
                    return Err(ConflictFamilyContributorCatalogError::new(
                        ConflictFamilyContributorCatalogErrorKind::MissingCarriedIdentity,
                        "batch-admission row must carry pre-execution selected-plan identities plus supporting conflict and independence identities",
                    ));
                }
            }
        }
    }

    if !(has_conflict && has_independence && has_batch) {
        return Err(ConflictFamilyContributorCatalogError::new(
            ConflictFamilyContributorCatalogErrorKind::MissingRequiredRow,
            "conflict-family contributor catalog requires one conflict, one independence, and one batch-admission row",
        ));
    }
    Ok(())
}

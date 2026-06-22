use super::classification::{
    WorthQueryAdoptionClassification, WorthQueryAdoptionForbiddenPattern,
    WorthQueryAdoptionInventoryOwner, WorthQueryAdoptionInventoryRow, WorthQueryAuthorityCategory,
};
use super::report::WorthQueryAdoptionInventoryReport;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQuerySyntheticProofDisposition {
    ReplacedByProductionSurface,
    DeniedByBoundary,
    ExplicitResidue,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQuerySyntheticProofDispositionRow {
    owner: WorthQueryAdoptionInventoryOwner,
    source_set: &'static str,
    forbidden_pattern: WorthQueryAdoptionForbiddenPattern,
    disposition: WorthQuerySyntheticProofDisposition,
    proof_surface: &'static str,
}

impl WorthQuerySyntheticProofDispositionRow {
    fn from_inventory_row(row: &WorthQueryAdoptionInventoryRow) -> Option<Self> {
        Some(Self {
            owner: row.owner(),
            source_set: row.source_set(),
            forbidden_pattern: row.forbidden_pattern()?,
            disposition: disposition_for(row),
            proof_surface: row.replacement_surface(),
        })
    }

    pub const fn owner(&self) -> WorthQueryAdoptionInventoryOwner {
        self.owner
    }

    pub const fn source_set(&self) -> &'static str {
        self.source_set
    }

    pub const fn forbidden_pattern(&self) -> WorthQueryAdoptionForbiddenPattern {
        self.forbidden_pattern
    }

    pub const fn disposition(&self) -> WorthQuerySyntheticProofDisposition {
        self.disposition
    }

    pub const fn proof_surface(&self) -> &'static str {
        self.proof_surface
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQuerySyntheticProofDispositionReport {
    rows: Vec<WorthQuerySyntheticProofDispositionRow>,
}

impl WorthQuerySyntheticProofDispositionReport {
    pub fn from_inventory(
        inventory: &WorthQueryAdoptionInventoryReport,
    ) -> Result<Self, WorthQuerySyntheticProofDispositionError> {
        let rows = inventory
            .rows()
            .iter()
            .filter_map(WorthQuerySyntheticProofDispositionRow::from_inventory_row)
            .collect::<Vec<_>>();
        validate_dispositions(inventory, &rows)?;
        Ok(Self { rows })
    }

    pub fn current() -> Result<Self, WorthQuerySyntheticProofDispositionError> {
        let inventory = WorthQueryAdoptionInventoryReport::cross_crate_reality_inventory()
            .map_err(WorthQuerySyntheticProofDispositionError::Inventory)?;
        Self::from_inventory(&inventory)
    }

    pub fn rows(&self) -> &[WorthQuerySyntheticProofDispositionRow] {
        &self.rows
    }

    pub fn rows_for(
        &self,
        disposition: WorthQuerySyntheticProofDisposition,
    ) -> impl Iterator<Item = &WorthQuerySyntheticProofDispositionRow> {
        self.rows
            .iter()
            .filter(move |row| row.disposition() == disposition)
    }

    pub fn require_source_set(
        &self,
        source_set: &str,
    ) -> Option<&WorthQuerySyntheticProofDispositionRow> {
        self.rows.iter().find(|row| row.source_set() == source_set)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQuerySyntheticProofDispositionError {
    Inventory(super::report::WorthQueryAdoptionInventoryError),
    MissingSyntheticDisposition,
    ProductionClosedAsResidue(&'static str),
    ResidueNotDiagnostic(&'static str),
    ResidueSurfaceNotExplicit(&'static str),
    MissingProofSurface(&'static str),
}

fn disposition_for(row: &WorthQueryAdoptionInventoryRow) -> WorthQuerySyntheticProofDisposition {
    match row.classification() {
        WorthQueryAdoptionClassification::Production => {
            WorthQuerySyntheticProofDisposition::ReplacedByProductionSurface
        }
        WorthQueryAdoptionClassification::ExplicitResidue => {
            WorthQuerySyntheticProofDisposition::ExplicitResidue
        }
        WorthQueryAdoptionClassification::CertificationOnly
        | WorthQueryAdoptionClassification::TestSupport => {
            WorthQuerySyntheticProofDisposition::DeniedByBoundary
        }
    }
}

fn validate_dispositions(
    inventory: &WorthQueryAdoptionInventoryReport,
    rows: &[WorthQuerySyntheticProofDispositionRow],
) -> Result<(), WorthQuerySyntheticProofDispositionError> {
    let forbidden_row_count = inventory
        .rows()
        .iter()
        .filter(|row| row.forbidden_pattern().is_some())
        .count();
    if rows.len() != forbidden_row_count {
        return Err(WorthQuerySyntheticProofDispositionError::MissingSyntheticDisposition);
    }
    for row in inventory
        .rows()
        .iter()
        .filter(|row| row.forbidden_pattern().is_some())
    {
        validate_inventory_disposition(row)?;
    }
    Ok(())
}

fn validate_inventory_disposition(
    row: &WorthQueryAdoptionInventoryRow,
) -> Result<(), WorthQuerySyntheticProofDispositionError> {
    if row.replacement_surface().is_empty() {
        return Err(
            WorthQuerySyntheticProofDispositionError::MissingProofSurface(row.source_set()),
        );
    }
    if row.classification() == WorthQueryAdoptionClassification::Production
        && disposition_for(row) == WorthQuerySyntheticProofDisposition::ExplicitResidue
    {
        return Err(
            WorthQuerySyntheticProofDispositionError::ProductionClosedAsResidue(row.source_set()),
        );
    }
    if row.classification() == WorthQueryAdoptionClassification::ExplicitResidue
        && row.authority_category() != WorthQueryAuthorityCategory::Diagnostic
    {
        return Err(
            WorthQuerySyntheticProofDispositionError::ResidueNotDiagnostic(row.source_set()),
        );
    }
    if row.classification() == WorthQueryAdoptionClassification::ExplicitResidue
        && !row
            .replacement_surface()
            .ends_with("/query_adoption/residue.rs")
    {
        return Err(
            WorthQuerySyntheticProofDispositionError::ResidueSurfaceNotExplicit(row.source_set()),
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn synthetic_proof_dispositions_cover_every_forbidden_inventory_row() {
        let inventory =
            WorthQueryAdoptionInventoryReport::cross_crate_reality_inventory().expect("inventory");
        let report = WorthQuerySyntheticProofDispositionReport::from_inventory(&inventory)
            .expect("synthetic proof disposition report");

        assert_eq!(
            report.rows().len(),
            inventory
                .rows()
                .iter()
                .filter(|row| row.forbidden_pattern().is_some())
                .count()
        );
        assert!(report
            .rows()
            .iter()
            .all(|row| !row.proof_surface().is_empty()));
    }

    #[test]
    fn synthetic_proof_dispositions_separate_replacement_denial_and_residue() {
        let report = WorthQuerySyntheticProofDispositionReport::current()
            .expect("synthetic proof disposition report");

        assert_eq!(
            report
                .rows_for(WorthQuerySyntheticProofDisposition::ReplacedByProductionSurface)
                .count(),
            5
        );
        assert_eq!(
            report
                .rows_for(WorthQuerySyntheticProofDisposition::DeniedByBoundary)
                .count(),
            5
        );
        assert_eq!(
            report
                .rows_for(WorthQuerySyntheticProofDisposition::ExplicitResidue)
                .count(),
            3
        );
        assert!(report
            .rows_for(WorthQuerySyntheticProofDisposition::ExplicitResidue)
            .all(|row| row.proof_surface().ends_with("/query_adoption/residue.rs")));
    }
}

use std::collections::BTreeSet;

use serde::Serialize;

use super::classification::{
    DerivedInvalidationAuthorityDisposition, DerivedInvalidationOldAuthorityKind,
    DerivedInvalidationProductCategory, DerivedInvalidationReplacementPhase,
};
use super::error::{
    DerivedInvalidationAuthorityInventoryError, DerivedInvalidationAuthorityInventoryErrorKind,
};
use super::report::DerivedInvalidationAuthorityInventoryReport;
use super::seed::DerivedInvalidationPhaseTwoSeed;
use super::source_scan::{
    scan_current_derived_invalidation_sources, DerivedInvalidationSourceScanReport,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedInvalidationAuthorityInventoryCloseout {
    inventory: DerivedInvalidationAuthorityInventoryReport,
    source_scan: DerivedInvalidationSourceScanReport,
    phase_two_seed: DerivedInvalidationPhaseTwoSeed,
}

impl DerivedInvalidationAuthorityInventoryCloseout {
    pub fn close(
        inventory: DerivedInvalidationAuthorityInventoryReport,
    ) -> Result<Self, DerivedInvalidationAuthorityInventoryError> {
        validate_required_categories(&inventory)?;
        validate_row_dispositions(&inventory)?;
        let source_scan = scan_current_derived_invalidation_sources(&inventory)?;
        if source_scan.uncovered_pattern_count() != 0 {
            let patterns = source_scan.uncovered_patterns().to_vec();
            return Err(DerivedInvalidationAuthorityInventoryError::new(
                DerivedInvalidationAuthorityInventoryErrorKind::UncoveredAuthorityPatterns {
                    patterns: patterns.clone(),
                },
                format!(
                    "source scan found uncovered old derived authority patterns: {:?}",
                    patterns
                ),
            ));
        }
        let phase_two_seed = DerivedInvalidationPhaseTwoSeed::from_inventory_report(&inventory);
        Ok(Self {
            inventory,
            source_scan,
            phase_two_seed,
        })
    }

    pub fn inventory(&self) -> &DerivedInvalidationAuthorityInventoryReport {
        &self.inventory
    }

    pub fn source_scan(&self) -> &DerivedInvalidationSourceScanReport {
        &self.source_scan
    }

    pub fn phase_two_seed(&self) -> &DerivedInvalidationPhaseTwoSeed {
        &self.phase_two_seed
    }
}

fn validate_required_categories(
    inventory: &DerivedInvalidationAuthorityInventoryReport,
) -> Result<(), DerivedInvalidationAuthorityInventoryError> {
    let observed = inventory
        .rows()
        .iter()
        .filter(|row| row.ordinary_path())
        .map(|row| row.product_category())
        .collect::<BTreeSet<_>>();
    for required in inventory.required_ordinary_categories() {
        if !observed.contains(required) {
            return Err(DerivedInvalidationAuthorityInventoryError::new(
                DerivedInvalidationAuthorityInventoryErrorKind::MissingCoveredProductCategory {
                    category: required.as_str(),
                },
                format!(
                    "missing covered derived product category `{}`",
                    required.as_str()
                ),
            ));
        }
    }
    Ok(())
}

fn validate_row_dispositions(
    inventory: &DerivedInvalidationAuthorityInventoryReport,
) -> Result<(), DerivedInvalidationAuthorityInventoryError> {
    for row in inventory.rows() {
        if !row.has_blocker_and_trigger() {
            return Err(DerivedInvalidationAuthorityInventoryError::new(
                DerivedInvalidationAuthorityInventoryErrorKind::MissingRowExitCondition {
                    surface: row.surface().to_string(),
                },
                format!("row `{}` has no blocker or removal trigger", row.surface()),
            ));
        }
        if row.ordinary_path()
            && !matches!(
                row.disposition(),
                DerivedInvalidationAuthorityDisposition::Migrate
                    | DerivedInvalidationAuthorityDisposition::Delete
            )
        {
            return Err(DerivedInvalidationAuthorityInventoryError::new(
                DerivedInvalidationAuthorityInventoryErrorKind::InvalidOrdinaryDisposition {
                    surface: row.surface().to_string(),
                    disposition: row.disposition().as_str(),
                },
                format!(
                    "ordinary derived authority `{}` must migrate or be deleted, not {}",
                    row.surface(),
                    row.disposition().as_str()
                ),
            ));
        }
        if row.disposition()
            == DerivedInvalidationAuthorityDisposition::CertificationBootstrapResidue
        {
            validate_certification_residue(row)?;
        }
        if row.authority_kind() == DerivedInvalidationOldAuthorityKind::WholeViewMaterialization
            && !row.ordinary_path()
            && row.cap().is_none()
        {
            return Err(DerivedInvalidationAuthorityInventoryError::new(
                DerivedInvalidationAuthorityInventoryErrorKind::UncappedWholeViewResidue {
                    surface: row.surface().to_string(),
                },
                format!(
                    "non-ordinary whole-view materialization `{}` requires a cap",
                    row.surface()
                ),
            ));
        }
        if row.replacement_phase() == DerivedInvalidationReplacementPhase::TrueQueryCapabilityGap
            && row.disposition() != DerivedInvalidationAuthorityDisposition::TrueQueryCapabilityGap
        {
            return Err(DerivedInvalidationAuthorityInventoryError::new(
                DerivedInvalidationAuthorityInventoryErrorKind::QueryGapDispositionMismatch {
                    surface: row.surface().to_string(),
                },
                format!(
                    "row `{}` claims Query-gap replacement without Query-gap disposition",
                    row.surface()
                ),
            ));
        }
    }
    Ok(())
}

fn validate_certification_residue(
    row: &super::row::DerivedInvalidationAuthorityInventoryRow,
) -> Result<(), DerivedInvalidationAuthorityInventoryError> {
    if row.ordinary_path() || !row.certification_or_bootstrap_only() || row.cap().is_none() {
        return Err(DerivedInvalidationAuthorityInventoryError::new(
            DerivedInvalidationAuthorityInventoryErrorKind::InvalidCertificationResidue {
                surface: row.surface().to_string(),
            },
            format!(
                "certification/bootstrap residue `{}` must be non-ordinary, marked certification-only, and capped",
                row.surface()
            ),
        ));
    }
    if matches!(
        row.product_category(),
        DerivedInvalidationProductCategory::MaterializedGraph
            | DerivedInvalidationProductCategory::TraversalViews
            | DerivedInvalidationProductCategory::LoopCycles
            | DerivedInvalidationProductCategory::RadialRings
            | DerivedInvalidationProductCategory::ShellViews
            | DerivedInvalidationProductCategory::VertexDisks
            | DerivedInvalidationProductCategory::WireViews
            | DerivedInvalidationProductCategory::ProjectionReadStage
    ) {
        return Err(DerivedInvalidationAuthorityInventoryError::new(
            DerivedInvalidationAuthorityInventoryErrorKind::InvalidCertificationResidue {
                surface: row.surface().to_string(),
            },
            format!(
                "covered ordinary category `{}` cannot close as certification residue",
                row.product_category().as_str()
            ),
        ));
    }
    Ok(())
}

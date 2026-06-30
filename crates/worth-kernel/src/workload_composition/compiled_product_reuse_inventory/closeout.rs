use std::collections::BTreeSet;
use std::path::Path;

use super::classification::CompiledProductReuseDisposition;
use super::error::CompiledProductReuseInventoryError;
use super::phase_two_seed::CompiledProductReusePhaseTwoSeed;
use super::report::CompiledProductReuseInventoryReport;
use super::source_scan::CompiledProductReuseSourceScanReport;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompiledProductReuseInventoryCloseout {
    inventory: CompiledProductReuseInventoryReport,
    source_scan: CompiledProductReuseSourceScanReport,
    phase_two_seed: CompiledProductReusePhaseTwoSeed,
}

impl CompiledProductReuseInventoryCloseout {
    pub fn close(
        inventory: CompiledProductReuseInventoryReport,
    ) -> Result<Self, CompiledProductReuseInventoryError> {
        Self::close_with_workspace_root(inventory, &super::source_scan::workspace_root()?)
    }

    pub(crate) fn close_with_workspace_root(
        inventory: CompiledProductReuseInventoryReport,
        workspace_root: &Path,
    ) -> Result<Self, CompiledProductReuseInventoryError> {
        validate_required_surfaces(&inventory)?;
        validate_required_categories(&inventory)?;
        validate_rows(&inventory)?;
        let source_scan = CompiledProductReuseSourceScanReport::from_inventory_with_workspace_root(
            &inventory,
            workspace_root,
        )?;
        if source_scan.uncovered_pattern_count() != 0 {
            return Err(CompiledProductReuseInventoryError::UncoveredSourcePattern(
                source_scan.uncovered_patterns().join("; "),
            ));
        }
        let phase_two_seed = CompiledProductReusePhaseTwoSeed::from_inventory(&inventory);
        Ok(Self {
            inventory,
            source_scan,
            phase_two_seed,
        })
    }

    pub fn inventory(&self) -> &CompiledProductReuseInventoryReport {
        &self.inventory
    }

    pub fn source_scan(&self) -> &CompiledProductReuseSourceScanReport {
        &self.source_scan
    }

    pub fn phase_two_seed(&self) -> &CompiledProductReusePhaseTwoSeed {
        &self.phase_two_seed
    }
}

fn validate_required_surfaces(
    inventory: &CompiledProductReuseInventoryReport,
) -> Result<(), CompiledProductReuseInventoryError> {
    let observed = inventory
        .rows()
        .iter()
        .map(|row| row.surface_identity())
        .collect::<BTreeSet<_>>();
    for required in inventory.required_surfaces() {
        if !observed.contains(required) {
            return Err(CompiledProductReuseInventoryError::MissingRequiredSurface(
                *required,
            ));
        }
    }
    Ok(())
}

fn validate_required_categories(
    inventory: &CompiledProductReuseInventoryReport,
) -> Result<(), CompiledProductReuseInventoryError> {
    let observed = inventory
        .rows()
        .iter()
        .map(|row| row.semantic_category())
        .collect::<BTreeSet<_>>();
    for required in inventory.required_covered_categories() {
        if !observed.contains(required) {
            return Err(CompiledProductReuseInventoryError::MissingCoveredCategory(
                *required,
            ));
        }
    }
    Ok(())
}

fn validate_rows(
    inventory: &CompiledProductReuseInventoryReport,
) -> Result<(), CompiledProductReuseInventoryError> {
    let mut surfaces = BTreeSet::new();
    for row in inventory.rows() {
        if !surfaces.insert(row.surface_identity()) {
            return Err(CompiledProductReuseInventoryError::DuplicateSurface(
                row.surface_identity(),
            ));
        }
        if row.blocker().trim().is_empty() || row.removal_trigger().trim().is_empty() {
            return Err(CompiledProductReuseInventoryError::MissingExitCondition(
                row.surface_identity(),
            ));
        }
        if row.ordinary_path()
            && !matches!(
                row.disposition(),
                CompiledProductReuseDisposition::Migrate | CompiledProductReuseDisposition::Delete
            )
        {
            return Err(
                CompiledProductReuseInventoryError::InvalidOrdinaryDisposition {
                    surface: row.surface_identity(),
                    disposition: row.disposition(),
                },
            );
        }
        if !row.ordinary_path()
            && matches!(row.disposition(), CompiledProductReuseDisposition::Cap)
            && row.cap().is_none()
        {
            return Err(
                CompiledProductReuseInventoryError::InvalidNonOrdinaryDisposition {
                    surface: row.surface_identity(),
                },
            );
        }
        if row.certification_only()
            && (!matches!(
                row.disposition(),
                CompiledProductReuseDisposition::Delete
                    | CompiledProductReuseDisposition::CertificationOnly
            ) || row.cap().is_none())
        {
            return Err(
                CompiledProductReuseInventoryError::InvalidNonOrdinaryDisposition {
                    surface: row.surface_identity(),
                },
            );
        }
    }
    Ok(())
}

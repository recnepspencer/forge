use std::collections::BTreeSet;

use serde::Serialize;

use super::{
    DerivedInvalidationFamilyCatalog, DerivedInvalidationFamilyCatalogError,
    DerivedInvalidationFamilyCatalogErrorKind, DerivedTopologyProductFamilyIdentity,
};
use crate::derived_topology::invalidation_plan::inventory::{
    DerivedInvalidationAuthorityDisposition, DerivedInvalidationAuthorityInventoryReport,
    DerivedInvalidationProductCategory,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedInvalidationFamilySourceCoverage {
    covered_families: Vec<DerivedTopologyProductFamilyIdentity>,
}

impl DerivedInvalidationFamilySourceCoverage {
    pub(crate) fn validate(
        catalog: &DerivedInvalidationFamilyCatalog,
        inventory: &DerivedInvalidationAuthorityInventoryReport,
    ) -> Result<Self, DerivedInvalidationFamilyCatalogError> {
        validate_catalog_seed_matches_inventory(catalog, inventory)?;
        let accountable_families = ordinary_family_accountability_from_inventory(inventory);
        validate_catalog_declares_source_families(catalog, &accountable_families)?;
        validate_required_families_have_source_rows(&accountable_families)?;

        Ok(Self {
            covered_families: accountable_families.into_iter().collect(),
        })
    }

    pub fn covered_families(&self) -> &[DerivedTopologyProductFamilyIdentity] {
        &self.covered_families
    }
}

fn validate_catalog_seed_matches_inventory(
    catalog: &DerivedInvalidationFamilyCatalog,
    inventory: &DerivedInvalidationAuthorityInventoryReport,
) -> Result<(), DerivedInvalidationFamilyCatalogError> {
    if catalog.phase_two_seed().inventory_digest() == inventory.report_digest() {
        return Ok(());
    }
    Err(DerivedInvalidationFamilyCatalogError::new(
        DerivedInvalidationFamilyCatalogErrorKind::InventorySeedMismatch,
        "derived invalidation family catalog must validate against its source inventory",
    ))
}

fn ordinary_family_accountability_from_inventory(
    inventory: &DerivedInvalidationAuthorityInventoryReport,
) -> BTreeSet<DerivedTopologyProductFamilyIdentity> {
    inventory
        .rows()
        .iter()
        .filter(|row| row.ordinary_path())
        .filter(|row| ordinary_disposition_accounts_for_family(row.disposition()))
        .filter_map(|row| family_for_category(row.product_category()))
        .collect()
}

fn ordinary_disposition_accounts_for_family(
    disposition: DerivedInvalidationAuthorityDisposition,
) -> bool {
    matches!(
        disposition,
        DerivedInvalidationAuthorityDisposition::Migrate
            | DerivedInvalidationAuthorityDisposition::Delete
    )
}

fn validate_catalog_declares_source_families(
    catalog: &DerivedInvalidationFamilyCatalog,
    source_families: &BTreeSet<DerivedTopologyProductFamilyIdentity>,
) -> Result<(), DerivedInvalidationFamilyCatalogError> {
    for family in source_families {
        if catalog.family(*family).is_none() {
            return Err(missing_catalog_family_for_inventory_source(*family));
        }
    }
    Ok(())
}

fn validate_required_families_have_source_rows(
    source_families: &BTreeSet<DerivedTopologyProductFamilyIdentity>,
) -> Result<(), DerivedInvalidationFamilyCatalogError> {
    for family in DerivedTopologyProductFamilyIdentity::REQUIRED {
        if !source_families.contains(&family) {
            return Err(missing_inventory_source_for_family(family));
        }
    }
    Ok(())
}

fn missing_catalog_family_for_inventory_source(
    family: DerivedTopologyProductFamilyIdentity,
) -> DerivedInvalidationFamilyCatalogError {
    DerivedInvalidationFamilyCatalogError::new(
        DerivedInvalidationFamilyCatalogErrorKind::MissingCatalogFamilyForInventorySource {
            family: family.as_str(),
        },
        format!(
            "derived product family `{}` has ordinary inventory accountability but no catalog declaration",
            family.as_str()
        ),
    )
}

fn missing_inventory_source_for_family(
    family: DerivedTopologyProductFamilyIdentity,
) -> DerivedInvalidationFamilyCatalogError {
    DerivedInvalidationFamilyCatalogError::new(
        DerivedInvalidationFamilyCatalogErrorKind::MissingInventorySourceForFamily {
            family: family.as_str(),
        },
        format!(
            "derived product family `{}` has no ordinary migration accountability inventory row",
            family.as_str()
        ),
    )
}

fn family_for_category(
    category: DerivedInvalidationProductCategory,
) -> Option<DerivedTopologyProductFamilyIdentity> {
    match category {
        DerivedInvalidationProductCategory::MaterializedGraph => {
            Some(DerivedTopologyProductFamilyIdentity::MaterializedGraph)
        }
        DerivedInvalidationProductCategory::TraversalViews => {
            Some(DerivedTopologyProductFamilyIdentity::TraversalViews)
        }
        DerivedInvalidationProductCategory::LoopCycles => {
            Some(DerivedTopologyProductFamilyIdentity::LoopCycles)
        }
        DerivedInvalidationProductCategory::RadialRings => {
            Some(DerivedTopologyProductFamilyIdentity::RadialRings)
        }
        DerivedInvalidationProductCategory::ShellViews => {
            Some(DerivedTopologyProductFamilyIdentity::ShellViews)
        }
        DerivedInvalidationProductCategory::VertexDisks => {
            Some(DerivedTopologyProductFamilyIdentity::VertexDisks)
        }
        DerivedInvalidationProductCategory::WireViews => {
            Some(DerivedTopologyProductFamilyIdentity::WireViews)
        }
        DerivedInvalidationProductCategory::ProjectionReadStage
        | DerivedInvalidationProductCategory::OperatorCloseout
        | DerivedInvalidationProductCategory::CertificationBootstrap => None,
    }
}

use crate::validation_authority_inventory::error::WorthValidationAuthorityInventoryError;
use crate::validation_authority_inventory::inventory::WorthValidationAuthorityInventory;

use super::discovered_source::WorthValidationAuthorityDiscoveredSource;
use super::scan_region::WorthValidationAuthorityDiscoveryReport;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorthValidationAuthorityReconciliation {
    discovered_source_count: usize,
    reconciled_source_count: usize,
    unclassified_sources: Vec<WorthValidationAuthorityDiscoveredSource>,
}

impl WorthValidationAuthorityReconciliation {
    pub fn from_inventory_and_discovery(
        inventory: &WorthValidationAuthorityInventory,
        discovery: &WorthValidationAuthorityDiscoveryReport,
    ) -> Result<Self, WorthValidationAuthorityInventoryError> {
        let unclassified_sources = discovery
            .discovered_sources()
            .iter()
            .filter(|source| !inventory.contains_discovered_source(source))
            .cloned()
            .collect::<Vec<_>>();
        if let Some(unclassified) = unclassified_sources.first() {
            return Err(
                WorthValidationAuthorityInventoryError::UnclassifiedDiscoveredSource(format!(
                    "{} contains `{}`",
                    unclassified.path().display(),
                    unclassified.pattern().pattern()
                )),
            );
        }
        Ok(Self {
            discovered_source_count: discovery.discovered_sources().len(),
            reconciled_source_count: discovery.discovered_sources().len(),
            unclassified_sources,
        })
    }

    pub const fn discovered_source_count(&self) -> usize {
        self.discovered_source_count
    }

    pub const fn reconciled_source_count(&self) -> usize {
        self.reconciled_source_count
    }

    pub fn unclassified_sources(&self) -> &[WorthValidationAuthorityDiscoveredSource] {
        &self.unclassified_sources
    }

    pub fn unclassified_discovered_source_count(&self) -> usize {
        self.unclassified_sources.len()
    }
}

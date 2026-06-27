mod catalog;
mod closeout;
mod coverage;
mod discovery;
mod error;
mod query_surface;
mod row;

#[cfg(test)]
mod catalog_test_support;
#[cfg(test)]
mod tests;

pub use closeout::{
    current_evidence_lookup_inventory, EvidenceLookupInventoryCloseout,
    EvidenceLookupInventoryCloseoutCounters,
};
pub use coverage::{
    EvidenceLookupCatalogRowDiscoveryStatus, EvidenceLookupCatalogValidationReport,
    EvidenceLookupCatalogValidationRow,
};
pub use error::{EvidenceLookupInventoryError, EvidenceLookupInventoryErrorKind};
pub use query_surface::{
    classify_evidence_lookup_query_surface, EvidenceLookupQuerySurfaceContext,
};
pub use row::{
    EvidenceLookupAuthorityKind, EvidenceLookupCertificationPosture, EvidenceLookupCostPosture,
    EvidenceLookupDisposition, EvidenceLookupInventoryRow, EvidenceLookupInventoryRowScope,
    EvidenceLookupOwner, EvidenceLookupQuerySurface, EvidenceLookupReplacementPhase,
};

#[cfg(test)]
pub(crate) use catalog::{
    covered_evidence_lookup_surfaces, CoveredEvidenceLookupSurface,
    EvidenceLookupCatalogDiscoveryExpectation,
};
#[cfg(test)]
pub(crate) use catalog_test_support::fixture_surface;
#[cfg(test)]
pub(crate) use closeout::EvidenceLookupInventoryCollector;
#[cfg(test)]
pub(crate) use coverage::validate_discovered_evidence_lookup_surfaces;
#[cfg(test)]
pub(crate) use discovery::{
    evidence_lookup_discovered_surface_report_for_roots, EvidenceLookupDiscoveredSurface,
    EvidenceLookupDiscoveryScanRoot,
};
#[cfg(test)]
pub(crate) use row::EvidenceLookupInventoryRowBuilder;

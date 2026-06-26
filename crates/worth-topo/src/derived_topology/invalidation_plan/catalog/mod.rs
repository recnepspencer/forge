mod closeout;
mod counters;
mod current;
mod error;
mod family;
mod family_catalog;
mod phase_three_seed;
mod source_coverage;

#[cfg(test)]
mod catalog_test_fixtures;
#[cfg(test)]
mod tests;

pub use closeout::DerivedInvalidationFamilyCatalogCloseout;
pub use counters::DerivedInvalidationFamilyCatalogCounters;
pub use current::current_derived_invalidation_family_catalog;
pub use error::{DerivedInvalidationFamilyCatalogError, DerivedInvalidationFamilyCatalogErrorKind};
#[cfg(test)]
pub(crate) use family::DerivedTopologyProductFamilyRecordInput;
pub use family::{
    DerivedTopologyConsumedGraphFacts, DerivedTopologyDiagnosticPosture,
    DerivedTopologyInvalidationPredicate, DerivedTopologyLegalityReceiptPosture,
    DerivedTopologyProductFamilyIdentity, DerivedTopologyProductFamilyRecord,
    DerivedTopologyQueryReceiptPosture, DerivedTopologySpatialEvidencePosture,
    DerivedTopologySupportPosture, DerivedTopologyUpdatePosture,
};
pub use family_catalog::DerivedInvalidationFamilyCatalog;
pub use phase_three_seed::DerivedInvalidationPhaseThreeSeed;
pub use source_coverage::DerivedInvalidationFamilySourceCoverage;

pub(crate) fn catalog_digest(parts: impl IntoIterator<Item = String>) -> String {
    let mut parts = parts.into_iter().collect::<Vec<_>>();
    parts.sort();
    let mut state = 0xcbf29ce484222325_u64;
    for byte in parts.join("|").bytes() {
        state ^= byte as u64;
        state = state.wrapping_mul(0x100000001b3);
    }
    format!("derived-invalidation-family-catalog:{state:016x}")
}

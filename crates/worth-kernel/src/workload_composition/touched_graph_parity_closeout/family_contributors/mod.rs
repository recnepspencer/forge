use schema::facade::platform::authority::touched_graph_parity_closeout::{
    TouchedGraphParityCoverageContributor as SharedCoverageContributor,
    TouchedGraphParityQuerySurfaceKind as SharedQuerySurfaceKind,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KernelTouchedGraphParityCoverageError {
    detail: String,
}

impl KernelTouchedGraphParityCoverageError {
    pub(crate) fn new(detail: impl Into<String>) -> Self {
        Self {
            detail: detail.into(),
        }
    }
}

pub(crate) type KernelTouchedGraphParityCoverageContributor = SharedCoverageContributor;
pub(crate) type KernelTouchedGraphParityQuerySurfaceKind = SharedQuerySurfaceKind;

pub(crate) use conflict_family::conflict_family_coverage_contributor_rows;
#[cfg(test)]
pub(crate) use conflict_family::{
    current_batch_admission_coverage_contributor, current_conflict_coverage_contributor,
    current_independence_coverage_contributor,
};
pub use conflict_family::{
    current_conflict_family_contributor_catalog, current_conflict_family_parity_claim,
    ConflictFamilyContributorCatalog, ConflictFamilyContributorCatalogError,
    ConflictFamilyContributorCatalogErrorKind, ConflictFamilyContributorCatalogRow,
    ConflictFamilyContributorRowKind, ConflictFamilyParityClaim, ConflictFamilyParityError,
    ConflictFamilyParityErrorKind, ConflictFamilyParityRow,
};
pub(crate) use public_projection_family::public_projection_family_coverage_contributor_rows_from_public_facade;
#[cfg(test)]
pub(crate) use public_projection_family::{
    current_derived_diagnostics_coverage_contributor, current_public_proof_coverage_contributor,
};
pub use public_projection_family::{
    current_public_projection_contributor_catalog, current_public_projection_parity_claim,
    PublicProjectionContributorCatalog, PublicProjectionContributorCatalogError,
    PublicProjectionContributorCatalogErrorKind, PublicProjectionContributorCatalogRow,
    PublicProjectionContributorRowKind, PublicProjectionParityClaim, PublicProjectionParityError,
    PublicProjectionParityErrorKind, PublicProjectionParityRow,
};
pub(crate) use replay_undo::replay_undo_coverage_contributor_rows_from_authorities;
#[cfg(test)]
pub(crate) use replay_undo::{
    current_replay_coverage_contributor, current_undo_coverage_contributor,
};
pub use replay_undo::{
    current_replay_undo_family_contributor_catalog, current_replay_undo_family_parity_claim,
    ReplayUndoContributorRowKind, ReplayUndoFamilyContributorCatalog,
    ReplayUndoFamilyContributorCatalogError, ReplayUndoFamilyContributorCatalogErrorKind,
    ReplayUndoFamilyContributorCatalogRow, ReplayUndoFamilyParityClaim,
    ReplayUndoFamilyParityError, ReplayUndoFamilyParityErrorKind, ReplayUndoFamilyParityRow,
};
pub(crate) use reuse_family::reuse_family_coverage_contributor_rows;
#[cfg(test)]
pub(crate) use reuse_family::{
    current_compiled_product_equivalence_coverage_contributor,
    current_compiled_product_reuse_coverage_contributor,
};
pub use reuse_family::{
    current_reuse_family_contributor_catalog, current_reuse_family_parity_claim,
    ReuseFamilyContributorCatalog, ReuseFamilyContributorCatalogError,
    ReuseFamilyContributorCatalogErrorKind, ReuseFamilyContributorCatalogRow,
    ReuseFamilyContributorRowKind, ReuseFamilyParityClaim, ReuseFamilyParityError,
    ReuseFamilyParityErrorKind, ReuseFamilyParityRow,
};
pub(crate) use spatial_family_catalog::spatial_coverage_contributor_rows;
#[cfg(test)]
pub(crate) use spatial_family_catalog::{
    current_spatial_family_contributor_catalog as current_spatial_catalog,
};
pub use spatial_family_catalog::{
    current_spatial_family_contributor_catalog, SpatialFamilyContributorCatalogError,
    SpatialFamilyContributorCatalogErrorKind,
};
#[cfg(test)]
pub(crate) use spatial_family_parity::spatial_family_parity_claim_from_catalog;
pub use spatial_family_parity::{
    current_spatial_family_parity_claim, SpatialFamilyParityClaim, SpatialFamilyParityError,
    SpatialFamilyParityErrorKind, SpatialFamilyParityRow,
};
pub use topology_family_catalog::current_topology_family_contributor_catalog;
pub use topology_family_parity::current_topology_family_declare_once_parity_claim;

mod conflict_family;
mod public_projection_family;
mod replay_undo;
mod reuse_family;
mod spatial_family_catalog;
mod spatial_family_parity;
mod topology_family_catalog;
mod topology_family_parity;

#[cfg(test)]
mod tests_spatial;
#[cfg(test)]
mod tests_topology;

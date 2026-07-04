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

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

pub(crate) type KernelTouchedGraphParityCoverageContributor = SharedCoverageContributor;
pub(crate) type KernelTouchedGraphParityQuerySurfaceKind = SharedQuerySurfaceKind;

pub(crate) use conflict_family::{
    conflict_family_coverage_contributor_rows, current_batch_admission_coverage_contributor,
    current_conflict_coverage_contributor, current_independence_coverage_contributor,
};
pub use conflict_family::{
    current_conflict_family_contributor_catalog, current_conflict_family_parity_claim,
    ConflictFamilyContributorCatalog, ConflictFamilyContributorCatalogError,
    ConflictFamilyContributorCatalogErrorKind, ConflictFamilyContributorCatalogRow,
    ConflictFamilyContributorRowKind, ConflictFamilyParityClaim, ConflictFamilyParityError,
    ConflictFamilyParityErrorKind, ConflictFamilyParityRow,
};
pub(crate) use public_projection_family::{
    current_derived_diagnostics_coverage_contributor, current_public_proof_coverage_contributor,
    public_projection_family_coverage_contributor_rows,
    public_projection_family_coverage_contributor_rows_from_public_facade,
};
pub use public_projection_family::{
    current_public_projection_contributor_catalog, current_public_projection_parity_claim,
    PublicProjectionContributorCatalog, PublicProjectionContributorCatalogError,
    PublicProjectionContributorCatalogErrorKind, PublicProjectionContributorCatalogRow,
    PublicProjectionContributorRowKind, PublicProjectionParityClaim, PublicProjectionParityError,
    PublicProjectionParityErrorKind, PublicProjectionParityRow,
};
pub(crate) use replay_undo::{
    current_replay_coverage_contributor, current_undo_coverage_contributor,
    replay_undo_coverage_contributor_rows, replay_undo_coverage_contributor_rows_from_authorities,
};
pub use replay_undo::{
    current_replay_undo_family_contributor_catalog, current_replay_undo_family_parity_claim,
    ReplayUndoContributorRowKind, ReplayUndoFamilyContributorCatalog,
    ReplayUndoFamilyContributorCatalogError, ReplayUndoFamilyContributorCatalogErrorKind,
    ReplayUndoFamilyContributorCatalogRow, ReplayUndoFamilyParityClaim,
    ReplayUndoFamilyParityError, ReplayUndoFamilyParityErrorKind, ReplayUndoFamilyParityRow,
};
pub(crate) use reuse_family::{
    current_compiled_product_equivalence_coverage_contributor,
    current_compiled_product_reuse_coverage_contributor, reuse_family_coverage_contributor_rows,
};
pub use reuse_family::{
    current_reuse_family_contributor_catalog, current_reuse_family_parity_claim,
    ReuseFamilyContributorCatalog, ReuseFamilyContributorCatalogError,
    ReuseFamilyContributorCatalogErrorKind, ReuseFamilyContributorCatalogRow,
    ReuseFamilyContributorRowKind, ReuseFamilyParityClaim, ReuseFamilyParityError,
    ReuseFamilyParityErrorKind, ReuseFamilyParityRow,
};
pub(crate) use spatial_family_catalog::spatial_coverage_contributor_rows;
pub use spatial_family_catalog::{
    current_spatial_family_contributor_catalog, SpatialFamilyContributorCatalogError,
    SpatialFamilyContributorCatalogErrorKind,
};
pub use spatial_family_parity::{
    current_spatial_family_parity_claim, SpatialFamilyParityClaim, SpatialFamilyParityError,
    SpatialFamilyParityErrorKind, SpatialFamilyParityRow,
};
pub(crate) use topology_family_catalog::topology_coverage_contributor_rows;
pub use topology_family_catalog::{
    current_topology_family_contributor_catalog, TopologyFamilyContributorCatalogError,
    TopologyFamilyContributorCatalogErrorKind,
};
pub use topology_family_parity::{
    current_topology_family_declare_once_parity_claim, TopologyFamilyDeclareOnceParityClaim,
    TopologyFamilyDeclareOnceParityError, TopologyFamilyDeclareOnceParityErrorKind,
    TopologyFamilyDeclareOnceParityRow,
};

mod conflict_family;
mod public_projection_family;
mod replay_undo;
mod reuse_family;
mod spatial_family_catalog;
mod spatial_family_parity;
mod topology_family_catalog;
mod topology_family_parity;

mod tests_spatial;
#[cfg(test)]
mod tests_topology;

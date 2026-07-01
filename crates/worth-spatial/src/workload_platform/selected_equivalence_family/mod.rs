mod basis_identity;
mod catalog;
mod declaration;
mod error;
mod family_identity;
mod posture;
mod selected_family;
mod selection;

#[cfg(test)]
mod tests;

pub use basis_identity::{
    SpatialSelectedCompatibilityBasisIdentity, SpatialSelectedEquivalenceBasisIdentity,
    SpatialSelectedFutureProofSeedIdentity, SpatialSelectedReuseBasisIdentity,
};
pub use catalog::{
    current_spatial_selected_equivalence_family_catalog, SpatialSelectedEquivalenceFamilyCatalog,
};
pub use error::{SpatialSelectedEquivalenceFamilyError, SpatialSelectedEquivalenceFamilyErrorKind};
pub use family_identity::SpatialSelectedEquivalenceFamilyIdentity;
pub use posture::{
    SpatialCompatibilityPosture, SpatialFreshnessRequirementPosture, SpatialOrderingNoisePosture,
    SpatialRenderedOutputComparisonPosture,
};
pub use selected_family::SelectedSpatialEquivalenceFamily;
pub use selection::select_spatial_equivalence_family;

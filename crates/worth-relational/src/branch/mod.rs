mod authority;
mod basis;
mod basis_axis_validation;
mod basis_counters;
mod basis_denial;
mod basis_descriptor_resolution;
mod basis_identity_validation;
mod basis_observation;
mod basis_readmission;
mod basis_registry;
mod basis_retention;
mod coordination;
mod fork;
mod fork_source_basis;
mod identity;
mod reference;
mod registry;
mod root;
mod root_checkpoint;
mod root_partition_access;
mod root_region;
mod root_regions;
mod root_runtime_access;
mod root_selection;
mod target;
mod version;

pub(crate) use authority::issue_relational_branch_mutation_authority;
pub use authority::{
    RelationalBranchMutationAuthority, RelationalBranchMutationAuthorityMarker,
    RelationalBranchObservationAuthority, RelationalBranchObservationAuthorityMarker,
    RelationalForkSourceAuthority, RelationalForkSourceAuthorityMarker,
};
pub(crate) use basis::AdmittedRelationalBranchBasisInner;
pub use basis::{
    AdmittedRelationalBranchBasis, RelationalBranchBasisDescriptor, RelationalBranchBasisPosture,
    ResolvedRelationalBasisDescriptor, RELATIONAL_BRANCH_BASIS_DESCRIPTOR_VERSION,
};
pub use basis_counters::RelationalBranchBasisCostCounters;
pub use basis_denial::{RelationalBranchBasisDenial, RelationalBranchBasisMismatchAxis};
pub(crate) use basis_registry::{
    RelationalBranchBasisRegistry, RelationalBranchBasisRegistryMetrics,
};
pub use coordination::RelationalBranchCoordinationCellId;
pub use fork::{RelationalForkDenial, RelationalForkOutcome};
pub use fork_source_basis::{AdmittedRelationalForkSourceBasis, RelationalForkSourceDescriptor};
pub use identity::{RelationalBranchIdentity, RelationalBranchIdentityDenial};
pub use reference::RelationalBranchCellDenial;
pub use reference::{
    relational_branch_observation, RelationalBranchComparisonBasis, RelationalBranchForkBasis,
    RelationalBranchObservationConstructionDenial, RelationalBranchReferenceObservation,
    RelationalBranchReferenceState,
};
pub(crate) use reference::{RelationalBranchCellCheckpoint, RelationalBranchReferenceCell};
pub(crate) use registry::RelationalBranchReferenceRegistry;
pub(crate) use root::{
    PreparedRelationalBranchRootCapture, RelationalBranchRoot, RelationalBranchRootCaptureDenial,
    RelationalBranchRootIdentityIssuer, RelationalBranchRootSchemaAuthority,
    RelationalBranchRootState, RelationalRootAuthoritativeAllocationKind,
    RelationalRootCorrectnessIndex,
};
pub(crate) use root_checkpoint::RelationalBranchRootCheckpoint;
pub(crate) use root_regions::{
    RelationalPersistentRegionAllocationKind, RelationalPersistentRegionSet,
};
pub(crate) use root_selection::SelectedRelationalBranchState;
pub use target::{RelationalBranchRootDescriptor, RelationalBranchTarget};
pub use version::RelationalBranchVersion;

#[cfg(test)]
#[path = "basis_admission_tests.rs"]
mod basis_admission_tests;
#[cfg(test)]
#[path = "basis_registry_scale_tests.rs"]
mod basis_registry_scale_tests;

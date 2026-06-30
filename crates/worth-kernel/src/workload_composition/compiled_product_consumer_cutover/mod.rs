mod closeout;
mod consumer_class;
mod coverage_target;
mod current_matrix;
mod dependency_matrix;
mod dependency_row;
mod error;
mod family_class;
mod future_cutover_lane;
mod proof_basis;
mod query_boundary_lane;

#[cfg(test)]
mod tests;

pub use consumer_class::KernelCompiledProductConsumerResponsibility;
pub use dependency_matrix::{
    current_kernel_compiled_product_consumer_dependency_matrix,
    KernelCompiledProductConsumerDependencyMatrix,
};
pub use dependency_row::{
    KernelCompiledProductConsumerClusterIdentity, KernelCompiledProductConsumerDependencyRow,
};
pub use error::{
    KernelCompiledProductConsumerDependencyError, KernelCompiledProductConsumerDependencyErrorKind,
};
pub use family_class::KernelCompiledProductFamilyClass;
pub use future_cutover_lane::KernelCompiledProductFutureCutoverLane;
pub use proof_basis::KernelCompiledProductProofBasis;
pub use query_boundary_lane::KernelCompiledProductQueryBoundaryLane;

pub(crate) use current_matrix::current_coverage_targets;

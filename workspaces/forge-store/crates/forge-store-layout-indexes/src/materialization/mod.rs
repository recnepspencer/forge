mod admission;
mod coverage;
mod coverage_basis;
mod denial;
mod freshness;
#[cfg(test)]
mod runtime_tests;
mod source;
#[cfg(test)]
mod source_binding_tests;
mod state;
#[cfg(test)]
pub(crate) mod test_support;
#[cfg(test)]
pub(crate) mod tests;
mod watermark;

pub use admission::AdmittedLayoutMaterialization;
pub(crate) use coverage::LayoutCoverageWitness;
pub use coverage_basis::AdmittedCoverageBasis;
#[cfg(test)]
pub use denial::CoverageGapWitness;
pub use denial::MaterializationDenial;
pub use freshness::{
    CurrentLayoutMaterialization, CurrentMaterializationFrontier, MaterializationFreshness,
    StaleLayoutMaterialization,
};
pub use source::{
    ImportedBlobMaterializationSourceIdentity, LayoutMaterializationSourceIdentity,
    LayoutMaterializationSourceKind, RestoredArtifactMaterializationSourceIdentity,
};
pub(crate) use state::{LayoutMaterializationState, MaterializationStateClass};
pub(crate) use watermark::{CoverageBasisKind, LayoutWatermark, PhysicalCoverageBasis};

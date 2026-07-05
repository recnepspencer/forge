mod classification;
mod counters;
mod denial;
mod identity;
mod input;
mod product;
mod rows;
#[cfg(test)]
mod tests;
mod validation;

pub use counters::PlanarBooleanOverlapRegionCandidateBoundaryCounters;
pub use denial::{
    PlanarBooleanDeniedOverlapRegionCandidateKind,
    PlanarBooleanOverlapRegionCandidateBoundaryDenial,
    PlanarBooleanOverlapRegionCandidateBoundaryDenialKind,
};
pub use input::PlanarBooleanOverlapRegionCandidateBoundaryInput;
pub use product::{
    PlanarBooleanAdmittedOverlapRegionSet, PlanarBooleanBoundaryOnlyOverlapOutcomeSet,
    PlanarBooleanDeniedOverlapRegionCandidateSet,
    PlanarBooleanOverlapRegionCandidateBoundaryBundle, PlanarBooleanOverlapRegionCandidateSet,
};
pub use rows::{
    PlanarBooleanAdmittedOverlapRegionRow, PlanarBooleanBoundaryOnlyOverlapOutcomeRow,
    PlanarBooleanDeniedOverlapRegionCandidateRow, PlanarBooleanOverlapRegionCandidateRow,
};

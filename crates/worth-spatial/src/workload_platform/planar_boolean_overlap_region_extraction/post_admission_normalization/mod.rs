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

pub use counters::PlanarBooleanPostAdmissionNormalizationCounters;
pub use denial::{
    PlanarBooleanPostAdmissionNormalizationDenial,
    PlanarBooleanPostAdmissionNormalizationDenialKind,
};
pub use input::PlanarBooleanPostAdmissionNormalizationInput;
pub use product::{
    PlanarBooleanOverlapRegionCanonicalWindingSet, PlanarBooleanPostAdmissionNormalizationBundle,
};
pub use rows::{
    PlanarBooleanOverlapRegionCanonicalWindingRow,
    PlanarBooleanOverlapRegionCanonicalWindingSourceKind,
};

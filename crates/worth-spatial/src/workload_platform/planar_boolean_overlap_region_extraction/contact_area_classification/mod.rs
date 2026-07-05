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

pub use counters::PlanarBooleanBoundaryContactClassificationCounters;
pub use denial::{
    PlanarBooleanBoundaryContactClassificationDenial,
    PlanarBooleanBoundaryContactClassificationDenialKind,
};
pub use input::PlanarBooleanBoundaryContactClassificationInput;
pub use product::{
    PlanarBooleanBoundaryContactClassificationBundle, PlanarBooleanPureBoundaryOnlyOutcomeSet,
    PlanarBooleanSharedBoundaryContactOutcomeSet,
};
pub use rows::{
    PlanarBooleanPureBoundaryOnlyOutcomeRow, PlanarBooleanSharedBoundaryContactOutcomeRow,
};

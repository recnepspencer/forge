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

pub use counters::PlanarBooleanSharedAreaAdmissionCounters;
pub use denial::{
    PlanarBooleanSharedAreaAdmissionDenial, PlanarBooleanSharedAreaAdmissionDenialKind,
};
pub use input::PlanarBooleanSharedAreaAdmissionInput;
pub use product::{
    PlanarBooleanMixedBoundaryAreaOutcomeSet, PlanarBooleanSharedAreaAdmissionBundle,
    PlanarBooleanSharedAreaAdmissionOutcomeSet,
};
pub use rows::{
    PlanarBooleanMixedBoundaryAreaOutcomeRow, PlanarBooleanSharedAreaAdmissionOutcomeRow,
};

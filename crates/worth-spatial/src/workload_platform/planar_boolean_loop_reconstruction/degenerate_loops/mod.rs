mod construction;
mod counters;
mod geometry;
mod identity;
mod input;
mod product;
mod row;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_adversarial;
#[cfg(test)]
mod tests_support;

pub use counters::PlanarBooleanDegenerateLoopOutcomeBoundaryCounters;
pub use input::PlanarBooleanDegenerateLoopOutcomeBoundaryInput;
pub use product::{
    PlanarBooleanDegenerateLoopOutcomeBoundary, PlanarBooleanDegenerateLoopOutcomeSet,
};
pub use row::{PlanarBooleanDegenerateLoopOutcome, PlanarBooleanDegenerateLoopOutcomeKind};

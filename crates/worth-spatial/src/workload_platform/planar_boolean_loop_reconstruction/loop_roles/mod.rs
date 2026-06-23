mod construction;
mod counters;
mod identity;
mod input;
mod product;
mod row;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_adversarial;

pub use counters::PlanarBooleanLoopRoleOutcomeBoundaryCounters;
pub use input::PlanarBooleanLoopRoleOutcomeBoundaryInput;
pub use product::{
    PlanarBooleanLoopContainmentEvidencePostureSet, PlanarBooleanLoopRoleOutcomeBoundary,
    PlanarBooleanLoopRoleOutcomeSet,
};
pub use row::{
    PlanarBooleanLoopClassifiedProductKind, PlanarBooleanLoopContainmentEvidencePosture,
    PlanarBooleanLoopContainmentEvidencePostureKind, PlanarBooleanLoopRoleOutcome,
    PlanarBooleanLoopRoleOutcomeKind,
};

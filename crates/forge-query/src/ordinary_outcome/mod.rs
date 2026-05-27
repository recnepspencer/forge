mod binding;
mod kind;
mod next_step;
mod orchestration;
mod outcome;
mod posture;
mod topology;

pub use kind::ForgeQueryOrdinaryPostureKind;
pub use next_step::ForgeQueryOrdinaryNextStep;
pub use outcome::ForgeQueryOrdinaryOutcome;
pub use posture::ForgeQueryOrdinaryPosture;
pub use topology::{
    ForgeQueryOrdinaryBindingCheckedTopologyKind, ForgeQueryOrdinaryCheckedTopology,
    ForgeQueryOrdinaryContinuationCheckedTopologyKind,
    ForgeQueryOrdinarySignalCompatibilityOrchestrationCheckedTopologyKind,
};

pub(crate) use binding::ordinary_outcome_from_binding_outcome;
pub(crate) use orchestration::ordinary_outcome_from_orchestration_terminal;

#[cfg(test)]
mod tests;

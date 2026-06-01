mod artifact;
mod input;
mod lower;
mod outcome;
mod transcript;

pub use artifact::{
    ForgeQuerySignalCompatibilityOrchestration, ForgeQuerySignalCompatibilityOrchestrationClass,
};
pub use input::ForgeQuerySignalCompatibilityOrchestrationInput;
pub use outcome::{
    ordinary_outcome_from_signal_compatibility_orchestration_checked,
    ForgeQuerySignalCompatibilityOrchestrationChecked,
    ForgeQuerySignalCompatibilityOrchestrationOutcome,
};
pub use transcript::ForgeQuerySignalCompatibilityOrchestrationTranscript;

pub(crate) use transcript::orchestrate_signal_compatibility_on_handle;

#[cfg(test)]
mod tests;

mod artifact;
mod input;
mod lower;
mod outcome;
mod transcript;

pub use artifact::{
    WorthQuerySignalCompatibilityOrchestration, WorthQuerySignalCompatibilityOrchestrationClass,
};
pub use input::WorthQuerySignalCompatibilityOrchestrationInput;
pub use outcome::{
    ordinary_outcome_from_signal_compatibility_orchestration_checked,
    WorthQuerySignalCompatibilityOrchestrationChecked,
    WorthQuerySignalCompatibilityOrchestrationOutcome,
};
pub use transcript::WorthQuerySignalCompatibilityOrchestrationTranscript;

pub(crate) use transcript::orchestrate_signal_compatibility_on_handle;

#[cfg(test)]
mod tests;

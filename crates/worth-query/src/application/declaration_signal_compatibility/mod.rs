mod artifact;
mod aspect_gate;
mod checked;
mod checked_input;
mod contract;
mod denial;
mod digest;
mod explain;
mod handle_gate;
mod lower;

pub use artifact::{
    WorthQueryDeclarationSignalCompatibility, WorthQueryDeclarationSignalCompatibilityClass,
};
pub use checked::{
    WorthQueryDeclarationSignalCompatibilityChecked, WorthQueryDeclarationSignalCompatibilityInput,
};
pub use contract::{
    WorthQueryDeclarationSignalCompatibilityContract,
    WorthQueryDeclarationSignalCompatibilitySupportReport,
    WorthQueryDeclarationSignalCompatibilitySupportRow,
    WorthQueryDeclarationSignalCompatibilitySupportStatus,
    WorthQueryDeclarationSignalExecutionFamily,
};
pub use denial::{
    WorthQueryDeclarationEntrySignalCompatibilityError,
    WorthQueryDeclarationSignalCompatibilityDeferred,
    WorthQueryDeclarationSignalCompatibilityDenialCause,
    WorthQueryDeclarationSignalCompatibilityDenied, WorthQueryDeclarationSignalCompatibilityFailed,
    WorthQueryDeclarationSignalCompatibilityTerminalError,
};
pub use explain::WorthQueryDeclarationSignalCompatibilityExplanation;

pub(crate) use checked::worth_query_checked_declaration_signal_compatibility_on_handle;
pub(crate) use contract::derive_signal_compatibility_support_report;

#[cfg(test)]
mod tests;

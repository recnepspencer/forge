mod artifact;
mod checked;
mod contract;
mod denial;
mod digest;
mod explain;
mod lower;

pub use artifact::{
    ForgeQueryDeclarationSignalCompatibility, ForgeQueryDeclarationSignalCompatibilityClass,
};
pub use checked::{
    ForgeQueryDeclarationSignalCompatibilityChecked, ForgeQueryDeclarationSignalCompatibilityInput,
};
pub use contract::{
    ForgeQueryDeclarationSignalCompatibilityContract,
    ForgeQueryDeclarationSignalCompatibilitySupportReport,
    ForgeQueryDeclarationSignalCompatibilitySupportRow,
    ForgeQueryDeclarationSignalCompatibilitySupportStatus,
    ForgeQueryDeclarationSignalExecutionFamily,
};
pub use denial::{
    ForgeQueryDeclarationEntrySignalCompatibilityError,
    ForgeQueryDeclarationSignalCompatibilityDeferred,
    ForgeQueryDeclarationSignalCompatibilityDenialCause,
    ForgeQueryDeclarationSignalCompatibilityDenied, ForgeQueryDeclarationSignalCompatibilityFailed,
    ForgeQueryDeclarationSignalCompatibilityTerminalError,
};
pub use explain::ForgeQueryDeclarationSignalCompatibilityExplanation;

pub(crate) use checked::forge_query_checked_declaration_signal_compatibility_on_handle;
pub(crate) use contract::derive_signal_compatibility_support_report;

#[cfg(test)]
mod tests;

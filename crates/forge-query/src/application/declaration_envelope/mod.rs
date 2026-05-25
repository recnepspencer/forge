mod artifact;
mod checked;
mod class;
mod denial;
mod digest;
mod explain;
mod input;

pub use artifact::ForgeQueryDeclarationEnvelope;
pub use checked::ForgeQueryDeclarationEnvelopeChecked;
pub use class::{ForgeQueryDeclarationEnvelopeClass, ForgeQueryDeclarationEnvelopeEvidenceOrigin};
pub use denial::{
    ForgeQueryDeclarationEntryEnvelopeError, ForgeQueryDeclarationEnvelopeDeferred,
    ForgeQueryDeclarationEnvelopeDenied, ForgeQueryDeclarationEnvelopeFailed,
    ForgeQueryDeclarationEnvelopeTerminalError,
};
pub use explain::ForgeQueryDeclarationEnvelopeExplanation;
pub use input::ForgeQueryDeclarationEnvelopeInput;

pub(crate) use checked::{
    forge_query_checked_declaration_envelope,
    forge_query_declaration_envelope_terminal_from_receipt_terminal,
};

#[cfg(test)]
mod tests;

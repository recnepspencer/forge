mod artifact;
mod checked;
mod denial;
mod digest;
mod explain;
mod input;
mod materialize;

pub use artifact::{
    ForgeQueryDeclarationReceipt, ForgeQueryDeclarationReceiptClass,
    ForgeQueryDeclarationReceiptKind,
};
pub use checked::ForgeQueryDeclarationReceiptChecked;
pub(crate) use checked::{
    forge_query_checked_declaration_receipt,
    forge_query_checked_declaration_receipt_with_materialized_profile,
};
pub use denial::{
    ForgeQueryDeclarationEntryReceiptError, ForgeQueryDeclarationReceiptDeferred,
    ForgeQueryDeclarationReceiptDenialCause, ForgeQueryDeclarationReceiptDenied,
    ForgeQueryDeclarationReceiptFailed, ForgeQueryDeclarationReceiptTerminalError,
};
pub use explain::ForgeQueryDeclarationReceiptExplanation;
pub use input::ForgeQueryDeclarationReceiptInput;
pub(crate) use materialize::receipt_materialized_profile_for_tier;

#[cfg(test)]
mod tests;

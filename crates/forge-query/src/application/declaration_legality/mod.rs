mod checked;
mod contract;
mod denial;
mod evaluate;
mod evidence;
mod input;
mod outcome;

pub use checked::ForgeQueryDeclarationLegalityChecked;
pub use contract::{ForgeQueryDeclarationLegalityClass, ForgeQueryDeclarationLegalityContract};
pub use denial::{
    ForgeQueryAsyncLegalityDenialKind, ForgeQueryDeclarationLegalityDenial,
    ForgeQueryTemporalLegalityDenialKind,
};
pub use evidence::ForgeQueryDeclarationLegalityEvidence;
pub use input::ForgeQueryDeclarationLegalityInput;
pub use outcome::ForgeQueryDeclarationAdmissionOrLegalityError;

pub(crate) use evaluate::review_declaration_legality;

#[cfg(test)]
mod tests;

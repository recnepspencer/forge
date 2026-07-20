mod checked;
mod contract;
mod denial;
mod evaluate;
mod evidence;
mod input;
mod outcome;

pub use checked::WorthQueryDeclarationLegalityChecked;
pub use contract::{WorthQueryDeclarationLegalityClass, WorthQueryDeclarationLegalityContract};
pub use denial::{
    WorthQueryAsyncLegalityDenialKind, WorthQueryDeclarationLegalityDenial,
    WorthQueryTemporalLegalityDenialKind,
};
pub use evidence::WorthQueryDeclarationLegalityEvidence;
pub use input::WorthQueryDeclarationLegalityInput;
pub use outcome::WorthQueryDeclarationAdmissionOrLegalityError;

pub(crate) use evaluate::review_declaration_legality;

#[cfg(test)]
mod tests;

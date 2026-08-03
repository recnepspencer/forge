mod basis;
mod classifier;
mod denial;
mod outcome;
mod owner;

pub use basis::UiChangeClassificationBasis;
pub(crate) use classifier::{UiChangeClassificationRequest, UiChangeClassifier};
pub use denial::{UiAuthoredFactDeclarationSide, UiChangeClassificationDenial};
pub(crate) use outcome::{UiAuthoredSourceClassification, UiAuthoredSourceSuccession};
pub use outcome::{
    UiChangeClassificationOutcome, UiClassifiedChange, UiEvidenceOnlySourceChange,
    UiObservedNoChangeReceipt,
};
pub(crate) use owner::authored::lower_differences as lower_authored_differences;

#[cfg(test)]
mod tests;

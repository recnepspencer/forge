use crate::validation::{IntegrityValidatedPreviousRootSelector, PhysicalIntegrityRejection};

#[derive(Debug)]
pub enum PreviousRootSelectorIntegrityValidation<'media> {
    Intact(IntegrityValidatedPreviousRootSelector<'media>),
    Rejected(PhysicalIntegrityRejection),
}

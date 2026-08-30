use crate::validation::{IntegrityValidatedCurrentRootSelector, PhysicalIntegrityRejection};

#[derive(Debug)]
pub enum CurrentRootSelectorIntegrityValidation<'media> {
    Intact(IntegrityValidatedCurrentRootSelector<'media>),
    Rejected(PhysicalIntegrityRejection),
}

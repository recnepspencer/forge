use crate::validation::{IntegrityValidatedRootManifest, PhysicalIntegrityRejection};

#[derive(Debug)]
pub enum RootManifestIntegrityValidation<'media> {
    Intact(IntegrityValidatedRootManifest<'media>),
    Rejected(PhysicalIntegrityRejection),
}

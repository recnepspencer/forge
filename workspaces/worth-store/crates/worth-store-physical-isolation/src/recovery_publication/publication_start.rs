use std::path::Path;

use super::{
    DurableRecoveryPublicationLocator, RecoveryPublicationDenial, RecoveryPublicationOwner,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryPublicationStartPosture {
    NotStarted,
    DurableLocatorPresent,
}

impl RecoveryPublicationOwner {
    pub fn classify_publication_start(
        publication_directory: &Path,
        publication_identity: [u8; 32],
        expected_plan_fingerprint: [u8; 32],
        expected_media_identity: [u8; 32],
    ) -> Result<RecoveryPublicationStartPosture, RecoveryPublicationDenial> {
        if publication_identity == [0; 32]
            || expected_plan_fingerprint == [0; 32]
            || expected_media_identity == [0; 32]
        {
            return Err(RecoveryPublicationDenial::InvalidBinding);
        }
        if !DurableRecoveryPublicationLocator::binding_exists(
            publication_directory,
            publication_identity,
        )? {
            return Ok(RecoveryPublicationStartPosture::NotStarted);
        }
        let locator = DurableRecoveryPublicationLocator::reopen_by_binding(
            publication_directory,
            publication_identity,
        )?;
        if locator.plan_fingerprint != expected_plan_fingerprint
            || locator.media_identity != expected_media_identity
        {
            return Err(RecoveryPublicationDenial::PublicationLocatorConflict);
        }
        Ok(RecoveryPublicationStartPosture::DurableLocatorPresent)
    }
}

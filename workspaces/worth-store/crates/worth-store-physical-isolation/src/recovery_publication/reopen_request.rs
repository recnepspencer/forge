use std::path::PathBuf;

use crate::CurrentPhysicalRoot;

#[derive(Debug, Clone)]
pub struct ReopenRecoveryPublicationRequest {
    pub(super) publication_directory: PathBuf,
    pub(super) current_root: CurrentPhysicalRoot,
    pub(super) publication_identity: [u8; 32],
    pub(super) publication_plan_fingerprint: [u8; 32],
    pub(super) candidate_media_identity: [u8; 32],
}

impl ReopenRecoveryPublicationRequest {
    pub fn new(
        publication_directory: impl Into<PathBuf>,
        current_root: CurrentPhysicalRoot,
        publication_identity: [u8; 32],
        publication_plan_fingerprint: [u8; 32],
        candidate_media_identity: [u8; 32],
    ) -> Self {
        Self {
            publication_directory: publication_directory.into(),
            current_root,
            publication_identity,
            publication_plan_fingerprint,
            candidate_media_identity,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ReopenRecoveryPublicationByIdentityRequest {
    pub(super) publication_directory: PathBuf,
    pub(super) publication_identity: [u8; 32],
    pub(super) publication_plan_fingerprint: [u8; 32],
    pub(super) candidate_media_identity: [u8; 32],
}

impl ReopenRecoveryPublicationByIdentityRequest {
    pub fn new(
        publication_directory: impl Into<PathBuf>,
        publication_identity: [u8; 32],
        publication_plan_fingerprint: [u8; 32],
        candidate_media_identity: [u8; 32],
    ) -> Self {
        Self {
            publication_directory: publication_directory.into(),
            publication_identity,
            publication_plan_fingerprint,
            candidate_media_identity,
        }
    }
}

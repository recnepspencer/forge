use super::{
    OfflineInspectionBudget, OfflineInspectionCancellation, OfflineInspectionCheckpoint,
    OfflineInspectionScope, OfflineInspectionSession,
};
use crate::media_acquisition::{
    acquire_read_only_media, OfflineMediaAcquisitionDenial, UntrustedOfflineMediaSet,
};

#[derive(Debug)]
pub struct OfflineStoreInspection {
    media: UntrustedOfflineMediaSet,
    scope: OfflineInspectionScope,
    budget: OfflineInspectionBudget,
    cancellation: OfflineInspectionCancellation,
}

impl OfflineStoreInspection {
    pub fn open(media: UntrustedOfflineMediaSet) -> Self {
        Self {
            media,
            scope: OfflineInspectionScope::all_physical_families(),
            budget: OfflineInspectionBudget::bounded(64 * 1024, u64::MAX)
                .expect("nonzero defaults"),
            cancellation: OfflineInspectionCancellation::new(),
        }
    }
    pub const fn scope(mut self, scope: OfflineInspectionScope) -> Self {
        self.scope = scope;
        self
    }
    pub const fn budget(mut self, budget: OfflineInspectionBudget) -> Self {
        self.budget = budget;
        self
    }
    pub fn cancellation(mut self, cancellation: OfflineInspectionCancellation) -> Self {
        self.cancellation = cancellation;
        self
    }
    pub fn start(self) -> Result<OfflineInspectionSession, OfflineMediaAcquisitionDenial> {
        self.fresh_session()
    }
    pub fn resume_from_checkpoint(
        self,
        checkpoint: &OfflineInspectionCheckpoint,
    ) -> Result<OfflineInspectionSession, OfflineMediaAcquisitionDenial> {
        let mut session = self.fresh_session()?;
        let _reused = session.apply_checkpoint(checkpoint)?;
        Ok(session)
    }
    pub fn resume_from_checkpoint_bytes(
        self,
        encoded_checkpoint: &[u8],
    ) -> Result<OfflineInspectionSession, OfflineMediaAcquisitionDenial> {
        let mut session = self.fresh_session()?;
        match OfflineInspectionCheckpoint::decode_with_owned_allocation_limit(
            encoded_checkpoint,
            session.checkpoint_decode_allocation_limit(),
        ) {
            Ok(checkpoint) => {
                let _reused = session.apply_owned_checkpoint(checkpoint)?;
                Ok(session)
            }
            Err(_) => {
                session.reject_checkpoint()?;
                Ok(session)
            }
        }
    }

    fn fresh_session(self) -> Result<OfflineInspectionSession, OfflineMediaAcquisitionDenial> {
        let started_at = std::time::Instant::now();
        OfflineInspectionSession::new(
            acquire_read_only_media(self.media, self.budget, &self.cancellation, started_at)?,
            self.scope,
            self.budget,
            self.cancellation,
            started_at,
        )
    }
}

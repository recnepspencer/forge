use worth_ui_host_contract::UiGlyphRasterTransactionPending;

#[cfg(test)]
use super::super::UiNativePhysicalSignalSettlement;
use super::super::{declarations::UiNativePhysicalSignalOperation, UiNativePhysicalSignalOwner};

impl UiNativePhysicalSignalOwner {
    #[cfg(test)]
    pub(in crate::native::physical_work_signal) fn cancel_atlas_upload(
        &mut self,
        pending: UiGlyphRasterTransactionPending,
    ) -> UiNativePhysicalSignalSettlement {
        let Some(identity) = self.route.atlas_upload(pending) else {
            self.note_stale();
            return UiNativePhysicalSignalSettlement::Stale;
        };
        let Ok(token) = self.begin_work(super::super::UiNativePhysicalSignalWork::AtlasUpload(
            identity,
        )) else {
            self.note_stale();
            return UiNativePhysicalSignalSettlement::Stale;
        };
        let cancelled = self
            .worker_mut()
            .and_then(|worker| worker.cancel_handle(token.handle()))
            .unwrap_or(false);
        if !cancelled {
            self.note_stale();
            return UiNativePhysicalSignalSettlement::Stale;
        }
        self.route.remove(token);
        self.counters.cancellations = self.counters.cancellations.saturating_add(1);
        self.wake.remove(token.work());
        UiNativePhysicalSignalSettlement::Rejected
    }

    pub(crate) fn cancel_atlas_upload_to_recovery(
        &mut self,
        pending: UiGlyphRasterTransactionPending,
    ) -> Result<super::UiNativePhysicalSignalRequestToken, ()> {
        let token = self.transition_atlas_upload_to_recovery(pending)?;
        self.counters.cancellations = self.counters.cancellations.saturating_add(1);
        Ok(token)
    }

    pub(crate) fn supersede_atlas_upload_to_recovery(
        &mut self,
        pending: UiGlyphRasterTransactionPending,
    ) -> Result<super::UiNativePhysicalSignalRequestToken, ()> {
        let token = self.transition_atlas_upload_to_recovery(pending)?;
        self.counters.cancellations = self.counters.cancellations.saturating_add(1);
        self.counters.supersessions = self.counters.supersessions.saturating_add(1);
        Ok(token)
    }

    pub(crate) fn transition_atlas_upload_to_recovery(
        &mut self,
        pending: UiGlyphRasterTransactionPending,
    ) -> Result<super::UiNativePhysicalSignalRequestToken, ()> {
        let identity = self.route.atlas_upload(pending).ok_or(())?;
        self.transition_work_to_recovery(super::super::UiNativePhysicalSignalWork::AtlasUpload(
            identity,
        ))
    }

    pub(crate) fn transition_presentation_to_recovery(
        &mut self,
        identity: super::super::UiNativePhysicalPresentationIdentity,
    ) -> Result<super::UiNativePhysicalSignalRequestToken, ()> {
        self.transition_work_to_recovery(super::super::UiNativePhysicalSignalWork::Presentation(
            identity,
        ))
    }

    pub(crate) fn cancel_presentation_to_recovery(
        &mut self,
        identity: super::super::UiNativePhysicalPresentationIdentity,
    ) -> Result<super::UiNativePhysicalSignalRequestToken, ()> {
        let before = self.observation();
        let token = self.transition_presentation_to_recovery(identity)?;
        self.counters.cancellations = self.counters.cancellations.saturating_add(1);
        let after = self.observation();
        self.record_transition_observation(
            super::super::transition_observation::UiNativePhysicalSignalTransitionObservation::from_owner_cancellation(
                token.work(),
                before,
                after,
            ),
        );
        Ok(token)
    }

    pub(in crate::native::physical_work_signal) fn transition_work_to_recovery(
        &mut self,
        work: super::super::UiNativePhysicalSignalWork,
    ) -> Result<super::UiNativePhysicalSignalRequestToken, ()> {
        let token = self.begin_work(work)?;
        if !self.worker_mut()?.cancel_handle(token.handle())? || !self.route.remove(token) {
            return Err(());
        }
        self.wake.remove(token.work());
        self.admit_recovery_work(work)
    }

    pub(in crate::native::physical_work_signal) fn admit_recovery_work(
        &mut self,
        work: super::super::UiNativePhysicalSignalWork,
    ) -> Result<super::UiNativePhysicalSignalRequestToken, ()> {
        let (handle, performed) = self
            .worker_mut()?
            .admit(UiNativePhysicalSignalOperation::Recovery, work)?;
        self.route.record(work, handle).map_err(|_| ())?;
        self.counters.recovery_schedules = self.counters.recovery_schedules.saturating_add(1);
        self.publish_performed(performed)?;
        self.begin_work(work)
    }
}

use super::{UiNativeHostState, UiNativePresentationPhysicalProgress};

pub(super) struct ReadyPresentation {
    pub(super) index: usize,
    pub(super) pending: crate::native::UiNativePendingPresentation,
    pub(super) resolving_recovery: bool,
}

impl UiNativeHostState {
    pub(super) fn acquire_ready_presentation(
        &mut self,
        identity: crate::native::physical_work_signal::UiNativePhysicalPresentationIdentity,
    ) -> Result<ReadyPresentation, UiNativePresentationPhysicalProgress> {
        let Some(index) = self
            .pending_presentations
            .iter()
            .position(|pending| pending.physical_work() == identity)
        else {
            return Err(UiNativePresentationPhysicalProgress::NoProgress);
        };
        let mut pending = self.pending_presentations.remove(index);
        let Ok(ready) = self
            .physical_signal
            .take_ready_presentation(identity, pending.physical_token())
        else {
            self.pending_presentations.insert(index, pending);
            return Err(UiNativePresentationPhysicalProgress::NoProgress);
        };
        if !pending.refresh_physical_token(ready.current()) {
            return Err(UiNativePresentationPhysicalProgress::NoProgress);
        }
        let resolving_recovery = self
            .physical_signal
            .token_uses_recovery(pending.physical_token());
        Ok(ReadyPresentation {
            index,
            pending,
            resolving_recovery,
        })
    }
}

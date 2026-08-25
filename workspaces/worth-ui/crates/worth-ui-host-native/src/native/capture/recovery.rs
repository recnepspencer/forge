use super::port::UiNativeCaptureReadbackPoll;
use super::readback::UiNativeReadbackLayout;
use super::state::{release_owners, UiNativeCaptureState, UiNativeRecoveringCapture};

impl UiNativeCaptureState {
    pub(super) fn retain_recovery(
        &mut self,
        layout: UiNativeReadbackLayout,
        readback: Box<dyn super::port::UiNativePendingCaptureReadback>,
        owners: Vec<crate::native::UiNativeResourceOwner>,
    ) {
        self.recovering.push(UiNativeRecoveringCapture {
            layout,
            readback,
            owners,
        });
    }

    pub(super) fn progress_recovery(
        &mut self,
        graphics: Option<&crate::native::UiNativePresentationAccess>,
        resources: &mut crate::native::UiNativeResourceRegistry,
    ) {
        let recovering = std::mem::take(&mut self.recovering);
        for recovery in recovering {
            match recovery.readback.poll_recovery(graphics) {
                UiNativeCaptureReadbackPoll::Captured(_)
                | UiNativeCaptureReadbackPoll::ArtifactIndeterminate => {
                    release_owners(resources, recovery.owners);
                    self.release_bytes(recovery.layout);
                }
                UiNativeCaptureReadbackPoll::Pending(readback)
                | UiNativeCaptureReadbackPoll::PhysicalCompletionIndeterminate(readback) => {
                    self.recovering.push(UiNativeRecoveringCapture {
                        readback,
                        ..recovery
                    });
                }
            }
        }
    }
}

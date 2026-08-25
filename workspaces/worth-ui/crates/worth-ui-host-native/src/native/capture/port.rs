use super::readback::{UiNativeReadback, UiNativeReadbackLayout, UiNativeReadbackPoll};

pub(super) trait UiNativeCaptureReadbackPort {
    fn begin(
        &mut self,
        graphics: Option<&crate::native::UiNativePresentationAccess>,
        layout: UiNativeReadbackLayout,
    ) -> Result<Box<dyn UiNativePendingCaptureReadback>, ()>;
}

pub(super) trait UiNativePendingCaptureReadback {
    fn poll(
        self: Box<Self>,
        graphics: Option<&crate::native::UiNativePresentationAccess>,
    ) -> UiNativeCaptureReadbackPoll;

    fn poll_recovery(
        self: Box<Self>,
        graphics: Option<&crate::native::UiNativePresentationAccess>,
    ) -> UiNativeCaptureReadbackPoll;
}

pub(super) enum UiNativeCaptureReadbackPoll {
    Pending(Box<dyn UiNativePendingCaptureReadback>),
    Captured(Box<[u8]>),
    ArtifactIndeterminate,
    PhysicalCompletionIndeterminate(Box<dyn UiNativePendingCaptureReadback>),
}

pub(super) struct UiWgpuNativeCaptureReadbackPort;

struct UiWgpuNativePendingCaptureReadback {
    readback: UiNativeReadback,
    device_generation: std::sync::Arc<crate::native::UiNativeDeviceGeneration>,
}

impl UiNativeCaptureReadbackPort for UiWgpuNativeCaptureReadbackPort {
    fn begin(
        &mut self,
        graphics: Option<&crate::native::UiNativePresentationAccess>,
        layout: UiNativeReadbackLayout,
    ) -> Result<Box<dyn UiNativePendingCaptureReadback>, ()> {
        let graphics = graphics.ok_or(())?;
        Ok(Box::new(UiWgpuNativePendingCaptureReadback {
            readback: UiNativeReadback::begin(
                graphics.device(),
                graphics.queue(),
                graphics.retained_target(),
                layout,
            ),
            device_generation: graphics.device_generation(),
        }))
    }
}

impl UiNativePendingCaptureReadback for UiWgpuNativePendingCaptureReadback {
    fn poll(
        self: Box<Self>,
        graphics: Option<&crate::native::UiNativePresentationAccess>,
    ) -> UiNativeCaptureReadbackPoll {
        let _ = graphics;
        match self.readback.poll(self.device_generation.device()) {
            UiNativeReadbackPoll::Pending(readback) => {
                UiNativeCaptureReadbackPoll::Pending(Box::new(UiWgpuNativePendingCaptureReadback {
                    readback,
                    device_generation: self.device_generation,
                }))
            }
            UiNativeReadbackPoll::Captured(bytes) => UiNativeCaptureReadbackPoll::Captured(bytes),
            UiNativeReadbackPoll::ArtifactIndeterminate => {
                UiNativeCaptureReadbackPoll::ArtifactIndeterminate
            }
            UiNativeReadbackPoll::PhysicalCompletionIndeterminate(readback) => {
                UiNativeCaptureReadbackPoll::PhysicalCompletionIndeterminate(Box::new(Self {
                    readback,
                    device_generation: self.device_generation,
                }))
            }
        }
    }

    fn poll_recovery(
        self: Box<Self>,
        graphics: Option<&crate::native::UiNativePresentationAccess>,
    ) -> UiNativeCaptureReadbackPoll {
        let _ = graphics;
        match self.readback.poll_recovery(self.device_generation.device()) {
            UiNativeReadbackPoll::Pending(readback) => {
                UiNativeCaptureReadbackPoll::Pending(Box::new(UiWgpuNativePendingCaptureReadback {
                    readback,
                    device_generation: self.device_generation,
                }))
            }
            UiNativeReadbackPoll::Captured(bytes) => UiNativeCaptureReadbackPoll::Captured(bytes),
            UiNativeReadbackPoll::ArtifactIndeterminate => {
                UiNativeCaptureReadbackPoll::ArtifactIndeterminate
            }
            UiNativeReadbackPoll::PhysicalCompletionIndeterminate(readback) => {
                UiNativeCaptureReadbackPoll::PhysicalCompletionIndeterminate(Box::new(Self {
                    readback,
                    device_generation: self.device_generation,
                }))
            }
        }
    }
}

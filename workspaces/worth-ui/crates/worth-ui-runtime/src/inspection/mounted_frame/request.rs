use worth_ui_host_contract::{UiMountedFrameIdentity, UiMountedInstanceIdentity};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiMountedInspectionRequest {
    target: UiMountedInspectionTarget,
    instance: Option<UiMountedInstanceIdentity>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiMountedInspectionTarget {
    Current,
    Frame(UiMountedFrameIdentity),
}

impl UiMountedInspectionRequest {
    pub const fn current() -> Self {
        Self {
            target: UiMountedInspectionTarget::Current,
            instance: None,
        }
    }

    pub const fn frame(frame: UiMountedFrameIdentity) -> Self {
        Self {
            target: UiMountedInspectionTarget::Frame(frame),
            instance: None,
        }
    }

    pub const fn for_instance(mut self, instance: UiMountedInstanceIdentity) -> Self {
        self.instance = Some(instance);
        self
    }

    pub const fn target(self) -> UiMountedInspectionTarget {
        self.target
    }

    pub const fn instance(self) -> Option<UiMountedInstanceIdentity> {
        self.instance
    }

    pub(crate) fn into_selection(self) -> crate::mounting::UiMountedFrameInspectionSelection {
        crate::mounting::UiMountedFrameInspectionSelection {
            target: match self.target {
                UiMountedInspectionTarget::Current => {
                    crate::mounting::UiMountedFrameInspectionTarget::Current
                }
                UiMountedInspectionTarget::Frame(frame) => {
                    crate::mounting::UiMountedFrameInspectionTarget::Frame(frame)
                }
            },
            instance: self.instance,
        }
    }
}

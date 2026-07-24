use crate::runtime::{
    WorthUiChildRangeHandle, WorthUiCommandHandle, WorthUiComponentHandle, WorthUiStateSlotHandle,
    WorthUiTokenHandle,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiOrdinaryFrameTarget {
    kind: WorthUiOrdinaryFrameTargetKind,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum WorthUiOrdinaryFrameTargetKind {
    RootShell,
    Component(WorthUiComponentHandle),
    ChildRange(WorthUiChildRangeHandle),
    Command(WorthUiCommandHandle),
    TokenSupport(WorthUiTokenHandle),
    StateSlot(WorthUiStateSlotHandle),
}

impl WorthUiOrdinaryFrameTarget {
    pub fn root_shell() -> Self {
        Self {
            kind: WorthUiOrdinaryFrameTargetKind::RootShell,
        }
    }

    pub fn component(handle: WorthUiComponentHandle) -> Self {
        Self {
            kind: WorthUiOrdinaryFrameTargetKind::Component(handle),
        }
    }

    pub fn command(handle: WorthUiCommandHandle) -> Self {
        Self {
            kind: WorthUiOrdinaryFrameTargetKind::Command(handle),
        }
    }

    pub fn child_range(handle: WorthUiChildRangeHandle) -> Self {
        Self {
            kind: WorthUiOrdinaryFrameTargetKind::ChildRange(handle),
        }
    }

    pub fn token_support(handle: WorthUiTokenHandle) -> Self {
        Self {
            kind: WorthUiOrdinaryFrameTargetKind::TokenSupport(handle),
        }
    }

    pub fn state_slot(handle: WorthUiStateSlotHandle) -> Self {
        Self {
            kind: WorthUiOrdinaryFrameTargetKind::StateSlot(handle),
        }
    }

    pub(crate) fn kind(self) -> WorthUiOrdinaryFrameTargetKind {
        self.kind
    }

    pub const fn is_command(self) -> bool {
        matches!(self.kind, WorthUiOrdinaryFrameTargetKind::Command(_))
    }
}

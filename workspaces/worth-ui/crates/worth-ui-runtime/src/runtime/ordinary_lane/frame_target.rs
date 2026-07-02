use crate::runtime::{WorthUiCommandHandle, WorthUiComponentHandle, WorthUiTokenHandle};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiOrdinaryFrameTarget {
    kind: WorthUiOrdinaryFrameTargetKind,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum WorthUiOrdinaryFrameTargetKind {
    RootShell,
    Component(WorthUiComponentHandle),
    Command(WorthUiCommandHandle),
    TokenSupport(WorthUiTokenHandle),
    #[cfg(test)]
    VirtualizedData(u32),
    #[cfg(test)]
    CanvasSpatial(u32),
    #[cfg(test)]
    RealtimeOverlay(u32),
    #[cfg(test)]
    ParseSourceForTest,
    #[cfg(test)]
    RegistryLookupForTest,
    #[cfg(test)]
    ArtifactScanForTest,
    #[cfg(test)]
    FullPlanScanForTest,
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

    pub fn token_support(handle: WorthUiTokenHandle) -> Self {
        Self {
            kind: WorthUiOrdinaryFrameTargetKind::TokenSupport(handle),
        }
    }

    pub(crate) fn kind(self) -> WorthUiOrdinaryFrameTargetKind {
        self.kind
    }

    pub(crate) const fn is_command(self) -> bool {
        matches!(self.kind, WorthUiOrdinaryFrameTargetKind::Command(_))
    }

    #[cfg(test)]
    pub(crate) fn virtualized_data_for_test(plan_index: u32) -> Self {
        Self {
            kind: WorthUiOrdinaryFrameTargetKind::VirtualizedData(plan_index),
        }
    }

    #[cfg(test)]
    pub(crate) fn canvas_spatial_for_test(plan_index: u32) -> Self {
        Self {
            kind: WorthUiOrdinaryFrameTargetKind::CanvasSpatial(plan_index),
        }
    }

    #[cfg(test)]
    pub(crate) fn realtime_overlay_for_test(plan_index: u32) -> Self {
        Self {
            kind: WorthUiOrdinaryFrameTargetKind::RealtimeOverlay(plan_index),
        }
    }

    #[cfg(test)]
    pub(crate) fn parse_source_for_test() -> Self {
        Self {
            kind: WorthUiOrdinaryFrameTargetKind::ParseSourceForTest,
        }
    }

    #[cfg(test)]
    pub(crate) fn registry_lookup_for_test() -> Self {
        Self {
            kind: WorthUiOrdinaryFrameTargetKind::RegistryLookupForTest,
        }
    }

    #[cfg(test)]
    pub(crate) fn artifact_scan_for_test() -> Self {
        Self {
            kind: WorthUiOrdinaryFrameTargetKind::ArtifactScanForTest,
        }
    }

    #[cfg(test)]
    pub(crate) fn full_plan_scan_for_test() -> Self {
        Self {
            kind: WorthUiOrdinaryFrameTargetKind::FullPlanScanForTest,
        }
    }
}

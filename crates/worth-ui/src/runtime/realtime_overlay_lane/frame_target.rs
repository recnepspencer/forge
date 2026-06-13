#[cfg(test)]
use crate::runtime::WorthUiComponentHandle;
use crate::runtime::WorthUiRendererSurfaceHandle;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiRealtimeFrameTarget {
    kind: WorthUiRealtimeFrameTargetKind,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum WorthUiRealtimeFrameTargetKind {
    RendererSurface(WorthUiRendererSurfaceHandle),
    #[cfg(test)]
    OrdinaryWidgetFallback(WorthUiComponentHandle),
    #[cfg(test)]
    HiddenOrdinaryLayoutPass(WorthUiRendererSurfaceHandle),
    #[cfg(test)]
    ForbiddenWorkSuppression(WorthUiRendererSurfaceHandle),
}

impl WorthUiRealtimeFrameTarget {
    pub fn renderer_surface(handle: WorthUiRendererSurfaceHandle) -> Self {
        Self {
            kind: WorthUiRealtimeFrameTargetKind::RendererSurface(handle),
        }
    }

    pub(crate) fn kind(self) -> WorthUiRealtimeFrameTargetKind {
        self.kind
    }

    #[cfg(test)]
    pub(crate) fn ordinary_widget_fallback_for_test(handle: WorthUiComponentHandle) -> Self {
        Self {
            kind: WorthUiRealtimeFrameTargetKind::OrdinaryWidgetFallback(handle),
        }
    }

    #[cfg(test)]
    pub(crate) fn hidden_ordinary_layout_pass_for_test(
        handle: WorthUiRendererSurfaceHandle,
    ) -> Self {
        Self {
            kind: WorthUiRealtimeFrameTargetKind::HiddenOrdinaryLayoutPass(handle),
        }
    }

    #[cfg(test)]
    pub(crate) fn forbidden_work_suppression_for_test(
        handle: WorthUiRendererSurfaceHandle,
    ) -> Self {
        Self {
            kind: WorthUiRealtimeFrameTargetKind::ForbiddenWorkSuppression(handle),
        }
    }
}

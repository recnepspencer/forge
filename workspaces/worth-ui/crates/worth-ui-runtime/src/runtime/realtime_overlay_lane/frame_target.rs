use crate::runtime::WorthUiRendererSurfaceHandle;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiRealtimeFrameTarget {
    kind: WorthUiRealtimeFrameTargetKind,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum WorthUiRealtimeFrameTargetKind {
    RendererSurface(WorthUiRendererSurfaceHandle),
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
}

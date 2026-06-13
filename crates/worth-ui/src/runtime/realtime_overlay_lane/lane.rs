#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiRealtimeOverlayLane {
    HudOverlay,
    RendererSurfaceHandoff,
}

impl WorthUiRealtimeOverlayLane {
    pub fn canonical_tag(self) -> u64 {
        match self {
            Self::HudOverlay => 1,
            Self::RendererSurfaceHandoff => 2,
        }
    }
}

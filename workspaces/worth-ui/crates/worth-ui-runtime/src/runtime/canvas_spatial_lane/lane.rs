#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum WorthUiCanvasSpatialLane {
    ViewportTransform,
    Draw,
    HitTest,
    Overlay,
    ToolState,
}

impl WorthUiCanvasSpatialLane {
    pub fn canonical_tag(self) -> u64 {
        match self {
            Self::ViewportTransform => 0x4353_5650_545f_3031,
            Self::Draw => 0x4353_4441_575f_3031,
            Self::HitTest => 0x4353_4849_545f_3031,
            Self::Overlay => 0x4353_4f56_455f_3031,
            Self::ToolState => 0x4353_544f_4f4c_3031,
        }
    }
}

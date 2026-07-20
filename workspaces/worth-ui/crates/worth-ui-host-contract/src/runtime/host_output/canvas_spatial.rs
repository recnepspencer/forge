#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiCanvasSpatialHostOutput {
    target: WorthUiCanvasSpatialHostOutputTarget,
    visible_primitive_count: u32,
    hit_test_region_count: u32,
    overlay_row_count: u16,
    tool_state_row_count: u16,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiCanvasSpatialHostOutputTarget {
    Viewport,
    Draw,
    HitTest,
    Overlay,
    ToolState,
}

impl WorthUiCanvasSpatialHostOutput {
    pub fn new(
        target: WorthUiCanvasSpatialHostOutputTarget,
        visible_primitive_count: u32,
        hit_test_region_count: u32,
        overlay_row_count: u16,
        tool_state_row_count: u16,
    ) -> Self {
        Self {
            target,
            visible_primitive_count,
            hit_test_region_count,
            overlay_row_count,
            tool_state_row_count,
        }
    }

    pub fn target(self) -> WorthUiCanvasSpatialHostOutputTarget {
        self.target
    }

    pub fn visible_primitive_count(self) -> u32 {
        self.visible_primitive_count
    }

    pub fn hit_test_region_count(self) -> u32 {
        self.hit_test_region_count
    }

    pub fn overlay_row_count(self) -> u16 {
        self.overlay_row_count
    }

    pub fn tool_state_row_count(self) -> u16 {
        self.tool_state_row_count
    }

    pub fn meaning_digest(self) -> u64 {
        u64::from(self.target as u8)
            ^ u64::from(self.visible_primitive_count).rotate_left(11)
            ^ u64::from(self.hit_test_region_count).rotate_left(23)
            ^ u64::from(self.overlay_row_count).rotate_left(37)
            ^ u64::from(self.tool_state_row_count).rotate_left(49)
    }
}

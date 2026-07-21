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
    Viewport {
        pan_delta_x: i32,
        pan_delta_y: i32,
        zoom_milli_factor: u32,
    },
    Draw,
    HitTest {
        viewport_x: i32,
        viewport_y: i32,
    },
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
        canvas_target_meaning_digest(self.target)
            ^ u64::from(self.visible_primitive_count).rotate_left(11)
            ^ u64::from(self.hit_test_region_count).rotate_left(23)
            ^ u64::from(self.overlay_row_count).rotate_left(37)
            ^ u64::from(self.tool_state_row_count).rotate_left(49)
    }
}

fn canvas_target_meaning_digest(target: WorthUiCanvasSpatialHostOutputTarget) -> u64 {
    match target {
        WorthUiCanvasSpatialHostOutputTarget::Viewport {
            pan_delta_x,
            pan_delta_y,
            zoom_milli_factor,
        } => {
            1_u64
                ^ (pan_delta_x as u32 as u64).rotate_left(7)
                ^ (pan_delta_y as u32 as u64).rotate_left(23)
                ^ u64::from(zoom_milli_factor).rotate_left(41)
        }
        WorthUiCanvasSpatialHostOutputTarget::Draw => 2,
        WorthUiCanvasSpatialHostOutputTarget::HitTest {
            viewport_x,
            viewport_y,
        } => {
            3_u64
                ^ (viewport_x as u32 as u64).rotate_left(11)
                ^ (viewport_y as u32 as u64).rotate_left(37)
        }
        WorthUiCanvasSpatialHostOutputTarget::Overlay => 4,
        WorthUiCanvasSpatialHostOutputTarget::ToolState => 5,
    }
}

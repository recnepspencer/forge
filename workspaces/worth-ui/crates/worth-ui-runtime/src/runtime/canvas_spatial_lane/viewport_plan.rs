use crate::runtime::WorthUiLaneHandle;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiCanvasViewportPlanDenialReason {
    ZeroZoomFactor,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiCanvasViewportPlanDenial {
    reason: WorthUiCanvasViewportPlanDenialReason,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiCanvasViewportPlan {
    lane_handle: WorthUiLaneHandle,
    pan_delta_x: i32,
    pan_delta_y: i32,
    zoom_milli_factor: u32,
}

impl WorthUiCanvasViewportPlan {
    pub fn pan_zoom(
        lane_handle: WorthUiLaneHandle,
        pan_delta_x: i32,
        pan_delta_y: i32,
        zoom_milli_factor: u32,
    ) -> Result<Self, WorthUiCanvasViewportPlanDenial> {
        if zoom_milli_factor == 0 {
            return Err(WorthUiCanvasViewportPlanDenial::new(
                WorthUiCanvasViewportPlanDenialReason::ZeroZoomFactor,
            ));
        }

        Ok(Self {
            lane_handle,
            pan_delta_x,
            pan_delta_y,
            zoom_milli_factor,
        })
    }

    pub fn lane_handle(self) -> WorthUiLaneHandle {
        self.lane_handle
    }

    pub fn pan_delta_x(self) -> i32 {
        self.pan_delta_x
    }

    pub fn pan_delta_y(self) -> i32 {
        self.pan_delta_y
    }

    pub fn zoom_milli_factor(self) -> u32 {
        self.zoom_milli_factor
    }
}

impl WorthUiCanvasViewportPlanDenial {
    fn new(reason: WorthUiCanvasViewportPlanDenialReason) -> Self {
        Self { reason }
    }

    pub fn reason(self) -> WorthUiCanvasViewportPlanDenialReason {
        self.reason
    }
}

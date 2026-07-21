use crate::runtime::WorthUiLaneHandle;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiCanvasViewportRequestDenialReason {
    ZeroZoomFactor,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiCanvasViewportRequestDenial {
    reason: WorthUiCanvasViewportRequestDenialReason,
}

/// Per-frame viewport input for an already admitted canvas lane.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiCanvasViewportRequest {
    lane_handle: WorthUiLaneHandle,
    pan_delta_x: i32,
    pan_delta_y: i32,
    zoom_milli_factor: u32,
}

impl WorthUiCanvasViewportRequest {
    pub fn pan_zoom(
        lane_handle: WorthUiLaneHandle,
        pan_delta_x: i32,
        pan_delta_y: i32,
        zoom_milli_factor: u32,
    ) -> Result<Self, WorthUiCanvasViewportRequestDenial> {
        if zoom_milli_factor == 0 {
            return Err(WorthUiCanvasViewportRequestDenial::new(
                WorthUiCanvasViewportRequestDenialReason::ZeroZoomFactor,
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

impl WorthUiCanvasViewportRequestDenial {
    fn new(reason: WorthUiCanvasViewportRequestDenialReason) -> Self {
        Self { reason }
    }

    pub fn reason(self) -> WorthUiCanvasViewportRequestDenialReason {
        self.reason
    }
}

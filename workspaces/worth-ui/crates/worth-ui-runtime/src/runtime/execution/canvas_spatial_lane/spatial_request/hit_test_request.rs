use crate::runtime::{WorthUiLaneHandle, WorthUiSpatialViewportPoint};

/// Per-frame hit-test input for an already admitted canvas lane.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiSpatialHitTestRequest {
    lane_handle: WorthUiLaneHandle,
    viewport_point: WorthUiSpatialViewportPoint,
}

impl WorthUiSpatialHitTestRequest {
    pub fn for_viewport_point(
        lane_handle: WorthUiLaneHandle,
        viewport_point: WorthUiSpatialViewportPoint,
    ) -> Self {
        Self {
            lane_handle,
            viewport_point,
        }
    }

    pub fn lane_handle(self) -> WorthUiLaneHandle {
        self.lane_handle
    }

    pub fn viewport_point(self) -> WorthUiSpatialViewportPoint {
        self.viewport_point
    }
}

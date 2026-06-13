use crate::runtime::WorthUiLaneHandle;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiCanvasOverlayPlan {
    lane_handle: WorthUiLaneHandle,
}

impl WorthUiCanvasOverlayPlan {
    pub fn for_lane(lane_handle: WorthUiLaneHandle) -> Self {
        Self { lane_handle }
    }

    pub fn lane_handle(self) -> WorthUiLaneHandle {
        self.lane_handle
    }
}

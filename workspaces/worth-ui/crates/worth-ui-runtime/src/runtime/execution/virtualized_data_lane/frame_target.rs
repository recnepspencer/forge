use crate::runtime::{WorthUiViewBindingHandle, WorthUiVisibleRange};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiVirtualizedDataFrameTarget {
    handle: WorthUiViewBindingHandle,
    visible_range: WorthUiVisibleRange,
}

impl WorthUiVirtualizedDataFrameTarget {
    pub fn view_binding(handle: WorthUiViewBindingHandle, range: WorthUiVisibleRange) -> Self {
        Self {
            handle,
            visible_range: range,
        }
    }

    pub fn handle(self) -> WorthUiViewBindingHandle {
        self.handle
    }

    pub fn visible_range(self) -> WorthUiVisibleRange {
        self.visible_range
    }
}

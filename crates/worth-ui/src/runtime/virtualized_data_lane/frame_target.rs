#[cfg(test)]
use crate::runtime::WorthUiComponentHandle;
use crate::runtime::{WorthUiViewBindingHandle, WorthUiVisibleRange};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiVirtualizedDataFrameTarget {
    kind: WorthUiVirtualizedDataFrameTargetKind,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum WorthUiVirtualizedDataFrameTargetKind {
    ViewBinding(WorthUiViewBindingHandle, WorthUiVisibleRange),
    #[cfg(test)]
    FullCollectionScan(WorthUiViewBindingHandle),
    #[cfg(test)]
    OffsetPagination(WorthUiViewBindingHandle),
    #[cfg(test)]
    Component(WorthUiComponentHandle),
}

impl WorthUiVirtualizedDataFrameTarget {
    pub fn view_binding(handle: WorthUiViewBindingHandle, range: WorthUiVisibleRange) -> Self {
        Self {
            kind: WorthUiVirtualizedDataFrameTargetKind::ViewBinding(handle, range),
        }
    }

    pub(crate) fn kind(self) -> WorthUiVirtualizedDataFrameTargetKind {
        self.kind
    }

    #[cfg(test)]
    pub(crate) fn full_collection_scan_for_test(handle: WorthUiViewBindingHandle) -> Self {
        Self {
            kind: WorthUiVirtualizedDataFrameTargetKind::FullCollectionScan(handle),
        }
    }

    #[cfg(test)]
    pub(crate) fn offset_pagination_for_test(handle: WorthUiViewBindingHandle) -> Self {
        Self {
            kind: WorthUiVirtualizedDataFrameTargetKind::OffsetPagination(handle),
        }
    }

    #[cfg(test)]
    pub(crate) fn component_for_test(handle: WorthUiComponentHandle) -> Self {
        Self {
            kind: WorthUiVirtualizedDataFrameTargetKind::Component(handle),
        }
    }
}

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

    pub fn digest_basis(self) -> String {
        match self.kind {
            WorthUiVirtualizedDataFrameTargetKind::ViewBinding(handle, range) => {
                format!(
                    "view_binding:{}:{}:{}:{}:{}:{}",
                    handle.plan_index(),
                    handle.plan_generation().as_u64(),
                    range.start_row(),
                    range.row_count(),
                    range.start_column(),
                    range.column_count()
                )
            }
            #[cfg(test)]
            WorthUiVirtualizedDataFrameTargetKind::FullCollectionScan(handle) => {
                format!(
                    "full_collection_scan:{}:{}",
                    handle.plan_index(),
                    handle.plan_generation().as_u64()
                )
            }
            #[cfg(test)]
            WorthUiVirtualizedDataFrameTargetKind::OffsetPagination(handle) => {
                format!(
                    "offset_pagination:{}:{}",
                    handle.plan_index(),
                    handle.plan_generation().as_u64()
                )
            }
            #[cfg(test)]
            WorthUiVirtualizedDataFrameTargetKind::Component(handle) => {
                format!(
                    "component:{}:{}",
                    handle.plan_index(),
                    handle.plan_generation().as_u64()
                )
            }
        }
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

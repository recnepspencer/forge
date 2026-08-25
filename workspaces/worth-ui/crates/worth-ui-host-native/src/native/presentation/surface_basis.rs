#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiNativeSurfaceBasisDisposition {
    Unchanged,
    Suspend,
    Replace,
}

pub(crate) fn classify(
    current_scale: f64,
    current_extent: [u32; 2],
    successor_scale: f64,
    successor_extent: [u32; 2],
) -> UiNativeSurfaceBasisDisposition {
    if current_scale == successor_scale && current_extent == successor_extent {
        UiNativeSurfaceBasisDisposition::Unchanged
    } else if successor_extent.contains(&0) {
        UiNativeSurfaceBasisDisposition::Suspend
    } else {
        UiNativeSurfaceBasisDisposition::Replace
    }
}

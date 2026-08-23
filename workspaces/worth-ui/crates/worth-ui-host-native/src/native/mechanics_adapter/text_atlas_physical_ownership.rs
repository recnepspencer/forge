use crate::native::text_atlas::{UiNativeGpuAtlasKind, UiNativeTextAtlasGpuPages};

pub(super) fn physical_ownership_matches(
    expected: (usize, usize),
    gpu: &UiNativeTextAtlasGpuPages,
    resources: &crate::native::UiNativeResourceRegistry,
) -> bool {
    let actual = (
        gpu.page_count(UiNativeGpuAtlasKind::Alpha),
        gpu.page_count(UiNativeGpuAtlasKind::Color),
    );
    let registered = resources.current();
    physical_ownership_counts_match(
        expected,
        actual,
        (registered.alpha_atlas_pages, registered.color_atlas_pages),
    )
}

pub(super) fn physical_ownership_counts_match(
    expected: (usize, usize),
    actual: (usize, usize),
    registered: (usize, usize),
) -> bool {
    expected == actual && actual == registered
}

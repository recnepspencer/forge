#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiNativeClientResourceObservation {
    peak_mounted_layouts: usize,
    peak_raster_cache_entries: usize,
    terminal_mounted_layouts: usize,
    terminal_raster_cache_entries: usize,
}

impl UiNativeClientResourceObservation {
    #[doc(hidden)]
    pub const fn reported(
        peak_mounted_layouts: usize,
        peak_raster_cache_entries: usize,
        terminal_mounted_layouts: usize,
        terminal_raster_cache_entries: usize,
    ) -> Self {
        Self {
            peak_mounted_layouts,
            peak_raster_cache_entries,
            terminal_mounted_layouts,
            terminal_raster_cache_entries,
        }
    }

    pub const fn peak_mounted_layouts(self) -> usize {
        self.peak_mounted_layouts
    }

    pub const fn peak_raster_cache_entries(self) -> usize {
        self.peak_raster_cache_entries
    }

    pub const fn terminal_mounted_layouts(self) -> usize {
        self.terminal_mounted_layouts
    }

    pub const fn terminal_raster_cache_entries(self) -> usize {
        self.terminal_raster_cache_entries
    }
}

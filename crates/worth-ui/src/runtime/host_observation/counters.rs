#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiHostObservationCounters {
    available_bounds_count: usize,
    viewport_count: usize,
    scroll_viewport_count: usize,
    text_metric_count: usize,
    icon_metric_count: usize,
    dpi_count: usize,
    elapsed_time_count: usize,
    source_reparse_count: usize,
    renderer_parse_count: usize,
}

impl WorthUiHostObservationCounters {
    pub(super) fn from_counts(
        bounds: usize,
        viewport: usize,
        scroll_viewport: usize,
        text_metric: usize,
        icon_metric: usize,
        dpi: usize,
        elapsed_time: usize,
    ) -> Self {
        Self {
            available_bounds_count: bounds,
            viewport_count: viewport,
            scroll_viewport_count: scroll_viewport,
            text_metric_count: text_metric,
            icon_metric_count: icon_metric,
            dpi_count: dpi,
            elapsed_time_count: elapsed_time,
            source_reparse_count: 0,
            renderer_parse_count: 0,
        }
    }

    pub fn available_bounds_count(self) -> usize {
        self.available_bounds_count
    }

    pub fn viewport_count(self) -> usize {
        self.viewport_count
    }

    pub fn scroll_viewport_count(self) -> usize {
        self.scroll_viewport_count
    }

    pub fn text_metric_count(self) -> usize {
        self.text_metric_count
    }

    pub fn icon_metric_count(self) -> usize {
        self.icon_metric_count
    }

    pub fn dpi_count(self) -> usize {
        self.dpi_count
    }

    pub fn elapsed_time_count(self) -> usize {
        self.elapsed_time_count
    }

    pub fn source_reparse_count(self) -> usize {
        self.source_reparse_count
    }

    pub fn renderer_parse_count(self) -> usize {
        self.renderer_parse_count
    }
}

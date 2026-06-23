#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiPrimitiveLayoutExecutionCounters {
    content_item_count: usize,
    layout_item_count: usize,
    source_parse_count: usize,
    artifact_scan_count: usize,
}

impl WorthUiPrimitiveLayoutExecutionCounters {
    pub(crate) fn new(
        content_item_count: usize,
        layout_item_count: usize,
        source_parse_count: usize,
        artifact_scan_count: usize,
    ) -> Self {
        Self {
            content_item_count,
            layout_item_count,
            source_parse_count,
            artifact_scan_count,
        }
    }

    pub fn content_item_count(&self) -> usize {
        self.content_item_count
    }

    pub fn layout_item_count(&self) -> usize {
        self.layout_item_count
    }

    pub fn source_parse_count(&self) -> usize {
        self.source_parse_count
    }

    pub fn artifact_scan_count(&self) -> usize {
        self.artifact_scan_count
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiLayoutAllocationCounters {
    container_count: usize,
    participating_child_count: usize,
    absent_child_count: usize,
    hug_child_count: usize,
    fill_child_count: usize,
    source_reparse_count: usize,
    renderer_parse_count: usize,
    artifact_scan_count: usize,
}

impl WorthUiLayoutAllocationCounters {
    pub(super) fn new(
        container_count: usize,
        participating_child_count: usize,
        absent_child_count: usize,
        hug_child_count: usize,
        fill_child_count: usize,
    ) -> Self {
        Self {
            container_count,
            participating_child_count,
            absent_child_count,
            hug_child_count,
            fill_child_count,
            source_reparse_count: 0,
            renderer_parse_count: 0,
            artifact_scan_count: 0,
        }
    }

    pub fn container_count(self) -> usize {
        self.container_count
    }

    pub fn participating_child_count(self) -> usize {
        self.participating_child_count
    }

    pub fn absent_child_count(self) -> usize {
        self.absent_child_count
    }

    pub fn hug_child_count(self) -> usize {
        self.hug_child_count
    }

    pub fn fill_child_count(self) -> usize {
        self.fill_child_count
    }

    pub fn source_reparse_count(self) -> usize {
        self.source_reparse_count
    }

    pub fn renderer_parse_count(self) -> usize {
        self.renderer_parse_count
    }

    pub fn artifact_scan_count(self) -> usize {
        self.artifact_scan_count
    }
}

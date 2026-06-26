#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadBypassRequiredRootCoverage {
    required_root: String,
    source_file_count: usize,
    audited_source_labels: Vec<String>,
}

impl WorthGraphReadBypassRequiredRootCoverage {
    pub(in crate::graph_read_access_inventory::inventory_lane) fn new(
        required_root: String,
        source_file_count: usize,
        audited_source_labels: Vec<String>,
    ) -> Self {
        Self {
            required_root,
            source_file_count,
            audited_source_labels,
        }
    }

    pub fn required_root(&self) -> &str {
        &self.required_root
    }

    pub const fn source_file_count(&self) -> usize {
        self.source_file_count
    }

    pub fn audited_source_labels(&self) -> &[String] {
        &self.audited_source_labels
    }

    pub const fn has_source_files(&self) -> bool {
        self.source_file_count > 0
    }
}

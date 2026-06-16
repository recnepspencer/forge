use super::WorthUiHeaderMenuGroup;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiHeaderFrameReceipt {
    groups: Vec<WorthUiHeaderMenuGroup>,
    projected_command_count: usize,
    source_parse_count: usize,
    registry_lookup_count: usize,
    artifact_tree_scan_count: usize,
}

impl WorthUiHeaderFrameReceipt {
    pub(crate) fn new(groups: Vec<WorthUiHeaderMenuGroup>, projected_command_count: usize) -> Self {
        Self {
            groups,
            projected_command_count,
            source_parse_count: 0,
            registry_lookup_count: 0,
            artifact_tree_scan_count: 0,
        }
    }

    pub fn groups(&self) -> &[WorthUiHeaderMenuGroup] {
        &self.groups
    }

    pub fn projected_command_count(&self) -> usize {
        self.projected_command_count
    }

    pub fn source_parse_count(&self) -> usize {
        self.source_parse_count
    }

    pub fn registry_lookup_count(&self) -> usize {
        self.registry_lookup_count
    }

    pub fn artifact_tree_scan_count(&self) -> usize {
        self.artifact_tree_scan_count
    }
}

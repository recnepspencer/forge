#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiLayoutAllocationRequest {
    root_node_id: String,
}

impl WorthUiLayoutAllocationRequest {
    pub fn for_root_node(root_node_id: impl Into<String>) -> Self {
        Self {
            root_node_id: root_node_id.into(),
        }
    }

    pub fn root_node_id(&self) -> &str {
        &self.root_node_id
    }
}

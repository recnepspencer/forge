#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiInspectionScope {
    Graph,
}

impl UiInspectionScope {
    pub fn graph() -> Self {
        Self::Graph
    }
}

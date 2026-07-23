#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiRendererResourceInvalidation {
    affected_resource_count: usize,
    ordinary_widget_subtrees_broadened: bool,
}

impl WorthUiRendererResourceInvalidation {
    pub(crate) fn narrowed_to_runtime_lane(affected_resource_count: usize) -> Self {
        Self {
            affected_resource_count,
            ordinary_widget_subtrees_broadened: false,
        }
    }

    pub fn affected_resource_count(&self) -> usize {
        self.affected_resource_count
    }

    pub fn ordinary_widget_subtrees_broadened(&self) -> bool {
        self.ordinary_widget_subtrees_broadened
    }
}

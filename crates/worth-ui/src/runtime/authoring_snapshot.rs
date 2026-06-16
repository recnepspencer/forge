use crate::source::{WorthUiContentSlotCatalog, WorthUiLayoutTopologyCatalog};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiRuntimeAuthoringSnapshot {
    layout_topology: WorthUiLayoutTopologyCatalog,
    content_slots: WorthUiContentSlotCatalog,
}

impl WorthUiRuntimeAuthoringSnapshot {
    pub(crate) fn new(
        layout_topology: WorthUiLayoutTopologyCatalog,
        content_slots: WorthUiContentSlotCatalog,
    ) -> Self {
        Self {
            layout_topology,
            content_slots,
        }
    }

    pub fn layout_topology(&self) -> &WorthUiLayoutTopologyCatalog {
        &self.layout_topology
    }

    pub fn content_slots(&self) -> &WorthUiContentSlotCatalog {
        &self.content_slots
    }
}

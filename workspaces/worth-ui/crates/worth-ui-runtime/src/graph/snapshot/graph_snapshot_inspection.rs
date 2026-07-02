use crate::graph::{UiGraphInspectionSupport, UiGraphSnapshot};

impl UiGraphSnapshot {
    pub fn inspection(&self) -> UiGraphInspectionSupport<'_> {
        UiGraphInspectionSupport::new(self)
    }
}

use crate::capability::ComponentId;

use super::WorthUiComponentCompatibility;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiComponentReloadReceipt {
    component_ids: Vec<ComponentId>,
    compatibility: WorthUiComponentCompatibility,
}

impl WorthUiComponentReloadReceipt {
    pub(crate) fn new(
        component_ids: Vec<ComponentId>,
        compatibility: WorthUiComponentCompatibility,
    ) -> Self {
        Self {
            component_ids,
            compatibility,
        }
    }

    pub fn component_ids(&self) -> &[ComponentId] {
        &self.component_ids
    }

    pub fn compatibility(&self) -> &WorthUiComponentCompatibility {
        &self.compatibility
    }
}

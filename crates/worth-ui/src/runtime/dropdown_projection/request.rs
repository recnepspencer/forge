use crate::capability::{CommandProjectionId, ComponentId};

use super::WorthUiDropdownAppearanceRequest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiDropdownProjectionRequest {
    projection_id: CommandProjectionId,
    single_select_component_id: ComponentId,
    multi_select_component_id: ComponentId,
    appearance_request: WorthUiDropdownAppearanceRequest,
}

impl WorthUiDropdownProjectionRequest {
    pub fn for_command_projection(
        projection_id: CommandProjectionId,
        single_select_component_id: ComponentId,
        multi_select_component_id: ComponentId,
        appearance_request: WorthUiDropdownAppearanceRequest,
    ) -> Self {
        Self {
            projection_id,
            single_select_component_id,
            multi_select_component_id,
            appearance_request,
        }
    }

    pub fn projection_id(&self) -> &CommandProjectionId {
        &self.projection_id
    }

    pub(crate) fn single_select_component_id(&self) -> &ComponentId {
        &self.single_select_component_id
    }

    pub(crate) fn multi_select_component_id(&self) -> &ComponentId {
        &self.multi_select_component_id
    }

    pub(crate) fn appearance_request(&self) -> &WorthUiDropdownAppearanceRequest {
        &self.appearance_request
    }
}

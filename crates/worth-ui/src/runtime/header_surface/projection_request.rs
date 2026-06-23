use crate::capability::{CommandProjectionId, ComponentId};
use crate::runtime::{WorthUiDropdownAppearanceRequest, WorthUiDropdownProjectionRequest};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiHeaderMenuProjectionRequest {
    title: String,
    projection_id: CommandProjectionId,
    single_select_component_id: ComponentId,
    multi_select_component_id: ComponentId,
}

impl WorthUiHeaderMenuProjectionRequest {
    pub fn new(
        title: impl Into<String>,
        projection_id: CommandProjectionId,
        single_select_component_id: ComponentId,
        multi_select_component_id: ComponentId,
    ) -> Self {
        Self {
            title: title.into(),
            projection_id,
            single_select_component_id,
            multi_select_component_id,
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn projection_id(&self) -> &CommandProjectionId {
        &self.projection_id
    }

    pub(crate) fn to_dropdown_request(
        &self,
        appearance_request: WorthUiDropdownAppearanceRequest,
    ) -> WorthUiDropdownProjectionRequest {
        WorthUiDropdownProjectionRequest::for_command_projection(
            self.projection_id.clone(),
            self.single_select_component_id.clone(),
            self.multi_select_component_id.clone(),
            appearance_request,
        )
    }
}

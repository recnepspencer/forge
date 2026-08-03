use super::{
    UiInstalledProjectionView, UiProjectionFieldRequirement, UiProjectionLifecycleRequirement,
    UiScalarSchemaRequirement,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiScalarProjectionRegistration {
    view: UiInstalledProjectionView,
    requirement: UiScalarSchemaRequirement,
}

impl UiScalarProjectionRegistration {
    pub fn text(
        view: UiInstalledProjectionView,
        selected_field: UiProjectionFieldRequirement,
    ) -> Self {
        Self::native(view, selected_field, super::UiProjectionNativeFamily::Text)
    }

    pub fn native(
        view: UiInstalledProjectionView,
        selected_field: UiProjectionFieldRequirement,
        native_family: super::UiProjectionNativeFamily,
    ) -> Self {
        Self {
            view,
            requirement: UiScalarSchemaRequirement::native(
                selected_field,
                native_family,
                UiProjectionLifecycleRequirement::Live,
            ),
        }
    }

    pub fn view(&self) -> &UiInstalledProjectionView {
        &self.view
    }

    pub fn requirement(&self) -> &UiScalarSchemaRequirement {
        &self.requirement
    }

    pub fn admit(
        self,
        workspace: &worth_query::facade::runtime::WorthQueryWorkspace,
    ) -> crate::UiScalarProjectionBindingAdmission {
        crate::projection_binding::admit_scalar_registration(self, workspace)
    }

    pub(crate) fn into_parts(self) -> (UiInstalledProjectionView, UiScalarSchemaRequirement) {
        (self.view, self.requirement)
    }
}

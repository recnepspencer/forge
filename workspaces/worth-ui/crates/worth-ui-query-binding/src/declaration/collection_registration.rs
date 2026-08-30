use super::{
    UiCollectionSchemaRequirement, UiCollectionSchemaRequirementError, UiInstalledProjectionView,
    UiProjectionFieldRequirement, UiProjectionLifecycleRequirement,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiCollectionProjectionRegistration {
    view: UiInstalledProjectionView,
    requirement: UiCollectionSchemaRequirement,
}

impl UiCollectionProjectionRegistration {
    pub fn text(
        view: UiInstalledProjectionView,
        row_identity_field: UiProjectionFieldRequirement,
        selected_fields: impl IntoIterator<Item = UiProjectionFieldRequirement>,
        requires_complete_result: bool,
        permits_continuation: bool,
    ) -> Result<Self, UiCollectionSchemaRequirementError> {
        Self::native(
            view,
            row_identity_field,
            selected_fields,
            super::UiProjectionNativeFamily::Text,
            requires_complete_result,
            permits_continuation,
        )
    }

    pub fn native(
        view: UiInstalledProjectionView,
        row_identity_field: UiProjectionFieldRequirement,
        selected_fields: impl IntoIterator<Item = UiProjectionFieldRequirement>,
        native_family: super::UiProjectionNativeFamily,
        requires_complete_result: bool,
        permits_continuation: bool,
    ) -> Result<Self, UiCollectionSchemaRequirementError> {
        Ok(Self {
            view,
            requirement: UiCollectionSchemaRequirement::native(
                row_identity_field,
                selected_fields,
                native_family,
                UiProjectionLifecycleRequirement::Live,
                requires_complete_result,
                permits_continuation,
            )?,
        })
    }

    pub fn view(&self) -> &UiInstalledProjectionView {
        &self.view
    }

    pub fn requirement(&self) -> &UiCollectionSchemaRequirement {
        &self.requirement
    }

    /// Declares one selected application field as the stable unsigned item
    /// key consumed by UI Selection. The field must already be part of this
    /// projection's selected payload.
    pub fn with_unsigned64_application_item_key_field(
        mut self,
        field: UiProjectionFieldRequirement,
    ) -> Self {
        self.requirement = self
            .requirement
            .with_unsigned64_application_item_key_field(field);
        self
    }

    pub fn admit(
        self,
        workspace: &worth_query::facade::runtime::WorthQueryWorkspace,
    ) -> crate::UiCollectionProjectionBindingAdmission {
        crate::projection_binding::admit_collection_registration(self, workspace)
    }

    pub(crate) fn into_parts(self) -> (UiInstalledProjectionView, UiCollectionSchemaRequirement) {
        (self.view, self.requirement)
    }
}

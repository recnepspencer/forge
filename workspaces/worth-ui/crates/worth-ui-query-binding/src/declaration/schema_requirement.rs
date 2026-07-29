use super::{
    UiProjectionFieldRequirement, UiProjectionLifecycleRequirement, UiProjectionNativeFamily,
    UiProjectionShape,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiScalarSchemaRequirement {
    selected_field: UiProjectionFieldRequirement,
    native_family: UiProjectionNativeFamily,
    lifecycle: UiProjectionLifecycleRequirement,
}

impl UiScalarSchemaRequirement {
    pub fn text(
        selected_field: UiProjectionFieldRequirement,
        lifecycle: UiProjectionLifecycleRequirement,
    ) -> Self {
        Self {
            selected_field,
            native_family: UiProjectionNativeFamily::Text,
            lifecycle,
        }
    }

    pub fn shape(&self) -> UiProjectionShape {
        UiProjectionShape::Scalar
    }

    pub fn selected_field(&self) -> &UiProjectionFieldRequirement {
        &self.selected_field
    }

    pub fn native_family(&self) -> UiProjectionNativeFamily {
        self.native_family
    }

    pub fn lifecycle(&self) -> UiProjectionLifecycleRequirement {
        self.lifecycle
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiCollectionSchemaRequirement {
    row_identity_field: UiProjectionFieldRequirement,
    selected_fields: Box<[UiProjectionFieldRequirement]>,
    native_family: UiProjectionNativeFamily,
    lifecycle: UiProjectionLifecycleRequirement,
    requires_complete_result: bool,
    permits_continuation: bool,
}

impl UiCollectionSchemaRequirement {
    pub fn text(
        row_identity_field: UiProjectionFieldRequirement,
        selected_fields: impl IntoIterator<Item = UiProjectionFieldRequirement>,
        lifecycle: UiProjectionLifecycleRequirement,
        requires_complete_result: bool,
        permits_continuation: bool,
    ) -> Self {
        Self {
            row_identity_field,
            selected_fields: selected_fields.into_iter().collect(),
            native_family: UiProjectionNativeFamily::Text,
            lifecycle,
            requires_complete_result,
            permits_continuation,
        }
    }

    pub fn shape(&self) -> UiProjectionShape {
        UiProjectionShape::Collection
    }

    pub fn row_identity_field(&self) -> &UiProjectionFieldRequirement {
        &self.row_identity_field
    }

    pub fn selected_fields(&self) -> &[UiProjectionFieldRequirement] {
        &self.selected_fields
    }

    pub fn native_family(&self) -> UiProjectionNativeFamily {
        self.native_family
    }

    pub fn lifecycle(&self) -> UiProjectionLifecycleRequirement {
        self.lifecycle
    }

    pub fn requires_complete_result(&self) -> bool {
        self.requires_complete_result
    }

    pub fn permits_continuation(&self) -> bool {
        self.permits_continuation
    }
}

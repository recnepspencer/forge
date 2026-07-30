use super::{
    UiProjectionFieldRequirement, UiProjectionLifecycleRequirement, UiProjectionNativeFamily,
    UiProjectionShape,
};
use std::collections::HashSet;

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
        Self::native(selected_field, UiProjectionNativeFamily::Text, lifecycle)
    }

    pub fn native(
        selected_field: UiProjectionFieldRequirement,
        native_family: UiProjectionNativeFamily,
        lifecycle: UiProjectionLifecycleRequirement,
    ) -> Self {
        Self {
            selected_field,
            native_family,
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiCollectionSchemaRequirementError {
    NoSelectedFields,
    DuplicateSelectedField,
}

impl UiCollectionSchemaRequirement {
    pub fn text(
        row_identity_field: UiProjectionFieldRequirement,
        selected_fields: impl IntoIterator<Item = UiProjectionFieldRequirement>,
        lifecycle: UiProjectionLifecycleRequirement,
        requires_complete_result: bool,
        permits_continuation: bool,
    ) -> Result<Self, UiCollectionSchemaRequirementError> {
        Self::native(
            row_identity_field,
            selected_fields,
            UiProjectionNativeFamily::Text,
            lifecycle,
            requires_complete_result,
            permits_continuation,
        )
    }

    pub fn native(
        row_identity_field: UiProjectionFieldRequirement,
        selected_fields: impl IntoIterator<Item = UiProjectionFieldRequirement>,
        native_family: UiProjectionNativeFamily,
        lifecycle: UiProjectionLifecycleRequirement,
        requires_complete_result: bool,
        permits_continuation: bool,
    ) -> Result<Self, UiCollectionSchemaRequirementError> {
        let selected_fields: Box<[_]> = selected_fields.into_iter().collect();
        if selected_fields.is_empty() {
            return Err(UiCollectionSchemaRequirementError::NoSelectedFields);
        }
        let mut distinct_fields = HashSet::with_capacity(selected_fields.len());
        if selected_fields
            .iter()
            .any(|field| !distinct_fields.insert(field.declared_name()))
        {
            return Err(UiCollectionSchemaRequirementError::DuplicateSelectedField);
        }
        Ok(Self {
            row_identity_field,
            selected_fields,
            native_family,
            lifecycle,
            requires_complete_result,
            permits_continuation,
        })
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

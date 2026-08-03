use super::{
    WorthUiProjectionCollectionSelection, WorthUiProjectionDeclarationError,
    WorthUiProjectionLifecycle, WorthUiProjectionNativeFamily, WorthUiProjectionRequirement,
    WorthUiProjectionRequirementParts, WorthUiProjectionShape,
};

impl WorthUiProjectionRequirement {
    pub fn scalar_text(
        declaration_identity: impl Into<String>,
        view_identity: impl Into<String>,
        selected_field: impl Into<String>,
        lifecycle: WorthUiProjectionLifecycle,
    ) -> Result<Self, WorthUiProjectionDeclarationError> {
        Self::scalar_native(
            declaration_identity,
            view_identity,
            selected_field,
            WorthUiProjectionNativeFamily::Text,
            lifecycle,
        )
    }

    pub fn scalar_native(
        declaration_identity: impl Into<String>,
        view_identity: impl Into<String>,
        selected_field: impl Into<String>,
        native_family: WorthUiProjectionNativeFamily,
        lifecycle: WorthUiProjectionLifecycle,
    ) -> Result<Self, WorthUiProjectionDeclarationError> {
        Self::build(WorthUiProjectionRequirementParts {
            declaration_identity: declaration_identity.into(),
            view_identity: view_identity.into(),
            shape: WorthUiProjectionShape::Scalar,
            selected_fields: vec![selected_field.into()],
            row_identity_field: None,
            native_family,
            lifecycle,
            collection_policy: None,
        })
    }

    pub fn collection_text(
        declaration_identity: impl Into<String>,
        view_identity: impl Into<String>,
        row_identity_field: impl Into<String>,
        selection: WorthUiProjectionCollectionSelection,
    ) -> Result<Self, WorthUiProjectionDeclarationError> {
        Self::collection_native(
            declaration_identity,
            view_identity,
            row_identity_field,
            WorthUiProjectionNativeFamily::Text,
            selection,
        )
    }

    pub fn collection_native(
        declaration_identity: impl Into<String>,
        view_identity: impl Into<String>,
        row_identity_field: impl Into<String>,
        native_family: WorthUiProjectionNativeFamily,
        selection: WorthUiProjectionCollectionSelection,
    ) -> Result<Self, WorthUiProjectionDeclarationError> {
        Self::build(WorthUiProjectionRequirementParts {
            declaration_identity: declaration_identity.into(),
            view_identity: view_identity.into(),
            shape: WorthUiProjectionShape::Collection,
            selected_fields: selection.selected_fields,
            row_identity_field: Some(row_identity_field.into()),
            native_family,
            lifecycle: selection.lifecycle,
            collection_policy: Some(selection.policy),
        })
    }
}

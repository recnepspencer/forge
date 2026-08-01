use crate::source::{
    WorthUiArtifactInputBodyAtom, WorthUiProjectionCollectionSelection,
    WorthUiProjectionDeclarationError, WorthUiProjectionLifecycle, WorthUiProjectionNativeFamily,
    WorthUiProjectionRequirement,
};

impl super::WorthUiRustAuthoredArtifactInputModule {
    pub fn try_with_query_scalar_text(
        self,
        declaration_identity: impl Into<String>,
        view_identity: impl Into<String>,
        selected_field: impl Into<String>,
        lifecycle: WorthUiProjectionLifecycle,
    ) -> Result<Self, WorthUiProjectionDeclarationError> {
        self.try_with_query_scalar_native(
            declaration_identity,
            view_identity,
            selected_field,
            WorthUiProjectionNativeFamily::Text,
            lifecycle,
        )
    }

    pub fn try_with_query_scalar_native(
        mut self,
        declaration_identity: impl Into<String>,
        view_identity: impl Into<String>,
        selected_field: impl Into<String>,
        native_family: WorthUiProjectionNativeFamily,
        lifecycle: WorthUiProjectionLifecycle,
    ) -> Result<Self, WorthUiProjectionDeclarationError> {
        let requirement = WorthUiProjectionRequirement::scalar_native(
            declaration_identity,
            view_identity,
            selected_field,
            native_family,
            lifecycle,
        )?;
        self.declarations
            .push(super::WorthUiRustAuthoredDeclaration::QueryScalar {
                name_text: requirement.declaration_identity().to_owned(),
                body_atoms: projection_body_atoms(&requirement),
            });
        Ok(self)
    }

    pub fn try_with_query_collection_text(
        self,
        declaration_identity: impl Into<String>,
        view_identity: impl Into<String>,
        row_identity_field: impl Into<String>,
        selection: WorthUiProjectionCollectionSelection,
    ) -> Result<Self, WorthUiProjectionDeclarationError> {
        self.try_with_query_collection_native(
            declaration_identity,
            view_identity,
            row_identity_field,
            WorthUiProjectionNativeFamily::Text,
            selection,
        )
    }

    pub fn try_with_query_collection_native(
        mut self,
        declaration_identity: impl Into<String>,
        view_identity: impl Into<String>,
        row_identity_field: impl Into<String>,
        native_family: WorthUiProjectionNativeFamily,
        selection: WorthUiProjectionCollectionSelection,
    ) -> Result<Self, WorthUiProjectionDeclarationError> {
        let requirement = WorthUiProjectionRequirement::collection_native(
            declaration_identity,
            view_identity,
            row_identity_field,
            native_family,
            selection,
        )?;
        self.declarations
            .push(super::WorthUiRustAuthoredDeclaration::QueryCollection {
                name_text: requirement.declaration_identity().to_owned(),
                body_atoms: projection_body_atoms(&requirement),
            });
        Ok(self)
    }
}

fn projection_body_atoms(
    requirement: &WorthUiProjectionRequirement,
) -> Vec<WorthUiArtifactInputBodyAtom> {
    let mut atoms = Vec::new();
    push_clause(&mut atoms, "view", requirement.view_identity());
    if let Some(row_identity) = requirement.row_identity_field() {
        push_clause(&mut atoms, "row", row_identity);
    }
    for field in requirement.selected_fields() {
        push_clause(&mut atoms, "field", field);
    }
    let native_family = match requirement.native_family() {
        WorthUiProjectionNativeFamily::Text => "text",
        WorthUiProjectionNativeFamily::Boolean => "boolean",
    };
    push_clause(&mut atoms, "require", native_family);
    push_clause(
        &mut atoms,
        "lifecycle",
        requirement.lifecycle().canonical_token(),
    );
    if let Some(policy) = requirement.collection_policy() {
        push_clause(
            &mut atoms,
            "completeness",
            if policy.requires_complete_result() {
                "complete"
            } else {
                "partial"
            },
        );
        push_clause(
            &mut atoms,
            "continuation",
            if policy.permits_continuation() {
                "allowed"
            } else {
                "forbidden"
            },
        );
    }
    atoms
}

fn push_clause(atoms: &mut Vec<WorthUiArtifactInputBodyAtom>, clause: &str, value: &str) {
    atoms.push(WorthUiArtifactInputBodyAtom::Identifier(clause.to_owned()));
    atoms.push(WorthUiArtifactInputBodyAtom::Identifier(value.to_owned()));
}

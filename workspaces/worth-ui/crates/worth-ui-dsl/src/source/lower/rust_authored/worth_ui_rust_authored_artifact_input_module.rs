mod revision;

use crate::source::{
    WorthUiArtifactInputBodyAtom, WorthUiProjectionCollectionSelection,
    WorthUiProjectionDeclarationError, WorthUiProjectionLifecycle, WorthUiProjectionNativeFamily,
    WorthUiProjectionRequirement, WorthUiSemanticArtifactDeclaration,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorthUiRustAuthoredArtifactInputModule {
    relative_module_path: String,
    declarations: Vec<WorthUiRustAuthoredDeclaration>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum WorthUiRustAuthoredDeclaration {
    Import {
        target_module_path: String,
    },
    Component {
        name_text: String,
        authored_identity: Option<String>,
        body_atoms: Vec<WorthUiArtifactInputBodyAtom>,
    },
    Surface {
        name_text: String,
        authored_identity: Option<String>,
        body_atoms: Vec<WorthUiArtifactInputBodyAtom>,
    },
    Binding {
        name_text: String,
        authored_identity: Option<String>,
        body_atoms: Vec<WorthUiArtifactInputBodyAtom>,
    },
    QueryScalar {
        name_text: String,
        body_atoms: Vec<WorthUiArtifactInputBodyAtom>,
    },
    QueryCollection {
        name_text: String,
        body_atoms: Vec<WorthUiArtifactInputBodyAtom>,
    },
    Token {
        name_text: String,
        authored_identity: Option<String>,
        value_text: String,
    },
    SemanticArtifact(WorthUiSemanticArtifactDeclaration),
}

impl WorthUiRustAuthoredArtifactInputModule {
    pub fn new(relative_module_path: impl Into<String>) -> Self {
        Self {
            relative_module_path: relative_module_path.into(),
            declarations: Vec::new(),
        }
    }

    pub fn with_import(mut self, target_module_path: impl Into<String>) -> Self {
        self.declarations
            .push(WorthUiRustAuthoredDeclaration::Import {
                target_module_path: target_module_path.into(),
            });
        self
    }

    pub fn with_component(mut self, name_text: impl Into<String>) -> Self {
        self.declarations
            .push(WorthUiRustAuthoredDeclaration::Component {
                name_text: name_text.into(),
                authored_identity: None,
                body_atoms: Vec::new(),
            });
        self
    }

    pub fn with_component_authored_identity(
        mut self,
        name_text: impl Into<String>,
        authored_identity: impl Into<String>,
    ) -> Self {
        self.declarations
            .push(WorthUiRustAuthoredDeclaration::Component {
                name_text: name_text.into(),
                authored_identity: Some(authored_identity.into()),
                body_atoms: Vec::new(),
            });
        self
    }

    pub fn with_component_body_atoms(
        mut self,
        name_text: impl Into<String>,
        body_atoms: impl IntoIterator<Item = WorthUiArtifactInputBodyAtom>,
    ) -> Self {
        self.declarations
            .push(WorthUiRustAuthoredDeclaration::Component {
                name_text: name_text.into(),
                authored_identity: None,
                body_atoms: body_atoms.into_iter().collect(),
            });
        self
    }

    pub fn with_component_body_atoms_and_authored_identity(
        mut self,
        name_text: impl Into<String>,
        authored_identity: impl Into<String>,
        body_atoms: impl IntoIterator<Item = WorthUiArtifactInputBodyAtom>,
    ) -> Self {
        self.declarations
            .push(WorthUiRustAuthoredDeclaration::Component {
                name_text: name_text.into(),
                authored_identity: Some(authored_identity.into()),
                body_atoms: body_atoms.into_iter().collect(),
            });
        self
    }

    pub fn with_surface(mut self, name_text: impl Into<String>) -> Self {
        self.declarations
            .push(WorthUiRustAuthoredDeclaration::Surface {
                name_text: name_text.into(),
                authored_identity: None,
                body_atoms: Vec::new(),
            });
        self
    }

    pub fn with_surface_body_atoms(
        mut self,
        name_text: impl Into<String>,
        body_atoms: impl IntoIterator<Item = WorthUiArtifactInputBodyAtom>,
    ) -> Self {
        self.declarations
            .push(WorthUiRustAuthoredDeclaration::Surface {
                name_text: name_text.into(),
                authored_identity: None,
                body_atoms: body_atoms.into_iter().collect(),
            });
        self
    }

    pub fn with_binding(mut self, name_text: impl Into<String>) -> Self {
        self.declarations
            .push(WorthUiRustAuthoredDeclaration::Binding {
                name_text: name_text.into(),
                authored_identity: None,
                body_atoms: Vec::new(),
            });
        self
    }

    pub fn with_binding_authored_identity(
        mut self,
        name_text: impl Into<String>,
        authored_identity: impl Into<String>,
    ) -> Self {
        self.declarations
            .push(WorthUiRustAuthoredDeclaration::Binding {
                name_text: name_text.into(),
                authored_identity: Some(authored_identity.into()),
                body_atoms: Vec::new(),
            });
        self
    }

    pub fn with_binding_body_atoms(
        mut self,
        name_text: impl Into<String>,
        body_atoms: impl IntoIterator<Item = WorthUiArtifactInputBodyAtom>,
    ) -> Self {
        self.declarations
            .push(WorthUiRustAuthoredDeclaration::Binding {
                name_text: name_text.into(),
                authored_identity: None,
                body_atoms: body_atoms.into_iter().collect(),
            });
        self
    }

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
            .push(WorthUiRustAuthoredDeclaration::QueryScalar {
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
            .push(WorthUiRustAuthoredDeclaration::QueryCollection {
                name_text: requirement.declaration_identity().to_owned(),
                body_atoms: projection_body_atoms(&requirement),
            });
        Ok(self)
    }

    pub fn with_token(
        mut self,
        name_text: impl Into<String>,
        value_text: impl Into<String>,
    ) -> Self {
        self.declarations
            .push(WorthUiRustAuthoredDeclaration::Token {
                name_text: name_text.into(),
                authored_identity: None,
                value_text: value_text.into(),
            });
        self
    }

    pub fn with_surface_authored_identity(
        mut self,
        name_text: impl Into<String>,
        authored_identity: impl Into<String>,
    ) -> Self {
        self.declarations
            .push(WorthUiRustAuthoredDeclaration::Surface {
                name_text: name_text.into(),
                authored_identity: Some(authored_identity.into()),
                body_atoms: Vec::new(),
            });
        self
    }

    pub fn with_token_authored_identity(
        mut self,
        name_text: impl Into<String>,
        authored_identity: impl Into<String>,
        value_text: impl Into<String>,
    ) -> Self {
        self.declarations
            .push(WorthUiRustAuthoredDeclaration::Token {
                name_text: name_text.into(),
                authored_identity: Some(authored_identity.into()),
                value_text: value_text.into(),
            });
        self
    }

    pub fn with_semantic_declaration(
        mut self,
        declaration: WorthUiSemanticArtifactDeclaration,
    ) -> Self {
        self.declarations
            .push(WorthUiRustAuthoredDeclaration::SemanticArtifact(
                declaration,
            ));
        self
    }

    pub(crate) fn relative_module_path(&self) -> &str {
        &self.relative_module_path
    }

    pub(crate) fn declarations(&self) -> &[WorthUiRustAuthoredDeclaration] {
        &self.declarations
    }

    pub(crate) fn source_revision_digest(&self) -> u64 {
        revision::module_digest(&self.relative_module_path, &self.declarations)
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

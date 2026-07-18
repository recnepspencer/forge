use crate::source::WorthUiArtifactInputBodyAtom;

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
    Token {
        name_text: String,
        authored_identity: Option<String>,
        value_text: String,
    },
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

    pub(crate) fn relative_module_path(&self) -> &str {
        &self.relative_module_path
    }

    pub(crate) fn declarations(&self) -> &[WorthUiRustAuthoredDeclaration] {
        &self.declarations
    }

    pub(crate) fn source_revision_digest(&self) -> u64 {
        let mut digest = 0xcbf2_9ce4_8422_2325u64;
        fold_text(&mut digest, "worth-ui:rust-authored-module:v1");
        fold_text(&mut digest, &self.relative_module_path);
        fold_u64(&mut digest, self.declarations.len() as u64);
        for declaration in &self.declarations {
            declaration.fold_source_revision(&mut digest);
        }
        digest
    }
}

impl WorthUiRustAuthoredDeclaration {
    fn fold_source_revision(&self, digest: &mut u64) {
        match self {
            Self::Import { target_module_path } => {
                fold_text(digest, "import");
                fold_text(digest, target_module_path);
            }
            Self::Component {
                name_text,
                authored_identity,
                body_atoms,
            } => fold_block_revision(
                digest,
                "component",
                name_text,
                authored_identity.as_deref(),
                body_atoms,
            ),
            Self::Surface {
                name_text,
                authored_identity,
                body_atoms,
            } => fold_block_revision(
                digest,
                "surface",
                name_text,
                authored_identity.as_deref(),
                body_atoms,
            ),
            Self::Binding {
                name_text,
                authored_identity,
                body_atoms,
            } => fold_block_revision(
                digest,
                "binding",
                name_text,
                authored_identity.as_deref(),
                body_atoms,
            ),
            Self::Token {
                name_text,
                authored_identity,
                value_text,
            } => {
                fold_text(digest, "token");
                fold_text(digest, name_text);
                fold_optional_text(digest, authored_identity.as_deref());
                fold_text(digest, value_text);
            }
        }
    }
}

fn fold_block_revision(
    digest: &mut u64,
    kind: &str,
    name_text: &str,
    authored_identity: Option<&str>,
    body_atoms: &[WorthUiArtifactInputBodyAtom],
) {
    fold_text(digest, kind);
    fold_text(digest, name_text);
    fold_optional_text(digest, authored_identity);
    fold_u64(digest, body_atoms.len() as u64);
    for atom in body_atoms {
        match atom {
            WorthUiArtifactInputBodyAtom::Identifier(value) => {
                fold_text(digest, "identifier");
                fold_text(digest, value);
            }
            WorthUiArtifactInputBodyAtom::StringLiteral(value) => {
                fold_text(digest, "string-literal");
                fold_text(digest, value);
            }
            WorthUiArtifactInputBodyAtom::KeywordImport => fold_text(digest, "keyword-import"),
            WorthUiArtifactInputBodyAtom::KeywordComponent => {
                fold_text(digest, "keyword-component")
            }
            WorthUiArtifactInputBodyAtom::KeywordSurface => fold_text(digest, "keyword-surface"),
            WorthUiArtifactInputBodyAtom::KeywordBinding => fold_text(digest, "keyword-binding"),
            WorthUiArtifactInputBodyAtom::KeywordToken => fold_text(digest, "keyword-token"),
            WorthUiArtifactInputBodyAtom::LeftBrace => fold_text(digest, "left-brace"),
            WorthUiArtifactInputBodyAtom::RightBrace => fold_text(digest, "right-brace"),
            WorthUiArtifactInputBodyAtom::Semicolon => fold_text(digest, "semicolon"),
            WorthUiArtifactInputBodyAtom::Equals => fold_text(digest, "equals"),
        }
    }
}

fn fold_optional_text(digest: &mut u64, value: Option<&str>) {
    match value {
        Some(value) => {
            fold_text(digest, "some");
            fold_text(digest, value);
        }
        None => fold_text(digest, "none"),
    }
}

fn fold_text(digest: &mut u64, text: &str) {
    fold_u64(digest, text.len() as u64);
    for byte in text.as_bytes() {
        *digest ^= u64::from(*byte);
        *digest = digest.wrapping_mul(0x100_0000_01b3);
    }
}

fn fold_u64(digest: &mut u64, value: u64) {
    for byte in value.to_le_bytes() {
        *digest ^= u64::from(byte);
        *digest = digest.wrapping_mul(0x100_0000_01b3);
    }
}

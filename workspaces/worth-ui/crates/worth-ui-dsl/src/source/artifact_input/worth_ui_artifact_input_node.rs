use crate::source::{
    WorthUiArtifactInputProvenance, WorthUiArtifactInputReference,
    WorthUiSemanticArtifactDeclaration,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum WorthUiArtifactInputNodeKind {
    Import,
    Component,
    Surface,
    Binding,
    QueryScalar,
    QueryCollection,
    Token,
    SemanticArtifact,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum WorthUiArtifactInputBodyAtom {
    Identifier(String),
    StringLiteral(String),
    KeywordImport,
    KeywordComponent,
    KeywordControl,
    KeywordIntent,
    KeywordSurface,
    KeywordBinding,
    KeywordQueryScalar,
    KeywordQueryCollection,
    KeywordToken,
    LeftBrace,
    RightBrace,
    Semicolon,
    Equals,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WorthUiArtifactInputNode {
    Import(WorthUiArtifactInputImportNode),
    Component(WorthUiArtifactInputBlockNode),
    Surface(WorthUiArtifactInputBlockNode),
    Binding(WorthUiArtifactInputBlockNode),
    QueryScalar(WorthUiArtifactInputBlockNode),
    QueryCollection(WorthUiArtifactInputBlockNode),
    Token(WorthUiArtifactInputTokenNode),
    SemanticArtifact(WorthUiArtifactInputSemanticArtifactNode),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorthUiArtifactInputImportNode {
    target: WorthUiArtifactInputReference,
    provenance: WorthUiArtifactInputProvenance,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorthUiArtifactInputBlockNode {
    name_text: String,
    authored_identity: Option<String>,
    body_atoms: Vec<WorthUiArtifactInputBodyAtom>,
    provenance: WorthUiArtifactInputProvenance,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorthUiArtifactInputTokenNode {
    name_text: String,
    authored_identity: Option<String>,
    value_text: String,
    provenance: WorthUiArtifactInputProvenance,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorthUiArtifactInputSemanticArtifactNode {
    declaration: WorthUiSemanticArtifactDeclaration,
    provenance: WorthUiArtifactInputProvenance,
}

impl WorthUiArtifactInputNode {
    pub fn kind(&self) -> WorthUiArtifactInputNodeKind {
        match self {
            Self::Import(_) => WorthUiArtifactInputNodeKind::Import,
            Self::Component(_) => WorthUiArtifactInputNodeKind::Component,
            Self::Surface(_) => WorthUiArtifactInputNodeKind::Surface,
            Self::Binding(_) => WorthUiArtifactInputNodeKind::Binding,
            Self::QueryScalar(_) => WorthUiArtifactInputNodeKind::QueryScalar,
            Self::QueryCollection(_) => WorthUiArtifactInputNodeKind::QueryCollection,
            Self::Token(_) => WorthUiArtifactInputNodeKind::Token,
            Self::SemanticArtifact(_) => WorthUiArtifactInputNodeKind::SemanticArtifact,
        }
    }

    #[cfg(test)]
    pub(crate) fn provenance(&self) -> &WorthUiArtifactInputProvenance {
        match self {
            Self::Import(node) => node.provenance(),
            Self::Component(node)
            | Self::Surface(node)
            | Self::Binding(node)
            | Self::QueryScalar(node)
            | Self::QueryCollection(node) => node.provenance(),
            Self::Token(node) => node.provenance(),
            Self::SemanticArtifact(node) => node.provenance(),
        }
    }
}

impl WorthUiArtifactInputSemanticArtifactNode {
    pub(crate) fn new(
        declaration: WorthUiSemanticArtifactDeclaration,
        provenance: WorthUiArtifactInputProvenance,
    ) -> Self {
        Self {
            declaration,
            provenance,
        }
    }

    pub fn declaration(&self) -> &WorthUiSemanticArtifactDeclaration {
        &self.declaration
    }

    pub fn provenance(&self) -> &WorthUiArtifactInputProvenance {
        &self.provenance
    }
}

impl WorthUiArtifactInputImportNode {
    pub(crate) fn new(
        target: WorthUiArtifactInputReference,
        provenance: WorthUiArtifactInputProvenance,
    ) -> Self {
        Self { target, provenance }
    }

    pub fn target(&self) -> &WorthUiArtifactInputReference {
        &self.target
    }

    pub fn provenance(&self) -> &WorthUiArtifactInputProvenance {
        &self.provenance
    }
}

impl WorthUiArtifactInputBlockNode {
    pub(crate) fn new(
        name_text: impl Into<String>,
        authored_identity: Option<String>,
        body_atoms: Vec<WorthUiArtifactInputBodyAtom>,
        provenance: WorthUiArtifactInputProvenance,
    ) -> Self {
        Self {
            name_text: name_text.into(),
            authored_identity,
            body_atoms,
            provenance,
        }
    }

    pub fn name_text(&self) -> &str {
        &self.name_text
    }

    pub fn body_atoms(&self) -> &[WorthUiArtifactInputBodyAtom] {
        &self.body_atoms
    }

    pub fn authored_identity(&self) -> Option<&str> {
        self.authored_identity.as_deref()
    }

    pub fn provenance(&self) -> &WorthUiArtifactInputProvenance {
        &self.provenance
    }
}

impl WorthUiArtifactInputTokenNode {
    pub(crate) fn new(
        name_text: impl Into<String>,
        authored_identity: Option<String>,
        value_text: impl Into<String>,
        provenance: WorthUiArtifactInputProvenance,
    ) -> Self {
        Self {
            name_text: name_text.into(),
            authored_identity,
            value_text: value_text.into(),
            provenance,
        }
    }

    pub fn name_text(&self) -> &str {
        &self.name_text
    }

    pub fn value_text(&self) -> &str {
        &self.value_text
    }

    pub fn authored_identity(&self) -> Option<&str> {
        self.authored_identity.as_deref()
    }

    pub fn provenance(&self) -> &WorthUiArtifactInputProvenance {
        &self.provenance
    }
}

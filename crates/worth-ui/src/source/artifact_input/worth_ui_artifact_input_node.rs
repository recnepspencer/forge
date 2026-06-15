use crate::source::{WorthUiArtifactInputProvenance, WorthUiArtifactInputReference};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum WorthUiArtifactInputNodeKind {
    Import,
    Component,
    Surface,
    Binding,
    Token,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum WorthUiArtifactInputBodyAtom {
    Identifier(String),
    NumberLiteral(u32),
    StringLiteral(String),
    KeywordImport,
    KeywordApp,
    KeywordWorkspace,
    KeywordPage,
    KeywordRuntime,
    KeywordLayout,
    KeywordContent,
    KeywordAppearance,
    KeywordComponent,
    KeywordSurface,
    KeywordBinding,
    KeywordToken,
    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    Comma,
    Colon,
    Semicolon,
    Equals,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum WorthUiArtifactInputNode {
    Import(WorthUiArtifactInputImportNode),
    Component(WorthUiArtifactInputBlockNode),
    Surface(WorthUiArtifactInputBlockNode),
    Binding(WorthUiArtifactInputBlockNode),
    Token(WorthUiArtifactInputTokenNode),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiArtifactInputImportNode {
    target: WorthUiArtifactInputReference,
    provenance: WorthUiArtifactInputProvenance,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiArtifactInputBlockNode {
    name_text: String,
    authored_identity: Option<String>,
    body_atoms: Vec<WorthUiArtifactInputBodyAtom>,
    provenance: WorthUiArtifactInputProvenance,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiArtifactInputTokenNode {
    name_text: String,
    authored_identity: Option<String>,
    value_text: String,
    provenance: WorthUiArtifactInputProvenance,
}

impl WorthUiArtifactInputNode {
    pub(crate) fn kind(&self) -> WorthUiArtifactInputNodeKind {
        match self {
            Self::Import(_) => WorthUiArtifactInputNodeKind::Import,
            Self::Component(_) => WorthUiArtifactInputNodeKind::Component,
            Self::Surface(_) => WorthUiArtifactInputNodeKind::Surface,
            Self::Binding(_) => WorthUiArtifactInputNodeKind::Binding,
            Self::Token(_) => WorthUiArtifactInputNodeKind::Token,
        }
    }

    pub(crate) fn provenance(&self) -> &WorthUiArtifactInputProvenance {
        match self {
            Self::Import(node) => node.provenance(),
            Self::Component(node) | Self::Surface(node) | Self::Binding(node) => node.provenance(),
            Self::Token(node) => node.provenance(),
        }
    }
}

impl WorthUiArtifactInputImportNode {
    pub(crate) fn new(
        target: WorthUiArtifactInputReference,
        provenance: WorthUiArtifactInputProvenance,
    ) -> Self {
        Self { target, provenance }
    }

    pub(crate) fn target(&self) -> &WorthUiArtifactInputReference {
        &self.target
    }

    pub(crate) fn provenance(&self) -> &WorthUiArtifactInputProvenance {
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

    pub(crate) fn name_text(&self) -> &str {
        &self.name_text
    }

    pub(crate) fn body_atoms(&self) -> &[WorthUiArtifactInputBodyAtom] {
        &self.body_atoms
    }

    pub(crate) fn authored_identity(&self) -> Option<&str> {
        self.authored_identity.as_deref()
    }

    pub(crate) fn provenance(&self) -> &WorthUiArtifactInputProvenance {
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

    pub(crate) fn name_text(&self) -> &str {
        &self.name_text
    }

    pub(crate) fn value_text(&self) -> &str {
        &self.value_text
    }

    pub(crate) fn authored_identity(&self) -> Option<&str> {
        self.authored_identity.as_deref()
    }

    pub(crate) fn provenance(&self) -> &WorthUiArtifactInputProvenance {
        &self.provenance
    }
}

use crate::source::WorthUiSourceSpan;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum WorthUiSourceTokenKind {
    Identifier(String),
    StringLiteral(String),
    KeywordImport,
    KeywordComponent,
    KeywordSurface,
    KeywordBinding,
    KeywordToken,
    LeftBrace,
    RightBrace,
    Semicolon,
    Equals,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiSourceToken {
    kind: WorthUiSourceTokenKind,
    span: WorthUiSourceSpan,
}

impl WorthUiSourceToken {
    pub(crate) fn new(kind: WorthUiSourceTokenKind, span: WorthUiSourceSpan) -> Self {
        Self { kind, span }
    }

    pub(crate) fn kind(&self) -> &WorthUiSourceTokenKind {
        &self.kind
    }

    pub(crate) fn span(&self) -> &WorthUiSourceSpan {
        &self.span
    }
}

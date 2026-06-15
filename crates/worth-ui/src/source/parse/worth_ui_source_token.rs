use crate::source::WorthUiSourceSpan;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum WorthUiSourceTokenKind {
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

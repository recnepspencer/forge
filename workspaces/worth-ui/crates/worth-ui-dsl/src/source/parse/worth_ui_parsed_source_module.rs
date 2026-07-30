use crate::source::{WorthUiSourceModuleId, WorthUiSourceSpan, WorthUiSourceTokenKind};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiParsedSourceModule {
    module_id: WorthUiSourceModuleId,
    declarations: Vec<WorthUiParsedSourceDeclaration>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum WorthUiParsedSourceDeclaration {
    Import(WorthUiParsedImportDeclaration),
    Component(WorthUiParsedBlockDeclaration),
    Surface(WorthUiParsedBlockDeclaration),
    Binding(WorthUiParsedBlockDeclaration),
    QueryScalar(WorthUiParsedBlockDeclaration),
    QueryCollection(WorthUiParsedBlockDeclaration),
    Token(WorthUiParsedTokenDeclaration),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiParsedImportDeclaration {
    path_text: String,
    span: WorthUiSourceSpan,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiParsedBlockDeclaration {
    name_text: String,
    span: WorthUiSourceSpan,
    body: WorthUiParsedBlockBody,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiParsedBlockBody {
    span: WorthUiSourceSpan,
    tokens: Vec<WorthUiSourceTokenKind>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiParsedTokenDeclaration {
    name_text: String,
    value_text: String,
    span: WorthUiSourceSpan,
    value_span: WorthUiSourceSpan,
}

impl WorthUiParsedSourceModule {
    pub(crate) fn new(
        module_id: WorthUiSourceModuleId,
        declarations: Vec<WorthUiParsedSourceDeclaration>,
    ) -> Self {
        Self {
            module_id,
            declarations,
        }
    }

    pub(crate) fn declarations(&self) -> &[WorthUiParsedSourceDeclaration] {
        &self.declarations
    }
}

impl WorthUiParsedImportDeclaration {
    pub(crate) fn new(path_text: String, span: WorthUiSourceSpan) -> Self {
        Self { path_text, span }
    }

    pub(crate) fn path_text(&self) -> &str {
        &self.path_text
    }

    pub(crate) fn span(&self) -> &WorthUiSourceSpan {
        &self.span
    }
}

impl WorthUiParsedBlockDeclaration {
    pub(crate) fn new(
        name_text: String,
        span: WorthUiSourceSpan,
        body: WorthUiParsedBlockBody,
    ) -> Self {
        Self {
            name_text,
            span,
            body,
        }
    }

    pub(crate) fn name_text(&self) -> &str {
        &self.name_text
    }

    pub(crate) fn span(&self) -> &WorthUiSourceSpan {
        &self.span
    }

    pub(crate) fn body(&self) -> &WorthUiParsedBlockBody {
        &self.body
    }
}

impl WorthUiParsedBlockBody {
    pub(crate) fn new(span: WorthUiSourceSpan, tokens: Vec<WorthUiSourceTokenKind>) -> Self {
        Self { span, tokens }
    }

    pub(crate) fn tokens(&self) -> &[WorthUiSourceTokenKind] {
        &self.tokens
    }
}

impl WorthUiParsedTokenDeclaration {
    pub(crate) fn new(
        name_text: String,
        value_text: String,
        span: WorthUiSourceSpan,
        value_span: WorthUiSourceSpan,
    ) -> Self {
        Self {
            name_text,
            value_text,
            span,
            value_span,
        }
    }

    pub(crate) fn name_text(&self) -> &str {
        &self.name_text
    }

    pub(crate) fn value_text(&self) -> &str {
        &self.value_text
    }

    pub(crate) fn span(&self) -> &WorthUiSourceSpan {
        &self.span
    }

    pub(crate) fn value_span(&self) -> &WorthUiSourceSpan {
        &self.value_span
    }
}

impl WorthUiParsedSourceModule {
    #[cfg(test)]
    pub(crate) fn equivalent_shape(&self, other: &Self) -> bool {
        self.module_id == other.module_id
            && self.declarations.len() == other.declarations.len()
            && self
                .declarations
                .iter()
                .zip(other.declarations.iter())
                .all(|(left, right)| left.equivalent_shape(right))
    }
}

impl WorthUiParsedSourceDeclaration {
    #[cfg(test)]
    pub(crate) fn equivalent_shape(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Import(left), Self::Import(right)) => left.path_text == right.path_text,
            (Self::Component(left), Self::Component(right))
            | (Self::Surface(left), Self::Surface(right))
            | (Self::Binding(left), Self::Binding(right))
            | (Self::QueryScalar(left), Self::QueryScalar(right))
            | (Self::QueryCollection(left), Self::QueryCollection(right)) => {
                left.name_text == right.name_text && left.body.tokens == right.body.tokens
            }
            (Self::Token(left), Self::Token(right)) => {
                left.name_text == right.name_text && left.value_text == right.value_text
            }
            _ => false,
        }
    }
}

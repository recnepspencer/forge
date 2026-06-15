use crate::source::{WorthUiSourceModuleId, WorthUiSourceSpan};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiParsedSourceModule {
    module_id: WorthUiSourceModuleId,
    declarations: Vec<WorthUiParsedSourceDeclaration>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum WorthUiParsedSourceDeclaration {
    Import(WorthUiParsedImportDeclaration),
    App(WorthUiParsedAuthoringDeclaration),
    Workspace(WorthUiParsedAuthoringDeclaration),
    Page(WorthUiParsedPageDeclaration),
    Runtime(WorthUiParsedAuthoringDeclaration),
    Layout(WorthUiParsedAuthoringDeclaration),
    Content(WorthUiParsedAuthoringDeclaration),
    Appearance(WorthUiParsedAuthoringDeclaration),
    Component(WorthUiParsedBlockDeclaration),
    Surface(WorthUiParsedBlockDeclaration),
    Binding(WorthUiParsedBlockDeclaration),
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
    tokens: Vec<crate::source::WorthUiSourceToken>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiParsedAuthoringDeclaration {
    name_text: String,
    span: WorthUiSourceSpan,
    body: WorthUiParsedBlockBody,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiParsedPageDeclaration {
    name_text: String,
    template_parameters: Vec<WorthUiParsedTemplateParameter>,
    span: WorthUiSourceSpan,
    body: WorthUiParsedBlockBody,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiParsedTemplateParameter {
    name_text: String,
    type_text: String,
    span: WorthUiSourceSpan,
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

    pub(crate) fn module_id(&self) -> &WorthUiSourceModuleId {
        &self.module_id
    }

    pub(crate) fn declarations(&self) -> &[WorthUiParsedSourceDeclaration] {
        &self.declarations
    }

    pub(crate) fn spans(&self) -> Vec<&WorthUiSourceSpan> {
        self.declarations
            .iter()
            .map(WorthUiParsedSourceDeclaration::span)
            .collect()
    }
}

impl WorthUiParsedSourceDeclaration {
    pub(crate) fn span(&self) -> &WorthUiSourceSpan {
        match self {
            Self::Import(declaration) => declaration.span(),
            Self::App(declaration)
            | Self::Workspace(declaration)
            | Self::Runtime(declaration)
            | Self::Layout(declaration)
            | Self::Content(declaration)
            | Self::Appearance(declaration) => declaration.span(),
            Self::Page(declaration) => declaration.span(),
            Self::Component(declaration)
            | Self::Surface(declaration)
            | Self::Binding(declaration) => declaration.span(),
            Self::Token(declaration) => declaration.span(),
        }
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
    pub(crate) fn new(
        span: WorthUiSourceSpan,
        tokens: Vec<crate::source::WorthUiSourceToken>,
    ) -> Self {
        Self { span, tokens }
    }

    pub(crate) fn span(&self) -> &WorthUiSourceSpan {
        &self.span
    }

    pub(crate) fn tokens(&self) -> &[crate::source::WorthUiSourceToken] {
        &self.tokens
    }
}

impl WorthUiParsedAuthoringDeclaration {
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

impl WorthUiParsedPageDeclaration {
    pub(crate) fn new(
        name_text: String,
        template_parameters: Vec<WorthUiParsedTemplateParameter>,
        span: WorthUiSourceSpan,
        body: WorthUiParsedBlockBody,
    ) -> Self {
        Self {
            name_text,
            template_parameters,
            span,
            body,
        }
    }

    pub(crate) fn name_text(&self) -> &str {
        &self.name_text
    }

    pub(crate) fn template_parameters(&self) -> &[WorthUiParsedTemplateParameter] {
        &self.template_parameters
    }

    pub(crate) fn span(&self) -> &WorthUiSourceSpan {
        &self.span
    }

    pub(crate) fn body(&self) -> &WorthUiParsedBlockBody {
        &self.body
    }
}

impl WorthUiParsedTemplateParameter {
    pub(crate) fn new(name_text: String, type_text: String, span: WorthUiSourceSpan) -> Self {
        Self {
            name_text,
            type_text,
            span,
        }
    }

    pub(crate) fn name_text(&self) -> &str {
        &self.name_text
    }

    pub(crate) fn type_text(&self) -> &str {
        &self.type_text
    }

    pub(crate) fn span(&self) -> &WorthUiSourceSpan {
        &self.span
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
    pub(crate) fn equivalent_shape(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Import(left), Self::Import(right)) => left.path_text == right.path_text,
            (Self::App(left), Self::App(right))
            | (Self::Workspace(left), Self::Workspace(right))
            | (Self::Runtime(left), Self::Runtime(right))
            | (Self::Layout(left), Self::Layout(right))
            | (Self::Content(left), Self::Content(right))
            | (Self::Appearance(left), Self::Appearance(right)) => {
                left.name_text == right.name_text
                    && equivalent_body_tokens(left.body.tokens(), right.body.tokens())
            }
            (Self::Page(left), Self::Page(right)) => {
                left.name_text == right.name_text
                    && equivalent_template_parameters(
                        left.template_parameters(),
                        right.template_parameters(),
                    )
                    && equivalent_body_tokens(left.body.tokens(), right.body.tokens())
            }
            (Self::Component(left), Self::Component(right))
            | (Self::Surface(left), Self::Surface(right))
            | (Self::Binding(left), Self::Binding(right)) => {
                left.name_text == right.name_text
                    && equivalent_body_tokens(left.body.tokens(), right.body.tokens())
            }
            (Self::Token(left), Self::Token(right)) => {
                left.name_text == right.name_text && left.value_text == right.value_text
            }
            _ => false,
        }
    }
}

fn equivalent_body_tokens(
    left: &[crate::source::WorthUiSourceToken],
    right: &[crate::source::WorthUiSourceToken],
) -> bool {
    left.len() == right.len()
        && left
            .iter()
            .zip(right.iter())
            .all(|(left, right)| left.kind() == right.kind())
}

fn equivalent_template_parameters(
    left: &[WorthUiParsedTemplateParameter],
    right: &[WorthUiParsedTemplateParameter],
) -> bool {
    left.len() == right.len()
        && left.iter().zip(right.iter()).all(|(left, right)| {
            left.name_text() == right.name_text() && left.type_text() == right.type_text()
        })
}

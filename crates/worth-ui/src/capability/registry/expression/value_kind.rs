#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiExpressionInputKind {
    BindingReference,
    BindingSet,
    TextLiteral,
    BooleanExpression,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiExpressionOutputKind {
    Boolean,
    PayloadObject,
    Text,
}

impl WorthUiExpressionOutputKind {
    pub const fn token(self) -> &'static str {
        match self {
            Self::Boolean => "boolean",
            Self::PayloadObject => "payload_object",
            Self::Text => "text",
        }
    }
}

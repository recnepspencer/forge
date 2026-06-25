use crate::capability::WorthUiExpressionOperatorId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiLiveViewExpressionInput {
    BindingReference(String),
    BindingSet(Vec<String>),
    TextLiteral(String),
    NestedExpression(Box<WorthUiLiveViewExpressionDeclaration>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiLiveViewExpressionDeclaration {
    expression_id: String,
    operator_id: WorthUiExpressionOperatorId,
    inputs: Vec<WorthUiLiveViewExpressionInput>,
}

impl WorthUiLiveViewExpressionDeclaration {
    pub(crate) fn new(
        expression_id: impl Into<String>,
        operator_id: WorthUiExpressionOperatorId,
        inputs: Vec<WorthUiLiveViewExpressionInput>,
    ) -> Self {
        Self {
            expression_id: expression_id.into(),
            operator_id,
            inputs,
        }
    }

    pub fn expression_id(&self) -> &str {
        &self.expression_id
    }

    pub fn operator_id(&self) -> WorthUiExpressionOperatorId {
        self.operator_id
    }

    pub fn inputs(&self) -> &[WorthUiLiveViewExpressionInput] {
        &self.inputs
    }
}

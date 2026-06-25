use crate::capability::{WorthUiExpressionOperatorDescriptor, WorthUiExpressionOutputKind};
use crate::runtime::live_view::digest::digest_parts;
use crate::runtime::{WorthUiQueryGraphExecutionReceipt, WorthUiRuntimeFactId};

use super::WorthUiLiveViewExpressionDeclaration;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiLiveViewExpressionOutputValue {
    Boolean(bool),
    PayloadShape(String),
    Text(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiLiveViewExpressionOutputReceipt {
    value: WorthUiLiveViewExpressionOutputValue,
    output_kind: WorthUiExpressionOutputKind,
    output_digest: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiLiveViewExpressionProjectionReceipt {
    live_view_id: String,
    expression_id: String,
    operator: WorthUiExpressionOperatorDescriptor,
    declaration_fact: WorthUiRuntimeFactId,
    projection_fact: WorthUiRuntimeFactId,
    output_fact: WorthUiRuntimeFactId,
    consumed_facts: Vec<WorthUiRuntimeFactId>,
    output: WorthUiLiveViewExpressionOutputReceipt,
    graph_execution: WorthUiQueryGraphExecutionReceipt,
    expression_digest: u64,
}

impl WorthUiLiveViewExpressionOutputReceipt {
    pub(crate) fn new(
        value: WorthUiLiveViewExpressionOutputValue,
        output_kind: WorthUiExpressionOutputKind,
    ) -> Self {
        let output_digest = digest_parts([output_kind.token(), value.digest_token().as_str()]);
        Self {
            value,
            output_kind,
            output_digest,
        }
    }

    pub fn value(&self) -> &WorthUiLiveViewExpressionOutputValue {
        &self.value
    }

    pub fn output_kind(&self) -> WorthUiExpressionOutputKind {
        self.output_kind
    }

    pub fn output_digest(&self) -> u64 {
        self.output_digest
    }
}

impl WorthUiLiveViewExpressionProjectionReceipt {
    pub(crate) fn new(
        live_view_id: &str,
        declaration: &WorthUiLiveViewExpressionDeclaration,
        operator: WorthUiExpressionOperatorDescriptor,
        consumed_facts: Vec<WorthUiRuntimeFactId>,
        output: WorthUiLiveViewExpressionOutputReceipt,
        graph_execution: WorthUiQueryGraphExecutionReceipt,
    ) -> Self {
        let expression_identity = format!("{}:{}", live_view_id, declaration.expression_id());
        let declaration_fact =
            WorthUiRuntimeFactId::live_view_expression_declaration(expression_identity.clone());
        let projection_fact =
            WorthUiRuntimeFactId::live_view_expression_projection(expression_identity.clone());
        let output_fact = WorthUiRuntimeFactId::live_view_expression_output(expression_identity);
        let expression_digest = digest_parts([
            live_view_id,
            declaration.expression_id(),
            declaration.operator_id().as_str(),
            operator.descriptor_digest().to_string().as_str(),
            output_fact.identity(),
            output.output_digest().to_string().as_str(),
        ]);
        Self {
            live_view_id: live_view_id.to_owned(),
            expression_id: declaration.expression_id().to_owned(),
            operator,
            declaration_fact,
            projection_fact,
            output_fact,
            consumed_facts,
            output,
            graph_execution,
            expression_digest,
        }
    }

    pub fn live_view_id(&self) -> &str {
        &self.live_view_id
    }

    pub fn expression_id(&self) -> &str {
        &self.expression_id
    }

    pub fn operator(&self) -> &WorthUiExpressionOperatorDescriptor {
        &self.operator
    }

    pub fn declaration_fact(&self) -> &WorthUiRuntimeFactId {
        &self.declaration_fact
    }

    pub fn projection_fact(&self) -> &WorthUiRuntimeFactId {
        &self.projection_fact
    }

    pub fn output_fact(&self) -> &WorthUiRuntimeFactId {
        &self.output_fact
    }

    pub fn consumed_facts(&self) -> &[WorthUiRuntimeFactId] {
        &self.consumed_facts
    }

    pub fn output(&self) -> &WorthUiLiveViewExpressionOutputReceipt {
        &self.output
    }

    pub fn query_graph_execution(&self) -> &WorthUiQueryGraphExecutionReceipt {
        &self.graph_execution
    }

    pub fn expression_digest(&self) -> u64 {
        self.expression_digest
    }
}

impl WorthUiLiveViewExpressionOutputValue {
    fn digest_token(&self) -> String {
        match self {
            Self::Boolean(value) => format!("boolean:{value}"),
            Self::PayloadShape(value) => format!("payload:{value}"),
            Self::Text(value) => format!("text:{value}"),
        }
    }
}

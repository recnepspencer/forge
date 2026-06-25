use super::super::expression::{
    WorthUiLiveViewExpressionOutputValue, WorthUiLiveViewExpressionProjectionReceipt,
};
use crate::runtime::live_view::digest::digest_parts;
use crate::runtime::{WorthUiQueryGraphExecutionReceipt, WorthUiRuntimeFactId};

use super::{WorthUiLiveViewPayloadProjectionDeclaration, WorthUiLiveViewPayloadShape};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiLiveViewPayloadProjectionReceipt {
    live_view_id: String,
    payload_id: String,
    shape: WorthUiLiveViewPayloadShape,
    consumed_facts: Vec<WorthUiRuntimeFactId>,
    expression_projection: WorthUiLiveViewExpressionProjectionReceipt,
    graph_execution: WorthUiQueryGraphExecutionReceipt,
    payload_projection_digest: u64,
}

impl WorthUiLiveViewPayloadProjectionReceipt {
    pub(crate) fn new(
        live_view_id: &str,
        declaration: &WorthUiLiveViewPayloadProjectionDeclaration,
        consumed_facts: Vec<WorthUiRuntimeFactId>,
        expression_projection: WorthUiLiveViewExpressionProjectionReceipt,
        graph_execution: WorthUiQueryGraphExecutionReceipt,
    ) -> Self {
        let shape = payload_shape_from_expression(&expression_projection);
        let payload_projection_digest = digest_parts(vec![
            live_view_id.to_owned(),
            declaration.payload_id().to_owned(),
            shape.token().to_owned(),
            expression_projection.expression_digest().to_string(),
        ]);
        Self {
            live_view_id: live_view_id.to_owned(),
            payload_id: declaration.payload_id().to_owned(),
            shape,
            consumed_facts,
            expression_projection,
            graph_execution,
            payload_projection_digest,
        }
    }

    pub fn live_view_id(&self) -> &str {
        &self.live_view_id
    }

    pub fn payload_id(&self) -> &str {
        &self.payload_id
    }

    pub fn shape(&self) -> &WorthUiLiveViewPayloadShape {
        &self.shape
    }

    pub fn consumed_facts(&self) -> &[WorthUiRuntimeFactId] {
        &self.consumed_facts
    }

    pub fn expression_projection(&self) -> &WorthUiLiveViewExpressionProjectionReceipt {
        &self.expression_projection
    }

    pub fn query_graph_execution(&self) -> &WorthUiQueryGraphExecutionReceipt {
        &self.graph_execution
    }

    pub fn payload_projection_digest(&self) -> u64 {
        self.payload_projection_digest
    }
}

fn payload_shape_from_expression(
    expression_projection: &WorthUiLiveViewExpressionProjectionReceipt,
) -> WorthUiLiveViewPayloadShape {
    match expression_projection.output().value() {
        WorthUiLiveViewExpressionOutputValue::PayloadShape(value) if value == "data" => {
            WorthUiLiveViewPayloadShape::DataPayloadValues
        }
        WorthUiLiveViewExpressionOutputValue::PayloadShape(value) if value == "payload" => {
            WorthUiLiveViewPayloadShape::PayloadValues
        }
        WorthUiLiveViewExpressionOutputValue::PayloadShape(value) => {
            WorthUiLiveViewPayloadShape::Unsupported(value.clone())
        }
        WorthUiLiveViewExpressionOutputValue::Boolean(_)
        | WorthUiLiveViewExpressionOutputValue::Text(_) => {
            WorthUiLiveViewPayloadShape::Unsupported("non_payload_expression".to_owned())
        }
    }
}
